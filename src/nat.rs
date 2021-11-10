use crate::*;

#[macro_export]
macro_rules! nat {
    () => {
        Null
    };
    (* $( $rest:tt )*) => {
        Succ<nat!($($rest)*)>
    }
}

pub trait Nat: Set {}

impl Nat for Null {}
impl<N: Set> Nat for ConsUnion<N, N> {}

pub type Succ<N> = <N as DSucc>::Output;

pub trait DSucc: Nat {
    type Output: Nat;
}

impl<N: Nat> DSucc for N {
    type Output = ConsUnion<N, N>;
}

pub type Cardinality<S> = <S as DCardinality>::Output;

pub trait DCardinality: Set {
    type Output: Nat;
}

impl<S> DCardinality for S
where
    S: DSimplify,
    Simplify<S>: DCardinalityHelper,
{
    type Output = <Simplify<S> as DCardinalityHelper>::Output;
}

pub trait DCardinalityHelper: Set {
    type Output: Nat;
}

impl DCardinalityHelper for Null {
    type Output = Null;
}

impl<E, S> DCardinalityHelper for ConsUnion<E, S>
where
    E: Set,
    S: DCardinalityHelper + DSimplify,
    Cardinality<S>: DSucc,
    Simplify<S>: DCardinalityHelper,
{
    type Output = Succ<Cardinality<S>>;
}

pub type Add<N1, N2> = <N1 as DAdd<N2>>::Output;

pub trait DAdd<N: Nat>: Nat {
    type Output: Nat;
}

impl<N1, N2> DAdd<N2> for N1
where
    N1: Nat + DExtend<ConsUnion<Null>>,
    N2: Nat + DExtend<ConsUnion<ConsUnion<Null>>>,
    Extend<N1, ConsUnion<Null>>: DUnion<Extend<N2, ConsUnion<ConsUnion<Null>>>>,
    Union<Extend<N1, ConsUnion<Null>>, Extend<N2, ConsUnion<ConsUnion<Null>>>>: DCardinality,
{
    type Output =
        Cardinality<Union<Extend<N1, ConsUnion<Null>>, Extend<N2, ConsUnion<ConsUnion<Null>>>>>;
}
