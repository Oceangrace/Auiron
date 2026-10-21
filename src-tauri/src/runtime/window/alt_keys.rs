use windows::Win32::UI::Input::KeyboardAndMouse::{VK_LMENU, VK_MENU, VK_RMENU};

#[derive(Default)]
pub(super) struct AltKeys {
    left: bool,
    right: bool,
}

impl AltKeys {
    pub(super) fn update(&mut self, key: u16, flags: u16) {
        let right = match key {
            key if key == VK_MENU.0 => flags & 2 != 0,
            key if key == VK_LMENU.0 => false,
            key if key == VK_RMENU.0 => true,
            _ => return,
        };

        let down = flags & 1 == 0;

        if right {
            self.right = down;

            return;
        }

        self.left = down;
    }

    pub(super) fn held(&self) -> bool {
        self.left || self.right
    }
}
