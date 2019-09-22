pub fn build_format(format_content: Vec<String>) -> Vec<(String, usize)> {
    let mut format_fields = Vec::new();
    for l in format_content {
        let mut s = l.split(',');
        let first_field = s.next();
        let second_field = s.next();
        match (first_field, second_field) {
                    (Some(ff), Some(sf)) => {
                        format_fields.push((ff.to_string(), sf.to_string().parse::<usize>().unwrap()));
                    }
                    _ => println!("Ignoring malformed format line '{}'. Format lines must be in the format <FIELD,LENGTH>.", l),
                }
    }
    format_fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_format_content_yields_empty_format() {
        let empty_format: Vec<String> = Vec::new();
        let empty_fields: Vec<(String, usize)> = Vec::new();
        assert_eq!(empty_fields, build_format(empty_format));
    }

    #[test]
    fn simple_format() {
        let mut simple_format: Vec<String> = Vec::new();
        simple_format.push("Direction,1".to_string());
        simple_format.push("Speed,5".to_string());

        let mut expected_fields: Vec<(String, usize)> = Vec::new();
        expected_fields.push(("Direction".to_string(), 1));
        expected_fields.push(("Speed".to_string(), 5));
        assert_eq!(expected_fields, build_format(simple_format));
    }
}
