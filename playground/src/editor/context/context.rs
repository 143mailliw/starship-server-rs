use enum_iterator::Sequence;

pub trait Context: Sequence + Copy + Clone + PartialEq {
    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq, Clone, Copy, Sequence)]
pub enum SidebarContext {
    Pages,
    Blocks,
    Tree,
}

impl Context for SidebarContext {
    fn name(&self) -> &str {
        match self {
            SidebarContext::Pages => "Pages",
            SidebarContext::Blocks => "Blocks",
            SidebarContext::Tree => "Tree",
        }
    }
}
