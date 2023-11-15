use crate::macros::redef_units;

redef_units!(
    pub enum Event {
        Clicked { mouse_x: u64, mouse_y: u64 },
        Changed,
        RightClicked { mouse_x: u64, mouse_y: u64 },
        Hovered { mouse_x: u64, mouse_y: u64 },
        Scrolled { view_x: u64, view_y: u64 },
    }
);
