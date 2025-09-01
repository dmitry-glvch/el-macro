use crate::bind;


#[test]
fn mut_with_handler() {

    bind!(mut x = Some(42), or |_| { }, unreachable!());
    x += 3;
    assert_eq!(x, 45);

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    {
        let mut y = 0;
        bind!(mut x = None::<i32>, or |_| y += 1, {
            assert_eq!(y, 1);
            return
        });
        unreachable!()
    }

}


#[test]
fn deref_and_shorthand() {

    let x = Box::new(Some(42));
    bind!(x, or return);
    assert_eq!(x, 42);

    use std::sync::{Arc, Mutex};

    let x = Arc::new(Mutex::new(45));
    bind!(mut x, or return);
    *x -= 3;
    assert_eq!(*x, 42);

}
