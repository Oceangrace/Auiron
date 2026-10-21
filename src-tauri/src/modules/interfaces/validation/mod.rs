//! Проверка целостности документов и допустимости настроек.

mod advanced_validator;
mod interface_validator;
mod widget_visual_validator;

pub(crate) use self::advanced_validator::AdvancedValidator;
pub(crate) use self::interface_validator::InterfaceValidator;
pub(crate) use self::widget_visual_validator::WidgetVisualValidator;
