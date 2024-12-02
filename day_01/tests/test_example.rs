use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Helper function to get test data directory
    fn get_test_data_path(file_name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("test_data");
        path.push(file_name);
        path
    }

    // Unit Tests
    #[test]
    fn test_parse_input_valid() -> Result<(), Box<dyn Error>> {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let (left, right) = parse_input(input)?;
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
        Ok(())
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "";
        let result = parse_input(input);
        assert!(result.is_ok());
        let (left, right) = result.unwrap();
        assert!(left.is_empty());
        assert!(right.is_empty());
    }

    #[test]
    fn test_parse_input_invalid_format() {
        let input = "3 4 5\n4 3";
        let result = parse_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_total_distance() {
        let left = vec![1, 2, 3, 3, 3, 4];
        let right = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(total_distance(left, right), 11);
    }

    #[test]
    fn test_total_distance_empty() {
        let left: Vec<i32> = vec![];
        let right: Vec<i32> = vec![];
        assert_eq!(total_distance(left, right), 0);
    }

    #[test]
    fn test_total_distance_single() {
        let left = vec![1];
        let right = vec![5];
        assert_eq!(total_distance(left, right), 4);
    }

    #[test]
    fn test_process_input_stage_one() -> Result<(), Box<dyn Error>> {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let result = process_input_stage_one(input)?;
        assert_eq!(result, 11);
        Ok(())
    }

    // Integration Tests
    #[test]
    fn test_example_case_from_file() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string(get_test_data_path("example_input.txt"))?;
        let result = process_input_stage_one(&input)?;
        assert_eq!(result, 11);
        Ok(())
    }

    #[test]
    fn test_empty_file() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string(get_test_data_path("empty_input.txt"))?;
        let result = process_input_stage_one(&input)?;
        assert_eq!(result, 0);
        Ok(())
    }

    #[test]
    fn test_invalid_input_file() {
        let input = fs::read_to_string(get_test_data_path("invalid_input.txt")).unwrap();
        let result = process_input_stage_one(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_large_numbers() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string(get_test_data_path("large_numbers.txt"))?;
        let result = process_input_stage_one(&input)?;
        assert!(result > 0);
        Ok(())
    }
}
