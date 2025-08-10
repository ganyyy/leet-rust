use std::{collections::HashSet, sync::LazyLock};

static POWER_OF_2: LazyLock<HashSet<[i32; 10]>> = LazyLock::new(|| {
    let mut powers = HashSet::new();
    let mut p = 1;
    while p < 1_000_000_000 {
        powers.insert(count_digits(p));
        p *= 2;
    }
    powers
});

pub fn reordered_power_of2(n: i32) -> bool {
    let mut digits = count_digits(n);
    POWER_OF_2.contains(&digits)
}

fn count_digits(mut n: i32) -> [i32; 10] {
    let mut digits = [0; 10];
    while n > 0 {
        digits[(n % 10) as usize] += 1;
        n /= 10;
    }
    digits
}
