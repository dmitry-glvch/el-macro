#[macro_export]
macro_rules! bind {

    ($($n: ident)+ = $e: expr, or $f: expr) => {
        let Ok($($n)+) = $crate::IntoResult::into_result($e) else { $f };
    };

    ($($n: ident)+ = $e: expr, or $h: expr, $f: expr) => {
        let $($n)+ = match $crate::IntoResult::into_result($e) {
            Ok($($n)+) => { $($n)+ },
            Err(err) => {
                $h(err);
                $f
            },
        };
    };

}


#[macro_export]
macro_rules! if_matches {

    ($e: expr, $p: pat => $f: expr) => {
        if let $p = $e {
            Some($f())
        } else {
            None
        }
    };

}


pub trait IntoResult {

    type Value;
    type Error;

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
