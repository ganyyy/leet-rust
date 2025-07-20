use std::collections::HashMap;

struct Solution;

#[derive(Default)]
struct Trie {
    next: HashMap<String, Trie>,
    name: String,
}

fn generate(t: &mut Trie, m: &mut HashMap<String, usize>) {
    if t.next.is_empty() {
        return;
    }
    let mut exprs = Vec::with_capacity(t.next.len());
    for (s, child) in &mut t.next {
        generate(child, m);
        exprs.push(format!("{}({})", s, child.name));
    }
    exprs.sort_unstable();
    t.name = exprs.join("");

    *m.entry(t.name.clone()).or_default() += 1;
}

fn dfs(t: &Trie, m: &HashMap<String, usize>, path: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
    if m.get(&t.name).map_or(false, |&v| v > 1) {
        return;
    }
    if !path.is_empty() {
        ans.push(path.clone());
    }

    for (s, child) in &t.next {
        path.push(s.clone());
        dfs(child, m, path, ans);
        path.pop();
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = paths.len();
        let mut root = Trie::default();
        for p in paths {
            let mut t = &mut root;
            for s in p {
                t = t.next.entry(s).or_default();
            }
        }
        let mut m = HashMap::with_capacity(1024);
        generate(&mut root, &mut m);

        let mut ans = Vec::with_capacity(n);
        let mut path = Vec::with_capacity(n);
        dfs(&root, &m, &mut path, &mut ans);
        ans
    }
}
