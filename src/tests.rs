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

    // Test ordered equality
    type_assert!(oeq_null, True, OrdEq<set!(), set!()>);
    type_assert!(noeq_null, False, OrdEq<set!(), set!(set!(), )>);
    type_assert!(oeq_nested, True, OrdEq<set!(set!(set!(), set!(), ), set!(set!(), ), ), set!(set!(set!(), set!(), ), set!(set!(), ), )>);
    type_assert!(noeq_nested, False, OrdEq<set!(set!(set!(), set!(), ), set!(set!(), ), ), set!(set!(set!(), ), set!(set!(), ), )>);

    // Test ordered element containment
    type_assert!(oin_null, False, OrdIn<set!(), set!()>);
    type_assert!(set_oin_null, False, OrdIn<set!(set!(), ), set!()>);
    type_assert!(null_oin_set, True, OrdIn<set!(), set!(set!(), )>);
    type_assert!(oin_nested, True, OrdIn<set!(set!(), ), set!(set!(set!(), ), set!(set!(set!(), ), ), )>);
    type_assert!(noin_nested, False, OrdIn<set!(set!(), ), set!(set!(), set!(set!(set!(), ), ), )>);

    // Test element containment
    type_assert!(in_null, False, OrdIn<set!(), set!()>);
    type_assert!(set_in_null, False, OrdIn<set!(set!(), ), set!()>);
    type_assert!(null_in_set, True, OrdIn<set!(), set!(set!(), )>);
    type_assert!(in_nested, True, OrdIn<set!(set!(), ), set!(set!(set!(), ), set!(set!(set!(), ), ), )>);
    type_assert!(nin_nested, False, OrdIn<set!(set!(), ), set!(set!(), set!(set!(set!(), ), ), )>);

    // Test set containment
    type_assert!(sub_null, True, Subset<set!(), set!()>);
    type_assert!(sub_basic, True, Subset<set!(set!(), ), set!(set!(), set!(set!(), ), )>);
    type_assert!(sup_basic, False, Subset<set!(set!(), set!(set!(), ), ), set!(set!(), )>);
    type_assert!(sub_nested, True, Subset<set!(set!(), set!(set!(), set!(), ), set!(), set!(set!(), set!(set!(), ), ), ), 
        set!(set!(set!(), set!(), ), set!(set!(), set!(set!(set!(), ), ), ), set!(set!(), ), set!(), set!(set!(), set!(set!(), ), ), )>);

    // Test equality
    type_assert!(eq_null, True, Eq<set!(), set!()>);
    type_assert!(eq_nested, True, Eq<set!(set!(), set!(set!(), set!(set!(), ), ), ), set!(set!(set!(), set!(set!(), ), ), set!(), )>);

    // Test simplification
    type_assert!(simp_null, Null, Simplify<set!()>);
    type_assert!(no_simp, set!(set!(),), Simplify<set!(set!(),)>);
    type_assert!(simp_2, set!(set!(),), Simplify<set!(set!(), set!(),)>);
}
