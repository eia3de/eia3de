fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("eia3de_client_desktop", log::LevelFilter::Info)
        .init();

    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("eia3de")
        .with_inner_size(winit::dpi::PhysicalSize::new(1920, 1080))
        .build(&event_loop)
        .unwrap();

    let surface = wgpu::Surface::create(&window);

    let adapter = futures::executor::block_on(async {
        wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .unwrap()
    });

    log::info!("adapter: {:#?}", adapter.get_info());

    let (device, queue) = futures::executor::block_on(async {
        adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
    });

    let mut swap_chain_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

    // TODO: use some font loader
    let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
        "../../assets/FiraSans-Regular.ttf"
    ))
    .unwrap();

    let golden_ratio = (1.0 + (5.0_f32).sqrt()) / 2.0;

    let loading_section = wgpu_glyph::Section {
        screen_position: (10.0, 10.0),

        text: (0..12)
            .rev()
            .map(|n| {
                wgpu_glyph::Text::new("Loading. ")
                    .with_scale(golden_ratio.powf(n as f32))
                    .with_color([1.0, 1.0, 1.0, 1.0])
            })
            .collect(),

        ..wgpu_glyph::Section::default()
    };

    let mut glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font)
        .initial_cache_size((1024, 1024))
        .build(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

    // we'd like to force an event immediately
    window.request_redraw();

    log::info!("starting event loop");
    event_loop.run(move |event, _, control_flow| {
        use winit::{event::*, event_loop::ControlFlow};

        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    log::info!("close requested");
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::Resized(physical_size) => {
                    log::info!("resized");

                    // XXX: kind of causes trouble when resizing a lot
                    swap_chain_desc.width = physical_size.width;
                    swap_chain_desc.height = physical_size.height;
                    swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    log::info!("scale factor changed");

                    swap_chain_desc.width = new_inner_size.width;
                    swap_chain_desc.height = new_inner_size.height;
                    swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
                }

                _ => {}
            },

            // TODO: we need a better loop, obviously
            Event::MainEventsCleared => {
                // XXX: might not be the behavior we want?
                let frame = match swap_chain.get_next_texture() {
                    Ok(frame) => frame,
                    Err(_) => return,
                };

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                }

                glyph_brush.queue(&loading_section);
                glyph_brush
                    .draw_queued(
                        &device,
                        &mut encoder,
                        &frame.view,
                        window.inner_size().width,
                        window.inner_size().height,
                    )
                    .unwrap();

                queue.submit(&[encoder.finish()]);
            }

            _ => {}
        }
    });
}
