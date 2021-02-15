// X11/XLib.h
// https://github.com/mirror/libX11/blob/master/include/X11/Xlib.h

use std::os::raw::{c_int, c_uint, c_ulong};

#[repr(transparent)]
pub struct Bool(c_int);
#[repr(transparent)]
pub struct Window(u32);
pub struct Display;
pub struct Screen;

#[allow(non_upper_case_globals)]
pub const True: Bool = Bool(1);
#[allow(non_upper_case_globals)]
pub const False: Bool = Bool(0);
#[allow(non_upper_case_globals)]
pub const None: Window = Window(0);
#[allow(non_upper_case_globals)]
pub const CurrentTime: c_ulong = 0;

#[link(name = "x11")]
extern {
    // https://linux.die.net/man/3/xopendisplay
    pub fn XOpenDisplay(display_name: *const u8) -> *mut Display;

    // https://linux.die.net/man/3/xclosedisplay
    pub fn XCloseDisplay(display: *mut Display) -> c_int;

    // Macro directly accesses struct member
    pub fn XDefaultScreenOfDisplay(display: *mut Display) -> *mut Screen;

    // Macro directly accesses struct member
    pub fn XRootWindowOfScreen(screen: *mut Screen) -> Window;

    // Macro directly accesses struct member
    pub fn XWidthOfScreen(screen: *mut Screen) -> c_int;

    // Macro directly accesses struct member
    pub fn XHeightOfScreen(screen: *mut Screen) -> c_int;

    // https://linux.die.net/man/3/xwarppointer
    pub fn XWarpPointer(
        display: *mut Display,
        src_w: Window,
        dest_w: Window,
        src_x: c_int,
        src_y: c_int,
        src_width: c_uint,
        src_height: c_uint,
        dest_x: c_int,
        dest_y: c_int,
    ) -> c_int;

    // https://linux.die.net/man/3/xquerypointer
    pub fn XQueryPointer(
        display: *mut Display,
        w: Window,
        root_return: *mut Window,
        child_return: *mut Window,
        root_x_return: *mut c_int,
        root_y_return: *mut c_int,
        win_x_return: *mut c_int,
        win_y_return: *mut c_int,
        mask_return: *mut c_uint,
    ) -> Bool;
}
