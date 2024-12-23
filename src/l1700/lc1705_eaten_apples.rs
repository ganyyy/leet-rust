#[allow(unused)]
pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};

    assert!(apples.len() == days.len());

    let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();

    let mut res = 0;
    apples.into_iter().enumerate().for_each(|(cur, apple)| {
        // 先把已经过期的排除掉
        while let (Some(&(Reverse(day), apple))) = heap.peek() {
            // println!("pop day: {}, apple: {}", day, apple);
            if day <= cur as i32 {
                heap.pop();
            } else {
                break;
            }
        }
        // 当前的苹果数量大于0，就加入到堆中
        if apple > 0 {
            let end_day = days[cur] + cur as i32;
            // println!("push day: {}, apple: {}", end_day, apple);
            heap.push((Reverse(end_day), apple));
        }

        // 尝试吃一个苹果
        let mut pop = false;
        if let Some(mut p) = heap.peek_mut() {
            res += 1;
            if p.1 > 1 {
                p.1 -= 1;
            } else {
                pop = true;
            }
        }
        if pop {
            heap.pop();
        }
    });

    let mut end_day = days.len() as i32;
    while let (Some((Reverse(day), apple))) = heap.pop() {
        // println!("pop day: {}, apple: {}", day, apple);
        if day <= end_day {
            continue;
        }
        let add = (day - end_day).min(apple);
        end_day += add;
        res += add;
    }

    res
}

mod test {

    #[test]
    fn test_eaten_apples() {
        let ret = super::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]);
        println!("ret: {}", ret);
    }
}
