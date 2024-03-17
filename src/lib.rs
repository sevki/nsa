//!  ![NSA](https://raw.githubusercontent.com/sevki/nsa/master/nsa.png)
//!
//! # NSA: Nosey Syscall Abstractions
//!  
//! NSA is a library that allows you to intercept and log system calls.
//! It uses the `dlsym` function to get the address of the original function and then calls it. This is a simple way to intercept and log system calls.
//!
//! ## Usage
//!
//! ```shell
//! cargo build --release  
//! LD_PRELOAD=./target/release/libnsa.so make
//! ```

// set docs rs logo and favicon
#![doc(html_logo_url = "https://raw.githubusercontent.com/sevki/nsa/master/nsa.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/sevki/nsa/master/nsa.png")]

use libc::c_char;
use std::ffi::CStr;

macro_rules! wrap_syscall {
    ($func_name:ident, $($arg_name:ident: $arg_type:ty),*) => {
        /// # Safety
        ///
        /// These functinos are inherently unsafe as they are calling system calls.
        ///
        #[no_mangle]
        pub unsafe extern "C" fn $func_name($($arg_name: $arg_type),*) -> i32 {
            let orig_func: unsafe extern "C" fn($($arg_type),*) -> i32 =
                unsafe { std::mem::transmute(libc::dlsym(libc::RTLD_NEXT, concat!(stringify!($func_name), "\0").as_ptr() as *const i8)) };

            println!("Intercepted {}", stringify!($func_name));
            println!("Arguments: ");
            $(
                println!(" {} {}: {}", stringify!($func_name), stringify!($arg_name), {
                    // check argtype is *const c_char
                    if stringify!($arg_type) == "*const c_char" {
                        if let Some(s) = unsafe { ($arg_name as *const c_char).as_ref() } {
                            let string = unsafe { CStr::from_ptr(s).to_string_lossy() };
                            format!("{:?}", string)
                        } else {
                            format!("{:?}", $arg_name)
                        }
                    } else {
                        format!("{:?}", $arg_name)
                    }
                });
            )*

            let result = orig_func($($arg_name),*);
            println!("Original {} returned {}", stringify!($func_name), result);

            result
        }
    };
    ($func_name:ident) => {
        /// # Safety
        ///
        /// These functinos are inherently unsafe as they are calling system calls.
        ///
        #[no_mangle]
        pub unsafe extern "C" fn $func_name() -> i32 {
            let orig_func: unsafe extern "C" fn() -> i32 =
                unsafe { std::mem::transmute(libc::dlsym(libc::RTLD_NEXT, concat!(stringify!($func_name), "\0").as_ptr() as *const i8)) };

            println!("Intercepted {}", stringify!($func_name));

            let result = orig_func();
            println!("Original {} returned {}", stringify!($func_name), result);

            result
        }
    };
}

// open
wrap_syscall!(open, path: *const c_char, flags: i32, mode: i32);
// openat
wrap_syscall!(openat, dirfd: i32, path: *const c_char, flags: i32, mode: i32);
// execve
wrap_syscall!(execve, path: *const c_char, argv: *const *const c_char, envp: *const *const c_char);
// access
wrap_syscall!(access, pathname: *const c_char, mode: i32);
// uname
wrap_syscall!(uname, buf: *mut libc::utsname);
// getuid
wrap_syscall!(getuid);
// getpid
wrap_syscall!(getpid);
// getppid
wrap_syscall!(getppid);
// fork
wrap_syscall!(fork);
