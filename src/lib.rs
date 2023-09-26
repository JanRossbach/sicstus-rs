//! High Level API for SICStus Prolog.

mod conversions;
pub mod error;
pub mod sp;
pub mod term;
mod util;

#[cfg(test)]
mod test_utils {
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    pub static TEST_ATOM_NAME_STR: &str = "test\0";
    // The mutex is needed to sync the global mock expectations. See https://docs.rs/mockall/latest/mockall/#static-methods
    lazy_static! {
        pub static ref MTX: Mutex<()> = Mutex::new(());
    }

    // When a test panics, it will poison the Mutex. Since we don't actually
    // care about the state of the data we ignore that it is poisoned and grab
    // the lock regardless.  If you just do `let _m = &MTX.lock().unwrap()`, one
    // test panicking will cause all other tests that try and acquire a lock on
    // that Mutex to also panic.
    pub fn get_lock() -> std::sync::MutexGuard<'static, ()> {
        match MTX.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}
