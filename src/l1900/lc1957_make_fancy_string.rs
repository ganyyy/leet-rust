struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut anst = String::with_capacity(s.len());
        let mut count = 0;
        let mut prev_char = '\0';
        for c in s.chars() {
            if c == prev_char {
                count += 1;
            } else {
                count = 1;
                prev_char = c;
            }
            if count < 3 {
                anst.push(c);
            }
        }
        anst
    }
}
