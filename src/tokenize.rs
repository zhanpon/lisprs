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
        if self.remaining.is_empty() {
            return None;
        }

        let (token, remaining) = if self.remaining.starts_with(['(', ')']) {
            self.remaining.split_at(1)
        } else if let Some(i) = self.remaining.find([' ', '(', ')']) {
            let (t, r) = self.remaining.split_at(i);
            (t, r.trim_start())
        } else {
            (self.remaining, "")
        };

        self.remaining = remaining;
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

        assert_eq!(
            tokenize("(+ 1 (* 2 3))"),
            vec!["(", "+", "1", "(", "*", "2", "3", ")", ")"]
        );
    }
}
