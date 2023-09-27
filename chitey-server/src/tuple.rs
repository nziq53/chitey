pub trait TupleAppend<T> {
    type ResultType;

    fn append(self, t: T) -> Self::ResultType;
}

impl<T> TupleAppend<T> for () {
    type ResultType = (T,);

    fn append(self, t: T) -> Self::ResultType {
        (t,)
    }
}

macro_rules! impl_tuple_append {
    ( () ) => {};
    ( ( $t0:ident $(, $types:ident)* ) ) => {
        impl<$t0, $($types,)* T> TupleAppend<T> for ($t0, $($types,)*) {
            // Trailing comma, just to be extra sure we are dealing
            // with a tuple and not a parenthesized type/expr.
            type ResultType = ($t0, $($types,)* T,);

            fn append(self, t: T) -> Self::ResultType {
                // Reuse the type identifiers to destructure ourselves:
                let ($t0, $($types,)*) = self;
                // Create a new tuple with the original elements, plus the new one:
                ($t0, $($types,)* t,)
            }
        }

        // Recurse for one smaller size:
        impl_tuple_append! { ($($types),*) }
    };
}

impl_tuple_append! {
  // Supports tuples up to size 12:
  (_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12)
}

#[test]
fn test_tuple_append() {
    let some_tuple: (i32, &str, bool) = (1, "Hello", true);
    println!("{:?}", some_tuple);

    let n: (u16, u16) = [3, 4].into();
    println!("{:?}", n);

    let with_world: (i32, &str, bool, &str) = some_tuple.append("World");
    println!("{:?}", with_world);
}

use http::Uri;
use impl_trait_for_tuples::impl_for_tuples;

pub trait Tuple {
    fn simbol_of_to_tuple(self) -> Self;
}

#[impl_for_tuples(0, 12)]
impl Tuple for TupleIdentifier {
    fn simbol_of_to_tuple(self) -> Self {
        self
    }
}

use derive_more::{AsRef, Deref, DerefMut, Display, From};
use urlpattern::{UrlPattern, UrlPatternInit};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, AsRef, Display, From)]
pub struct Path<T>(T);

impl<T> Path<T>
{
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn new(t: T) -> Path<T> {
        Self(t)
    }
    pub fn tuple_new(ptn: UrlPattern, uri: Uri) -> Path<T> {
        
    }
}


// impl Into<Path<()>> for () {
//     fn into(self) -> Path<()> {
//         Path::new(self)
//     }
// }
// impl<T> Into<Path<(T,)>> for (T,)
// where
//     T: Tuple,
// {
//     fn into(self) -> Path<(T,)> {
//         Path::new(self)
//     }
// }
// impl<T, T2> Into<Path<(T, T2)>> for (T, T2)
// where
//     T: Tuple,
//     T2: Tuple,
// {
//     fn into(self) -> Path<(T, T2)> {
//         Path::new(self)
//     }
// }
