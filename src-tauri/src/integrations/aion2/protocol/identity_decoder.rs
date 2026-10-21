use super::BinaryReader;
use super::DecoderKind;
use super::OpcodeTable;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;

pub(super) struct IdentityDecoder;

impl IdentityDecoder {
    pub(super) fn decode(body: &[u8], kind: DecoderKind) -> Result<GameEvent, String> {
        let mut reader = BinaryReader::new(body);

        reader.skip(2)?;

        let entity_id = EntityId::new(reader.varint()?)?;

        if entity_id.get() > 9_999_999 {
            return Err("Invalid identity entity".into());
        }

        if kind == DecoderKind::StatsChanged {
            return Ok(GameEvent::SelfIdentified {
                entity_id,
                authoritative: false,
            });
        }

        reader.skip(4)?;

        if reader.byte()? & 1 == 0 {
            return Ok(GameEvent::Identity {
                entity_id,
                name: None,
                own: false,
                server_id: None,
                class_name: None,
                level: None,
            });
        }

        let size = reader.byte()?;

        if !(1..=48).contains(&size) {
            return Err("Invalid identity name length".into());
        }

        let name = std::str::from_utf8(reader.bytes(usize::from(size))?)
            .map_err(|error| error.to_string())?;

        let placeholder = name.strip_prefix('$').is_some_and(|rest| {
            !rest.is_empty() && rest.bytes().all(|byte| byte.is_ascii_alphanumeric())
        });

        if !placeholder
            && (!(1..=12).contains(&name.chars().count())
                || !name.chars().all(char::is_alphanumeric))
        {
            return Err("Invalid identity name".into());
        }

        let name = (!placeholder).then(|| name.to_owned());

        let own = kind == DecoderKind::SelfIdentity;

        let mut server_id = None;

        let mut class_name = None;

        let mut level = None;

        if own {
            let server = reader.u16()?;

            if !(1000..=2999).contains(&server) {
                return Err("Invalid identity server".into());
            }

            server_id = Some(server);

            if reader.remaining() >= 4 {
                let raw = reader.u32()?;

                class_name = super::class_decoder::ClassDecoder::profile(raw);
            }

            if reader.remaining() >= 5 {
                reader.skip(1)?;

                let raw = reader.u32()?;

                level = (1..=200).contains(&raw).then_some(raw);
            }
        }

        Ok(GameEvent::Identity {
            entity_id,
            name,
            own,
            server_id,
            class_name,
            level,
        })
    }

    pub(super) fn scan(body: &[u8], table: &OpcodeTable) -> Vec<GameEvent> {
        let mut result = Vec::new();

        for at in 0..body.len().saturating_sub(8) {
            let opcode = u16::from_le_bytes([body[at], body[at + 1]]);

            let Some(kind) = table.decoder(opcode) else {
                continue;
            };

            if matches!(
                kind,
                DecoderKind::SelfIdentity | DecoderKind::Identity | DecoderKind::PlayerSpawn
            ) && let Ok(event) = Self::decode(&body[at..], kind)
            {
                result.push(event);
            }
        }

        result
    }
}
