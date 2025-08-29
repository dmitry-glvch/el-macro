mod into_result;

#[cfg(test)]
mod test;


pub use into_result::IntoResult;


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
/// # use el_macro::{bind, bind::IntoResult};
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
        let Ok($n) = $crate::bind::IntoResult::into_result($e) else { $f };
    };

    (mut $n: ident = $e: expr, or $f: expr) => {
        let mut $n = {
            $crate::bind!($n = $e, or $f);
            $n
        };
    };

    ($n: ident = $e: expr, or $h: expr, $f: expr) => {
        let $n = match $crate::bind::IntoResult::into_result($e) {
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
