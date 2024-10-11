#[allow(unused)]
pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;

    let mut cnt: HashMap<i32, i32> = HashMap::new();

    nums1.into_iter().filter(|&x| x % k == 0).for_each(|mut x| {
        x /= k;
        let mut d = 1;
        while d * d <= x {
            if x % d == 0 {
                *cnt.entry(d).or_insert(0) += 1;
                if d * d < x {
                    *cnt.entry(x / d).or_insert(0) += 1;
                }
            }
            d += 1;
        }
    });

    nums2.iter().map(|x| *cnt.get(x).unwrap_or(&0) as i64).sum()
}
