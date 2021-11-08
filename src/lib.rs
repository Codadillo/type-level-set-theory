#![feature(generic_associated_types)]

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

pub trait Element {}

pub trait Set {}

pub struct Null;

pub struct ConsUnion<E: Element, S: Set = Null>(PhantomData<(E, S)>);

impl Set for Null {}

impl<E: Element, L: Set> Set for ConsUnion<E, L> {}

impl<S: Set> Element for S {}

pub type Eq<E1, E2> = <E1 as DEq<E2>>::Output;

pub trait DEq<E: Element>: Element {
    type Output: Bool;
}

impl DEq<Null> for Null {
    type Output = True;
}

impl<E: Element, S: Set> DEq<ConsUnion<E, S>> for Null {
    type Output = False;
}

impl<E: Element, S: Set> DEq<Null> for ConsUnion<E, S> {
    type Output = False;
}

impl<E1, E2, S1, S2> DEq<ConsUnion<E1, S1>> for ConsUnion<E2, S2>
where
    E1: Element + DEq<E2>,
    E2: Element,
    S1: Set + DEq<S2>,
    S2: Set,
    Eq<E1, E2>: DAnd<Eq<S1, S2>>,
{
    type Output = And<Eq<E1, E2>, Eq<S1, S2>>;
}

pub type Contains<S, E> = <S as DContains<E>>::Output;

pub trait DContains<E: Element>: Set {
    type Output: Bool;
}

impl<E: Element> DContains<E> for Null {
    type Output = False;
}

impl<E1, E2, S> DContains<E1> for ConsUnion<E2, S>
where
    E1: Element + DEq<E2>,
    E2: Element,
    S: Set + DContains<E1>,
    Eq<E1, E2>: DOr<Contains<S, E1>>,
{
    type Output = Or<Eq<E1, E2>, Contains<S, E1>>;
}
