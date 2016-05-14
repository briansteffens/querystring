use std::collections::HashMap;

pub fn parse(querystring: &str)
             -> Result<HashMap<String, String>, &'static str> {
    let mut ret = HashMap::new();

    for part in querystring.split("&") {
        if part == "" {
            continue;
        }

        let kv: Vec<&str> = part.split('=').collect();

        if kv.len() != 2 {
            return Err("Invalid querystring format");
        }

        ret.insert(String::from(kv[0]), String::from(kv[1]));
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use parse;

    #[test]
    fn typical_case() {
        let parsed = parse("hi=hello&hey=7").unwrap();

        assert_eq!(parsed.len(), 2);

        assert!(parsed.contains_key("hi"));
        assert_eq!(parsed.get("hi").unwrap(), "hello");

        assert!(parsed.contains_key("hey"));
        assert_eq!(parsed.get("hey").unwrap(), "7");
    }

    #[test]
    fn single_pair() {
        let parsed = parse("greetings=hello").unwrap();

        assert_eq!(parsed.len(), 1);

        assert!(parsed.contains_key("greetings"));
        assert_eq!(parsed.get("greetings").unwrap(), "hello");
    }

    #[test]
    fn no_pairs() {
        let parsed = parse("").unwrap();

        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn empty_value() {
        let parsed = parse("nothing=").unwrap();

        assert_eq!(parsed.len(), 1);

        assert!(parsed.contains_key("nothing"));
        assert_eq!(parsed.get("nothing").unwrap(), "");
    }

    #[test]
    fn extra_equal_signs_in_kvp() {
        assert!(parse("this=is=wrong").is_err());
    }
}
