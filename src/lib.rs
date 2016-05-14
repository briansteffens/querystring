use std::collections::HashMap;

pub fn parse(querystring: &str) -> HashMap<String, String> {
    let mut ret = HashMap::new();

    for part in querystring.split("&") {
        if part == "" {
            continue;
        }

        let kv: Vec<&str> = part.split('=').collect();

        if kv.len() != 2 {
            panic!("Invalid querystring format");
        }

        ret.insert(String::from(kv[0]), String::from(kv[1]));
    }

    ret
}

#[cfg(test)]
mod tests {
    use parse;

    #[test]
    fn typical_case() {
        let parsed = parse("hi=hello&hey=7");

        assert_eq!(2, parsed.len());

        assert!(parsed.contains_key("hi"));
        assert_eq!(parsed.get("hi").unwrap(), "hello");

        assert!(parsed.contains_key("hey"));
        assert_eq!(parsed.get("hey").unwrap(), "7");
    }

    #[test]
    fn empty() {
        let parsed = parse("");

        assert_eq!(0, parsed.len());
    }
}
