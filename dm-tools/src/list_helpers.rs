pub fn insert_last_and_dedup(list: &[String], elem: &str) -> Vec<String> {
    let mut res = vec![];
    for x in list {
        if x != elem {
            res.push(x.to_string())
        }
    }
    res.push(elem.to_string());
    res
}

pub fn remove_non_existing(list: &[String]) -> Vec<String> {
    let mut res = vec![];
    for x in list {
        let path = std::path::Path::new(x);
        if path.exists() {
            res.push(x.to_string())
        }
    }
    res
}
