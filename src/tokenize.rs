struct Tokenizer<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        if self.remaining.starts_with(['(', ')']) {
            let token = &self.remaining[..1];
            self.remaining = &self.remaining[1..];
            Some(token)
        } else if let Some(i) = self.remaining.find([' ', '(', ')']) {
            let (token, remaining) = self.remaining.split_at(i);
            self.remaining = remaining.trim_start();
            Some(token)
        } else {
            let token = self.remaining;
            self.remaining = "";
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer {
            remaining: "(+ 1 2)",
        };

        let tokens: Vec<&str> = tokenizer.collect();

        assert_eq!(tokens, vec!["(", "+", "1", "2", ")"]);
    }
}
