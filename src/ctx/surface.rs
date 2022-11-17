use std::ffi;

use ash::vk;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use super::QueuesCtx;

pub struct WindowCtx {
    pub window: winit::window::Window,
    pub surface_khr: vk::SurfaceKHR,
    pub surface_loader: ash::extensions::khr::Surface,
}

impl WindowCtx {
    pub fn new(
        entry: &ash::Entry,
        logical_device: ash::Device,
        physical_device: vk::PhysicalDevice,
        instance: &ash::Instance,
        window: winit::window::Window,
        queues: QueuesCtx,
    ) -> (Self, Vec<ffi::CString>) {
        let surface_extensions =
            ash_window::enumerate_required_extensions(window.raw_display_handle()).unwrap();

        let surface_extensions = unsafe {
            surface_extensions
                .iter()
                .map(|&raw_str| ffi::CStr::from_ptr(raw_str).to_owned())
                .collect()
        };
        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
            .unwrap()
        };

        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        let surface_capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .unwrap()
        };

        let surface_present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap()
        };

        let surface_formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap()
        };

        let queue_family_indices = [queues.graphics_index as u32];

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            surface,
            min_image_count: 3,
            image_format: surface_formats.first().unwrap().format,
            image_color_space: surface_formats.first().unwrap().color_space,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count: queue_family_indices.len() as u32,
            present_mode: vk::PresentModeKHR::FIFO,
            pre_transform: surface_capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            ..Default::default()
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &logical_device);

        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap()
        };

        (
            Self {
                window,
                surface_khr: surface,
                surface_loader,
            },
            surface_extensions,
        )
    }
}
