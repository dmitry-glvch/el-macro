use super::*;


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
