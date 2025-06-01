mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_naive() {
        let source = r#"{"some": 23, "other": null, "something": "something", "double": 213.2, "other": [2343, true, false, "testing", {"other":null, "root": "tree"}], "sub": { "demo": "demo 2" } }"#;
        let (_, pr) = parser::json(source).unwrap();
    }

    #[test]
    fn test_bigger_json() {
        let source = include_str!("../tests/example1.json");
        let (_, pr) = parser::json(source).unwrap();
    }
}
