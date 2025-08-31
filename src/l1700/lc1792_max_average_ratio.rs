#[derive(Eq, PartialEq, Debug)]
struct Class {
    pass: i64,
    total: i64,
}

impl std::cmp::Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // (pass+1)/(total+1) - pass/total = (total-pass)/(total*(total+1))
        ((self.total - self.pass) * other.total * (other.total + 1))
            .cmp(&((other.total - other.pass) * self.total * (self.total + 1)))
    }
}

impl std::cmp::PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let length = classes.len();
    let mut heap = std::collections::BinaryHeap::new();
    for c in classes {
        heap.push(Class {
            pass: c[0] as i64,
            total: c[1] as i64,
        });
    }

    for _ in 0..extra_students {
        if let Some(mut c) = heap.pop() {
            println!("Pop {:?}", c);
            c.pass += 1;
            c.total += 1;
            heap.push(c);
        }
    }

    let mut ret = 0.0;
    while let Some(c) = heap.pop() {
        ret += c.pass as f64 / c.total as f64;
    }
    ret / length as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_average_ratio() {
        let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let extra_students = 2;
        let result = max_average_ratio(classes, extra_students);
        println!("Result: {}", result);
    }
}
