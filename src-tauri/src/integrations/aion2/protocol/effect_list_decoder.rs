use super::BinaryReader;
use super::effect_decoder::EffectDecoder;
use super::effect_list_candidate::EffectListCandidate;
use crate::core::game::EffectTiming;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::ObservedEffect;

pub(super) struct EffectListDecoder;

impl EffectListDecoder {
    pub(super) fn decode(body: &[u8], terminator: &[u8]) -> Result<Vec<GameEvent>, String> {
        if body.len() > 65_536 {
            return Err("Effect list search size limit".into());
        }

        let mut reader = BinaryReader::new(body);

        reader.skip(2)?;

        let entity_id = EntityId::new(reader.varint()?)?;

        let mut matches = Vec::new();

        for at in reader.position()..body.len().saturating_sub(25) {
            let count = body[at];

            if !(1..=128).contains(&count) || !Self::flag(body.get(at + 1)) {
                continue;
            }

            let candidates = Self::list(body, at + 1, count, terminator);

            if candidates.len() == 1 {
                matches.extend(candidates);
            }
        }

        if matches.len() != 1 {
            return Ok(Vec::new());
        }

        let candidate = matches.pop().ok_or("Missing effect list candidate")?;

        Ok(vec![GameEvent::EffectsListed {
            entity_id,
            effects: candidate.into_entries(),
        }])
    }

    fn list(body: &[u8], at: usize, count: u8, after: &[u8]) -> Vec<EffectListCandidate> {
        let mut candidates = vec![EffectListCandidate::new(at, Vec::new())];

        for index in 0..count {
            candidates = candidates
                .iter()
                .flat_map(|candidate| {
                    Self::step(body, candidate, index == count - 1, after).unwrap_or_default()
                })
                .collect();

            if candidates.is_empty() || candidates.len() > 4 {
                return Vec::new();
            }
        }

        candidates
    }

    fn step(
        body: &[u8],
        candidate: &EffectListCandidate,
        last: bool,
        after: &[u8],
    ) -> Result<Vec<EffectListCandidate>, String> {
        if !Self::flag(body.get(candidate.position())) {
            return Ok(Vec::new());
        }

        let mut reader = BinaryReader::new(&body[candidate.position()..]);

        let effect = EffectDecoder::entry(&mut reader, true)?;

        if !Self::valid(&effect) {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();

        for tail in [12, 13] {
            let end = candidate.position() + reader.position() + tail;

            let valid = if last {
                body.get(end..end + after.len()) == Some(after)
            } else {
                Self::flag(body.get(end))
            };

            if valid {
                let mut entries = candidate.entries().to_vec();

                entries.push(effect.clone());
                result.push(EffectListCandidate::new(end, entries));
            }
        }

        Ok(result)
    }

    fn valid(effect: &ObservedEffect) -> bool {
        if effect.effect_id().get() >= 1_000_000_000 {
            return false;
        }

        match effect.timing() {
            EffectTiming::Untimed => true,
            EffectTiming::Timed {
                duration_ms,
                ends_at_server_ms: Some(end),
            } => *duration_ms <= 34_560_000_000 && *end >= 1_704_067_200_000,
            _ => false,
        }
    }

    fn flag(value: Option<&u8>) -> bool {
        matches!(value, Some(16 | 17 | 19))
    }
}
