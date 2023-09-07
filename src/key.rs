/// A typed wrapper for key of Ops submitted into driver
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key<T> {
    user_data: usize,
    _p: std::marker::PhantomData<fn(T)>,
}

impl<T> Unpin for Key<T> {}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self {
            user_data: self.user_data,
            _p: std::marker::PhantomData,
        }
    }
}

impl<T> Copy for Key<T> {}

impl<T> Key<T> {
    /// Create a new `Key` with the given user data. If this `Key` won't be used
    /// to poll data from drivers, use [`Key::new_dummy`] instead.
    ///
    /// # Safety
    ///
    /// Caller needs to ensure that `T` does correspond to `user_data` in driver
    /// this `Key` is created with.
    pub unsafe fn new(user_data: usize) -> Self {
        Self {
            user_data,
            _p: std::marker::PhantomData,
        }
    }

    /// Get the user data.
    pub fn user_data(&self) -> usize {
        self.user_data
    }
}

impl Key<()> {
    /// Create a new dummy `Key` with the given user data
    pub fn new_dummy(user_data: usize) -> Self {
        // Safety: `()` is not `OpCode` so this won't be used to poll data from drivers.
        unsafe { Self::new(user_data) }
    }
}

impl<T> std::ops::Deref for Key<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.user_data
    }
}
