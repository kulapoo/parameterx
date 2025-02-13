use std::ops::{Add, Sub, Mul, Div};
use num_traits::PrimInt;
use std::fmt::Debug;

pub trait IntegerBehavior:
    PrimInt +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Copy +
    Debug +
    'static +
    Send +
    Sync
{
    fn is_within_range(&self, min: Self, max: Self) -> bool {
        *self >= min && *self <= max
    }
}

impl<T> IntegerBehavior for T where T:
    PrimInt +
    Add<Output = T> +
    Sub<Output = T> +
    Mul<Output = T> +
    Div<Output = T> +
    Copy +
    Debug +
    Send +
    Sync +
    'static
{}

#[derive(Debug, Clone)]
pub struct IntVec<T: IntegerBehavior>(pub Vec<T>);

impl<T> ToString for IntVec<T>
where
    T: IntegerBehavior
{
    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl <T:IntegerBehavior> From<IntVec<T>> for Vec<T> {
    fn from(value: IntVec<T>) -> Self {
        value.0
    }
}
