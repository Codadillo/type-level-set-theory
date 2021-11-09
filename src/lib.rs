use std::marker::PhantomData;

pub mod boolean;
pub mod tests;

pub use boolean::*;

#[macro_export]
macro_rules! set {
    () => {
        Null
    };
    ( $e:ty, $( $rest:tt )* ) => {
        ConsUnion<$e, set!($( $rest )*)>
    };
}

pub trait Set {}

pub struct Null;

pub struct ConsUnion<E: Set, S: Set = Null>(PhantomData<(E, S)>);

impl Set for Null {}

impl<E: Set, L: Set> Set for ConsUnion<E, L> {}

pub type OrdEq<E1, E2> = <E1 as DHardEq<E2>>::Output;

pub trait DHardEq<E: Set>: Set {
    type Output: Bool;
}

impl DHardEq<Null> for Null {
    type Output = True;
}

impl<E: Set, S: Set> DHardEq<ConsUnion<E, S>> for Null {
    type Output = False;
}

impl<E: Set, S: Set> DHardEq<Null> for ConsUnion<E, S> {
    type Output = False;
}

impl<E1, E2, S1, S2> DHardEq<ConsUnion<E1, S1>> for ConsUnion<E2, S2>
where
    E1: DHardEq<E2>,
    E2: Set,
    S1: Set + DHardEq<S2>,
    S2: Set,
    OrdEq<E1, E2>: DAnd<OrdEq<S1, S2>>,
{
    type Output = And<OrdEq<E1, E2>, OrdEq<S1, S2>>;
}

pub type OrdIn<E, S> = <E as DOrdIn<S>>::Output;

pub trait DOrdIn<S: Set>: Set {
    type Output: Bool;
}

impl<E: Set> DOrdIn<Null> for E {
    type Output = False;
}

impl<E1, E2, S> DOrdIn<ConsUnion<E2, S>> for E1
where
    E1: DOrdIn<S> + DHardEq<E2>,
    E2: Set,
    S: Set,
    OrdEq<E1, E2>: DOr<OrdIn<E1, S>>,
{
    type Output = Or<OrdEq<E1, E2>, OrdIn<E1, S>>;
}

pub type Subset<S1, S2> = <S2 as DSubset<S1>>::Output;
pub type Eq<S1, S2> = And<Subset<S1, S2>, Subset<S2, S1>>;

pub trait DSubset<S: Set>: Set {
    type Output: Bool;
}

impl<S: Set> DSubset<Null> for S {
    type Output = True;
}

impl<E, S1, S2> DSubset<ConsUnion<E, S1>> for S2
where
    E: DOrdIn<S2>,
    S1: Set,
    S2: DSubset<S1>,
    OrdIn<E, S2>: DAnd<Subset<S1, S2>>,
{
    type Output = And<OrdIn<E, S2>, Subset<S1, S2>>;
}

pub type In<E, S> = <E as DIn<S>>::Output;

pub trait DIn<S: Set>: Set {
    type Output: Bool;
}

impl<E: Set> DIn<Null> for E {
    type Output = False;
}

impl<E1, E2, S> DIn<ConsUnion<E2, S>> for E1
where
    E1: DIn<S> + DHardEq<E2> + DSubset<E2>,
    E2: DSubset<E1>,
    S: Set,
    Eq<E1, E2>: DOr<In<E1, S>>,
    Subset<E1, E2>: DAnd<Subset<E2, E1>>,
{
    type Output = Or<Eq<E1, E2>, In<E1, S>>;
}

pub type Simplify<S> = <S as DSimplify>::Output;

pub trait DSimplify: Set {
    type Output: Set;
}

impl DSimplify for Null {
    type Output = Null;
}

impl<E, S> DSimplify for ConsUnion<E, S>
where
    E: DIn<S>,
    S: DSimplify,
    In<E, S>: DIfThenElse<Simplify<S>, ConsUnion<E, Simplify<S>>>,
    IfThenElse<In<E, S>, Simplify<S>, ConsUnion<E, Simplify<S>>>: Set,
{
    type Output = IfThenElse<In<E, S>, Simplify<S>, ConsUnion<E, Simplify<S>>>;
}
