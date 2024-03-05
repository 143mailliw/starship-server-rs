#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TreeError {
    ChildrenUnsupported,
    Loop,
    SelfParent,
    ChildBorrowed,
    ParentBorrowed,
    PageBorrowed,
    TreeNodeBorrowed,
    DoesNotExist,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventError {
    WrittenLuaError,
    CompiledLuaError,
    InternalRuntimeError,
    UnknownActionCalled,
    UnknownServerActionCalled,
    EventTypeMismatch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathError {
    // read errors
    NodeNotFound(String),
    PageNotFound(String),
    InvalidInput,

    // write errors
    NoPage,
    BrokenParent,
}
