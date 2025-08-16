use std::env;
use std::fs;
use std::error::Error;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
        
    }
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        
        args.next();
        
        let query = match args.next() {
            Some(x) => x,
            None => return Err("nary a query")
        };

        let file_path = match args.next() {
            Some(x) => x,
            None => return Err("nary a file_path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(query))
        .collect()
}


fn search_case_insensitive<'a>(
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