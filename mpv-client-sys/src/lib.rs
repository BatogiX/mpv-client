#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_os = "windows")]
mod sym {
    use super::*;

    // Define function pointers that mpv will initialize
    #[no_mangle]
    pub static mut pfn_mpv_client_api_version: Option<unsafe extern "C" fn() -> ::std::os::raw::c_ulong> = None;

    #[no_mangle]
    pub static mut pfn_mpv_error_string: Option<unsafe extern "C" fn(::std::os::raw::c_int) -> *const ::std::os::raw::c_char> = None;

    #[no_mangle]
    pub static mut pfn_mpv_free: Option<unsafe extern "C" fn(*mut ::std::os::raw::c_void)> = None;

    #[no_mangle]
    pub static mut pfn_mpv_client_name: Option<unsafe extern "C" fn(*mut mpv_handle) -> *const ::std::os::raw::c_char> = None;

    #[no_mangle]
    pub static mut pfn_mpv_client_id: Option<unsafe extern "C" fn(*mut mpv_handle) -> i64> = None;

    #[no_mangle]
    pub static mut pfn_mpv_create: Option<unsafe extern "C" fn() -> *mut mpv_handle> = None;

    #[no_mangle]
    pub static mut pfn_mpv_initialize: Option<unsafe extern "C" fn(*mut mpv_handle) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_destroy: Option<unsafe extern "C" fn(*mut mpv_handle)> = None;

    #[no_mangle]
    pub static mut pfn_mpv_terminate_destroy: Option<unsafe extern "C" fn(*mut mpv_handle)> = None;

    #[no_mangle]
    pub static mut pfn_mpv_create_client: Option<unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> *mut mpv_handle> = None;

    #[no_mangle]
    pub static mut pfn_mpv_create_weak_client: Option<unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> *mut mpv_handle> = None;

    #[no_mangle]
    pub static mut pfn_mpv_wait_event: Option<unsafe extern "C" fn(*mut mpv_handle, f64) -> *mut mpv_event> = None;

    #[no_mangle]
    pub static mut pfn_mpv_command: Option<unsafe extern "C" fn(*mut mpv_handle, *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_command_ret: Option<unsafe extern "C" fn(*mut mpv_handle, *mut *const ::std::os::raw::c_char, *mut mpv_node) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_command_async: Option<unsafe extern "C" fn(*mut mpv_handle, u64, *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_set_property: Option<unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char, mpv_format, *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_get_property: Option<unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char, mpv_format, *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_observe_property: Option<unsafe extern "C" fn(*mut mpv_handle, u64, *const ::std::os::raw::c_char, mpv_format) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_unobserve_property: Option<unsafe extern "C" fn(*mut mpv_handle, u64) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_hook_add: Option<unsafe extern "C" fn(*mut mpv_handle, u64, *const ::std::os::raw::c_char, ::std::os::raw::c_int) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_hook_continue: Option<unsafe extern "C" fn(*mut mpv_handle, u64) -> ::std::os::raw::c_int> = None;

    #[no_mangle]
    pub static mut pfn_mpv_event_name: Option<unsafe extern "C" fn(mpv_event_id) -> *const ::std::os::raw::c_char> = None;

    #[no_mangle]
    pub static mut pfn_mpv_free_node_contents: Option<unsafe extern "C" fn(*mut mpv_node)> = None;

    macro_rules! wrap {
        (fn $name:ident($($arg:ident: $ty:ty),*)) => {
            #[inline(always)]
            pub unsafe fn $name($($arg: $ty),*) {
                paste::paste! {
                    [<pfn_ $name>].unwrap()($($arg),*)
                }
            }
        };
        (fn $name:ident($($arg:ident: $ty:ty),*) -> $ret:ty) => {
            #[inline(always)]
            pub unsafe fn $name($($arg: $ty),*) -> $ret {
                paste::paste! {
                    [<pfn_ $name>].unwrap()($($arg),*)
                }
            }
        };
    }

    wrap!(fn mpv_create() -> *mut mpv_handle);
    wrap!(fn mpv_initialize(ctx: *mut mpv_handle) -> ::std::os::raw::c_int);
    wrap!(fn mpv_destroy(ctx: *mut mpv_handle));
    wrap!(fn mpv_terminate_destroy(ctx: *mut mpv_handle));
    wrap!(fn mpv_create_client(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char) -> *mut mpv_handle);
    wrap!(fn mpv_create_weak_client(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char) -> *mut mpv_handle);
    wrap!(fn mpv_wait_event(ctx: *mut mpv_handle, timeout: f64) -> *mut mpv_event);
    wrap!(fn mpv_client_name(ctx: *mut mpv_handle) -> *const ::std::os::raw::c_char);
    wrap!(fn mpv_client_id(ctx: *mut mpv_handle) -> i64);
    wrap!(fn mpv_command(ctx: *mut mpv_handle, args: *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int);
    wrap!(fn mpv_command_ret(ctx: *mut mpv_handle, args: *mut *const ::std::os::raw::c_char, result: *mut mpv_node) -> ::std::os::raw::c_int);
    wrap!(fn mpv_command_async(ctx: *mut mpv_handle, reply_userdata: u64, args: *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int);
    wrap!(fn mpv_set_property(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char, format: mpv_format, data: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
    wrap!(fn mpv_get_property(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char, format: mpv_format, data: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
    wrap!(fn mpv_observe_property(mpv: *mut mpv_handle, reply_userdata: u64, name: *const ::std::os::raw::c_char, format: mpv_format) -> ::std::os::raw::c_int);
    wrap!(fn mpv_unobserve_property(mpv: *mut mpv_handle, registered_reply_userdata: u64) -> ::std::os::raw::c_int);
    wrap!(fn mpv_hook_add(ctx: *mut mpv_handle, reply_userdata: u64, name: *const ::std::os::raw::c_char, priority: ::std::os::raw::c_int) -> ::std::os::raw::c_int);
    wrap!(fn mpv_hook_continue(ctx: *mut mpv_handle, id: u64) -> ::std::os::raw::c_int);
    wrap!(fn mpv_event_name(event: mpv_event_id) -> *const ::std::os::raw::c_char);
    wrap!(fn mpv_error_string(error: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char);
    wrap!(fn mpv_free_node_contents(node: *mut mpv_node));
    wrap!(fn mpv_free(data: *mut ::std::os::raw::c_void));
}

#[cfg(target_os = "windows")]
pub use sym::*;
