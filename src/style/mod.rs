
pub trait Node {
    type Element;
    fn parent() -> Option<Self>;
    fn prev_sibling() -> Option<Self>;
    fn next_sibling() -> Option<Self>;
    fn first_child() -> Option<Self>;
    fn last_child() -> Option<Self>;
}
