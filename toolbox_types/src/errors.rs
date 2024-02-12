#[derive(Debug)]
pub enum TreeError {
    ChildrenUnsupported,
    Loop,
    SelfParent,
    ChildBorrowed,
    ParentBorrowed,
    TreeNodeBorrowed,
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
