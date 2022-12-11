#[macro_export(local_inner_macros)]
macro_rules! edn {
    // Hide distracting implementation details from the generated rustdoc.
    ($($edn:tt)+) => {
        edn_internal!($($edn)+)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! edn_internal{
       //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: edn_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        edn_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        edn_internal_vec![$($elems),*]
    };

    // Next element is `nil`.
    (@array [$($elems:expr,)*] nil $($rest:tt)*) => {
        edn_internal!(@array [$($elems,)* edn_internal!(nil)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        edn_internal!(@array [$($elems,)* edn_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        edn_internal!(@array [$($elems,)* edn_internal!(false)] $($rest)*)
    };

    // ???
   // (@array [$($elems:expr,)*] $next:ident $($rest:tt)*) => {
    //    edn_internal!(@array [$($elems,)* edn_internal!(edn_inner_symbol!($next))] $($rest)*)
    //};

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        edn_internal!(@array [$($elems,)* edn_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    // TODO

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        edn_internal!(@array [$($elems,)* edn_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        edn_internal!(@array [$($elems,)* edn_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        edn_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected)
    };

     //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: edn_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    /*
    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        edn_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected);
    };

     // Insert the last entry without trailing comma.
     (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: nil $($rest:tt)*) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        edn_internal!(@object $object [$($key)+] (edn_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        edn_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };


     // Munch a token into the current key.
     (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

     */
    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: edn_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (nil) => {
        $crate::Value::Nil
    };

    (true) => {
        $crate::Value::Boolean(true)
    };

    (false) => {
        $crate::Value::Boolean(false)
    };

    ([]) => {
        $crate::Value::Vector(edn_internal_vec![])
    };

    (()) => {
        $crate::Value::List(edn_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Value::Vector(edn_internal!(@array [] $($tt)+))
    };

    (#{}) => {
        $crate::Value::Set(edn_internal_vec![])
    };

    ({}) => {
        $crate::Value::Map(edn_internal_vec![])
    };

    ({ $($tt:tt)+ }) => {
        $crate::Value::Map({
            let mut object = $crate::Map::new();
            json_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    (: $keyword:tt) => {
        $crate::Value::Keyword(::std::stringify!($keyword).into())
    };

   // ($sym:ident) => {
   //     $crate::Value::Symbol(stringify!($sym).into())
   // };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::to_value(&$other).unwrap()
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! edn_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! edn_inner_symbol {
    ($content:tt) => {
        $crate::Value::Symbol(::std::stringify!($content).into())
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! edn_inner_keyward {
    ($($content:tt)*) => {
        $crate::Value::Keyword(::std::stringify!($($content)*).into())
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! edn_unexpected {
    () => {};
}
