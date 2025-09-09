pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    let num = languages.len(); // 人数
    let n = n as usize; // 语言数量
    let mut learned = vec![vec![false; n + 1]; num];
    for (i, lang) in languages.iter().enumerate() {
        for &l in lang {
            learned[i][l as usize] = true;
        }
    }

    let mut total = 0; // 需要教的人数
    let mut visited = vec![false; num];
    let mut count = vec![0; n + 1];

    let mut add = |n: usize| {
        if !visited[n] {
            total += 1;
            visited[n] = true;
            for &lang in &languages[n] {
                count[lang as usize] += 1;
            }
        }
    };

    for f in friendships {
        let a = f[0] as usize - 1;
        let b = f[1] as usize - 1;
        let mut can_communicate = false;
        for &lang in &languages[a] {
            if learned[b][lang as usize] {
                can_communicate = true;
                break;
            }
        }
        if can_communicate {
            continue;
        }
        add(a);
        add(b);
    }

    total - *count.iter().max().unwrap()
}
