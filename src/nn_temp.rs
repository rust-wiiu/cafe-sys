//! NN TEMP

#[cfg(target_arch = "powerpc")]
imports_section!(
    "nn_temp",
    [
        "TEMPChangeDir",
        "TEMPChangeDirAsync",
        "TEMPChangeOthersMode",
        "TEMPChangeOthersModeAsync",
        "TEMPCreateAndInitTempDir",
        "TEMPGetDirGlobalPath",
        "TEMPGetDirPath",
        "TEMPGetFreeSpaceSize",
        "TEMPGetFreeSpaceSizeAsync",
        "TEMPGetStat",
        "TEMPGetStatAsync",
        "TEMPInit",
        "TEMPMakeDir",
        "TEMPMakeDirAsync",
        "TEMPMountTempDir",
        "TEMPOpenDir",
        "TEMPOpenDirAsync",
        "TEMPOpenFile",
        "TEMPOpenFileAsync",
        "TEMPOpenNewFile",
        "TEMPOpenNewFileAsync",
        "TEMPRemove",
        "TEMPRemoveAsync",
        "TEMPRename",
        "TEMPRenameAsync",
        "TEMPShutdown",
        "TEMPShutdownTempDir",
        "TEMPUnmountTempDir"
    ],
    []
);
