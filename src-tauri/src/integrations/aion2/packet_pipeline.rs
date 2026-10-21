use super::StreamReset;
use super::StreamUpdate;
use super::capture::CapturedPacket;
use super::catalog::NpcIndex;
use super::network::PacketReader;
use super::network::StreamKey;
use super::network::TcpSegment;
use super::network::TcpStream;
use super::protocol::DecodeBudget;
use super::protocol::FrameBuffer;
use super::protocol::MessageDecoder;
use super::protocol::OpcodeTable;
use crate::core::game::GameCapabilities;
use crate::core::game::GameEvent;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub(crate) struct PacketPipeline {
    decoder: MessageDecoder,
    catalog: Arc<super::catalog::RuntimeCatalog>,
    capabilities: GameCapabilities,
    streams: HashMap<StreamKey, TcpStream>,
    framers: HashMap<StreamKey, FrameBuffer>,
    ignored: HashSet<StreamKey>,
    active: Option<StreamKey>,
    observed: Option<StreamKey>,
    network_errors: u64,
    stream_errors: u64,
    last_network_error: Option<String>,
    last_stream_error: Option<String>,
}

impl PacketPipeline {
    const MAX_STREAMS: usize = 128;
    const MAX_PACKET_EVENTS: usize = 16_384;

    pub(crate) fn new(
        table: OpcodeTable,
        npcs: NpcIndex,
        catalog: Arc<super::catalog::RuntimeCatalog>,
    ) -> Self {
        let capabilities = table.capabilities();

        Self {
            decoder: MessageDecoder::new(table, npcs, catalog.clone()),
            catalog,
            capabilities,
            streams: HashMap::new(),
            framers: HashMap::new(),
            ignored: HashSet::new(),
            active: None,
            observed: None,
            network_errors: 0,
            stream_errors: 0,
            last_network_error: None,
            last_stream_error: None,
        }
    }

    pub(crate) fn catalog(&self) -> &Arc<super::catalog::RuntimeCatalog> {
        &self.catalog
    }

    pub(crate) fn capabilities(&self) -> &GameCapabilities {
        &self.capabilities
    }

    pub(crate) fn active_stream(&self) -> Option<&StreamKey> {
        self.active.as_ref()
    }

    pub(crate) fn observed_stream(&self) -> Option<&StreamKey> {
        self.observed.as_ref()
    }

    pub(crate) fn resume_observed_stream(&mut self) {
        self.active = self.observed.clone();
    }

    pub(crate) fn decoder(&self) -> &MessageDecoder {
        &self.decoder
    }

    pub(crate) fn network_errors(&self) -> u64 {
        self.network_errors
    }

    pub(crate) fn stream_errors(&self) -> u64 {
        self.stream_errors
    }

    pub(crate) fn last_network_error(&self) -> Option<&str> {
        self.last_network_error.as_deref()
    }

    pub(crate) fn unfinished_bytes(&self) -> usize {
        self.framers.values().map(FrameBuffer::pending_bytes).sum()
    }

    pub(crate) fn last_stream_error(&self) -> Option<&str> {
        self.last_stream_error.as_deref()
    }

    pub(crate) fn reset(&mut self) {
        self.streams.clear();
        self.framers.clear();
        self.ignored.clear();
        self.active = None;
        self.observed = None;
    }

    pub(crate) fn push(&mut self, packet: &CapturedPacket, server_ports: &[u16]) -> StreamUpdate {
        let segment = match PacketReader::read(packet) {
            Ok(Some(segment)) => segment,
            Ok(None) => return StreamUpdate::new(None, Vec::new()),
            Err(error) => {
                self.network_errors += 1;
                self.last_network_error = Some(error);

                return StreamUpdate::new(None, Vec::new());
            }
        };

        if !server_ports.is_empty() && !server_ports.contains(&segment.source_port()) {
            return StreamUpdate::new(None, Vec::new());
        }

        let key = segment.key().clone();

        if segment.reset() {
            let events = if segment.finished() && !segment.payload().is_empty() {
                self.ingest(segment, packet.timestamp_us())
                    .unwrap_or_else(|error| {
                        self.stream_errors += 1;
                        self.last_stream_error = Some(error);

                        Vec::new()
                    })
            } else {
                Vec::new()
            };

            let reset = (self.active.as_ref() == Some(&key)).then_some(StreamReset::Closed);

            if reset.is_some() {
                self.reset();
            }

            self.drop_stream(&key);
            self.ignored.remove(&key);

            return StreamUpdate::new(reset, events);
        }

        match self.ingest(segment, packet.timestamp_us()) {
            Ok(events) => StreamUpdate::new(None, events),
            Err(error) => {
                let reset = (self.active.as_ref() == Some(&key))
                    .then(|| StreamReset::Interrupted(error.clone()));

                self.last_stream_error = Some(error);
                self.stream_errors += 1;

                if reset.is_some() {
                    self.reset();
                }

                self.drop_stream(&key);

                StreamUpdate::new(reset, Vec::new())
            }
        }
    }

    fn ingest(&mut self, segment: TcpSegment, at_us: u64) -> Result<Vec<GameEvent>, String> {
        let key = segment.key().clone();

        if segment.payload().is_empty() || self.ignored.contains(&key) {
            return Ok(Vec::new());
        }

        if !self.streams.contains_key(&key) {
            if self.streams.len() + self.ignored.len() >= Self::MAX_STREAMS {
                return Err("TCP stream count limit".into());
            }

            if segment.payload().len() >= 3
                && (20..=23).contains(&segment.payload()[0])
                && segment.payload()[1] == 3
            {
                self.ignored.insert(key);

                return Ok(Vec::new());
            }

            self.streams.insert(key.clone(), TcpStream::new());
            self.framers.insert(key.clone(), FrameBuffer::new());
        }

        let chunks = self
            .streams
            .get_mut(&key)
            .ok_or("Missing TCP stream")?
            .push(segment)?;
        let mut events = Vec::new();
        let mut budget = DecodeBudget::new();

        for chunk in chunks {
            let frames = self
                .framers
                .get_mut(&key)
                .ok_or("Missing frame buffer")?
                .push(&chunk)?;

            for frame in frames {
                let decoded = self.decoder.decode(&frame, &mut budget, at_us)?;

                if !decoded.is_empty() {
                    self.observed = Some(key.clone());

                    let character = decoded.iter().any(|event| {
                        matches!(
                            event,
                            GameEvent::Identity { own: true, .. }
                                | GameEvent::SelfIdentified {
                                    authoritative: true,
                                    ..
                                }
                                | GameEvent::SkillsLearned { .. }
                                | GameEvent::MapChanged { .. }
                        )
                    });

                    if character
                        || (self.active.is_none()
                            && decoded
                                .iter()
                                .any(|event| !matches!(event, GameEvent::Clock { .. })))
                    {
                        self.active = Some(key.clone());
                    }
                }

                if decoded.len() > Self::MAX_PACKET_EVENTS.saturating_sub(events.len()) {
                    return Err("Per-packet event count limit".into());
                }

                events.extend(decoded);
            }
        }

        Ok(events)
    }

    fn drop_stream(&mut self, key: &StreamKey) {
        self.streams.remove(key);
        self.framers.remove(key);
    }
}
