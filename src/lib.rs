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

struct MyStruct;

impl MyStruct {
    // 需要获取所有权
    fn consume(self) {
        println!("Consumed!");
    }

    // 需要一个不可变引用
    fn borrow(&self) {
        println!("Borrowed!");
    }

    // 需要一个可变引用
    fn borrow_mut(&mut self) {
        println!("Mutably borrowed!");
    }

    // 需要一个装箱的所有权
    fn boxed_consume(self: Box<Self>) {
        println!("Boxed consumed!");
    }
}

fn main() {
    let s = MyStruct;

    // 调用 consume 方法
    s.consume();

    let s = MyStruct;

    // 调用 borrow 方法
    s.borrow();

    let mut s = MyStruct;

    // 调用 borrow_mut 方法
    s.borrow_mut();

    let s = Box::new(MyStruct);

    // 调用 boxed_consume 方法
    s.boxed_consume();
}
