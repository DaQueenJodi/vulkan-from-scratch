use ash::vk;

pub struct RenderCtx {
    render_pass: vk::RenderPass,
    render_pipeline: vk::RenderPass,
}
/*
impl RenderCtx {
    pub fn new(device: ash::Device) -> Self {
        let render_passattachments = [
            vk::AttachmentDescription {
                format: vk::Format::R8G8B8A8_SRGB,
                samples: vk::SampleCountFlags::TYPE_1, // only sample once, multisampling hard

            }
        ];

        let render_pass_create_info = vk::RenderPassCreateInfo {
            flags: vk::RenderPassCreateFlags::empty(),
            p_attachments:
        };

        let render_pass = unsafe {
            device
                .create_render_pass(&render_pass_create_info, None)
                .unwrap()
        };

        Self {}
    }
}
*/
