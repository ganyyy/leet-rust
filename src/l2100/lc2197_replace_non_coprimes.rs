pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            let temp = a;
            a = b % a;
            b = temp;
        }
        b
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a / gcd(a, b) * b
    }

    let mut stack = Vec::new();
    for mut num in nums {
        while let Some(&last) = stack.last() {
            if gcd(last, num) == 1 {
                break;
            }
            num = lcm(last, num);
            stack.pop();
        }
        stack.push(num);
    }
    stack
}
