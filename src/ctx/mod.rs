use std::ffi;

use ash::vk;

use self::debug::DebugCtx;

mod debug;

pub struct Ctx {
    pub instance: ash::Instance,
    pub entry: ash::Entry,
    pub debug: DebugCtx,
}

impl Ctx {
    pub fn new(
        app_info: &vk::ApplicationInfo,
        layer_names: &[&str],
        extension_names: &[&str],
    ) -> Self {
        let entry = ash::Entry::linked();

        let str_vec_to_raw_p_vec = |string: &[&str]| -> Vec<*const i8> {
            let string_c: Vec<ffi::CString> = string
                .iter()
                .map(|&string| ffi::CString::new(string).unwrap())
                .collect();
            string_c.iter().map(|string| string.as_ptr()).collect()
        };

        let layer_names_p = str_vec_to_raw_p_vec(layer_names);
        let extension_names_p = str_vec_to_raw_p_vec(extension_names);

        let instance_create_info = vk::InstanceCreateInfo {
            p_application_info: app_info,
            pp_enabled_layer_names: layer_names_p.as_ptr(),
            pp_enabled_extension_names: extension_names_p.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
            enabled_layer_count: layer_names.len() as u32,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

        let debug = DebugCtx::new(&entry, &instance);

        Self {
            instance,
            entry,
            debug,
        }
    }
}
