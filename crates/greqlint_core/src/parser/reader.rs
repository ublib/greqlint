pub struct Reader<'a> {
    source: &'a str,
    pub pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        Reader {
            pos: 0,
            source: input,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.peek_n(1)
    }

    pub fn peek_n(&self, n: usize) -> Option<char> {
        self.source.chars().nth(self.pos + n - 1)
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn skip_until(&mut self, c: char) {
        while let Some(cur) = self.peek() {
            if cur == c {
                break;
            }
            self.next();
        }
    }
}

#[cfg(test)]
mod reader_tets {
    use super::*;
    #[test]
    fn test_reader() {
        let mut reader = Reader::new("abc");
        assert_eq!(reader.next(), Some('a'));
        assert_eq!(reader.next(), Some('b'));
        assert_eq!(reader.next(), Some('c'));
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn test_peek() {
        let mut reader = Reader::new("abc");
        assert_eq!(reader.peek(), Some('a'));
        assert_eq!(reader.peek(), Some('a'));
        assert_eq!(reader.next(), Some('a'));
        assert_eq!(reader.peek(), Some('b'));
        assert_eq!(reader.peek(), Some('b'));
        assert_eq!(reader.next(), Some('b'));
        assert_eq!(reader.peek(), Some('c'));
        assert_eq!(reader.peek(), Some('c'));
        assert_eq!(reader.next(), Some('c'));
        assert_eq!(reader.peek(), None);
        assert_eq!(reader.peek(), None);
        assert_eq!(reader.next(), None);
        assert_eq!(reader.peek(), None);
        assert_eq!(reader.peek(), None);
    }

    #[test]
    fn test_peek_n() {
        let reader = Reader::new("abc");
        assert_eq!(reader.peek_n(1), Some('a'));
        assert_eq!(reader.peek_n(2), Some('b'));
        assert_eq!(reader.peek_n(3), Some('c'));
        assert_eq!(reader.peek_n(4), None);
        assert_eq!(reader.peek_n(5), None);
        assert_eq!(reader.peek_n(6), None);
    }

    #[test]
    fn test_skip() {
        let mut reader = Reader::new("abc");
        reader.skip(1);
        assert_eq!(reader.peek(), Some('b'));
        reader.skip(1);
        assert_eq!(reader.peek(), Some('c'));
        reader.skip(1);
        assert_eq!(reader.peek(), None);
        reader.skip(1);
        assert_eq!(reader.peek(), None);
    }

    #[test]
    fn test_skip_until() {
        let mut reader = Reader::new("aaaaaaaaabcccccccc");
        reader.skip_until('b');
        assert_eq!(reader.peek(), Some('b'));
        reader.skip_until('c');
        assert_eq!(reader.peek(), Some('c'));
        reader.skip_until('d');
        assert_eq!(reader.peek(), None);
    }
}
