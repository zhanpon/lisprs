struct Tokenizer<'a> {
    remaining: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer {
            remaining: value.trim_start(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.remaining;
        if s.is_empty() {
            return None;
        }

        let (token, remaining) = if s.starts_with(['(', ')']) {
            s.split_at(1)
        } else if let Some(i) = s.find([' ', '(', ')']) {
            s.split_at(i)
        } else {
            (s, "")
        };

        self.remaining = remaining.trim_start();
        Some(token)
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
