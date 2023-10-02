use http::Method;

#[derive(Debug, Clone, PartialEq)]
pub enum Guard {
    Get = 0,
    Post = 1,
}

// Implement <Method> == <Guard> comparisons
impl PartialEq<Method> for Guard {
    fn eq(&self, other: &Method) -> bool {
        (*self == Guard::Get && other == Method::GET) ||
        (*self == Guard::Post && other == Method::POST)
    }
}
