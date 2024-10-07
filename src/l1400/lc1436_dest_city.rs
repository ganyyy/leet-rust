#[allow(unused)]
pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    use std::collections::HashSet;
    let mut set_a = HashSet::with_capacity(paths.len());
    let mut set_b = HashSet::new();

    for path in paths.iter() {
        let (start, end) = (&path[0], &path[1]);
        set_b.remove(start);
        if !set_a.contains(end) {
            set_b.insert(end);
        }
        set_a.insert(start);
    }

    if let Some(city) = set_b.iter().next() {
        city.to_string()
    } else {
        unreachable!()
    }
}
