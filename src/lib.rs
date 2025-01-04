pub mod config;
pub mod connection;
pub mod event;
pub mod listner;
pub mod repository;
pub mod resp;

pub trait Service<Req> {
    type Response;
    type Error;
    fn call(&mut self, request: Req) -> Result<Self::Response, Self::Error>;
}

#[cfg(test)]
pub mod test_helper {
    #[macro_export]
    macro_rules! test_helper {
        ($StructName:ident, { $( $field_name:ident: $field_type:ty, $field_value:expr ),* }, $( $([$mod:ident$( $expected:expr )?])? $test_name:ident, $test_body:tt );* $(;)? ) => {
            struct $StructName{
                $(
                    $field_name: $field_type,
                )*
            }
            impl $StructName{
                fn setup() -> Self {
                    Self {
                        $(
                            $field_name: $field_value
                        )*
                    }
                }
            }
            test_helper!(@after_struct $StructName, { $( mut $field_name, )* }, $( $( [$mod $($expected)? ] )? $test_name, $test_body );*);
        };
        (@after_struct $StructName:ident, $fields:tt, $( $( [$mod:ident $($expected:expr)?] )? $test_name:ident, $test_body:tt );* ) => {
            $(
                test_helper!(@call_test $( [$mod $($expected)?] )? $test_name, $StructName, $fields; $test_body);
            )*
        };

        (@call_test $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            #[test]
            fn $test_name() {
                #[allow(unused_variables, unused_mut)]
                let $StructName $fields = $StructName::setup();
                $body
            }
        };

        (@call_test [ok] $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            test_helper!(@call_test $test_name, $StructName, $fields; { assert!($body.is_ok()); } );
        };
        (@call_test [err] $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            test_helper!(@call_test $test_name, $StructName, $fields; { assert!($body.is_err()); } );
        };
        (@call_test [true] $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            test_helper!(@call_test $test_name, $StructName, $fields; { assert!($body); } );
        };
        (@call_test [false] $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            test_helper!(@call_test $test_name, $StructName, $fields; { assert!(!$body); } );
        };
        (@call_test [eq $expected:expr] $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            test_helper!(@call_test $test_name, $StructName, $fields; { assert_eq!($body, $expected); } );
        };
        (@call_test [ne $expected:expr] $test_name:ident, $StructName:ident, $fields:tt; $body:tt) => {
            test_helper!(@call_test $test_name, $StructName, $fields; { assert_ne!($body, $expected); } );
        };
    }
    pub use test_helper;

    test_helper! {Abc, { num: usize, 0},
    test_name, {
        num += 1;
        assert_eq!(num, 1);
    };
    resets, {
        assert_eq!(num, 0);
    };
    [ok]
    is_ok, { Ok::<(),()>(()) };
    [err]
    is_err, { Err::<(),()>(()) };
    [true]
    is_true, {
        true
    };
    [false]
    is_false, {
        false
    };
    [eq 2]
    is_eq, {
        num + 2
    };
    [ne 2]
    is_ne, {
        num
    };
    }
}
