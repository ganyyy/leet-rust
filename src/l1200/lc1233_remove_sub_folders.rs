pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
    if folder.is_empty() {
        return vec![];
    }

    folder.sort();
    let mut ret = vec![folder[0].clone()];

    for folder in folder.iter().skip(1) {
        if let Some(last) = ret.last() {
            if folder.starts_with(last) && folder.as_bytes()[last.len()] == b'/' {
                continue; // Skip this folder as it is a subfolder of the last one
            }
            ret.push(folder.clone());
        }
    }

    ret
}
