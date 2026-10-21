use super::game_process::GameProcess;
use super::npcap_api::NpcapApi;
use super::npcap_session::NpcapSession;
use crate::config::ApplicationConfiguration;
use crate::integrations::aion2::network::PacketReader;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub(crate) struct CaptureProbe;

impl CaptureProbe {
    pub(crate) fn run() -> Result<String, String> {
        let configuration = ApplicationConfiguration::load()?;

        let windows = std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .ok_or("SystemRoot is unavailable")?;

        let api = Rc::new(NpcapApi::load(
            &windows.join(configuration.capture().library()),
        )?);

        let adapters = api.adapters()?.len();

        let listener =
            TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).map_err(|error| error.to_string())?;

        let address = listener.local_addr().map_err(|error| error.to_string())?;

        let mut client = TcpStream::connect_timeout(&address, Duration::from_secs(2))
            .map_err(|error| error.to_string())?;

        let (mut server, _) = listener.accept().map_err(|error| error.to_string())?;

        client
            .set_write_timeout(Some(Duration::from_secs(2)))
            .map_err(|error| error.to_string())?;
        server
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|error| error.to_string())?;

        let executable = std::env::current_exe().map_err(|error| error.to_string())?;

        let name = executable
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("Cannot identify the probe process")?;

        let connection = GameProcess::read(name)?.ok_or("Probe process was not found")?;

        let mut session = NpcapSession::open(api, &connection)?;

        let payload = uuid::Uuid::new_v4().to_string();

        client
            .write_all(payload.as_bytes())
            .map_err(|error| error.to_string())?;

        let mut received = vec![0; payload.len()];

        server
            .read_exact(&mut received)
            .map_err(|error| error.to_string())?;

        Self::observe(&mut session, payload.as_bytes())?;

        if received != payload.as_bytes() {
            return Err("Npcap did not observe the loopback probe payload".into());
        }

        let mut extra_client = TcpStream::connect_timeout(&address, Duration::from_secs(2))
            .map_err(|error| error.to_string())?;

        let (extra_server, _) = listener.accept().map_err(|error| error.to_string())?;

        let queued = uuid::Uuid::new_v4().to_string();

        client
            .write_all(queued.as_bytes())
            .map_err(|error| error.to_string())?;
        server
            .read_exact(&mut received)
            .map_err(|error| error.to_string())?;

        let expanded = GameProcess::read(name)?.ok_or("Probe process disappeared")?;

        session.synchronize(&expanded)?;
        Self::observe(&mut session, queued.as_bytes())?;

        let additional = uuid::Uuid::new_v4().to_string();

        extra_client
            .write_all(additional.as_bytes())
            .map_err(|error| error.to_string())?;
        Self::observe(&mut session, additional.as_bytes())?;
        drop(extra_client);
        drop(extra_server);

        let reduced = GameProcess::read(name)?.ok_or("Probe process disappeared")?;

        session.synchronize(&reduced)?;

        let final_payload = uuid::Uuid::new_v4().to_string();

        client
            .write_all(final_payload.as_bytes())
            .map_err(|error| error.to_string())?;
        Self::observe(&mut session, final_payload.as_bytes())?;

        serde_json::to_string_pretty(&serde_json::json!({
            "adapters": adapters,
            "processEndpoints": connection.endpoints().len(),
            "expandedEndpoints": expanded.endpoints().len(),
            "reducedEndpoints": reduced.endpoints().len(),
            "capturedPayloadBytes": payload.len() + queued.len() + additional.len() + final_payload.len(),
            "endpointRefreshPreservedQueuedPayload": true,
            "lostPackets": session.lost_packets()?,
        }))
        .map_err(|error| error.to_string())
    }

    fn observe(session: &mut NpcapSession, payload: &[u8]) -> Result<(), String> {
        let started = Instant::now();

        while started.elapsed() < Duration::from_secs(2) {
            if let Some(packet) = session.next()?
                && let Some(segment) = PacketReader::read(&packet)?
                && segment.payload() == payload
            {
                return Ok(());
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        Err("Npcap did not preserve the probe payload across endpoint refresh".into())
    }
}
