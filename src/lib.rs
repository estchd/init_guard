#[macro_export]
macro_rules! init_guard {
    ($global_vis:vis $global_name:ident) => {
        $global_vis mod $global_name {
            use std::sync::{Mutex, Once};
            static $global_name: Mutex<Once> = Mutex::<Once>::new(Once::new());

            pub fn has_init() -> bool {
                let init_once = $global_name.lock().unwrap();
                return init_once.is_completed();
            }

            pub fn init() -> Result<(),()> {
                let init_once = $global_name.lock().unwrap();
                if init_once.is_completed() {return Err(()); }

                init_once.call_once(|| {});
                Ok(())
            }
        }
    }
}
