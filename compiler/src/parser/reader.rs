use crate::parser::ast::ExprKind::Primary;

#[derive(Clone, Debug)]
pub struct Reader<S> {
    source: Vec<S>,
    cursor: usize,
}

impl<S> Reader<S> where S: Clone + PartialEq {
    pub fn init(source: Vec<S>) -> Self {
        Reader { source, cursor: 0 }
    }

    pub fn peek_single(&self) -> Option<S> {
        match self.peek(1) {
            Some(char) => Some(char[0].clone()),
            None => None,
        }
    }

    pub fn peek(&self, n: usize) -> Option<Vec<S>> {
        if self.cursor + n - 1 >= self.source.len() {
            return None;
        }

        Some(self.source[self.cursor..self.cursor + n].to_vec())
    }

    pub fn next_single(&mut self) -> Option<S> {
        let next = match self.next(1) {
            Some(char) => Some(char[0].clone()),
            None => None,
        };
        next
    }

    pub fn next(&mut self, n: usize) -> Option<Vec<S>> {
        if self.cursor + n - 1 >= self.source.len() {
            return None;
        }

        let next = Some(&self.source[self.cursor..self.cursor + n]);
        self.cursor += n;
        next.map(|slice| slice.to_vec())
    }

    #[inline]
    pub fn bump(&mut self) {
        self.skip(1);
    }

    #[inline]
    pub fn skip(&mut self, n: usize) {
        self.cursor += n;
    }
    
    pub fn get_pos(&self) -> usize {
        self.cursor
    }

    pub fn collect_until(&mut self, until: S) -> Vec<S> {

        let mut collected: Vec<S> = Vec::new();

        loop {
            let peek = self.peek_single();
            match peek {
                Some(peek) => {
                    if self.source[self.cursor] == until {
                        return collected;
                    } else {
                        collected.push(peek.clone());
                        self.bump();
                    }
                }
                None => {
                    return collected;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;

    #[test]
    fn test_reader() {
        let mut reader = Reader::init("javascript".as_bytes().to_vec());
        let next = reader.next(4);
        assert!(next.is_some());
        assert_eq!(String::from_utf8(next.unwrap()).unwrap(), "java");
        let peek = reader.peek(6);
        assert!(peek.is_some());
        assert_eq!(String::from_utf8(peek.unwrap()).unwrap(), "script");
        let next = reader.next(6);
        assert!(next.is_some());
        assert_eq!(String::from_utf8(next.unwrap()).unwrap(), "script");
        let peek = reader.peek(1);
        assert!(peek.is_none());
        let next = reader.next(1);
        assert!(next.is_none());
    }

    #[test]
    fn test_collect_until() {
        let mut reader = Reader::init("Testing]".chars().collect());

        let testing = reader.collect_until(']');

        let expected: Vec<char> = "Testing".chars().collect();
        assert_eq!(testing, expected);

        assert_eq!(reader.next_single(), Some(']'));
    }
}
