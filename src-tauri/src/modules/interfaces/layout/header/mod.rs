//! Допустимые действия, расположение и вид кнопок заголовка.

mod group_button_display;
mod group_button_placement;
mod group_header_action;
mod group_header_button;
mod group_header_icon;

pub(crate) use self::group_button_display::GroupButtonDisplay;
pub(crate) use self::group_button_placement::GroupButtonPlacement;
pub(crate) use self::group_header_action::GroupHeaderAction;
pub(crate) use self::group_header_button::GroupHeaderButton;
pub(crate) use self::group_header_icon::GroupHeaderIcon;
