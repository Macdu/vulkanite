//! Functions that provide interop with the raw-window-handle crate.
//!
//! These are largely inspired from the ash-window crate

use raw_window_handle::RawDisplayHandle;

use crate::vk;

#[cfg(all(
    any(target_os = "macos", target_os = "ios"),
    not(feature = "raw-window-metal")
))]
compile_error!("Feature raw-window-metal should be enabled along raw-window-handle when compiling on MacOS/iOS");

/// Given a raw display handle, return a list of instance extensions that must be available and specified
/// when creating a vulkan instance in order to create a surface on the current device
/// The surface extension [vk::KHR_SURFACE] is always included if the returned value is not an error
/// In case creating a swapchain is not supported given the display handle, the error [vk::Status::ErrorExtensionNotPresent]
/// will be returned
pub fn enumerate_required_extensions(
    display_handle: &RawDisplayHandle,
) -> vk::Result<&'static [vk::InstanceExtensionName]> {
    let extensions = match display_handle {
        RawDisplayHandle::Windows(_) => &[vk::KHR_SURFACE.name, vk::KHR_WIN32_SURFACE.name],
        RawDisplayHandle::Wayland(_) => &[vk::KHR_SURFACE.name, vk::KHR_WAYLAND_SURFACE.name],
        RawDisplayHandle::Xlib(_) => &[vk::KHR_SURFACE.name, vk::KHR_XLIB_SURFACE.name],
        RawDisplayHandle::Xcb(_) => &[vk::KHR_SURFACE.name, vk::KHR_XCB_SURFACE.name],
        RawDisplayHandle::Android(_) => &[vk::KHR_SURFACE.name, vk::KHR_ANDROID_SURFACE.name],
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            &[vk::KHR_SURFACE.name, vk::EXT_METAL_SURFACE.name]
        }

        _ => return Err(vk::Status::ErrorExtensionNotPresent),
    };

    Ok(extensions)
}

pub mod raw {
    use std::os::raw::c_void;

    use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

    use crate::vk;

    /// Given an instance, a display handle and a window handle, create a surface associated with
    /// the instance from these handles. Note that the underlying display/window must live at least
    /// as long as the surface
    pub fn create_surface(
        instance: &vk::raw::Instance,
        allocator: Option<&vk::AllocationCallbacks>,
        dispatcher: &vk::CommandsDispatcher,
        display_handle: &RawDisplayHandle,
        window_handle: &RawWindowHandle,
    ) -> vk::Result<vk::raw::SurfaceKHR> {
        match (display_handle, window_handle) {
            (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
                let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
                    .hwnd(window.hwnd.get() as *const c_void)
                    .hinstance(
                        window
                            .hinstance
                            .ok_or(vk::Status::ErrorInitializationFailed)?
                            .get() as *const c_void,
                    );
                vk::raw::create_win32_surface_khr(instance, &surface_desc, allocator, dispatcher)
            }

            (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
                let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
                    .display(unsafe { display.display.cast().as_ref() })
                    .surface(unsafe { window.surface.cast().as_ref() });
                vk::raw::create_wayland_surface_khr(instance, &surface_desc, allocator, dispatcher)
            }

            (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
                let surface_desc = vk::XlibSurfaceCreateInfoKHR::default()
                    .dpy(unsafe {
                        display
                            .display
                            .ok_or(vk::Status::ErrorInitializationFailed)?
                            .cast()
                            .as_ref()
                    })
                    .window(window.window);
                vk::raw::create_xlib_surface_khr(instance, &surface_desc, allocator, dispatcher)
            }

            (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                let surface_desc = vk::XcbSurfaceCreateInfoKHR::default()
                    .connection(unsafe {
                        display
                            .connection
                            .ok_or(vk::Status::ErrorInitializationFailed)?
                            .cast()
                            .as_ref()
                    })
                    .window(window.window.get());
                vk::raw::create_xcb_surface_khr(instance, &surface_desc, allocator, dispatcher)
            }

            (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
                let surface_desc = vk::AndroidSurfaceCreateInfoKHR::default()
                    .window(unsafe { window.a_native_window.cast().as_ref() });
                vk::raw::create_android_surface_khr(instance, &surface_desc, allocator, dispatcher)
            }

            #[cfg(all(target_os = "macos", feature = "raw-window-metal"))]
            (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
                use raw_window_metal::{appkit, Layer};

                let layer = match appkit::metal_layer_from_handle(window) {
                    Layer::Existing(layer) | Layer::Allocated(layer) => layer,
                };

                let surface_desc = vk::MetalSurfaceCreateInfoEXT::default()
                    .layer(unsafe { layer.cast().as_ref() });
                vk::raw::create_metal_surface_ext(instance, &surface_desc, allocator, dispatcher)
            }

            #[cfg(all(target_os = "ios", feature = "raw-window-metal"))]
            (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
                use raw_window_metal::{uikit, Layer};

                let layer = match uikit::metal_layer_from_handle(window) {
                    Layer::Existing(layer) | Layer::Allocated(layer) => layer,
                };

                let surface_desc = vk::MetalSurfaceCreateInfoEXT::default()
                    .layer(unsafe { layer.cast().as_ref() });
                vk::raw::create_metal_surface_ext(instance, &surface_desc, allocator, dispatcher)
            }
            _ => Err(vk::Status::ErrorExtensionNotPresent),
        }
    }
}

pub mod rs {
    use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

    use crate::{vk, Allocator, Dispatcher};

    /// See [crate::window::raw::create_surface]
    pub fn create_surface<D: Dispatcher, A: Allocator>(
        instance: &vk::rs::Instance<D, A>,
        display_handle: &RawDisplayHandle,
        window_handle: &RawWindowHandle,
    ) -> vk::Result<vk::rs::SurfaceKHR> {
        super::raw::create_surface(
            instance,
            instance.get_allocator().get_allocation_callbacks().as_ref(),
            instance.get_dispatcher().get_command_dispatcher(),
            display_handle,
            window_handle,
        )
    }
}
