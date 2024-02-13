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

pub enum Type {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Date(i64),
    UntypedTable,
    UserType { type_id: String },
}

impl TryInto<i64> for Type {
    type Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Integer(v) => v,
            _ => Self::Error,
        }
    }
}

impl TryInto<f64> for Type {
    type Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Float(v) => v,
            _ => Self::Error,
        }
    }
}

impl TryInto<String> for Type {
    type Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            String(v) => v,
            _ => Self::Error,
        }
    }
}

impl TryInto<bool> for Type {
    type Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            String(v) => v,
            _ => Self::Error,
        }
    }
}
