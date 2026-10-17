use ash::{Entry, Instance, extensions::ext::DebugUtils, version::EntryV1_0, vk};
use std::{
    ffi::{CStr, CString},
    os::raw::c_void,
};

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

pub const REQUIRED_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

/// Modern Vulkan debug callback using VK_EXT_debug_utils
unsafe extern "system" fn vulkan_debug_utils_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };
    let message = unsafe { CStr::from_ptr(callback_data.p_message) };

    match severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            log::debug!("[{:?}] {:?}", types, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!("[{:?}] {:?}", types, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!("[{:?}] {:?}", types, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!("[{:?}] {:?}", types, message);
        }
        _ => {
            log::error!("[UNKNOWN] {:?}", message);
        }
    }

    vk::FALSE
}

/// Build layer names and pointers
pub fn get_layer_names_and_pointers() -> (Vec<CString>, Vec<*const i8>) {
    let layer_names = REQUIRED_LAYERS
        .iter()
        .map(|name| CString::new(*name).unwrap())
        .collect::<Vec<_>>();

    let layer_names_ptrs = layer_names
        .iter()
        .map(|name| name.as_ptr())
        .collect::<Vec<_>>();

    (layer_names, layer_names_ptrs)
}

/// Check if validation layers are supported
pub fn check_validation_layer_support(entry: &Entry) {
    let available_layers = entry
        .enumerate_instance_layer_properties()
        .expect("Failed to enumerate instance layers");

    for required in REQUIRED_LAYERS.iter() {
        let found = available_layers.iter().any(|layer| {
            let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) }
                .to_str()
                .unwrap();
            name == *required
        });

        if !found {
            panic!("Validation layer not supported: {}", required);
        }
    }
}

/// Setup modern debug messenger
pub fn setup_debug_messenger(
    entry: &Entry,
    instance: &Instance,
) -> Option<(DebugUtils, vk::DebugUtilsMessengerEXT)> {
    if !ENABLE_VALIDATION_LAYERS {
        return None;
    }

    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback));

    let debug_utils = DebugUtils::new(entry, instance);

    let messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&create_info, None)
            .expect("Failed to create debug utils messenger")
    };

    Some((debug_utils, messenger))
}
