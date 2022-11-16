use std::ffi;

use ash::vk;

use crate::ctx::debug::my_debug_callback;

use self::debug::DebugCtx;
mod debug;
mod rendering;

pub struct Ctx {
    pub queues: QueuesCtx,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,
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

        let layer_names_c: Vec<ffi::CString> = layer_names
            .iter()
            .map(|&name| ffi::CString::new(name).unwrap())
            .collect();
        let layer_names_p: Vec<*const i8> =
            layer_names_c.iter().map(|name| name.as_ptr()).collect();

        let extension_names_c: Vec<ffi::CString> = extension_names
            .iter()
            .map(|&name| ffi::CString::new(name).unwrap())
            .collect();

        let extension_names_p: Vec<*const i8> =
            extension_names_c.iter().map(|name| name.as_ptr()).collect();

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

        let instance_create_info = vk::InstanceCreateInfo {
            p_next: &debug_utils_messenger_info as *const _ as *const ffi::c_void,
            p_application_info: app_info,
            pp_enabled_layer_names: layer_names_p.as_ptr(),
            pp_enabled_extension_names: extension_names_p.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
            enabled_layer_count: layer_names.len() as u32,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

        let debug = DebugCtx::new(&entry, &instance);

        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };

        let physical_device = physical_devices[0]; // TODO: select this methodically instead of just picking the first device

        let (queues, logical_device) = QueuesCtx::new(&instance, physical_device);

        let command_pool_create_info = vk::CommandPoolCreateInfo {
            queue_family_index: queues.transfer_index as u32,
            flags: vk::CommandPoolCreateFlags::empty(),
            ..Default::default()
        };

        let command_pool = unsafe {
            logical_device
                .create_command_pool(&command_pool_create_info, None)
                .unwrap()
        };


        Self {
            queues,
            logical_device,
            physical_device,
            instance,
            entry,
            debug,
        }
    }
}

impl Drop for Ctx {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
            self.debug
                .debug_utils_loader
                .destroy_debug_utils_messenger(self.debug.debug_utils_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}

pub struct QueuesCtx {
    pub transfer_queue: vk::Queue,
    pub graphics_queue: vk::Queue,
    pub transfer_index: usize,
    pub graphics_index: usize,
}

impl QueuesCtx {
    pub fn new(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> (Self, ash::Device) {
        // todo: make this not bundled together, for now its just convenient
        let mut transfer_queue: Option<vk::Queue>;
        let graphics_queue: Option<vk::Queue>;
        let mut transfer_index: Option<usize> = None;
        let mut graphics_index: Option<usize> = None;

        // get list of queue families and pick out a graphics and transfer queue
        let physical_device_queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        for queue_family_index in 0..physical_device_queue_family_properties.len() {
            let queue_family_properties =
                physical_device_queue_family_properties[queue_family_index];
            if queue_family_properties
                .queue_flags
                .contains(vk::QueueFlags::GRAPHICS)
            {
                graphics_index = Some(queue_family_index);
            } else if transfer_index.is_none()
                || queue_family_properties
                    .queue_flags
                    .contains(vk::QueueFlags::TRANSFER)
            // default to non-graphics queue, but prefer TRANSFER queue {
            {
                transfer_index = Some(queue_family_index);
            }
        }

        let queue_priorities = [1.0]; // make both queues high priority for now

        let graphics_queue_device_create_info = vk::DeviceQueueCreateInfo {
            queue_count: 1,
            queue_family_index: graphics_index.unwrap() as u32,
            flags: vk::DeviceQueueCreateFlags::empty(),
            p_queue_priorities: &queue_priorities as *const _ as *const f32,
            ..Default::default()
        };

        let transfer_queue_device_create_info = vk::DeviceQueueCreateInfo {
            queue_count: 1,
            queue_family_index: transfer_index.unwrap() as u32,
            flags: vk::DeviceQueueCreateFlags::empty(),
            p_queue_priorities: &queue_priorities as *const _ as *const f32,
            ..Default::default()
        };

        let queue_create_infos = [
            graphics_queue_device_create_info,
            transfer_queue_device_create_info,
        ];

        let device_create_info = vk::DeviceCreateInfo {
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: &queue_create_infos as *const _,
            flags: vk::DeviceCreateFlags::empty(),
            ..Default::default()
        };

        let logical_device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap()
        };

        graphics_queue = match graphics_index {
            Some(index) => unsafe { Some(logical_device.get_device_queue(index as u32, 0)) },
            None => {
                panic!("could not find a graphics queue for some reason, good luck with that ig")
            }
        };
        transfer_queue = match transfer_index {
            Some(index) => unsafe { Some(logical_device.get_device_queue(index as u32, 0)) },
            None => {
                eprintln!("could not find dedicated transfer queue, going to use the graphics queue for both operations!");
                graphics_queue
            }
        };

        if graphics_queue.is_none() {
            panic!("could not get graphics queue :(")
        }
        if transfer_queue.is_none() {
            transfer_queue = graphics_queue
        }

        (
            Self {
                graphics_index: graphics_index.unwrap(),
                transfer_index: transfer_index.unwrap(),
                transfer_queue: transfer_queue.unwrap(),
                graphics_queue: graphics_queue.unwrap(),
            },
            logical_device,
        )
    }
}
