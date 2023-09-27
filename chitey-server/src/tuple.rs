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

impl Tuple for (String,) {
    fn simbol_of_to_tuple(self) -> Self {
        self
    }
}

pub struct TupleWrapper<T>
where
    T: Tuple,
{
    inner: Box<T>,
}

impl<T> TupleWrapper<T>
where
    T: Tuple,
{
    pub fn new(tuple: T) -> Self {
        Self {
            inner: Box::new(tuple),
        }
    }
}

impl Into<TupleWrapper<()>> for () {
    fn into(self) -> TupleWrapper<()> {
        TupleWrapper::new(self)
    }
}
impl<T> Into<TupleWrapper<(T,)>> for (T,)
where
    T: Tuple,
{
    fn into(self) -> TupleWrapper<(T,)> {
        TupleWrapper::new(self)
    }
}
impl<T, T2> Into<TupleWrapper<(T, T2)>> for (T, T2)
where
    T: Tuple,
    T2: Tuple,
{
    fn into(self) -> TupleWrapper<(T, T2)> {
        TupleWrapper::new(self)
    }
}
