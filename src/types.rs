use libc::{c_char, c_void};

#[repr(C)]
pub struct GEOSWKTReader {
    private: [u8; 0],
}
#[repr(C)]
pub struct GEOSWKTWriter {
    private: [u8; 0],
}
#[repr(C)]
pub struct GEOSPreparedGeometry {
    private: [u8; 0],
}
#[repr(C)]
pub struct GEOSCoordSequence {
    private: [u8; 0],
}
#[repr(C)]
pub struct GEOSGeometry {
    private: [u8; 0],
}
#[repr(C)]
pub struct GEOSContextHandle_HS {
    private: [u8; 0],
}
#[allow(non_camel_case_types)]
pub type GEOSContextHandle_t = *mut GEOSContextHandle_HS;
#[allow(non_camel_case_types)]
pub type GEOSMessageHandler_r =
    Option<unsafe extern "C" fn(message: *const c_char, userdata: *mut c_void)>;
