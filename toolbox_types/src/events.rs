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

#[derive(Debug)]
pub enum Type {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Date(i64),
    UntypedTable,
    UserType { type_id: String },
}

impl Type {
    pub fn try_into_integer(self) -> Result<i64, Self> {
        if let Self::Integer(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_float(self) -> Result<f64, Self> {
        if let Self::Float(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_string(self) -> Result<String, Self> {
        if let Self::String(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_boolean(self) -> Result<bool, Self> {
        if let Self::Boolean(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
