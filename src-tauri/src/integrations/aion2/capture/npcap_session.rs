use super::CapturedPacket;
use super::capture_channel::CaptureChannel;
use super::game_connection::GameConnection;
use super::npcap_api::NpcapApi;
use std::rc::Rc;

pub(super) struct NpcapSession {
    api: Rc<NpcapApi>,
    handles: Vec<CaptureChannel>,
    cursor: usize,
}

impl NpcapSession {
    pub(super) fn open(api: Rc<NpcapApi>, connection: &GameConnection) -> Result<Self, String> {
        let mut session = Self {
            api,
            handles: Vec::new(),
            cursor: 0,
        };

        session.synchronize(connection)?;

        if session.handles.is_empty() {
            return Err("No Npcap adapter matches the game connections".into());
        }

        Ok(session)
    }

    pub(super) fn synchronize(&mut self, connection: &GameConnection) -> Result<(), String> {
        for channel in &mut self.handles {
            channel.refresh(connection);
        }

        self.handles.retain(|channel| !channel.expired());

        for adapter in self.api.adapters()? {
            for endpoint in connection.endpoints() {
                if !adapter.accepts(endpoint.local_address())
                    || self
                        .handles
                        .iter()
                        .any(|channel| channel.matches(adapter.name(), endpoint))
                {
                    continue;
                }

                if self.handles.len() >= 128 {
                    return Err("Live capture channel limit".into());
                }

                self.handles.push(CaptureChannel::open(
                    self.api.clone(),
                    adapter.name(),
                    endpoint.clone(),
                )?);
            }
        }

        self.cursor = 0;

        Ok(())
    }

    pub(super) fn next(&mut self) -> Result<Option<CapturedPacket>, String> {
        for _ in 0..self.handles.len() {
            let index = self.cursor;

            self.cursor = (self.cursor + 1) % self.handles.len();

            if let Some(packet) = self.handles[index].next()? {
                return Ok(Some(packet));
            }
        }

        Ok(None)
    }

    pub(super) fn lost_packets(&mut self) -> Result<bool, String> {
        let mut lost = false;

        for handle in &mut self.handles {
            lost |= handle.lost_packets()?;
        }

        Ok(lost)
    }
}
