//! GX2 resource diagnostics

unsafe extern "C" {
    /// GX2TempGetGPUVersion
    #[doc(alias = "GX2TempGetGPUVersion")]
    #[link_name = "GX2TempGetGPUVersion"]
    pub unsafe fn get_gpu_version() -> u32;

    /// GX2TempGetResourceCount
    #[doc(alias = "GX2TempGetResourceCount")]
    #[link_name = "GX2TempGetResourceCount"]
    pub unsafe fn get_resource_count() -> u32;

    /// GX2TempGetNonUserMemoryResourceCount
    #[doc(alias = "GX2TempGetNonUserMemoryResourceCount")]
    #[link_name = "GX2TempGetNonUserMemoryResourceCount"]
    pub unsafe fn get_non_user_memory_resource_count() -> u32;

    /// GX2TempDumpResources
    #[doc(alias = "GX2TempDumpResources")]
    #[link_name = "GX2TempDumpResources"]
    pub unsafe fn dump_resources();

    /// GX2TempDumpGPUResourceContext
    #[doc(alias = "GX2TempDumpGPUResourceContext")]
    #[link_name = "GX2TempDumpGPUResourceContext"]
    pub unsafe fn dump_gpu_resource_context();
}
