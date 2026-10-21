use crate::core::game::EntityId;
use crate::integrations::aion2::catalog::FieldBossCatalog;
use crate::modules::interfaces::document::WidgetKind;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::FormulaDefinition;
use crate::modules::interfaces::evaluation::FormulaEngine;
use crate::modules::interfaces::evaluation::VisualChanges;
use crate::modules::interfaces::layout::GroupDataBinding;
use crate::modules::interfaces::layout::GroupDataSource;
use crate::modules::interfaces::layout::GroupFlow;
use crate::modules::interfaces::layout::GroupHeaderButton;
use crate::modules::interfaces::layout::GroupHeaderIcon;
use crate::modules::interfaces::rules::TriggerGroup;
use crate::modules::interfaces::rules::TriggerRule;
use crate::modules::interfaces::rules::TriggerUnit;
use crate::modules::interfaces::rules::ValueTrigger;
use std::collections::HashSet;

pub(crate) struct AdvancedValidator;

impl AdvancedValidator {
    pub(crate) fn node(node: &WidgetNode) -> Result<(), String> {
        let s = &node.style;

        Self::number(node.sounds.voice_rate, 50.0, 200.0)?;

        if node.sounds.show_text.chars().count() > 160
            || node.sounds.hide_text.chars().count() > 160
            || node.sounds.voice.chars().count() > 512
        {
            return Err("Invalid speech setting".into());
        }

        VisualChanges::color(&s.animation_color)?;

        for opacity in [
            s.background_opacity,
            node.group.fill_opacity,
            node.group.header_opacity,
        ]
        .into_iter()
        .flatten()
        {
            Self::number(opacity, 0.0, 100.0)?;
        }

        for (value, min, max) in [
            (s.outline_width, 0.0, 3.0),
            (s.layer, 0.0, 50.0),
            (s.zoom, 0.0, 40.0),
            (s.warn_below_seconds, 0.0, 3600.0),
            (s.range_min, -1e9, 1e9),
            (s.range_max, -1e9, 1e9),
            (s.transition_ms, 0.0, 3000.0),
            (s.bar_icon_size, 8.0, 256.0),
            (s.bar_icon_x, -2000.0, 2000.0),
            (s.bar_icon_y, -2000.0, 2000.0),
            (s.bar_icon_padding, 0.0, 100.0),
            (s.animation_intensity, 0.0, 100.0),
            (s.animation_threshold, 0.0, 3600.0),
            (s.animation_duration, 100.0, 60000.0),
            (s.motion_duration, 50.0, 3000.0),
            (node.sounds.volume, 0.0, 100.0),
            (node.group.header_font_size, 6.0, 96.0),
            (node.group.limit, 0.0, 128.0),
            (node.group.motion_ms, 0.0, 3000.0),
            (node.group.span_seconds, 0.1, 3600.0),
        ] {
            Self::number(value, min, max)?;
        }

        if s.range_max <= s.range_min
            || s.path.len() > 128
            || s.animation_path.len() > 128
            || s.right_text.chars().count() > 2048
            || s.ticks.len() > 16
            || node.load.map_ids.len() > 512
            || node.load.dungeon_ids.len() > 512
            || node.load.npc_ids.len() > 512
            || node.load.player_class.len() > 128
            || node.load.zone.len() > 256
            || node
                .entity_id
                .as_ref()
                .is_some_and(|id| id.is_empty() || id.len() > 64)
        {
            return Err("Invalid advanced widget settings".into());
        }

        for tick in &s.ticks {
            Self::number(*tick, -1e9, 1e9)?;
        }

        for color in [&s.outline_color, &s.spark_color, &s.tick_color] {
            VisualChanges::color(color)?;
        }

        if let Some(color) = &s.gradient {
            VisualChanges::color(color)?;
        }

        if let Some(rules) = &node.rules {
            Self::rules(rules)?;
        }

        Self::buttons(&node.header_buttons)?;

        if let Some(binding) = &node.binding {
            if binding.source == GroupDataSource::WorldBosses
                && (node.kind != WidgetKind::DynamicGroup || node.group.flow == GroupFlow::Timeline)
            {
                return Err("World bosses require a row, column or grid dynamic group".into());
            }
            if binding.source == GroupDataSource::SkillHistory
                && (node.kind != WidgetKind::DynamicGroup
                    || node.group.flow != GroupFlow::Timeline
                    || node.children.iter().any(|child| {
                        child.kind != WidgetKind::Icon
                            || !child.children.is_empty()
                            || child.binding.is_some()
                    }))
            {
                return Err("Skill history requires a timeline with icon templates".into());
            }

            if binding.source == GroupDataSource::Effects && node.kind != WidgetKind::DynamicGroup {
                return Err("Effect rows require a dynamic group".into());
            }

            Self::binding(binding)?;
        }

        Ok(())
    }

    pub(crate) fn number(value: f64, min: f64, max: f64) -> Result<(), String> {
        if !value.is_finite() || !(min..=max).contains(&value) {
            return Err("Setting is out of range".into());
        }

        Ok(())
    }

    fn text(text: &str, max: usize, empty: bool) -> Result<(), String> {
        if text.chars().count() > max || (!empty && text.trim().is_empty()) {
            return Err("Invalid text field".into());
        }

        Ok(())
    }

    pub(crate) fn rules(group: &TriggerGroup) -> Result<(), String> {
        if group.rules.is_empty() || group.rules.len() > 8 {
            return Err("Between 1 and 8 trigger rules".into());
        }

        let mut ids = HashSet::new();

        for rule in &group.rules {
            Self::text(rule.id(), 100, false)?;

            if !ids.insert(rule.id()) {
                return Err("Duplicate rule ID".into());
            }

            match rule {
                TriggerRule::Boss { trigger, .. } => {
                    if FieldBossCatalog::find(trigger.boss_id).is_none() {
                        return Err("Unknown world boss".into());
                    }
                    Self::number(trigger.before_seconds, 0.0, 86400.0)?;
                    Self::number(trigger.duration_ms, 100.0, 3_600_000.0)?;
                }
                TriggerRule::Value { trigger, .. } => Self::value_rule(trigger)?,
                TriggerRule::Effect { trigger, .. } => {
                    Self::text(&trigger.member, 64, true)?;
                    Self::number(trigger.min_stacks, 1.0, 999.0)?;
                    Self::number(trigger.remaining_below_seconds, 0.0, 3600.0)?;

                    if trigger.effects.len() > 32 {
                        return Err("Too many effects".into());
                    }
                }
                TriggerRule::Event { trigger, .. } => {
                    Self::number(trigger.duration_ms, 100.0, 3_600_000.0)?;

                    if let Some(value) = trigger.min_value {
                        Self::number(value, 0.0, 1e12)?;
                    }

                    if let Some(value) = trigger.delay_ms {
                        Self::number(value, 0.0, 3_600_000.0)?;
                    }

                    if let Some(key) = &trigger.source_actor_key {
                        Self::text(key, 256, false)?;
                    }

                    if let Some(schedule) = &trigger.schedule {
                        Self::number(schedule.first_ms, 0.0, 3_600_000.0)?;
                        Self::number(schedule.interval_ms, 0.0, 600_000.0)?;
                    }
                }
            }
        }

        if group.data_rule_id.as_ref().is_some_and(|id| {
            !group
                .rules
                .iter()
                .any(|rule| rule.id() == id && !matches!(rule, TriggerRule::Value { .. }))
        }) {
            return Err("Trigger data source is missing".into());
        }

        Ok(())
    }

    pub(crate) fn value_rule(value: &ValueTrigger) -> Result<(), String> {
        Self::text(&value.path, 128, false)?;
        Self::text(&value.value, 128, true)
    }

    fn buttons(buttons: &[GroupHeaderButton]) -> Result<(), String> {
        if buttons.len() > 6 {
            return Err("Up to 6 header buttons".into());
        }

        let mut ids = HashSet::new();

        for button in buttons {
            Self::text(&button.id, 100, false)?;
            Self::text(&button.label, 128, true)?;

            if !ids.insert(&button.id) {
                return Err("Duplicate button".into());
            }

            Self::button_icon(&button.icon)?;
        }

        Ok(())
    }

    fn button_icon(icon: &str) -> Result<(), String> {
        Self::text(icon, 200, false)?;
        let game = icon.strip_prefix("game:").is_some_and(|file| {
            file.strip_suffix(".png").is_some_and(|stem| {
                !stem.is_empty()
                    && stem
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
            })
        });

        if game
            || serde_json::from_value::<GroupHeaderIcon>(serde_json::Value::String(icon.into()))
                .is_ok()
        {
            return Ok(());
        }

        Err("Unknown setting variant".into())
    }

    fn binding(binding: &GroupDataBinding) -> Result<(), String> {
        let minutes: HashSet<_> = binding.boss_alert_minutes.iter().collect();

        if minutes.len() != binding.boss_alert_minutes.len()
            || minutes.iter().any(|minute| ![1, 3, 5].contains(minute))
        {
            return Err("Boss alerts support unique 1, 3 and 5 minute thresholds".into());
        }

        let ids: HashSet<_> = binding.boss_ids.iter().collect();

        if binding.boss_ids.len() > 48
            || ids.len() != binding.boss_ids.len()
            || binding
                .boss_ids
                .iter()
                .any(|id| FieldBossCatalog::find(*id).is_none())
        {
            return Err("Invalid world boss selection".into());
        }

        Self::number(f64::from(binding.limit), 1.0, 128.0)?;

        if let Some(effects) = &binding.effects {
            Self::number(effects.icon_size, 8.0, 256.0)?;

            if !matches!(effects.unit, TriggerUnit::Player | TriggerUnit::Target) {
                return Err("Effect groups support player or target units".into());
            }
        }

        if binding.source == GroupDataSource::Effects && binding.effects.is_none() {
            return Err("Effect group settings are missing".into());
        }

        if binding.views.len() > 16 {
            return Err("Too many views".into());
        }

        for id in [
            &binding.actor_id,
            &binding.target_id,
            &binding.active_view_id,
        ]
        .into_iter()
        .flatten()
        {
            Self::text(id, 100, false)?;
        }

        for id in [&binding.actor_id, &binding.target_id]
            .into_iter()
            .flatten()
        {
            id.parse::<u64>()
                .ok()
                .and_then(|id| EntityId::new(id).ok())
                .ok_or("Invalid table entity ID")?;
        }

        let mut ids = HashSet::new();

        for view in &binding.views {
            Self::text(&view.id, 100, false)?;
            Self::text(&view.name, 128, false)?;

            if !ids.insert(&view.id) {
                return Err("Duplicate view ID".into());
            }

            for id in [&view.actor_id, &view.target_id].into_iter().flatten() {
                Self::text(id, 100, false)?;
                id.parse::<u64>()
                    .ok()
                    .and_then(|id| EntityId::new(id).ok())
                    .ok_or("Invalid table view entity ID")?;
            }
        }

        if binding
            .active_view_id
            .as_ref()
            .is_some_and(|id| !ids.contains(id))
        {
            return Err("Active table view is missing".into());
        }

        Ok(())
    }

    pub(crate) fn formulas(formulas: &[FormulaDefinition]) -> Result<(), String> {
        FormulaEngine::validate(formulas)
    }
}
