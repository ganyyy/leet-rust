pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    if fruits.len() < 3 {
        return fruits.len() as i32;
    }

    let mut first = (0, -1);
    let mut second = (0, -1);
    let mut ret = 0;
    let mut start = 0;

    for (i, &fruit) in fruits.iter().enumerate() {
        if first.1 == -1 || first.0 == fruit {
            first.1 = i as i32;
            first.0 = fruit;
        } else if second.1 == -1 || second.0 == fruit {
            second.1 = i as i32;
            second.0 = fruit;
        } else {
            ret = ret.max(i as i32 - start);
            if first.1 < second.1 {
                start = first.1 + 1;
                first = (fruit, i as i32);
            } else {
                start = second.1 + 1;
                second = (fruit, i as i32);
            }
        }
    }

    ret.max(fruits.len() as i32 - start)
}
