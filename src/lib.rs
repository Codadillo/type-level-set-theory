use std::marker::PhantomData;

pub mod boolean;
pub mod nat;
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

pub type OrdEq<E1, E2> = <E1 as DOrdEq<E2>>::Output;

pub trait DOrdEq<E: Set>: Set {
    type Output: Bool;
}

impl DOrdEq<Null> for Null {
    type Output = True;
}

impl<E: Set, S: Set> DOrdEq<ConsUnion<E, S>> for Null {
    type Output = False;
}

impl<E: Set, S: Set> DOrdEq<Null> for ConsUnion<E, S> {
    type Output = False;
}

impl<E1, E2, S1, S2> DOrdEq<ConsUnion<E1, S1>> for ConsUnion<E2, S2>
where
    E1: DOrdEq<E2>,
    E2: Set,
    S1: Set + DOrdEq<S2>,
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
    E1: DOrdIn<S> + DOrdEq<E2>,
    E2: Set,
    S: Set,
    OrdEq<E1, E2>: DOr<OrdIn<E1, S>>,
{
    type Output = Or<OrdEq<E1, E2>, OrdIn<E1, S>>;
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
    E1: DIn<S> + DOrdEq<E2> + DSubset<E2>,
    E2: DSubset<E1>,
    S: Set,
    Eq<E1, E2>: DOr<In<E1, S>>,
    Subset<E1, E2>: DAnd<Subset<E2, E1>>,
{
    type Output = Or<Eq<E1, E2>, In<E1, S>>;
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
    E: DIn<S2>,
    S1: Set,
    S2: DSubset<S1>,
    In<E, S2>: DAnd<Subset<S1, S2>>,
{
    type Output = And<In<E, S2>, Subset<S1, S2>>;
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

pub type Union<S1, S2> = <S1 as DUnion<S2>>::Output;

pub trait DUnion<S: Set>: Set {
    type Output: Set;
}

impl DUnion<Null> for Null {
    type Output = Null;
}

impl<E: Set, S: Set> DUnion<Null> for ConsUnion<E, S> {
    type Output = ConsUnion<E, S>;
}

impl<E: Set, S: Set> DUnion<ConsUnion<E, S>> for Null {
    type Output = ConsUnion<E, S>;
}

impl<E1, E2, S1, S2> DUnion<ConsUnion<E2, S2>> for ConsUnion<E1, S1>
where
    E1: Set,
    E2: Set,
    S1: Set,
    S2: Set,
    ConsUnion<E2, ConsUnion<E1, S1>>: DUnion<S2>,
{
    type Output = Union<ConsUnion<E2, ConsUnion<E1, S1>>, S2>;
}

pub type Intersection<S1, S2> = <S1 as DIntersection<S2>>::Output;

pub trait DIntersection<S: Set>: Set {
    type Output: Set;
}

impl<E: Set, S: Set> DIntersection<Null> for ConsUnion<E, S> {
    type Output = Null;
}

impl<E: Set, S: Set> DIntersection<ConsUnion<E, S>> for Null {
    type Output = Null;
}

impl<E1, E2, S1, S2> DIntersection<ConsUnion<E1, S1>> for ConsUnion<E2, S2>
where
    E1: DIn<ConsUnion<E2, S2>>,
    E2: Set,
    S1: DIntersection<ConsUnion<E2, S2>>,
    S2: Set,
    In<E1, ConsUnion<E2, S2>>: DIfThenElse<
        ConsUnion<E1, Intersection<S1, ConsUnion<E2, S2>>>,
        Intersection<S1, ConsUnion<E2, S2>>,
    >,
    IfThenElse<
        In<E1, ConsUnion<E2, S2>>,
        ConsUnion<E1, Intersection<S1, ConsUnion<E2, S2>>>,
        Intersection<S1, ConsUnion<E2, S2>>,
    >: Set,
{
    type Output = IfThenElse<
        In<E1, ConsUnion<E2, S2>>,
        ConsUnion<E1, Intersection<S1, ConsUnion<E2, S2>>>,
        Intersection<S1, ConsUnion<E2, S2>>,
    >;
}

pub type Tuple<S1, S2> = ConsUnion<S1, ConsUnion<ConsUnion<S1, ConsUnion<S2>>>>;

pub type Extend<S, X, Rev = False> = <S as DExtend<X, Rev>>::Output;

pub trait DExtend<X: Set, Rev: Bool = False>: Set {
    type Output: Set;
}

impl<X: Set, Rev: Bool> DExtend<X, Rev> for Null {
    type Output = Null;
}

impl<X, E, S> DExtend<X> for ConsUnion<E, S>
where
    X: Set,
    E: Set,
    S: DExtend<X>,
{
    type Output = ConsUnion<Tuple<E, X>, Extend<S, X>>;
}

impl<X, E, S> DExtend<X, True> for ConsUnion<E, S>
where
    X: Set,
    E: Set,
    S: DExtend<X>,
{
    type Output = ConsUnion<Tuple<X, E>, Extend<S, X>>;
}

pub type CartesianProd<S1, S2> = <S1 as DCartesianProd<S2>>::Output;

pub trait DCartesianProd<S: Set>: Set {
    type Output: Set;
}

impl DCartesianProd<Null> for Null {
    type Output = Null;
}

impl<E: Set, S: Set> DCartesianProd<Null> for ConsUnion<E, S> {
    type Output = Null;
}

impl<E: Set, S: Set> DCartesianProd<ConsUnion<E, S>> for Null {
    type Output = Null;
}

impl<E1, E2, S1, S2> DCartesianProd<ConsUnion<E2, S2>> for ConsUnion<E1, S1>
where
    E1: Set,
    E2: Set,
    S1: DExtend<E2> + DCartesianProd<S2>,
    S2: DExtend<E1, True>,
    CartesianProd<S1, S2>: DUnion<Union<Extend<S1, E2>, Extend<S2, E1, True>>>,
    Extend<S1, E2>: DUnion<Extend<S2, E1, True>>,
{
    type Output = ConsUnion<
        Tuple<E1, E2>,
        Union<CartesianProd<S1, S2>, Union<Extend<S1, E2>, Extend<S2, E1, True>>>,
    >;
}
