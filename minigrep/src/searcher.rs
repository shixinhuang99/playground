use std::error::Error;
use std::fs;

use crate::args::Args;

pub fn search(args: Args) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(&args.file_path)?;

    let res = search_from_content(&args, content);

    Ok(res)
}

fn search_from_content(args: &Args, content: String) -> Vec<String> {
    let Args {
        query, ignore_case, ..
    } = args;

    if *ignore_case {
        return find_ignore_case(query, content);
    }

    find(query, content)
}

fn find(query: &String, content: String) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| line.contains(query).then_some(line.to_string()))
        .collect()
}

fn find_ignore_case(query: &String, content: String) -> Vec<String> {
    let query = query.to_lowercase();

    content
        .lines()
        .filter_map(|line| {
            line.to_lowercase()
                .contains(&query)
                .then_some(line.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_from_content() {
        let args = Args {
            query: "hello".to_string(),
            file_path: "test".to_string(),
            ignore_case: false,
        };
        let content = "hello world\nfoo bar\nhello\nbaz".to_string();
        let res = search_from_content(&args, content);
        assert_eq!(res, vec!["hello world", "hello"]);
    }

    #[test]
    fn test_search_from_content_empty_content() {
        let args = Args {
            query: "hello".to_string(),
            file_path: "test".to_string(),
            ignore_case: false,
        };
        let content = "".to_string();
        let res = search_from_content(&args, content);
        assert_eq!(res, Vec::<String>::new());
    }

    #[test]
    fn test_search_from_content_ignore_case() {
        let args = Args {
            query: "HELLO".to_string(),
            file_path: "test".to_string(),
            ignore_case: true,
        };
        let content = "hello world\nfoo bar\nHELLO\nbaz".to_string();
        let res = search_from_content(&args, content);
        assert_eq!(res, vec!["hello world", "HELLO"]);
    }
}
