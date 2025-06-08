use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Token(pub(crate) usize);

impl From<Token> for usize {
    fn from(value: Token) -> Self {
        value.0
    }
}

impl Token {
    pub fn new_sync_allocator() -> SyncTokenAllocator {
        Default::default()
    }

    pub fn new_unsync_allocator() -> UnsyncTokenAllocator {
        Default::default()
    }
}

pub trait GetToken {
    fn token(&self) -> Token;
}

pub trait SetToken {
    fn set_token(&mut self, new: Token);
}

#[derive(Default)]
pub struct SyncTokenAllocator {
    counter: AtomicUsize,
}

impl SyncTokenAllocator {
    pub fn alloc(&self) -> Option<Token> {
        let val = self.counter.fetch_add(1, Ordering::AcqRel);

        if val == usize::MAX {
            None
        } else {
            Some(Token(val))
        }
    }
}

#[derive(Default)]
pub struct UnsyncTokenAllocator {
    counter: usize,
}

impl UnsyncTokenAllocator {
    pub fn alloc(&mut self) -> Option<Token> {
        let val = self.counter;
        self.counter += 1;

        if val == usize::MAX {
            None
        } else {
            Some(Token(val))
        }
    }
}
