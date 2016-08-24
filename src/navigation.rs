/* automatically generated by rust-bindgen */

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum XPLMNavType {
    xplm_Nav_Unknown = 0,
    xplm_Nav_Airport = 1,
    xplm_Nav_NDB = 2,
    xplm_Nav_VOR = 4,
    xplm_Nav_ILS = 8,
    xplm_Nav_Localizer = 16,
    xplm_Nav_GlideSlope = 32,
    xplm_Nav_OuterMarker = 64,
    xplm_Nav_MiddleMarker = 128,
    xplm_Nav_InnerMarker = 256,
    xplm_Nav_Fix = 512,
    xplm_Nav_DME = 1024,
    xplm_Nav_LatLon = 2048,
}
pub type XPLMNavRef = ::std::os::raw::c_int;
extern "C" {
    pub fn XPLMGetFirstNavAid() -> XPLMNavRef;
    pub fn XPLMGetNextNavAid(inNavAidRef: XPLMNavRef) -> XPLMNavRef;
    pub fn XPLMFindFirstNavAidOfType(inType: XPLMNavType) -> XPLMNavRef;
    pub fn XPLMFindLastNavAidOfType(inType: XPLMNavType) -> XPLMNavRef;
    pub fn XPLMFindNavAid(inNameFragment: *const ::std::os::raw::c_char,
                          inIDFragment: *const ::std::os::raw::c_char,
                          inLat: *mut ::std::os::raw::c_float,
                          inLon: *mut ::std::os::raw::c_float,
                          inFrequency: *mut ::std::os::raw::c_int,
                          inType: XPLMNavType) -> XPLMNavRef;
    pub fn XPLMGetNavAidInfo(inRef: XPLMNavRef, outType: *mut XPLMNavType,
                             outLatitude: *mut ::std::os::raw::c_float,
                             outLongitude: *mut ::std::os::raw::c_float,
                             outHeight: *mut ::std::os::raw::c_float,
                             outFrequency: *mut ::std::os::raw::c_int,
                             outHeading: *mut ::std::os::raw::c_float,
                             outID: *mut ::std::os::raw::c_char,
                             outName: *mut ::std::os::raw::c_char,
                             outReg: *mut ::std::os::raw::c_char);
    pub fn XPLMCountFMSEntries() -> ::std::os::raw::c_int;
    pub fn XPLMGetDisplayedFMSEntry() -> ::std::os::raw::c_int;
    pub fn XPLMGetDestinationFMSEntry() -> ::std::os::raw::c_int;
    pub fn XPLMSetDisplayedFMSEntry(inIndex: ::std::os::raw::c_int);
    pub fn XPLMSetDestinationFMSEntry(inIndex: ::std::os::raw::c_int);
    pub fn XPLMGetFMSEntryInfo(inIndex: ::std::os::raw::c_int,
                               outType: *mut XPLMNavType,
                               outID: *mut ::std::os::raw::c_char,
                               outRef: *mut XPLMNavRef,
                               outAltitude: *mut ::std::os::raw::c_int,
                               outLat: *mut ::std::os::raw::c_float,
                               outLon: *mut ::std::os::raw::c_float);
    pub fn XPLMSetFMSEntryInfo(inIndex: ::std::os::raw::c_int,
                               inRef: XPLMNavRef,
                               inAltitude: ::std::os::raw::c_int);
    pub fn XPLMSetFMSEntryLatLon(inIndex: ::std::os::raw::c_int,
                                 inLat: ::std::os::raw::c_float,
                                 inLon: ::std::os::raw::c_float,
                                 inAltitude: ::std::os::raw::c_int);
    pub fn XPLMClearFMSEntry(inIndex: ::std::os::raw::c_int);
    pub fn XPLMGetGPSDestinationType() -> XPLMNavType;
    pub fn XPLMGetGPSDestination() -> XPLMNavRef;
}
