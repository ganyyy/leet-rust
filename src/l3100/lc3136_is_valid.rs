pub fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        return false;
    }

    let mut has_vowel = false;
    let mut has_consonant = false;

    for c in word.chars() {
        match c {
            // 元音字母
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                has_vowel = true;
            }
            // 辅音字母
            'a'..='z' | 'A'..='Z' => {
                has_consonant = true;
            }
            // 数字
            '0'..='9' => {}
            // 其他字符都是无效的
            _ => return false,
        }
    }

    has_vowel && has_consonant
}
