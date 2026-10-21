use super::BossTrigger;
use super::EffectTrigger;
use super::EventTrigger;
use super::RuleKind;
use super::ValueTrigger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum TriggerRule {
    Boss {
        #[schemars(length(min = 1, max = 100))]
        id: String,
        observe: bool,
        trigger: BossTrigger,
    },
    Value {
        #[schemars(length(min = 1, max = 100))]
        id: String,
        observe: bool,
        trigger: ValueTrigger,
    },
    Effect {
        #[schemars(length(min = 1, max = 100))]
        id: String,
        observe: bool,
        trigger: EffectTrigger,
    },
    Event {
        #[schemars(length(min = 1, max = 100))]
        id: String,
        observe: bool,
        trigger: EventTrigger,
    },
}

impl TriggerRule {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Boss { id, .. }
            | Self::Value { id, .. }
            | Self::Effect { id, .. }
            | Self::Event { id, .. } => id,
        }
    }

    pub(crate) fn observe(&self) -> bool {
        match self {
            Self::Boss { observe, .. }
            | Self::Value { observe, .. }
            | Self::Effect { observe, .. }
            | Self::Event { observe, .. } => *observe,
        }
    }

    pub(crate) fn kind(&self) -> RuleKind {
        match self {
            Self::Boss { .. } => RuleKind::Boss,
            Self::Value { .. } => RuleKind::Value,
            Self::Effect { .. } => RuleKind::Effect,
            Self::Event { .. } => RuleKind::Event,
        }
    }

    pub(crate) fn match_ids(&self) -> Vec<u32> {
        match self {
            Self::Boss { .. } | Self::Value { .. } => Vec::new(),
            Self::Effect { trigger, .. } => trigger.effects.iter().map(|id| id.get()).collect(),
            Self::Event { trigger, .. } => [trigger.skill_id, trigger.effect_id]
                .into_iter()
                .flatten()
                .map(|id| id.get())
                .collect(),
        }
    }
}
