use std::sync::atomic::{AtomicUsize, Ordering};

/// Token is used to distinguish requests from different clients, internally
/// represented as a `usize`.
///
/// You can first create an allocator and then allocate a `Token`.
/// [`SyncTokenAllocator`] can be used in multi-threaded environments and
/// [`UnsyncTokenAllocator`] for single-threaded environments.
///
/// ```
/// # use rpcore_core::server::Token;
/// let allocator = Token::new_sync_allocator();
/// let token = allocator.alloc().unwrap();
///
/// let num: usize = token.into(); // Convert Token into raw
/// assert!(num > 0); // Allocated value is always > 0
/// ```
///
/// The default value `Token(0)` is reserved for an unallocated state or guest
/// identity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn is_guest(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Default)]
pub struct SyncTokenAllocator {
    counter: AtomicUsize,
}

impl SyncTokenAllocator {
    pub fn alloc(&self) -> Option<Token> {
        let val = 1 + self.counter.fetch_add(1, Ordering::AcqRel);

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
        self.counter += 1;
        let val = self.counter;

        if val == usize::MAX {
            None
        } else {
            Some(Token(val))
        }
    }
}

pub trait GetToken {
    fn token(&self) -> Token;
}

pub trait SetToken {
    fn set_token(&mut self, new: Token);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithToken<T> {
    pub token: Token,
    pub data: T,
}

impl<T> WithToken<T> {
    pub fn new(data: T) -> Self {
        Self {
            token: Default::default(),
            data,
        }
    }
}

impl<T> GetToken for WithToken<T> {
    fn token(&self) -> Token {
        self.token
    }
}

impl<T> SetToken for WithToken<T> {
    fn set_token(&mut self, new: Token) {
        self.token = new;
    }
}
