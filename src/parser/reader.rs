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
        self.pos += 1;
        Some(&self.source[self.pos..self.pos + n])
    }
}
