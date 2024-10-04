mod l1900;
mod l2000;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::boxed;

    use super::*;

    #[test]
    fn it_works() {
        let _ = <[i32]>::into_vec(boxed::Box::new([1, 2, 3]));
        let bx: Box<[i32]> = Box::new([1, 2, 3]);
        let _ = bx.into_vec();

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
