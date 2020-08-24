#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
//! A Rust library containing a macro to chain comparison operators succintly.
//! The syntax is similar to that found in Python and Julia.

/// Succintly chain comparison operators like in Python and Julia.
///
/// `chain!` allows you to write comparisons that need to be simultaneously true
/// more concisely. Instead of writing `a < b && b < c`, you can just write
/// `chain!(a < b < c)`. `chain!` has the added benefit that each argument is
/// only evaluated once, rather than being evaluated for both the left and right
/// comparisons. Arguments are lazily evaluated from left to right so that any
/// arguments after the first failing comparison are not evaluated. `chain!`
/// supports the comparison operators `<`, `<=`, `>`, `>=`, `==`, `!=` in any
/// order.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate cmpchain;
/// // Check if a value falls between two bounds
/// let x = 8;
/// if chain!(4 < x <= 10) {
///     assert!(true);
///     // ...
/// }
/// # else { assert!(false); }
/// ```
///
/// ```
/// // Check for equality of multiple values
/// # #[macro_use] extern crate cmpchain;
/// assert!(chain!(4 == 2 * 2 == 12 / 3));
/// ```
#[macro_export]
macro_rules! chain {
    // @wrap acts somewhat like a function, iterating through the input tokens
    // and placing parentheses around all the terms separated by the comparison
    // operators and then passing these tokens to @op
    // Thus for example it transforms 5 + 4 < 10 <= 20 * 2 into
    // (5 + 4) < (10) <= (20 * 2)

    // @wrap uses two square brackets containing tokens to save its current
    // state as it processes tokens. The first contains everything that has
    // been parsed so far, and the second contains the tokens that have
    // appeared since the previous comparison operator. This means that when
    // a new comparison operator is encountered, the tokens in the second
    // bracket can be wrapped in parentheses and added to the first bracket.

    // For example to call it for 5 + 4 < 10 + 5< 20 you would do
    // chain!(@wrap [] [5] + 4 < 10 + 5 < 20)
    // and part way through parsing the calls could be
    // chain!(@wrap [(5 + 4)] [10 +] 5 < 20)
    
    (@wrap [$($prev:tt)*] [$($cur:tt)+] <  $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) <] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)+] <= $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) <=] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)+] >  $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) >] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)+] >= $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) >=] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)+] == $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) ==] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)+] != $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) !=] [$next] $($rest)*)
    };

    (@arg_err $op:tt) => {
        compile_error!(concat!(
            "Expected two arguments for \"", stringify!($op), "\""
        ));
    };
    // Match errors where a comparison operator is left trailing at the end of
    // the input, and call error function
    (@wrap [$($a:tt)*] [$($b:tt)*] < ) => { chain!(@arg_err <)  };
    (@wrap [$($a:tt)*] [$($b:tt)*] <=) => { chain!(@arg_err <=) };
    (@wrap [$($a:tt)*] [$($b:tt)*] > ) => { chain!(@arg_err >)  };
    (@wrap [$($a:tt)*] [$($b:tt)*] >=) => { chain!(@arg_err >=) };
    (@wrap [$($a:tt)*] [$($b:tt)*] ==) => { chain!(@arg_err ==) };
    (@wrap [$($a:tt)*] [$($b:tt)*] !=) => { chain!(@arg_err !=) };

    // Matches when all the tokens have been parsed. Then calls @op on the
    // wrapped tokens
    (@wrap [$($prev:tt)*] [$($cur:tt)+]) => { chain!(@op $($prev)* ($($cur)*)) };

    // Matches when the next token to parse isnt a comparison operator, and just
    // adds this next token to the current capture group
    (@wrap [$($prev:tt)*] [$($cur:tt)+] $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)*] [$($cur)* $next] $($rest)*)
    };

    // @op acts like a function that recursively expands chained comparison
    // operators into a scope returning a boolean. This scope takes the left
    // most comparison and then evaluates its arguments, saving the values.
    // It then evaluates the comparison of these saved values, and if they
    // are true recursively calls itself with the next comparison, using the
    // second of the saved values as the first argument to the new comparison
    // to prevent repeated evaluation
    (@op $a:tt $op:tt $b:tt) => {{ $a $op $b }};
    (@op $a:tt $op:tt $b:tt $($rest:tt)+) => {{
        let a = $a;
        let b = $b;
        a $op b && chain!(@op b $($rest)*)
    }};

    // Error if for some reason the arguments to op cant be properly parsed as
    // a conditional
    (@op $($rest:tt)*) => {{
        compile_error!("Expected comparison operator (<, <=, >, >=, ==, !=)");
    }};
    
    // Throw errors if there is no left hand argument to the first comparison
    (<  $($rest:tt)*) => { chain!(@arg_err <)  };
    (<= $($rest:tt)*) => { chain!(@arg_err <=) };
    (>  $($rest:tt)*) => { chain!(@arg_err >)  };
    (>= $($rest:tt)*) => { chain!(@arg_err >=) };
    (== $($rest:tt)*) => { chain!(@arg_err ==) };
    (!= $($rest:tt)*) => { chain!(@arg_err !=) };

    // Entrypoint
    ($first:tt $($rest:tt)*) => {
        chain!(@wrap [] [$first] $($rest)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_chaining() {
        // Check that basic comparisons without chaining still work
        assert!(chain!(1 < 2));
        assert!(chain!(1 <= 2));
        assert!(chain!(1 != 2));
    }

    #[test]
    fn three_args() {
        assert!(chain!(1 < 3 > 2));
        assert!(chain!(1 != 4 >= 2));
        assert!(chain!(5 == 5 <= 5));
    }

    #[test]
    fn side_effects() {
        // Pass in parameters that have side effects and check they are only
        // evaluated once and that arguments are evaluated left to right
        let mut results: Vec<i32> = Vec::new();
        let mut side_effect = |val: i32| {
            results.push(val);
            val
        };
        assert!(chain!(side_effect(1) < side_effect(2) != side_effect(3)));
        assert_eq!(results, &[1, 2, 3]);

        // Check that arguments are lazy evaluated so that if a comparison fails,
        // arguments in comparisons to the right of it arent evaluated
        let mut results: Vec<i32> = Vec::new();
        let mut side_effect = |val: i32| {
            results.push(val);
            val
        };
        assert!(chain!(side_effect(1) == side_effect(2) < side_effect(3)) == false);
        assert_eq!(results, &[1, 2]);
    }

    #[test]
    fn other_operators() {
        // Check that other operators like + are valid inbetween comparison
        // operators without terms being encapsulated in parentheses
        assert!(chain!(1 + 2 == 6 / 2 == 3));
        assert!(chain!(4 < 4 * 2 <= 4 * 3));
    }

    #[test]
    fn compile_fail_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/*.rs");
    }
}
