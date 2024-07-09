use libloading::Library;
use std::cell::Cell;
use std::error::Error;
use std::fmt::{self, Display};

use crate::GetInstanceProcAddrSignature;

pub(super) struct VulkanLibHolder(pub(super) Cell<Option<Library>>);
unsafe impl Sync for VulkanLibHolder {}

pub(super) static DYNAMIC_VULKAN_LIB: VulkanLibHolder = VulkanLibHolder(Cell::new(None));

#[derive(Debug)]
pub enum LoadingError {
    LibraryLoadFailure(libloading::Error),
    MissingEntryPoint(MissingEntryPoint),
}

impl fmt::Display for LoadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LibraryLoadFailure(err) => fmt::Display::fmt(err, f),
            Self::MissingEntryPoint(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl Error for LoadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::LibraryLoadFailure(err) => err,
            Self::MissingEntryPoint(err) => err,
        })
    }
}

impl From<MissingEntryPoint> for LoadingError {
    fn from(err: MissingEntryPoint) -> Self {
        Self::MissingEntryPoint(err)
    }
}

#[derive(Clone, Debug)]
pub struct MissingEntryPoint;
impl Display for MissingEntryPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Cannot load `vkGetInstanceProcAddr` symbol from library")
    }
}
impl std::error::Error for MissingEntryPoint {}

/// Safety: do not drop Library before the entry point
pub(super) unsafe fn load_proc_addr_and_lib(
) -> Result<(GetInstanceProcAddrSignature, Library), LoadingError> {
    // code from ash
    #[cfg(windows)]
    const LIB_PATH: &str = "vulkan-1.dll";

    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "ios", target_os = "android"))
    ))]
    const LIB_PATH: &str = "libvulkan.so.1";

    #[cfg(target_os = "android")]
    const LIB_PATH: &str = "libvulkan.so";

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const LIB_PATH: &str = "libvulkan.dylib";

    let lib = Library::new(LIB_PATH).map_err(LoadingError::LibraryLoadFailure)?;

    let get_instance_proc_addr = lib
        .get(c"vkGetInstanceProcAddr".to_bytes())
        .map_err(LoadingError::LibraryLoadFailure)?;

    Ok((*get_instance_proc_addr, lib))
}
