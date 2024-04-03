#[derive(Clone, Copy)]
pub struct Reader<'a> {
    source: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn init(source: &'a str) -> Self {
        Reader {
            source: source.as_bytes(),
            pos: 0,
        }
    }

    pub fn peek(&self, n: usize) -> Option<&[u8]> {
        if self.pos + n + 1 >= self.source.len() {
            return None;
        }

        Some(&self.source[self.pos..self.pos + n])
    }

    pub fn next(&mut self) -> Option<&'a u8> {
        if self.pos + 1 >= self.source.len() {
            return None;
        }
        self.pos += 1;
        Some(&self.source[self.pos + 1])
    }
}
