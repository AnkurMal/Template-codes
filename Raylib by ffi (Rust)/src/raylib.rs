#![allow(dead_code, unused_variables)]

use std::os::raw::*;

/// The style of the macro is:
/// ```
/// cstring!(cstr) or cstring!(format_cstr)
/// ```
/// Returns a string as a cstring.
/// 
/// # Argument
/// The string or the format string to be returned as cstring.
/// 
/// # Errors
/// This will throw an error if the supplied string contain an
/// internal 0 byte.
/// 
/// # Examples
/// ```
/// let text = cstring!("hello there!");
/// let text = cstring!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! cstring {
    ($cstr:expr) => {{
        std::ffi::CString::new($cstr).expect("Failed to create CString.")
    }};
    ($($cstr:tt)*) => {{
        std::ffi::CString::new(format!($($cstr)*)).expect("Failed to create CString.")
    }}
}

type CstringPtr = *const c_char;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum TraceLogLevel {
    LOG_ALL,       
    LOG_TRACE,      
    LOG_DEBUG,          
    LOG_INFO,           
    LOG_WARNING,       
    LOG_ERROR,         
    LOG_FATAL,          
    LOG_NONE          
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}

impl Color {
    pub const LIGHTGRAY : Self = Color {r: 200, g: 200, b: 200, a: 255};
    pub const GRAY      : Self = Color {r: 130, g: 130, b: 130, a: 255};
    pub const DARKGRAY  : Self = Color {r: 80, g: 80, b: 80, a: 255};
    pub const YELLOW    : Self = Color {r: 253, g: 249, b: 0, a: 255};
    pub const GOLD      : Self = Color {r: 255, g: 203, b: 0, a: 255};
    pub const ORANGE    : Self = Color {r: 255, g: 161, b: 0, a: 255};
    pub const PINK      : Self = Color {r: 255, g: 109, b: 194, a: 255};
    pub const RED       : Self = Color {r: 230, g: 41, b: 55, a: 255};
    pub const MAROON    : Self = Color {r: 190, g: 33, b: 55, a: 255};
    pub const GREEN     : Self = Color {r: 0, g: 228, b: 48, a: 255};
    pub const LIME      : Self = Color {r: 0, g: 158, b: 47, a: 255};
    pub const DARKGREEN : Self = Color {r: 0, g: 117, b: 44, a: 255};
    pub const SKYBLUE   : Self = Color {r: 102, g: 191, b: 255, a: 255};
    pub const BLUE      : Self = Color {r: 0, g: 121, b: 241, a: 255};
    pub const DARKBLUE  : Self = Color {r: 0, g: 82, b: 172, a: 255};
    pub const PURPLE    : Self = Color {r: 200, g: 122, b: 255, a: 255};
    pub const VIOLET    : Self = Color {r: 135, g: 60, b: 190, a: 255};
    pub const DARKPURPLE: Self = Color {r: 112, g: 31, b: 126, a: 255};
    pub const BEIGE     : Self = Color {r: 211, g: 176, b: 131, a: 255};
    pub const BROWN     : Self = Color {r: 127, g: 106, b: 79, a: 255};
    pub const DARKBROWN : Self = Color {r: 79, g: 63, b: 47, a: 255};
    pub const WHITE     : Self = Color {r: 255, g: 255, b: 255, a: 255};
    pub const BLACK     : Self = Color {r: 0, g: 0, b: 0, a: 255};
    pub const BLANK     : Self = Color {r: 0, g: 0, b: 0, a: 0};
    pub const MAGENTA   : Self = Color {r: 255, g: 0, b: 255, a: 255};
    pub const RAYWHITE  : Self = Color {r: 245, g: 245, b: 245, a: 255};
    
    pub fn new(r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) -> Self {
        Color {r, g, b, a}
    }
}

#[link(name="raylib")]
#[link(name="gdi32")]
#[link(name="winmm")]
extern {
    pub fn SetTraceLogLevel(logLevel: TraceLogLevel);
    pub fn InitWindow(width: c_int, height: c_int, title: CstringPtr);
    pub fn SetTargetFPS(fps: c_int);
    pub fn WindowShouldClose() -> bool;
    pub fn BeginDrawing();   
    pub fn ClearBackground(color: Color);
    pub fn DrawText(text: CstringPtr, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
    pub fn DrawFPS(posX: c_int, posY: c_int);
    pub fn EndDrawing();
    pub fn CloseWindow();    
}
