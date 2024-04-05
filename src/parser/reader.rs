#[derive(Clone)]
pub struct Reader {
    source: Vec<char>,
    pos: usize,
}

impl Reader {
    pub fn init(source: &str) -> Self {
        let chars = source.chars().to_owned();
        let buf = chars.collect::<Vec<char>>();
        Reader { source: buf, pos: 0 }
    }

    pub fn peek(&self, n: usize) -> Option<&[char]> {
        if self.pos + n - 1 >= self.source.len() {
            return None;
        }

        Some(&self.source[self.pos..self.pos + n])
    }

    pub fn next(&mut self, n: usize) -> Option<&[char]> {
        if self.pos + n - 1 >= self.source.len() {
            return None;
        }

        let next = Some(&self.source[self.pos..self.pos + n]);
        self.pos += n;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;

    #[test]
    fn test_reader() {
        let mut reader = Reader::init("javascript");
        let next = reader.next(4);
        assert!(next.is_some());
        assert_eq!(next.unwrap().into_iter().collect::<String>(), String::from("java"));
        let peek = reader.peek(6);
        assert!(peek.is_some());
        assert_eq!(peek.unwrap().into_iter().collect::<String>(), String::from("script"));
        let next = reader.next(6);
        assert!(next.is_some());
        assert_eq!(next.unwrap().into_iter().collect::<String>(), String::from("script"));
        let peek = reader.peek(1);
        assert!(peek.is_none());
        let next = reader.next(1);
        assert!(next.is_none());
    }
}
