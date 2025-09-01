//! The [`crate::bind!`] macro and related [`crate::bind::IntoResult`] trait


mod into_result;

#[cfg(test)]
mod test;


pub use into_result::IntoResult;


/// Binds the unwrapped value.
///
/// Provides the ability to write concise code to get the value or get goin' in a context where
/// [ErrorPropagationExpression (`?`)](https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try)
/// is not sufficient, such as when controlling execution flow with `break` or `continue`,
/// or not applicable, as in function that does not return [`Result`] or [`Option`].
///
/// [Tests](IntoResult) whether the value of the provided expression can be unwrapped.
/// Creates a variable binding if the value can be unwrapped. Otherwise, executes
/// the error handler and evaluates the execution flow control expression.
///
/// # Syntax
///
/// ```text
/// bind!([mut] <var-name> [= <value-expr>], or [<err-handler>,] <flow-ctl>);
/// ```
///
/// - `mut` — indicator keyword to make the binding mutable.
/// - `<var-name>` — name of the newly created variable.
/// - `<value-expr>` — expression whose value is [being tested](IntoResult) to contain
///   an unwrappable value. If not specified, the existing value of the variable `<var-name>`
///   will be used to create new variable with the same name.
/// - `<err-handler>` — optional error handler that is called if there's no value to unwrap,
///   with error object passed as the only argument.
/// - `<flow-ctl>` — expression used to control the execution flow in a case
///   when there's no value to unwrap.
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
/// Omitting the `<value-expr>`:
/// ```
/// # use el_macro::bind;
/// #
/// let x = Some(42);
/// bind!(x /* = x */, or return);
/// assert_eq!(x, 42);
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
///     if error_code >= 0 { "no error" } else { "unknown error "}.into()
/// }
/// ```
#[macro_export]
macro_rules! bind {

    ($n: ident = $e: expr, or $f: expr) => {
        let $n = {
            use $crate::bind::IntoResult;
            match $e.into_result() {
                Ok(val) => val,
                Err(_) => $f,
            }
        };
    };

    (mut $n: ident = $e: expr, or $f: expr) => {
        let mut $n = {
            $crate::bind!($n = $e, or $f);
            $n
        };
    };

    ($n: ident, or $f: expr) => {
        $crate::bind!($n = $n, or $f);
    };

    (mut $n: ident, or $f: expr) => {
        $crate::bind!(mut $n = $n, or $f);
    };

    ($n: ident = $e: expr, or $h: expr, $f: expr) => {
        let $n = {
            use $crate::bind::IntoResult;
            match $e.into_result() {
                Ok($n) => { $n },
                Err(err) => {
                    $h(err);
                    $f
                },
            }
        };
    };

    (mut $n: ident = $e: expr, or $h: expr, $f: expr) => {
        let mut $n = {
            $crate::bind!($n = $e, or $h, $f);
            $n
        };
    };

    ($n: ident, or $h: expr, $f: expr) => {
        $crate::bind!($n = $n, or $h, $f);
    };

    (mut $n: ident, or $h: expr, $f: expr) => {
        $crate::bind!(mut $n = $n, or $h, $f);
    };

}
