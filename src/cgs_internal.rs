use std::os::raw::c_int;
use core_foundation::array::CFArrayRef;
use core_foundation::string::CFStringRef;
use core_foundation::dictionary::CFDictionaryRef;

extern "C" {
    pub fn CGSCopyCurrentSessionDictionary() -> CFDictionaryRef;
    pub fn CGSSessionCreateSessionIDWithOptions(path: CFStringRef, argv: CFArrayRef, flags: c_int, outSession: *mut c_int);
}
