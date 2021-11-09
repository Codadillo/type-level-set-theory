pub trait Bool {}

pub struct True;
pub struct False;

impl Bool for True {}
impl Bool for False {}

pub type Or<A, B> = <A as DOr<B>>::Output;

pub trait DOr<A: Bool>: Bool {
    type Output: Bool;
}

impl DOr<True> for False {
    type Output = True;
}

impl DOr<True> for True {
    type Output = True;
}

impl DOr<False> for True {
    type Output = True;
}

impl DOr<False> for False {
    type Output = False;
}

pub type And<A, B> = <A as DAnd<B>>::Output;

pub trait DAnd<A: Bool>: Bool {
    type Output: Bool;
}

impl DAnd<True> for True {
    type Output = True;
}

impl DAnd<True> for False {
    type Output = False;
}

impl DAnd<False> for True {
    type Output = False;
}

impl DAnd<False> for False {
    type Output = False;
}

pub type IfThenElse<B, T, K> = <B as DIfThenElse<T, K>>::Output;

pub trait DIfThenElse<T, K>: Bool {
    type Output;
}

impl<T, K> DIfThenElse<T, K> for True {
    type Output = T;
}

impl<T, K> DIfThenElse<T, K> for False {
    type Output = K;
}
