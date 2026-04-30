use ::std::option::Option;

use super::*;

#[no_mangle]
pub static mut pfn_mpv_client_api_version: Option<unsafe extern "C" fn() -> ::std::os::raw::c_ulong> = None;

#[no_mangle]
pub static mut pfn_mpv_error_string: Option<
    unsafe extern "C" fn(::std::os::raw::c_int) -> *const ::std::os::raw::c_char,
> = None;

#[no_mangle]
pub static mut pfn_mpv_free: Option<unsafe extern "C" fn(*mut ::std::os::raw::c_void)> = None;

#[no_mangle]
pub static mut pfn_mpv_client_name: Option<unsafe extern "C" fn(*mut mpv_handle) -> *const ::std::os::raw::c_char> =
    None;

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
pub static mut pfn_mpv_create_client: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> *mut mpv_handle,
> = None;

#[no_mangle]
pub static mut pfn_mpv_create_weak_client: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> *mut mpv_handle,
> = None;

#[no_mangle]
pub static mut pfn_mpv_load_config_file: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_get_time_ns: Option<unsafe extern "C" fn(*mut mpv_handle) -> i64> = None;

#[no_mangle]
pub static mut pfn_mpv_get_time_us: Option<unsafe extern "C" fn(*mut mpv_handle) -> i64> = None;

#[no_mangle]
pub static mut pfn_mpv_free_node_contents: Option<unsafe extern "C" fn(*mut mpv_node)> = None;

#[no_mangle]
pub static mut pfn_mpv_set_option: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        *const ::std::os::raw::c_char,
        mpv_format,
        *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_set_option_string: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        *const ::std::os::raw::c_char,
        *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_command: Option<
    unsafe extern "C" fn(*mut mpv_handle, *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_command_node: Option<
    unsafe extern "C" fn(*mut mpv_handle, *mut mpv_node, *mut mpv_node) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_command_ret: Option<
    unsafe extern "C" fn(*mut mpv_handle, *mut *const ::std::os::raw::c_char, *mut mpv_node) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_command_string: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_command_async: Option<
    unsafe extern "C" fn(*mut mpv_handle, u64, *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_command_node_async: Option<
    unsafe extern "C" fn(*mut mpv_handle, u64, *mut mpv_node) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_abort_async_command: Option<unsafe extern "C" fn(*mut mpv_handle, u64)> = None;

#[no_mangle]
pub static mut pfn_mpv_set_property: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        *const ::std::os::raw::c_char,
        mpv_format,
        *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_set_property_string: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        *const ::std::os::raw::c_char,
        *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_del_property: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_set_property_async: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        u64,
        *const ::std::os::raw::c_char,
        mpv_format,
        *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_get_property: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        *const ::std::os::raw::c_char,
        mpv_format,
        *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_get_property_string: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
> = None;

#[no_mangle]
pub static mut pfn_mpv_get_property_osd_string: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
> = None;

#[no_mangle]
pub static mut pfn_mpv_get_property_async: Option<
    unsafe extern "C" fn(*mut mpv_handle, u64, *const ::std::os::raw::c_char, mpv_format) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_observe_property: Option<
    unsafe extern "C" fn(*mut mpv_handle, u64, *const ::std::os::raw::c_char, mpv_format) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_unobserve_property: Option<unsafe extern "C" fn(*mut mpv_handle, u64) -> ::std::os::raw::c_int> =
    None;

#[no_mangle]
pub static mut pfn_mpv_event_name: Option<unsafe extern "C" fn(mpv_event_id) -> *const ::std::os::raw::c_char> = None;

#[no_mangle]
pub static mut pfn_mpv_event_to_node: Option<
    unsafe extern "C" fn(*mut mpv_node, *mut mpv_event) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_request_event: Option<
    unsafe extern "C" fn(*mut mpv_handle, mpv_event_id, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_request_log_messages: Option<
    unsafe extern "C" fn(*mut mpv_handle, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_wait_event: Option<unsafe extern "C" fn(*mut mpv_handle, f64) -> *mut mpv_event> = None;

#[no_mangle]
pub static mut pfn_mpv_wakeup: Option<unsafe extern "C" fn(*mut mpv_handle)> = None;

#[no_mangle]
pub static mut pfn_mpv_set_wakeup_callback: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        Option<unsafe extern "C" fn(*mut ::std::os::raw::c_void)>,
        *mut ::std::os::raw::c_void,
    ),
> = None;

#[no_mangle]
pub static mut pfn_mpv_wait_async_requests: Option<unsafe extern "C" fn(*mut mpv_handle)> = None;

#[no_mangle]
pub static mut pfn_mpv_hook_add: Option<
    unsafe extern "C" fn(
        *mut mpv_handle,
        u64,
        *const ::std::os::raw::c_char,
        ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
> = None;

#[no_mangle]
pub static mut pfn_mpv_hook_continue: Option<unsafe extern "C" fn(*mut mpv_handle, u64) -> ::std::os::raw::c_int> =
    None;

#[no_mangle]
pub static mut pfn_mpv_get_wakeup_pipe: Option<unsafe extern "C" fn(*mut mpv_handle) -> ::std::os::raw::c_int> = None;

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
#[inline]
pub unsafe fn mpv_client_api_version() -> ::std::os::raw::c_ulong {
    pfn_mpv_client_api_version.expect("mpv_client_api_version not initialized by mpv")()
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
#[inline]
pub unsafe fn mpv_error_string(error: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
    pfn_mpv_error_string.expect("mpv_error_string not initialized by mpv")(error)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `data` pointer must be valid and point to memory allocated by mpv.
#[inline]
pub unsafe fn mpv_free(data: *mut ::std::os::raw::c_void) {
    pfn_mpv_free.expect("mpv_free not initialized by mpv")(data)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_client_name(ctx: *mut mpv_handle) -> *const ::std::os::raw::c_char {
    pfn_mpv_client_name.expect("mpv_client_name not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_client_id(ctx: *mut mpv_handle) -> i64 {
    pfn_mpv_client_id.expect("mpv_client_id not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
#[inline]
pub unsafe fn mpv_create() -> *mut mpv_handle {
    pfn_mpv_create.expect("mpv_create not initialized by mpv")()
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_initialize(ctx: *mut mpv_handle) -> ::std::os::raw::c_int {
    pfn_mpv_initialize.expect("mpv_initialize not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_destroy(ctx: *mut mpv_handle) {
    pfn_mpv_destroy.expect("mpv_destroy not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_terminate_destroy(ctx: *mut mpv_handle) {
    pfn_mpv_terminate_destroy.expect("mpv_terminate_destroy not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_create_client(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char) -> *mut mpv_handle {
    pfn_mpv_create_client.expect("mpv_create_client not initialized by mpv")(ctx, name)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_create_weak_client(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char) -> *mut mpv_handle {
    pfn_mpv_create_weak_client.expect("mpv_create_weak_client not initialized by mpv")(ctx, name)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `filename` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_load_config_file(
    ctx: *mut mpv_handle,
    filename: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    pfn_mpv_load_config_file.expect("mpv_load_config_file not initialized by mpv")(ctx, filename)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_get_time_ns(ctx: *mut mpv_handle) -> i64 {
    pfn_mpv_get_time_ns.expect("mpv_get_time_ns not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_get_time_us(ctx: *mut mpv_handle) -> i64 {
    pfn_mpv_get_time_us.expect("mpv_get_time_us not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `node` pointer must be valid and point to an `mpv_node`.
#[inline]
pub unsafe fn mpv_free_node_contents(node: *mut mpv_node) {
    pfn_mpv_free_node_contents.expect("mpv_free_node_contents not initialized by mpv")(node)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_set_option(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
    format: mpv_format,
    data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    pfn_mpv_set_option.expect("mpv_set_option not initialized by mpv")(ctx, name, format, data)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` and `data` pointers must be valid null-terminated C strings.
#[inline]
pub unsafe fn mpv_set_option_string(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
    data: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    pfn_mpv_set_option_string.expect("mpv_set_option_string not initialized by mpv")(ctx, name, data)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `args` pointer must be a valid null-terminated array of C strings.
#[inline]
pub unsafe fn mpv_command(ctx: *mut mpv_handle, args: *mut *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    pfn_mpv_command.expect("mpv_command not initialized by mpv")(ctx, args)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_command_node(
    ctx: *mut mpv_handle,
    args: *mut mpv_node,
    result: *mut mpv_node,
) -> ::std::os::raw::c_int {
    pfn_mpv_command_node.expect("mpv_command_node not initialized by mpv")(ctx, args, result)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `args` pointer must be a valid null-terminated array of C strings.
/// The `result` pointer must be valid and point to an `mpv_node`.
#[inline]
pub unsafe fn mpv_command_ret(
    ctx: *mut mpv_handle,
    args: *mut *const ::std::os::raw::c_char,
    result: *mut mpv_node,
) -> ::std::os::raw::c_int {
    pfn_mpv_command_ret.expect("mpv_command_ret not initialized by mpv")(ctx, args, result)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `args` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_command_string(ctx: *mut mpv_handle, args: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    pfn_mpv_command_string.expect("mpv_command_string not initialized by mpv")(ctx, args)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `args` pointer must be a valid null-terminated array of C strings.
#[inline]
pub unsafe fn mpv_command_async(
    ctx: *mut mpv_handle,
    reply_userdata: u64,
    args: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    pfn_mpv_command_async.expect("mpv_command_async not initialized by mpv")(ctx, reply_userdata, args)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_command_node_async(
    ctx: *mut mpv_handle,
    reply_userdata: u64,
    args: *mut mpv_node,
) -> ::std::os::raw::c_int {
    pfn_mpv_command_node_async.expect("mpv_command_node_async not initialized by mpv")(ctx, reply_userdata, args)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_abort_async_command(ctx: *mut mpv_handle, reply_userdata: u64) {
    pfn_mpv_abort_async_command.expect("mpv_abort_async_command not initialized by mpv")(ctx, reply_userdata)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
/// The `data` pointer must be valid and point to data of the specified format.
#[inline]
pub unsafe fn mpv_set_property(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
    format: mpv_format,
    data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    pfn_mpv_set_property.expect("mpv_set_property not initialized by mpv")(ctx, name, format, data)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` and `data` pointers must be valid null-terminated C strings.
#[inline]
pub unsafe fn mpv_set_property_string(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
    data: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    pfn_mpv_set_property_string.expect("mpv_set_property_string not initialized by mpv")(ctx, name, data)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_del_property(ctx: *mut mpv_handle, name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    pfn_mpv_del_property.expect("mpv_del_property not initialized by mpv")(ctx, name)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_set_property_async(
    ctx: *mut mpv_handle,
    reply_userdata: u64,
    name: *const ::std::os::raw::c_char,
    format: mpv_format,
    data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    pfn_mpv_set_property_async.expect("mpv_set_property_async not initialized by mpv")(
        ctx,
        reply_userdata,
        name,
        format,
        data,
    )
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
/// The `data` pointer must be valid and point to memory of sufficient size for the specified format.
#[inline]
pub unsafe fn mpv_get_property(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
    format: mpv_format,
    data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    pfn_mpv_get_property.expect("mpv_get_property not initialized by mpv")(ctx, name, format, data)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_get_property_string(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char {
    pfn_mpv_get_property_string.expect("mpv_get_property_string not initialized by mpv")(ctx, name)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_get_property_osd_string(
    ctx: *mut mpv_handle,
    name: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char {
    pfn_mpv_get_property_osd_string.expect("mpv_get_property_osd_string not initialized by mpv")(ctx, name)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_get_property_async(
    ctx: *mut mpv_handle,
    reply_userdata: u64,
    name: *const ::std::os::raw::c_char,
    format: mpv_format,
) -> ::std::os::raw::c_int {
    pfn_mpv_get_property_async.expect("mpv_get_property_async not initialized by mpv")(
        ctx,
        reply_userdata,
        name,
        format,
    )
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `mpv` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_observe_property(
    mpv: *mut mpv_handle,
    reply_userdata: u64,
    name: *const ::std::os::raw::c_char,
    format: mpv_format,
) -> ::std::os::raw::c_int {
    pfn_mpv_observe_property.expect("mpv_observe_property not initialized by mpv")(mpv, reply_userdata, name, format)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `mpv` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_unobserve_property(mpv: *mut mpv_handle, registered_reply_userdata: u64) -> ::std::os::raw::c_int {
    pfn_mpv_unobserve_property.expect("mpv_unobserve_property not initialized by mpv")(mpv, registered_reply_userdata)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
#[inline]
pub unsafe fn mpv_event_name(event: mpv_event_id) -> *const ::std::os::raw::c_char {
    pfn_mpv_event_name.expect("mpv_event_name not initialized by mpv")(event)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
#[inline]
pub unsafe fn mpv_event_to_node(dst: *mut mpv_node, src: *mut mpv_event) -> ::std::os::raw::c_int {
    pfn_mpv_event_to_node.expect("mpv_event_to_node not initialized by mpv")(dst, src)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_request_event(
    ctx: *mut mpv_handle,
    event: mpv_event_id,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    pfn_mpv_request_event.expect("mpv_request_event not initialized by mpv")(ctx, event, enable)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `min_level` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_request_log_messages(
    ctx: *mut mpv_handle,
    min_level: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    pfn_mpv_request_log_messages.expect("mpv_request_log_messages not initialized by mpv")(ctx, min_level)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_wait_event(ctx: *mut mpv_handle, timeout: f64) -> *mut mpv_event {
    pfn_mpv_wait_event.expect("mpv_wait_event not initialized by mpv")(ctx, timeout)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_wakeup(ctx: *mut mpv_handle) {
    pfn_mpv_wakeup.expect("mpv_wakeup not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_set_wakeup_callback(
    ctx: *mut mpv_handle,
    cb: ::std::option::Option<unsafe extern "C" fn(*mut ::std::os::raw::c_void)>,
    cb_ctx: *mut ::std::os::raw::c_void,
) {
    pfn_mpv_set_wakeup_callback.expect("mpv_set_wakeup_callback not initialized by mpv")(ctx, cb, cb_ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_wait_async_requests(ctx: *mut mpv_handle) {
    pfn_mpv_wait_async_requests.expect("mpv_wait_async_requests not initialized by mpv")(ctx)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `name` pointer must be a valid null-terminated C string.
#[inline]
pub unsafe fn mpv_hook_add(
    ctx: *mut mpv_handle,
    reply_userdata: u64,
    name: *const ::std::os::raw::c_char,
    priority: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    pfn_mpv_hook_add.expect("mpv_hook_add not initialized by mpv")(ctx, reply_userdata, name, priority)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
/// The `id` must be a valid hook ID returned by `mpv_hook_add`.
#[inline]
pub unsafe fn mpv_hook_continue(ctx: *mut mpv_handle, id: u64) -> ::std::os::raw::c_int {
    pfn_mpv_hook_continue.expect("mpv_hook_continue not initialized by mpv")(ctx, id)
}

/// # Safety
///
/// This function must only be called after mpv has initialized the function pointers
/// via `mpv_open_cplugin`. Calling before initialization will panic.
/// The `ctx` pointer must be valid and obtained from `mpv_create`.
#[inline]
pub unsafe fn mpv_get_wakeup_pipe(ctx: *mut mpv_handle) -> ::std::os::raw::c_int {
    pfn_mpv_get_wakeup_pipe.expect("mpv_get_wakeup_pipe not initialized by mpv")(ctx)
}
