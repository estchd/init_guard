//! # INIT GUARD
//!
//! The Init_Guard Crate provides a Synchronization Primitive, that can be used to guard against double initialization.
//!
//! For this, the init_guard macro is exported.
//! The init_guard macro creates a new module that contains everything needed for the init_guard.
//!
//! The Module contains two public Methods, init() and has_init().
//!
//! ## has_init()
//!
//! The has_init function has the following definition:
//!
//! ```
//! fn has_init() -> bool
//! ```
//!
//! The has_init function returns true, if the init_guard was already initialized.
//!
//! ## init()
//!
//! The init function has the following definition:
//!
//! ```
//! fn init() -> Result<(),()>
//! ```
//!
//! The init function returns Ok, if the init_guard was succesfully initialized and Err, if it was already initialized before
//!
//! ## Usage Example
//!
//! ```
//! init_guard!(HAS_LOGGER_INIT); // Create the init_guard
//!
//! fn init_logger() -> Result<(),String> {
//!     match HAS_LOGGER_INIT::init() {
//!         Ok(_) => {},
//!         Err(_) => {return Err("Logger is already initialized!".to_string())}
//!     }
//!     // Everything after this is now safe from double initialization
//!
//!     // Do your actual logger initialization here
//! }
//! ```

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[macro_export]
macro_rules! init_guard {
    ($global_vis:vis $global_name:ident) => {
        $global_vis mod $global_name {
            use std::sync::{Mutex, Once};
            lazy_static! {
                static ref MUTEX_ONCE: Mutex<Once> = Mutex::<Once>::new(Once::new());
            }

            pub fn has_init() -> bool {
                let once = MUTEX_ONCE.lock().unwrap();
                return once.is_completed();
            }

            pub fn init() -> Result<(),()> {
                let once = MUTEX_ONCE.lock().unwrap();
                if once.is_completed() {return Err(()); }

                once.call_once(|| {});
                Ok(())
            }
        }
    }
}