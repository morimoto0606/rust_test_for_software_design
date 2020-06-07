use regex::Regex;

pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct FixedStringMatcher {
    pattern: String,
}
impl FixedStringMatcher {
    pub fn new(pattern: String) -> FixedStringMatcher{
        FixedStringMatcher{pattern: pattern}
    }
}

impl MatcherTrait for FixedStringMatcher {
    fn execute(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

pub struct ExtendRegexpMatcher {
    pattern: Regex,
}

impl ExtendRegexpMatcher {
    pub fn new(pattern: String) -> ExtendRegexpMatcher {
        ExtendRegexpMatcher {
            pattern: Regex::new(&pattern).unwrap(),
        }
    }
}

impl MatcherTrait for ExtendRegexpMatcher {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

pub enum Matcher {
    ExtendedRegexp(ExtendRegexpMatcher),
    FixedString(FixedStringMatcher),
}

impl Matcher {
    pub fn new(pattern: String, is_fixed_string_mode:bool) -> 
    Matcher{
        if is_fixed_string_mode {
            Matcher::FixedString(FixedStringMatcher::new(pattern.to_string()))
        } else {
            Matcher::ExtendedRegexp(ExtendRegexpMatcher::new(pattern.to_string()))
        }
    }

    pub fn execute(&self, line: &str) -> bool {
        match self {
            Matcher::FixedString(m) => m.execute(line),
            Matcher::ExtendedRegexp(m)=> m.execute(line)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_extended_regexp_matcher() {
        let matcher = Matcher::new("Z".to_string(), false);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+.".to_string(), true);
        assert_eq!(false, matcher.execute("aaa bbb"));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
