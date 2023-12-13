use sptlc_ast::Pos;

pub struct CodeStream<'source> {
    source: &'source str,
    pos: Pos,
}

impl<'source> CodeStream<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            pos: Pos::default(),
        }
    }

    pub fn next(&mut self) -> char {
        let ch = self.current();

        self.pos.update(ch);

        ch
    }

    pub fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    pub fn index(&self) -> usize {
        self.pos.index()
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        self.index() == self.source.len()
    }

    // chars

    pub fn current(&self) -> char {
        self.source[self.pos.index()..].chars().next().unwrap()
    }

    pub fn check(&self, ch: char) -> bool {
        self.current() == ch
    }

    pub fn try_consume(&mut self, ch: char) -> bool {
        if self.check(ch) {
            self.next();

            true
        } else {
            false
        }
    }

    // slices

    pub fn slice(&self, start: usize, end: usize) -> &'source str {
        &self.source[start..end]
    }

    pub fn slice_from_current(&self, length: usize) -> &'source str {
        let start = self.pos.index();
        let end = start + length;

        &self.source[start..end]
    }

    pub fn check_slice(&self, slice: &str) -> bool {
        self.slice_from_current(slice.len()) == slice
    }
}
