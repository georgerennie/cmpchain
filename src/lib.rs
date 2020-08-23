#[macro_export]
macro_rules! chain {
    (@wrap [$($prev:tt)*] [$($cur:tt)*] < $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) <] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)*] <= $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) <=] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)*] > $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) >] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)*] >= $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) >=] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)*] == $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) ==] [$next] $($rest)*)
    };
    (@wrap [$($prev:tt)*] [$($cur:tt)*] != $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($prev)* ($($cur)*) !=] [$next] $($rest)*)
    };

    (@wrap [$($prev:tt)*] [$($cur:tt)*]) => { chain!(@op $($prev)* ($($cur)*)) };

    (@wrap [$($wrapped:tt)*] [$($cur:tt)*] $next:tt $($rest:tt)*) => {
        chain!(@wrap [$($wrapped)*] [$($cur)* $next] $($rest)*)
    };

    (@op $a:tt $op:tt $b:tt) => {{ $a $op $b }};

    (@op $a:tt $op:tt $b:tt $($rest:tt)+) => {{
        let a = $a;
        let b = $b;
        a $op b && chain!(@op b $($rest)*)
    }};

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
}
