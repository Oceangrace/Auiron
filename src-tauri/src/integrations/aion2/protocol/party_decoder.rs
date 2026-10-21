use super::BinaryReader;
use super::class_decoder::ClassDecoder;
use crate::core::game::CharacterId;
use crate::core::game::GameEvent;
use crate::core::game::PartyMember;
use crate::core::game::PartyRoster;
use crate::core::game::UnitIdentity;

pub(super) struct PartyDecoder;

impl PartyDecoder {
    pub(super) fn decode(body: &[u8]) -> Result<Vec<GameEvent>, String> {
        let mut reader = BinaryReader::new(body);

        reader.skip(6)?;

        let title_length = usize::from(reader.byte()?);

        std::str::from_utf8(reader.bytes(title_length)?).map_err(|_| "Invalid party title")?;

        let size = reader.byte()?;

        let dungeon = reader.u32()?;

        reader.skip(13)?;

        let count = reader.varint()?;

        if !(1..=12).contains(&size) || !(1..=12).contains(&count) {
            return Err("Invalid party header".into());
        }

        let count = count as u8;

        let Some(first) = Self::next(body, reader.position(), 0, count) else {
            return Ok(Vec::new());
        };

        reader.seek(first)?;

        let mut members = Vec::new();

        let mut complete = false;

        for _ in 0..count {
            let member = Self::member(&mut reader)?;

            let slot = member.slot();

            members.push(member);

            if slot == count {
                complete = true;
                break;
            }

            let Some(next) = Self::next(body, reader.position(), slot, count) else {
                complete = Self::vacants(body, reader.position(), slot, count);
                break;
            };

            reader.seek(next)?;
        }

        Ok(vec![GameEvent::Party {
            roster: PartyRoster::new(members, complete, (dungeon > 0).then_some(dungeon)),
        }])
    }

    fn member(reader: &mut BinaryReader<'_>) -> Result<PartyMember, String> {
        reader.byte()?;

        let slot = reader.byte()?;

        let database_id = u64::from_le_bytes(
            reader
                .bytes(8)?
                .try_into()
                .map_err(|_| "Invalid party character ID")?,
        );

        let server = (database_id >> 48) as u16;

        let length = usize::from(reader.byte()?);

        let name = std::str::from_utf8(reader.bytes(length)?)
            .map_err(|_| "Invalid party name")?
            .to_owned();

        if !Self::valid_name(&name) || database_id == 0 {
            return Err("Invalid party identity".into());
        }

        let class = ClassDecoder::profile(reader.u32()?);

        let level = reader.u32()?;

        let gear = reader.u32()?;

        let mut probe = reader.fork();

        let prefix = probe.bytes(probe.remaining().min(11))?;

        // Profile servers can differ from the server encoded in the character ID.
        // Keep the known anchor, then accept the observed two-server/status layout.
        let anchor = prefix
            .windows(2)
            .position(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]) == server)
            .or_else(|| {
                prefix.windows(5).position(|bytes| {
                    (1..=9999).contains(&u16::from_le_bytes([bytes[0], bytes[1]]))
                        && (1..=9999).contains(&u16::from_le_bytes([bytes[2], bytes[3]]))
                        && (1..=4).contains(&bytes[4])
                })
            })
            .ok_or("Missing party server anchor")?;

        reader.skip(anchor + 5)?;

        let power = reader.i64()?;

        if !(1..=200).contains(&level) || gear > 1_000_000 || !(0..=100_000_000).contains(&power) {
            return Err("Invalid party profile".into());
        }

        let mut identity = UnitIdentity::default();

        identity.observe(&Some(name), class, Some(server), Some(level));

        Ok(PartyMember::new(
            slot,
            CharacterId::new(database_id)?,
            identity,
            gear,
            power as u64,
        ))
    }

    fn next(body: &[u8], start: usize, slot: u8, count: u8) -> Option<usize> {
        let end = body
            .len()
            .saturating_sub(20)
            .min(start + 48 * usize::from(count - slot));

        (start..end).find(|at| Self::candidate(body, *at, slot, count))
    }

    fn candidate(body: &[u8], at: usize, slot: u8, count: u8) -> bool {
        let Some(prefix) = body.get(at..at + 11) else {
            return false;
        };

        let next = prefix[1];

        let server = u16::from_le_bytes([prefix[8], prefix[9]]);

        let length = usize::from(prefix[10]);

        if next <= slot
            || next > count
            || !(1..=9999).contains(&server)
            || !(1..=40).contains(&length)
        {
            return false;
        }

        body.get(at + 11..at + 11 + length)
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .is_some_and(Self::valid_name)
    }

    fn valid_name(name: &str) -> bool {
        (1..=12).contains(&name.chars().count()) && name.chars().all(char::is_alphanumeric)
    }

    fn vacants(body: &[u8], mut at: usize, slot: u8, count: u8) -> bool {
        for wanted in slot + 1..=count {
            let end = body.len().saturating_sub(11).min(at + 48);

            let found = (at..=end).find(|position| {
                body.get(*position..*position + 11).is_some_and(|bytes| {
                    bytes[0] == 0 && bytes[1] == wanted && bytes[2..].iter().all(|byte| *byte == 0)
                })
            });

            let Some(position) = found else {
                return false;
            };

            at = position + 11;
        }

        true
    }
}
