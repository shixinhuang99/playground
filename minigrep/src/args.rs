#[derive(Debug)]
pub struct Args {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Args {
    pub fn build(args: Vec<String>) -> Result<Args, &'static str> {
        let (options, args) = args_partition(args);

        let query = args
            .get(1)
            .cloned()
            .ok_or_else(|| error_msgs::MISSING_QUERY)?;

        let file_path = args
            .get(2)
            .cloned()
            .ok_or_else(|| error_msgs::MISSING_FILE_PATH)?;

        let ignore_case = options.contains(&"--ignore-case".to_string());

        Ok(Args {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn args_partition(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    args.into_iter()
        .partition(|ele| ele.starts_with("-") || ele.starts_with("--"))
}

mod error_msgs {
    pub const MISSING_QUERY: &str = "missing query";
    pub const MISSING_FILE_PATH: &str = "missing file path";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let args = vec![
            String::from("program_name"),
            String::from("query"),
            String::from("file_path"),
        ];
        let result = Args::build(args);
        assert!(result.is_ok());

        let args = vec![String::from("program_name"), String::from("query")];
        let result = Args::build(args);
        assert_eq!(error_msgs::MISSING_FILE_PATH, result.unwrap_err());

        let args = vec![String::from("program_name")];
        let result = Args::build(args);
        assert_eq!(error_msgs::MISSING_QUERY, result.unwrap_err());
    }

    #[test]
    fn test_build_ignore_case() {
        let args = vec![
            String::from("program_name"),
            String::from("query"),
            String::from("file_path"),
            String::from("--ignore-case"),
        ];
        let result = Args::build(args);
        assert_eq!(true, result.unwrap().ignore_case);

        let args = vec![
            String::from("program_name"),
            String::from("--ignore-case"),
            String::from("query"),
            String::from("file_path"),
        ];
        let result = Args::build(args);
        assert_eq!(true, result.unwrap().ignore_case);

        let args = vec![
            String::from("program_name"),
            String::from("query"),
            String::from("--ignore-case"),
            String::from("file_path"),
        ];
        let result = Args::build(args);
        assert_eq!(true, result.unwrap().ignore_case);

        let args = vec![
            String::from("program_name"),
            String::from("query"),
            String::from("file_path"),
        ];
        let result = Args::build(args);
        assert_eq!(false, result.unwrap().ignore_case);
    }
}
