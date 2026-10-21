//! Типы и параметры анимации виджетов.

mod animation_change;
mod animation_when;
mod condition_animation;
mod motion_kind;
mod widget_animation;

pub(crate) use self::animation_change::AnimationChange;
pub(crate) use self::animation_when::AnimationWhen;
pub(crate) use self::condition_animation::ConditionAnimation;
pub(crate) use self::motion_kind::MotionKind;
pub(crate) use self::widget_animation::WidgetAnimation;
