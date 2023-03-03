struct Tokenizer<'a> {
    remaining: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer { remaining: value }
    }
}

fn scan_one_token(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();

    if s.is_empty() {
        None
    } else if s.starts_with(['(', ')']) {
        Some(s.split_at(1))
    } else if let Some(i) = s.find([' ', '(', ')']) {
        Some(s.split_at(i))
    } else {
        Some((s, ""))
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        scan_one_token(self.remaining).map(|(token, remaining)| {
            self.remaining = remaining;
            token
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(s: &str) -> Vec<&str> {
        let tokenizer = Tokenizer::from(s);

        tokenizer.collect()
    }

    #[test]
    fn test_tokenizer() {
        assert_eq!(tokenize("(+ 1 2)"), vec!["(", "+", "1", "2", ")"]);
        assert_eq!(tokenize("(+  1 2)"), vec!["(", "+", "1", "2", ")"]);
        assert_eq!(tokenize(" (+  1 2)"), vec!["(", "+", "1", "2", ")"]);
        assert_eq!(tokenize(" ( + 1 2)"), vec!["(", "+", "1", "2", ")"]);

        assert_eq!(
            tokenize("(+ 1 (* 2 3))"),
            vec!["(", "+", "1", "(", "*", "2", "3", ")", ")"]
        );
    }
}
