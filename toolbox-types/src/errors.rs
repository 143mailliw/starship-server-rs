pub enum TreeError {
    ChildrenUnsupported,
    Loop,
    DoesNotExist,
}

pub enum EventError {
    WrittenLuaError,
    CompiledLuaError,
    InternalRuntimeError,
    UnknownActionCalled,
    UnknownServerActionCalled,
    EventTypeMismatch,
}
