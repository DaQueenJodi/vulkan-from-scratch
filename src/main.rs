use ash::vk;
use ctx::Ctx;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

mod ctx;

fn main() {
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };

    let layer_names = ["VK_LAYER_KHRONOS_validation"];
    let extension_names = ["VK_EXT_debug_utils"];

    let ctx = Ctx::new(&app_info, &layer_names, &extension_names);
}
