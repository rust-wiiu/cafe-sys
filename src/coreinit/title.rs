//! Title

unsafe extern "C" {
    /// OSGetTitleID
    #[doc(alias = "OSGetTitleID")]
    #[link_name = "OSGetTitleID"]
    pub unsafe fn get_title_id() -> u64;
}
