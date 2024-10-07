#[allow(unused)]
pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    let mut max_heap = BinaryHeap::new();
    let mut fuel = start_fuel;
    let mut stops = 0;
    let mut prev = 0;

    ///
    /// 使用 Reverse 包装，使得 BinaryHeap 为最小堆
    /// ```
    /// let mut max_heap = BinaryHeap::new();
    /// ```
    for station in stations.iter().chain(std::iter::once(&vec![target, 0])) {
        let distance = station[0] - prev;
        while fuel < distance {
            if let Some(max) = max_heap.pop() {
                fuel += max; // refuel
                stops += 1; // add a stop
            } else {
                return -1;
            }
        }
        fuel -= distance;
        prev = station[0];
        max_heap.push(station[1]);
    }
    stops
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    #[test]
    fn test_min_heap() {
        let mut min_heap = std::collections::BinaryHeap::new();

        min_heap.push(Reverse(3));
        min_heap.push(Reverse(2));
        min_heap.push(Reverse(1));

        assert_eq!(min_heap.pop(), Some(Reverse(1)));
        assert_eq!(min_heap.pop(), Some(Reverse(2)));
        assert_eq!(min_heap.pop(), Some(Reverse(3)));

        min_heap.push(Reverse(4));
        let val = min_heap.pop();
        if let Some(val) = val {
            assert_eq!(val, Reverse(4));
        } else {
            panic!("min_heap.pop() should return Some(Reverse(4))");
        }
    }
}
