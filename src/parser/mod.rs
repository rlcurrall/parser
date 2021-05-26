pub mod states;

pub struct Parser<S> {
    inner: S,
}

impl Parser<states::Default> {
    pub fn new() -> Self {
        Self {
            inner: states::Default {},
        }
    }
}

impl<S> Parser<S> {
    pub fn transition(&mut self, state: S) {
        self.inner = state
    }
}
