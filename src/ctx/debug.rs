use std::ffi;

use ash::vk;

pub struct DebugCtx {
    pub debug_utils_loader: ash::extensions::ext::DebugUtils,
    pub debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugCtx {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Self {
        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

        use vk::DebugUtilsMessageSeverityFlagsEXT as SeverityFlags;
        use vk::DebugUtilsMessageTypeFlagsEXT as TypeFlags;

        let debug_utils_messenger_info = vk::DebugUtilsMessengerCreateInfoEXT {
            message_severity: SeverityFlags::INFO
                | SeverityFlags::WARNING
                | SeverityFlags::ERROR
                | SeverityFlags::VERBOSE,
            message_type: TypeFlags::GENERAL | TypeFlags::VALIDATION | TypeFlags::PERFORMANCE,
            pfn_user_callback: Some(my_debug_callback),
            ..Default::default()
        };
        let debug_utils_messenger = unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&debug_utils_messenger_info, None)
                .unwrap()
        };

        DebugCtx {
            debug_utils_messenger,
            debug_utils_loader,
        }
    }
}

unsafe extern "system" fn my_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    user_data: *mut ffi::c_void,
) -> vk::Bool32 {
    println!("[{message_severity:?}][{message_types:?}]: {callback_data:?}");

    vk::FALSE
}
