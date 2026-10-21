use super::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::HealthSnapshot;
use crate::core::game::NpcDefinitionId;
use crate::integrations::aion2::catalog::NpcIndex;

pub(super) struct SpawnDecoder;

impl SpawnDecoder {
    // Flag 0x08 adds a 12-byte block before resources in captured NPC and summon spawns.
    const EXTENDED_BLOCK_FLAG: u8 = 0x08;
    const EXTENDED_BLOCK_BYTES: usize = 12;

    pub(super) fn decode(body: &[u8], npcs: &NpcIndex) -> Result<GameEvent, String> {
        let mut reader = BinaryReader::new(body);

        reader.skip(2)?;

        let raw = reader.varint()?;

        // Compact spawn references are a protocol encoding, not a skill-ID normalization.
        let entity_id = EntityId::new(if raw > 1_000_000 {
            (raw & 16_383) | 16_384
        } else {
            raw
        })?;

        let spawn_kind = reader.byte()?;

        let mut npc_id = None;

        let mut health = None;

        for at in reader.position() + 3..body.len().saturating_sub(2).min(reader.position() + 60) {
            if body[at] != 0
                || !matches!(body[at + 1], 0x00 | 0x08 | 0x40 | 0x48)
                || body[at + 2] != 2
            {
                continue;
            }

            let raw = u32::from_le_bytes([body[at - 3], body[at - 2], body[at - 1], 0]);

            let Ok(candidate) = NpcDefinitionId::new(raw) else {
                continue;
            };

            if !npcs.contains(candidate) {
                continue;
            }

            npc_id = Some(candidate);

            let extension = if body[at + 1] & Self::EXTENDED_BLOCK_FLAG != 0 {
                Self::EXTENDED_BLOCK_BYTES
            } else {
                0
            };

            health = Self::health(body, at + 19 + extension)?;

            break;
        }

        Ok(GameEvent::Spawned {
            entity_id,
            npc_id,
            spawn_kind,
            health,
        })
    }

    fn health(body: &[u8], at: usize) -> Result<Option<HealthSnapshot>, String> {
        let Some(marker) = body.get(at..at + 3) else {
            return Ok(None);
        };

        // The two bytes before the resource flag vary between instances of the same NPC.
        // Their meaning is unknown; neither is a required zero (035716 NPC selection capture).
        if marker[2] != 1 {
            return Ok(None);
        }

        let mut reader = BinaryReader::new(&body[at + 3..]);

        let current = reader.varint()?;

        let maximum = reader.varint()?;

        if maximum == 0 || maximum > 2_000_000_000 || current > maximum {
            return Ok(None);
        }

        Ok(Some(HealthSnapshot::new(
            Some(current as u32),
            Some(maximum as u32),
        )))
    }
}
