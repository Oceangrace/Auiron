use super::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::HealthSnapshot;
use crate::core::game::ManaSnapshot;

pub(super) struct PropertyDecoder;

impl PropertyDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let entity_id = EntityId::new(reader.varint()?)?;

        let flags = reader.byte()?;

        if flags & !3 != 0 {
            return Err("Unknown entity property flags".into());
        }

        let mut mana = None;

        let mut maximum_mana = None;

        let mut health = None;

        let mut maximum_health = None;

        if flags & 1 != 0 {
            let count = reader.byte()?;

            for _ in 0..count {
                let property = reader.byte()?;

                let value = reader.u32()?;

                match property {
                    1 => mana = Some(value),
                    8 => maximum_mana = Some(value),
                    _ => {}
                }
            }
        }

        if flags & 2 != 0 {
            let count = reader.byte()?;

            for _ in 0..count {
                let property = reader.byte()?;

                let value = reader.i64()?;

                if matches!(property, 0 | 7) {
                    let value = u32::try_from(value).map_err(|_| "Invalid health property")?;

                    match property {
                        0 => health = Some(value),
                        7 => maximum_health = Some(value),
                        _ => {}
                    }
                }
            }
        }

        if reader.remaining() != 0 {
            return Err("Unknown entity property suffix".into());
        }

        Ok(vec![GameEvent::PropertiesChanged {
            entity_id,
            health: HealthSnapshot::new(health, maximum_health),
            mana: ManaSnapshot::new(mana, maximum_mana),
        }])
    }
}
