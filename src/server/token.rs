use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token(pub(crate) usize);

impl Token {
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<Token> for usize {
    fn from(value: Token) -> Self {
        value.0
    }
}

pub trait WithToken {
    fn set_token(&mut self, new_token: Token);
    fn token(&self) -> Token;
}

#[derive(Debug, Default)]
pub struct TokenAllocator {
    counter: AtomicUsize,
}

impl TokenAllocator {
    fn new() -> Self {
        Default::default()
    }

    fn alloc(&self) -> Option<Token> {
        let token = self.counter.fetch_add(1, Ordering::Relaxed);
        if token == usize::MAX {
            None
        } else {
            Some(Token(token))
        }
    }
}

#[derive(Debug, Default)]
pub struct UnsyncTokenAllocator {
    counter: Cell<usize>,
}

impl UnsyncTokenAllocator {
    fn new() -> Self {
        Default::default()
    }

    fn alloc(&self) -> Option<Token> {
        let token = self.counter.get();
        self.counter.set(token.wrapping_add(1));

        if token == usize::MAX {
            None
        } else {
            Some(Token(token))
        }
    }
}
