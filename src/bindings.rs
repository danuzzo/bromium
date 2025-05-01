use std::os::raw::c_long;
// use winapi::shared::windef::RECT;

// External function declarations
#[link(name = "uixpath", kind = "static")]
unsafe extern "C" {
    pub unsafe fn InitUiTreeWalk();
    pub unsafe fn UnInitUiTreeWalk();
    pub unsafe fn GetUiXPath(left: c_long, top: c_long, s: *mut u16, nMaxCount: c_long) -> c_long;
    // pub unsafe fn HighlightCachedUI(lpRumtimeId: *mut u16, pRect: *mut RECT) -> c_long;
}