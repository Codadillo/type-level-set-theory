#[macro_export]
macro_rules! type_assert {
    ($name:ident, $a:ty, $b:ty) => {
        mod $name {
            use super::*;

            trait TestTrait {
                fn test() {}
            }
            impl TestTrait for $a {}
            fn _test_fn() {
                <$b as TestTrait>::test();
            }
        }
    };
}

pub mod set {
    use crate::*;

    // Test the set macro
    type_assert!(macro_null, Null, set!());
    type_assert!(macro_1, ConsUnion<Null>, set!(set!(),));
    type_assert!(macro_2, ConsUnion<Null, ConsUnion<Null>>, set!(set!(), set!(), ));
    type_assert!(
        set_macro_nested,
        ConsUnion<ConsUnion<Null>, ConsUnion<Null>>,
        set!(set!(set!(),), set!(),)
    );

    // Test equality
    type_assert!(eq_null, True, Eq<set!(), set!()>);
    type_assert!(neq_null, False, Eq<set!(), set!(set!(), )>);
    type_assert!(eq_nested, True, Eq<set!(set!(set!(), set!(), ), set!(set!(), ), ), set!(set!(set!(), set!(), ), set!(set!(), ), )>);
    type_assert!(neq_nested, False, Eq<set!(set!(set!(), set!(), ), set!(set!(), ), ), set!(set!(set!(), ), set!(set!(), ), )>);

    // Test containment
    type_assert!(in_null, False, Contains<set!(), set!()>);
    type_assert!(set_in_null, False, Contains<set!(), set!(set!(), )>);
    type_assert!(null_in_set, True, Contains<set!(set!(), ), set!()>);
    type_assert!(in_nested, True, Contains<set!(set!(set!(), ), set!(set!(set!(), ), ), ), set!(set!(), )>);
    type_assert!(nin_nested, False, Contains<set!(set!(), set!(set!(set!(), ), ), ), set!(set!(), )>);
}
