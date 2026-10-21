use super::CaptureProcess;
use super::CaptureRead;
use super::CaptureTransport;
use super::CapturedPacket;
use super::game_connection::GameConnection;
use super::game_process::GameProcess;
use super::npcap_api::NpcapApi;
use super::npcap_session::NpcapSession;
use crate::config::CaptureConfiguration;
use crate::core::game::SourceStatus;
use crate::integrations::aion2::network::PacketReader;
use crate::integrations::aion2::network::StreamKey;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub(crate) struct LiveTransport {
    configuration: CaptureConfiguration,
    api: Option<Rc<NpcapApi>>,
    connection: Option<GameConnection>,
    session: Option<NpcapSession>,
    refreshed: Option<Instant>,
    reset_pending: bool,
    unowned: HashSet<StreamKey>,
}

impl LiveTransport {
    // Bound rejected sockets between periodic ownership refreshes.
    const MAX_UNOWNED_STREAMS: usize = 256;
    // Foreign sockets must not make a queued game packet wait for the next worker tick.
    const MAX_FILTERED_PACKETS: usize = 128;

    pub(crate) fn new(configuration: CaptureConfiguration) -> Self {
        Self {
            configuration,
            api: None,
            connection: None,
            session: None,
            refreshed: None,
            reset_pending: false,
            unowned: HashSet::new(),
        }
    }

    fn refresh(&mut self) -> Result<(), String> {
        let connection = GameProcess::read(self.configuration.process_name())?;

        self.unowned.clear();

        let same_process = self
            .connection
            .as_ref()
            .zip(connection.as_ref())
            .is_some_and(|(previous, current)| previous.same_process(current));

        if !same_process && connection != self.connection {
            self.session = None;
            self.reset_pending = true;
        }

        self.connection = connection;

        if let Some(session) = &mut self.session {
            if let Some(connection) = &self.connection {
                session.synchronize(connection)?;
            }

            if session.lost_packets()? {
                self.reset_pending = true;
            }

            return Ok(());
        }

        let Some(connection) = &self.connection else {
            return Ok(());
        };

        if connection.endpoints().is_empty() {
            return Ok(());
        }

        let api = self.api.as_ref().ok_or("Npcap is not started")?.clone();

        self.session = Some(NpcapSession::open(api, connection)?);

        Ok(())
    }

    fn incoming(&mut self, packet: CapturedPacket) -> Result<Option<CapturedPacket>, String> {
        let Ok(Some(segment)) = PacketReader::read(&packet) else {
            return Ok(None);
        };

        let stream = segment.key();

        if self
            .connection
            .as_ref()
            .is_some_and(|connection| connection.accepts(stream))
        {
            return Ok(Some(packet));
        }

        if self.unowned.contains(stream) {
            return Ok(None);
        }

        // The first server payload can arrive before the scheduled endpoint refresh.
        // Validate the new socket now, while its initial packet is still available.
        let current = GameProcess::read(self.configuration.process_name())?;

        if self
            .connection
            .as_ref()
            .zip(current.as_ref())
            .is_some_and(|(previous, current)| previous.same_process(current))
        {
            self.connection = current;

            if self
                .connection
                .as_ref()
                .is_some_and(|connection| connection.accepts(stream))
            {
                return Ok(Some(packet));
            }
        }

        if self.unowned.len() < Self::MAX_UNOWNED_STREAMS {
            self.unowned.insert(stream.clone());
        }

        Ok(None)
    }
}

impl CaptureTransport for LiveTransport {
    fn start(&mut self) -> Result<(), String> {
        let windows = std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .ok_or("SystemRoot is unavailable")?;

        self.api = Some(Rc::new(NpcapApi::load(
            &windows.join(self.configuration.library()),
        )?));
        self.refreshed = None;

        Ok(())
    }

    fn poll(&mut self) -> Result<CaptureRead, String> {
        if self.refreshed.is_none_or(|last| {
            last.elapsed() >= Duration::from_millis(self.configuration.refresh_ms())
        }) {
            self.refreshed = Some(Instant::now());

            if let Err(error) = self.refresh() {
                self.session = None;

                return Err(error);
            }
        }

        if self.reset_pending {
            self.reset_pending = false;

            return Ok(CaptureRead::Reset);
        }

        for _ in 0..Self::MAX_FILTERED_PACKETS {
            let packet = match &mut self.session {
                Some(session) => match session.next() {
                    Ok(packet) => packet,
                    Err(error) => {
                        self.session = None;

                        return Err(error);
                    }
                },
                None => None,
            };

            let Some(packet) = packet else {
                break;
            };

            if let Some(packet) = self.incoming(packet)? {
                return Ok(CaptureRead::Packet(packet));
            }
        }

        Ok(CaptureRead::Pending)
    }

    fn now_us(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |time| time.as_micros().min(u128::from(u64::MAX)) as u64)
    }

    fn mode(&self) -> SourceStatus {
        SourceStatus::Live
    }

    fn server_ports(&self) -> &[u16] {
        &[]
    }

    fn stop(&mut self) {
        self.session = None;
        self.connection = None;
        self.api = None;
        self.refreshed = None;
        self.unowned.clear();
    }

    fn process(&self) -> Option<CaptureProcess> {
        self.connection.as_ref().and_then(GameConnection::process)
    }
}
