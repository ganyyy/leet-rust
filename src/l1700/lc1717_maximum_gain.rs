pub fn maximum_gain(s: String, mut x: i32, mut y: i32) -> i32 {
    use std::mem::swap;

    let (mut a, mut b) = (b'a', b'b');
    if x < y {
        swap(&mut a, &mut b);
        swap(&mut x, &mut y);
    }

    let (mut c1, mut c2) = (0, 0);
    let mut ret = 0;

    s.as_bytes().iter().for_each(|&c| {
        if c == a {
            c1 += 1;
        } else if c == b {
            if c1 > 0 {
                c1 -= 1;
                ret += x;
            } else {
                c2 += 1;
            }
        } else {
            ret += y * c1.min(c2);
            c1 = 0;
            c2 = 0;
        }
    });

    ret + y * c1.min(c2)
}
