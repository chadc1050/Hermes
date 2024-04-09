#[derive(Clone)]
pub struct Reader {
    source: Vec<char>,
    cursor: usize,
}

impl Reader {
    pub fn init(source: &str) -> Self {
        let chars = source.chars().to_owned();
        let buf = chars.collect::<Vec<char>>();
        Reader { source: buf, cursor: 0 }
    }

    pub fn peek_single(&self) -> Option<char> {
        match self.peek(1) {
            Some(char) => Some(char[0]),
            None => None,
        }
    }

    pub fn peek(&self, n: usize) -> Option<&[char]> {
        if self.cursor + n - 1 >= self.source.len() {
            return None;
        }

        Some(&self.source[self.cursor..self.cursor + n])
    }

    pub fn next_single(&mut self) -> Option<char> {
        let next = match self.next(1) {
            Some(char) => Some(char[0]),
            None => None,
        };
        next
    }

    pub fn next(&mut self, n: usize) -> Option<&[char]> {
        if self.cursor + n - 1 >= self.source.len() {
            return None;
        }

        let next = Some(&self.source[self.cursor..self.cursor + n]);
        self.cursor += n;
        next
    }

    #[inline]
    pub fn bump(&mut self) {
        self.skip(1);
    }

    #[inline]
    pub fn skip(&mut self, n: usize) {
        self.cursor += n;
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
