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


impl<'a, T> IntoResult for &'a std::sync::Mutex<T> {

    type Value = std::sync::MutexGuard<'a, T>;
    type Error = std::sync::PoisonError<Self::Value>;

    fn into_result(self) -> Result<Self::Value, Self::Error> {
        self.lock()
    }

}
