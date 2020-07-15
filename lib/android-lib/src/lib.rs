use std::os::raw::{c_char};

use node_lib::hello_world::{greeter_client::GreeterClient, HelloRequest};

use std::ffi::{CStr, CString};
use tokio::runtime::Runtime;

static mut RUNTIME: Option<Runtime> = None;
static mut CLIENT: Option<GreeterClient<tonic::transport::Channel>> = None;

#[no_mangle]
pub extern "C" fn hello(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(recp) => recp,
    };
    println!("Hello, {}!", recipient);
    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn connect(to: *const c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(why) => {
            println!("invalid address: {}", why);
            return false;
        },
        Ok(addr) => addr,
    };
    println!("Connecting to {}...", recipient);
    let mut rntm = Runtime::new().expect("tokio runtime not available");
    let greeter = rntm.block_on(GreeterClient::connect(recipient)).expect("failed to connect");
    println!("connected!");
    unsafe {
        CLIENT = Some(greeter);
        RUNTIME = Some(rntm);
    }
    true
}

#[no_mangle]
pub extern "C" fn send(name: *const c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(name) };
    let name = match c_str.to_str() {
        Err(why) => {
            println!("invalid address: {}", why);
            return false;
        },
        Ok(name) => name,
    };

    let request = tonic::Request::new(HelloRequest {
        name: name.into()
    });

    unsafe {
        if let Some(ref mut cl) = CLIENT {
            if let Some(ref mut rntm) = RUNTIME {
                let response = rntm.block_on(cl.say_hello(request)).expect("failed to say hello");
                println!("RESPONSE={:?}", response);
            }
        }
    };
    false
}
