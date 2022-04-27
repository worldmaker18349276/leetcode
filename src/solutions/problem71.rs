pub trait Problem71 {
    fn simplify_path(path: String) -> String;
}

struct Solution;

impl Problem71 for Solution {
    fn simplify_path(path: String) -> String {
        let mut res: Vec<&str> = Vec::new();
        for name in path.split('/').skip(1) {
            match name {
                "" | "." => continue,
                ".." => {
                    res.pop();
                }
                _ => {
                    res.push(name);
                }
            }
        }
        
        if res.is_empty() {
            return String::from("/");
        }

        res.insert(0, "");
        res.join("/")
    }
}
