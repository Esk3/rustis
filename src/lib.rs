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
        (
            $StructName:ident { $( $field_name:ident: $field_type:ty, $field_value:expr );* }
            $( $( #[$header:meta] )*
            $( [$mod:ident$( $expected:expr )?] )?
            $( ( $( $arg:expr ),* ) )*
            $test_name:ident( $( $param:ident ),* ) $test_body:tt $(;)? )*
         ) => {
            test_helper!(@create_struct $StructName { $( $field_name: $field_type ),* } { $( $field_name: $field_value ),* } );
            test_helper!( @after_struct
                $StructName, { $( mut $field_name ),* },
                $( $( #[$header] )*
                $( [$mod $($expected)? ] )?
                ( $( $( ( $arg ) ),* )* ) $test_name( $( $param ),* ), $test_body );*
                );
        };
        ( @create_struct $StructName:ident $declaration:tt $init:tt ) => {
            struct $StructName $declaration
            impl $StructName{
                fn setup() -> Self {
                    Self $init
                }
            }
        };
        ( @after_struct $StructName:ident, $fields:tt,
            $( $( #[$header:meta] )*
            $( [$mod:ident $($expected:expr)?] )?
            ( $( $arg:expr ),* )
            $test_name:ident( $( $param:ident ),* ), $test_body:tt );*
        ) => {
            $(
                test_helper!(@call_test $( #[$header] )* $( [$mod $($expected)?] )? ( $( $arg ),* ) $test_name, ( $( $param ),* ), $StructName, $fields; $test_body);
            )*
        };

        (@call_test $( #[$header:meta] )* $( [$mod:ident $( $expected:expr )?] )? ( $( $arg:expr ),* ) $test_name:ident, ( $( $param:ident ),* ), $StructName:ident, $fields:tt; $body:tt) => {
            #[test]
            $( #[$header] )*
            fn $test_name() {
                #[allow(unused_variables, unused_mut)]
                let $StructName $fields = $StructName::setup();
                let ( $( $param ),* ) = ( $( $arg ),* );
                test_helper!(@test_body $( mod: [$mod $( $expected )?] )? body: $body );
            }
        };

        (@test_body mod: [ok] body: $body:tt) => { assert!($body.is_ok()); };
        (@test_body mod: [err] body: $body:tt) => { assert!($body.is_err()); };
        (@test_body mod: [true] body: $body:tt) => { assert!($body); };
        (@test_body mod: [false] body: $body:tt) => { assert!(!$body); };
        (@test_body mod: [eq $expected:expr] body: $body:tt) => { assert_eq!($body, $expected); };
        (@test_body mod: [ne $expected:expr] body: $body:tt) => { assert_ne!($body, $expected); };
        (@test_body body: $body:tt) => { $body };
    }
    pub use test_helper;

    test_helper! {Abc { num: usize, 0}
    test_name() {
        num += 1;
        assert_eq!(num, 1);
    };
    resets() {
        assert_eq!(num, 0);
    };
    [ok]
    is_ok() { Ok::<(),()>(()) };
    [err]
    is_err() { Err::<(),()>(()) };
    [true]
    is_true() {
        true
    };
    [false]
    is_false() {
        false
    };
    [eq 2]
    is_eq() {
        num + 2
    };
    [ne 2]
    is_ne() {
        num
    };
    #[ignore="abc"]
    [ok]
    is_ignore() {
        panic!();
        Ok::<(),()>(())
    };
    #[should_panic(expected="")]
    is_panic() {
        panic!();
    };
    #[should_panic(expected="")]
    ("next", "acb")
    params(param, param2) {
        panic!("{param}, {param2}");
    };
    }
}
