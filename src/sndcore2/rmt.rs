//! AX controller speaker output

unsafe extern "C" {
    /// AXRmtGetSamplesLeft
    #[doc(alias = "AXRmtGetSamplesLeft")]
    #[link_name = "AXRmtGetSamplesLeft"]
    pub unsafe fn get_samples_left() -> i32;

    /// AXRmtGetSamples
    ///
    /// Copies up to `samples` rendered samples of controller `channel`. Returns the number copied.
    #[doc(alias = "AXRmtGetSamples")]
    #[link_name = "AXRmtGetSamples"]
    pub unsafe fn get_samples(channel: i32, buffer: *mut i16, samples: i32) -> i32;

    /// AXRmtAdvancePtr
    #[doc(alias = "AXRmtAdvancePtr")]
    #[link_name = "AXRmtAdvancePtr"]
    pub unsafe fn advance_ptr(samples: i32) -> i32;
}
