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

#[macro_export]
macro_rules! set_assert_eq {
    ( $name:ident, $a:ty, $b:ty ) => {
        type_assert!($name, True, Eq<$a, $b>);
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
    set_assert_eq!(eq_null, set!(), set!());
    set_assert_eq!(
        eq_nested,
        set!(set!(), set!(set!(), set!(set!(),),),),
        set!(set!(set!(), set!(set!(),),), set!(),)
    );
    set_assert_eq!(
        eq_nested_order,
        set!(set!(set!(set!(),), set!(),), set!(set!(),), set!(),),
        set!(set!(), set!(set!(),), set!(set!(), set!(set!(),),),)
    );

    // Test simplification
    type_assert!(simp_null, Null, Simplify<set!()>);
    type_assert!(no_simp, set!(set!(),), Simplify<set!(set!(),)>);
    type_assert!(simp_2, set!(set!(),), Simplify<set!(set!(), set!(),)>);

    // Test union
    set_assert_eq!(union_null, set!(set!(), ), Union<set!(), set!(set!(), )>);
    set_assert_eq!(union_nested,
        set!(set!(), set!(), set!(set!(set!(), ), ), set!(), set!(set!(), ), ),
        Union<
            set!(set!(), set!(set!(), ), ),
            set!(set!(set!(set!(), ), ), set!(), set!(), )
        >
    );

    // Test tuple
    set_assert_eq!(tuple, set!(set!(), set!(set!(), set!(set!(), ), ), ), Tuple<set!(), set!(set!(), )>);

    // Test extension
    set_assert_eq!(ext_null, set!(), Extend<set!(), set!(set!(), set!(set!(), ), )>);
    set_assert_eq!(ext_by_null,
        set!(Tuple<set!(), set!()>, Tuple<set!(set!(), ), set!()>, ),
        Extend<set!(set!(), set!(set!(), ), ), set!()>
    );
    set_assert_eq!(
        ext_nested,
        set!(
            Tuple<Tuple<set!(), set!()>, set!(set!(),)>,
            Tuple<Tuple<set!(set!(),), set!()>, set!(set!(),)>,
        ),
        Extend<Extend<set!(set!(), set!(set!(),),), set!()>, set!(set!(),)>
    );

    // Test cartesian product
    set_assert_eq!(prod_null, set!(), CartesianProd<set!(set!(), ), set!()>);
    set_assert_eq!(prod_squared,
        set!(
            Tuple<set!(), set!()>,
            Tuple<set!(set!(), ), set!()>,
            Tuple<set!(), set!(set!(), )>,
            Tuple<set!(set!(), ), set!(set!(), )>,
        ),
        CartesianProd<
            set!(set!(), set!(set!(), ),),
            set!(set!(), set!(set!(), ),)
        >
    );
}

pub mod nat {
    use crate::{nat::*, *};

    // Test the nat macro
    set_assert_eq!(macro_0, set!(), nat!());
    set_assert_eq!(macro_1, set!(set!(),), nat!(*));
    set_assert_eq!(macro_2, set!(set!(), set!(set!(),),), nat!(**));
    set_assert_eq!(
        macro_3,
        set!(set!(), set!(set!(),), set!(set!(), set!(set!(),),),),
        nat!(***)
    );

    // Test cardinality
    set_assert_eq!(card_0, nat!(), Cardinality<set!()>);
    set_assert_eq!(card_1, nat!(*), Cardinality<set!(set!(), set!(),)>);
    set_assert_eq!(
        card_2,
        nat!(**),
        Cardinality<set!(set!(set!(),), set!(), set!(set!(),),)>
    );

    // Test adding
    set_assert_eq!(add_0_0, nat!(), Add<nat!(), nat!()>);
    set_assert_eq!(add_0_1, nat!(*), Add<nat!(*), nat!()>);
    set_assert_eq!(add_3_2, nat!(*****), Add<nat!(***), nat!(**)>);
    set_assert_eq!(add_4_10, nat!(**************), Add<nat!(****), nat!(**********)>);
}
