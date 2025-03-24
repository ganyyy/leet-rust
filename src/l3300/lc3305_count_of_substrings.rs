impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        // 至少包含k个元音字母的子串数量
        fn f(word: String, k: i32) -> i64 {
            const VOWEL_MASK: i32 = 1065233;
            let mut vowel_count = [0; 5];
            let mut vowel_size = 0;
            let mut consonant_count = 0;
            let mut ans = 0i64;
            let mut left = 0;

            let bs = word.as_bytes();
            bs.iter().for_each(|&b| {
                if let Some(idx) = Solution::index(b) {
                    if vowel_count[idx] == 0 {
                        vowel_size += 1;
                    }
                    vowel_count[idx] += 1;
                } else {
                    consonant_count += 1;
                }

                while vowel_size == 5 && consonant_count >= k {
                    if let Some(idx) = Solution::index(bs[left]) {
                        vowel_count[idx] -= 1;
                        if vowel_count[idx] == 0 {
                            vowel_size -= 1;
                        }
                    } else {
                        consonant_count -= 1;
                    }
                    left += 1;
                }
                ans += left as i64;
            });
            ans
        }

        f(word.clone(), k) - f(word, k + 1)
    }

    fn index(b: u8) -> Option<usize> {
        match b {
            b'a' => Some(0),
            b'e' => Some(1),
            b'i' => Some(2),
            b'o' => Some(3),
            b'u' => Some(4),
            _ => None,
        }
    }

    fn fold(str: String) -> bool {
        let mut flag = -1;
        str.as_bytes().iter().fold(0, |v, c| {
            flag *= -1;
            v + flag * (*c as i32 - 'a' as i32)
        }) == 0
    }
}

pub struct Solution;
