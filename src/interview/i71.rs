use crate::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let names: Vec<&str> = path.split("/").collect();
        let mut stack = vec![];

        for name in names {
            if name == ".." {
                stack.pop();
            } else if name != "." && !name.is_empty() {
                stack.push(name);
            }
        }

        format!("/{}", stack.join("/"))
    }
}
