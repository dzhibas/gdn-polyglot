/// need to tokenize json with start & end for just strings
/// ignoring everything else, and then if needed rewrite it

/// {
///  "translate": "demo,or,obj.demo",
///  "demo": "string to translate",
///  "or": ["string 1", "String 2"],
///  "if": ["string3"],
///  "obj": {
///      "demo": "one more"
///  }
/// }

mod parser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_naive() {
        let source = r"{'something': 'something' }";
        dbg!(source);
        //assert_eq!(result, 4);
    }
}
