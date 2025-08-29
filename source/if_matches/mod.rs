/// Maps pattern's bound variables to `Some` if the provided expression matches the pattern.
///
/// Evaluates the expression `e` against the pattern `p` and maps
/// the bound variables of `p` into `Some` if the expression matches
/// and the optional match guard expression `c` evaluates to `true`.
///
/// Mapping is performed by the closure, the body of which is provided as the `m` argument.
/// Inside `m`, the bound variables of `p` as well as variables from the outer scope are available.
///
/// Yields `None` if the expression does not match the pattern.
///
///
/// # Syntax
///
/// ```text
/// if_matches!(<expression>, <pattern> [if <match-guard>] => <mapping-closure-body>)
/// ```
///
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use el_macro::if_matches;
/// #
/// let a = Some(41);
/// let b = Some(43);
/// let avg = |x: i32, y: i32| (x + y) / 2;
///
/// let x = if_matches!((a, b), (Some(x), Some(y)) => avg(x, y));
/// assert!(x.is_some_and(|val| val == 42));
///
/// let x = if_matches!((a, None::<u8>), (Some(x), Some(_)) => a);
/// assert!(x.is_none());
/// ```
///
/// Usage with match guard:
/// ```
/// # use el_macro::if_matches;
/// #
/// let vol = Some(100);
///
/// let bins = Some(25);
/// let per_bin = if_matches!((vol, bins), (Some(v), Some(b)) if b != 0 => v / b);
/// assert!(per_bin.is_some_and(|share| share == 4));
///
/// let bins = Some(0);
/// let per_bin = if_matches!((vol, bins), (Some(v), Some(b)) if b != 0 => v / b);
/// assert!(per_bin.is_none());
/// ```
#[macro_export]
macro_rules! if_matches {

    ($e: expr, $p: pat $(if $c:expr)? => $m: expr) => {
        match $e {
            $p $(if $c)? => Some((|| $m)()),
            _ => None,
        }
    };

}
