pub fn valid_substring_count(word1: String, word2: String) -> i64 {
    if word1.len() < word2.len() {
        return 0;
    }

    let mut diff = [0i32; 26];

    word2
        .as_bytes()
        .iter()
        .for_each(|&c| diff[c as usize - 'a' as usize] += 1);

    let mut need = diff.iter().filter(|x| x.is_positive()).count() as i32;

    let mut left: i32 = 0;
    let mut ret: i64 = 0;

    let word1 = word1.as_bytes();

    word1.iter().for_each(|&c| {
        let idx = c as usize - 'a' as usize;
        diff[idx] -= 1;
        if diff[idx] == 0 {
            need -= 1;
        }

        while need == 0 {
            let left_char = word1[left as usize];
            let left_idx = left_char as usize - 'a' as usize;
            if diff[left_idx] == 0 {
                need += 1;
            }
            diff[left_idx] += 1;
            left += 1;
        }
        ret += left as i64;
    });

    ret
}
