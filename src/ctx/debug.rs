use colored::Colorize;
use std::ffi;

use ash::vk;

use vk::DebugUtilsMessageSeverityFlagsEXT as SeverityFlags;
use vk::DebugUtilsMessageTypeFlagsEXT as TypeFlags;

pub struct DebugCtx {
    pub debug_utils_loader: ash::extensions::ext::DebugUtils,
    pub debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugCtx {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Self {
        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);
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

pub unsafe extern "system" fn my_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut ffi::c_void,
) -> vk::Bool32 {

    // get real message text
    let callback_data = *callback_data;

    let callback_message_raw = callback_data.p_message;

    let callback_message = ffi::CStr::from_ptr(callback_message_raw);

    // change message color based on severity to make it easier to see
    let callback_message_color = match message_severity {
        SeverityFlags::WARNING => colored::Color::Yellow,
        SeverityFlags::INFO => colored::Color::Cyan,
        SeverityFlags::ERROR => colored::Color::Red,
        SeverityFlags::VERBOSE => colored::Color::Blue,
        _ => panic!("unexpected severity type"),
    };
    
    // format message and add color, then print back to the terminal
    let message = format!("[{message_severity:#?}][{message_types:#?}]: {callback_message:?}");

    println!("{}", message.color(callback_message_color));

    vk::FALSE
}
