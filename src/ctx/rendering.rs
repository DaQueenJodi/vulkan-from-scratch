use ash::vk;

pub struct RenderCtx {
    render_pass: vk::RenderPass,
    render_pipeline: vk::RenderPass,
}

impl RenderCtx {
    pub fn new(device: ash::Device) -> Self {
        let render_pass_attachments = [
            vk::AttachmentDescription {
                format: vk::Format::R8G8B8A8_SRGB,
                samples: vk::SampleCountFlags::TYPE_1, // only sample once, multisampling hard
                store_op: vk::AttachmentStoreOp::STORE,
                load_op: vk::AttachmentLoadOp::DONT_CARE,
                initial_layout: vk::ImageLayout::GENERAL,
                final_layout: vk::ImageLayout::GENERAL,
                flags: vk::AttachmentDescriptionFlags::empty(),
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            }
        ];


        let input_attachments = [
            vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::GENERAL,
            }      
        ];

        let render_pass_subpasses = [
            vk::SubpassDescription {
                pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
                input_attachment_count: input_attachments.len() as u32,
                p_input_attachments: &input_attachments as *const _,
                ..Default::default()
            }
        ];

        let render_pass_create_info = vk::RenderPassCreateInfo {
            flags: vk::RenderPassCreateFlags::empty(),
            p_attachments: &render_pass_attachments as *const _,
            attachment_count: render_pass_attachments.len() as u32,
            p_subpasses: &render_pass_subpasses as *const _,
            subpass_count: render_pass_subpasses.len() as u32,
            ..Default::default()
        };

        let render_pass = unsafe {
            device
                .create_render_pass(&render_pass_create_info, None)
                .unwrap()
        };


        let pipeline_cache_create_info = vk::PipelineCacheCreateInfo {
                initial_data_size: 0,
                
        };

        let pipeline_cache = unsafe { device.create_pipeline_cache(create_info, None).unwrap() };

        let render_pipeline = device.create_graphics_pipelines(pipeline_cache, create_infos, allocation_callbacks)

        Self {
            render_pass,
            render_pipeline,
        }
    }
}

