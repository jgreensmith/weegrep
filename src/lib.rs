
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            v.push(line);
        }
    }
    v
}
pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let mut v = Vec::new();
    for line in contents.lines(){
        let l = line.to_lowercase();
        let q = query.to_lowercase();
        if l.contains(&q){
            v.push(line);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query_lower = "scoops";
        let query_upper = "scOOps";
        let contents = "\
There is a
scoops in this contents.
somewhere";
        assert_eq!(vec!["scoops in this contents."], search(query_lower, contents));
        let v: Vec<&str> = Vec::new();
        assert_eq!(v, search(query_upper, contents));
    }

    #[test]
    fn case_insensitive() {
        let q1 = "scOops";
        let q2 = "there";
        let q3 = "There";
        let contents = "\
There is a
scoops in this contents.
somewhere";
        assert_eq!(vec!["scoops in this contents."], search_case_insensitive(q1, contents));
        assert_eq!(vec!["There is a"], search_case_insensitive(q2, contents));
        assert_eq!(vec!["There is a"], search_case_insensitive(q3, contents));
    }
}