pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
       fn test_colorize_output() {
           assert_eq!(colorize_output("test", "red").to_string(), "test".red().to_string());
       }
   
    #[test]
    fn test_display_welcome() {
        display_welcome("Test Project", "This is a test project.");
    }

    #[test]
    fn test_bold_heading() {
        assert_eq!(bold_heading("Heading").to_string(), "Heading".bold().to_string());
    }

}