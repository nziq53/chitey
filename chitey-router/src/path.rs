use derive_more::{AsRef, Deref, DerefMut, Display, From};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, AsRef, Display, From)]
pub struct Path<T>(T);

impl<T> Path<T> {
    /// Unwrap into inner `T` value.
    pub fn into_inner(self) -> T {
        self.0
    }
}
