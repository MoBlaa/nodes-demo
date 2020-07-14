use std::os::raw::{c_char};

use crate::hello_world::greeter_client::GreeterClient;

use std::ffi::CStr;

static client: Option<GreeterClient> = None;

#[no_mangle]
pub extern "C" fn connect(to: *const c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(why) => {
            println!("Failed to create address from string: {}", why);
            "none"
        }
        Ok(addr) => addr,
    };
    println!("Connecting to {}...", recipient);
    true
}