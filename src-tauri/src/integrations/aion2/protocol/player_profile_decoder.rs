use super::BinaryReader;
use super::DecoderKind;
use super::identity_decoder::IdentityDecoder;
use crate::core::game::CharacterId;
use crate::core::game::GameEvent;

pub(super) struct PlayerProfileDecoder;

impl PlayerProfileDecoder {
    /// Typed attributes present in every observed profile; others (205, 214) vary per character.
    const MARKERS: [u16; 3] = [206, 208, 551];
    const MAX_ATTRIBUTES: u8 = 16;

    pub(super) fn decode(body: &[u8]) -> Result<Option<GameEvent>, String> {
        let GameEvent::Identity {
            entity_id,
            name: Some(_),
            ..
        } = IdentityDecoder::decode(body, DecoderKind::PlayerSpawn)?
        else {
            return Ok(None);
        };

        if body.len() > 65_536 {
            return Err("Player profile exceeds the inspection limit".into());
        }

        let mut accepted = None;

        for at in 8..body.len().saturating_sub(36) {
            if !(Self::MARKERS.len() as u8..=Self::MAX_ATTRIBUTES).contains(&body[at]) {
                continue;
            }

            let Some((character_id, level)) = Self::profile(&body[at - 8..]) else {
                continue;
            };

            if accepted.is_some() {
                return Err("Ambiguous player character profile".into());
            }

            accepted = Some(GameEvent::CharacterIdentified {
                entity_id,
                character_id,
                level,
            });
        }

        Ok(accepted)
    }

    fn profile(body: &[u8]) -> Option<(CharacterId, u32)> {
        let mut reader = BinaryReader::new(body);

        let id = u64::from_le_bytes(reader.bytes(8).ok()?.try_into().ok()?);

        let character_id = CharacterId::new(id).ok()?;

        let count = reader.byte().ok()?;

        let mut previous = 0;

        let mut markers = 0;

        for _ in 0..count {
            let attribute = reader.u16().ok()?;

            if attribute <= previous {
                return None;
            }

            previous = attribute;
            markers += usize::from(Self::MARKERS.contains(&attribute));
            reader.skip(4).ok()?;
        }

        if markers != Self::MARKERS.len() {
            return None;
        }

        let level = reader.u32().ok()?;

        if !(1..=200).contains(&level) || reader.u32().ok()? != 0 || reader.u32().ok()? > 1_000_000
        {
            return None;
        }

        Some((character_id, level))
    }
}
