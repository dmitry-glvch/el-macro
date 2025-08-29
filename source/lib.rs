/// Binds expression `e` to a new variable `n` if it ['has a value' / 'is ok'](IntoResult).
/// Otherwise, executes the optional error handler `h` and evaluates the `f` expression.
///
///
/// Provides the ability to write concise code to get the value or get goin' in a context where
/// [ErrorPropagationExpression (`?`)](https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try)
/// is not sufficient, such as when controlling execution flow with `break` or `continue`,
/// or not applicable, as in function that does not return [`Result`] or [`Option`].
///
/// The new variable name `n` may contain `mut` keyword to create a mutable binding.
/// 
/// The error value is passed as the only argument to the `h` error handler
/// if the latter is present. Determining whether the given expression `e` contains a valid
/// value or and error (and what are they) is done through [`IntoResult`] trait,
/// which is already implemented for [`Result`] and [`Option`].
/// 
/// The `f` expression is used to control the execution flow in a case when
/// the `e`expression contains an error, making binding impossible.
///
///
/// # Syntax
///
/// ```text
/// bind!([mut] <variable-name> = <value-expression>, or [<error-handler>,] <flow-control-expression>);
/// ```
///
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use el_macro::bind;
/// #
/// // binds `x` to the value 42, does not return
/// bind!(x = Some(42), or return);
/// assert_eq!(x, 42);
///
/// // creates a mutable binding `x` to the value 42, does not return
/// bind!(mut x = Some(42), or return);
/// x += 3;
/// assert_eq!(x, 45);
///
/// // returns
/// bind!(x = None::<i32>, or return);
/// unreachable!();
/// // returns as well, why wouldn't it
/// bind!(mut x = None::<i32>, or return);
/// unreachable!();
/// ```
///
/// Handling error values:
/// ```
/// # use el_macro::bind;
/// #
/// let okish = Some(42).ok_or("error");
/// let errorish = None::<i32>.ok_or("error");
///
/// let handle_error = |err: &str| eprintln!("{err}!");
///
/// // binds `x` to the value 42, does not return
/// bind!(x = okish, or handle_error, return);
/// assert_eq!(x, 42);
///
/// 'omit_handler: {
///     bind!(x = None::<i32>, or {
///         // it's possible to omit the error handler
///         // and perform handling in the flow control block
///         eprintln!("no value for x");
///         // you can control the execution flow however you like
///         break 'omit_handler
///     });
///     unreachable!();
/// }
///
/// // prints 'error!' and returns
/// bind!(x = errorish, or handle_error, return);
/// unreachable!();
/// ```
///
/// Using with a custom type:
/// ```
/// # use el_macro::{bind, IntoResult};
/// #
/// struct NegativeIsError(i32);
///
/// // returns some external descriptor on success,
/// // negative number on failure
/// fn external_get_descriptor(must_succeed: bool) -> i32 {
///     // actual external call here
///     if must_succeed { 42 } else { -1 }
/// }
///
/// // successfully binds `x` to the value 42
/// bind!(x = NegativeIsError(external_get_descriptor(true)), or return);
/// assert_eq!(x, 42);
///
/// // prints 'error -1: unknown error' and returns
/// bind!(x = NegativeIsError(external_get_descriptor(false)), or print_error, return);
/// unreachable!();
///
/// // specifies how to determine whether `NegativeIsError`
/// // contains a valid descriptor or an error
/// impl IntoResult for NegativeIsError {
///
///     type Value = i32;
///     type Error = ExternalCallError;
///
///     fn into_result(self) -> Result<Self::Value, Self::Error> {
///         (self.0 >= 0)
///             .then_some(self.0)
///             .ok_or(ExternalCallError {
///                 code: self.0,
///                 desc: get_error_desc(self.0),
///             })
///     }
///
/// }
///
/// struct ExternalCallError {
///     code: i32,
///     desc: String,
/// }
///
/// fn print_error(err: ExternalCallError) {
///     let ExternalCallError { code, desc, .. } = err;
///     eprintln!("error {code}: {desc}");
/// }
///
/// fn get_error_desc(error_code: i32) -> String {
///     if error_code >= 0 {
///         "no_error".to_string()
///     } else {
///         "unknown error".to_string()
///     }
/// }
/// ```
#[macro_export]
macro_rules! bind {

    ($n: ident = $e: expr, or $f: expr) => {
        let Ok($n) = $crate::IntoResult::into_result($e) else { $f };
    };

    (mut $n: ident = $e: expr, or $f: expr) => {
        let mut $n = {
            $crate::bind!($n = $e, or $f);
            $n
        };
    };

    ($n: ident = $e: expr, or $h: expr, $f: expr) => {
        let $n = match $crate::IntoResult::into_result($e) {
            Ok($n) => { $n },
            Err(err) => {
                $h(err);
                $f
            },
        };
    };

    (mut $n: ident = $e: expr, or $h: expr, $f: expr) => {
        let mut $n = {
            $crate::bind!($n = $e, or $h, $f);
            $n
        };
    };

}


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


/// Tells the `bind` macro whether the given expression yielded a bindable value or an error.
///
/// Enables the `bind` macro to determine by representing the value yielded
/// by the given expression as `Result` whether to create a variable and bind it to the value,
/// or to call the optional error handler and evaluate the execution flow control block.
///
/// Implemented by default for `Result` and `Option`, with `()` as `Error` for the latter.
///
/// For the usage example, refer to the `bind` macro documentation, which includes
/// an example of using it with user-defined types.
pub trait IntoResult {

    /// Type of the value that the `bind` macro binds the created variable to.
    type Value;
    /// Type of the error that the `bind` macro passes as the only argument
    /// to the optional error handler.
    type Error;

    /// Represents the yielded value as `Result`
    fn into_result(self) -> Result<Self::Value, Self::Error>;

}


impl<T> IntoResult for Option<T> {

    type Value = T;
    type Error = ();

    fn into_result(self) -> Result<Self::Value, Self::Error> {
        self.ok_or(())
    }

}


impl<T, E> IntoResult for Result<T, E> {

    type Value = T;
    type Error = E;

    fn into_result(self) -> Result<Self::Value, Self::Error> {
        self
    }

}
