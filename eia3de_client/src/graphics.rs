//! Graphics

use crate::{windowing::Window, ManualSetup, ManualSetupHandler};
use rendy::{
    command::{Families, QueueId, RenderPassEncoder},
    factory::Factory,
    graph::{
        present::PresentNode, render::*, Graph, GraphBuilder, GraphContext, NodeBuffer, NodeImage,
    },
    hal,
    memory::Dynamic,
    mesh::PosColor,
    resource::{Buffer, BufferInfo, DescriptorSetLayout, Escape, Handle},
    shader::{ShaderKind, SourceLanguage, SpirvReflection, StaticShaderInfo},
};
use specs::prelude::*;

pub type Backend = rendy::vulkan::Backend;

/// Resource
#[derive(Debug)]
pub struct Rendy {
    pub graph: Option<Graph<Backend, ()>>,
    pub families: Families<Backend>,
    pub factory: Factory<Backend>,
}

impl ManualSetup for Rendy {
    fn setup(res: &mut Resources) {
        let config: rendy::factory::Config = Default::default();
        let (factory, families) = rendy::factory::init(config).expect("failed to init rendy");
        res.insert(Self {
            factory,
            families,
            graph: None,
        });
    }
}

impl Drop for Rendy {
    fn drop(&mut self) {
        if let Some(graph) = self.graph.take() {
            graph.dispose(&mut self.factory, &mut ());
        }
    }
}

/// System
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct ManageGraph;

impl<'a> System<'a> for ManageGraph {
    type SystemData = (
        ReadStorage<'a, Window>,
        Write<'a, ManageGraphReader, ManualSetupHandler>,
        Write<'a, Rendy, ManualSetupHandler>,
    );

    fn run(&mut self, (windows, mut reader, mut rendy): Self::SystemData) {
        let windows_changed = windows.channel().read(&mut reader.0).count() > 0;
        let should_teardown = rendy.graph.is_some() && windows_changed;

        if should_teardown {
            let graph = rendy.graph.take().unwrap();

            graph.dispose(&mut rendy.factory, &mut ());
        }

        let at_least_one_window = windows.join().count() > 0;
        let should_build = rendy.graph.is_none() && at_least_one_window;

        if should_build {
            let mut builder = GraphBuilder::new();

            for (window,) in (&windows,).join() {
                let surface = rendy.factory.create_surface(window.0.clone());

                let color = builder.create_image(
                    surface.kind(),
                    1,
                    rendy.factory.get_surface_format(&surface),
                    Some(hal::command::ClearValue::Color([1.0, 1.0, 1.0, 1.0].into())),
                );

                let pass = builder.add_node(
                    TriangleRenderPipeline::builder()
                        .into_subpass()
                        .with_color(color)
                        .into_pass(),
                );

                builder.add_node(
                    PresentNode::builder(&rendy.factory, surface, color).with_dependency(pass),
                );
            }

            let graph = builder
                .build(
                    unsafe { &mut *(&rendy.factory as *const _ as *mut _) },
                    unsafe { &mut *(&rendy.families as *const _ as *mut _) },
                    &mut (),
                )
                .unwrap();

            rendy.graph = Some(graph);
        }
    }
}

/// Resource
#[derive(Debug)]
pub struct ManageGraphReader(pub ReaderId<ComponentEvent>);

impl ManualSetup for ManageGraphReader {
    fn setup(res: &mut Resources) {
        WriteStorage::<Window>::setup(res);
        let reader = WriteStorage::<Window>::fetch(&res).register_reader();
        res.insert(Self(reader));
    }
}

/// System
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct RunGraphics;

impl<'a> System<'a> for RunGraphics {
    type SystemData = (Write<'a, Rendy, ManualSetupHandler>,);

    fn run(&mut self, (mut rendy,): Self::SystemData) {
        let factory: &mut Factory<Backend> =
            unsafe { &mut *(&rendy.factory as *const _ as *mut _) };
        let families: &mut Families<Backend> =
            unsafe { &mut *(&rendy.families as *const _ as *mut _) };

        factory.maintain(families);

        if let Some(graph) = rendy.graph.as_mut() {
            graph.run(factory, families, &mut ());
        }
    }
}

////////////////////////////////////////////////////////

lazy_static::lazy_static! {
    static ref VERTEX: StaticShaderInfo = StaticShaderInfo::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/shader.vert"),
        ShaderKind::Vertex,
        SourceLanguage::GLSL,
        "main",
    );

    static ref FRAGMENT: StaticShaderInfo = StaticShaderInfo::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/shader.frag"),
        ShaderKind::Fragment,
        SourceLanguage::GLSL,
        "main",
    );

    static ref SHADERS: rendy::shader::ShaderSetBuilder =
        rendy::shader::ShaderSetBuilder::default()
            .with_vertex(&*VERTEX).unwrap()
            .with_fragment(&*FRAGMENT).unwrap();

    static ref SHADER_REFLECTION: SpirvReflection = SHADERS.reflect().unwrap();
}

#[derive(Debug, Default)]
struct TriangleRenderPipelineDesc;

#[derive(Debug)]
struct TriangleRenderPipeline<B: hal::Backend> {
    vertex: Option<Escape<Buffer<B>>>,
}

impl<B, T> SimpleGraphicsPipelineDesc<B, T> for TriangleRenderPipelineDesc
where
    B: hal::Backend,
    T: ?Sized,
{
    type Pipeline = TriangleRenderPipeline<B>;

    fn depth_stencil(&self) -> Option<hal::pso::DepthStencilDesc> {
        None
    }

    fn load_shader_set(&self, factory: &mut Factory<B>, _aux: &T) -> rendy::shader::ShaderSet<B> {
        SHADERS.build(factory).unwrap()
    }

    fn vertices(
        &self,
    ) -> Vec<(
        Vec<hal::pso::Element<hal::format::Format>>,
        hal::pso::ElemStride,
        hal::pso::InstanceRate,
    )> {
        return vec![SHADER_REFLECTION
            .attributes_range(..)
            .unwrap()
            .gfx_vertex_input_desc(0)];
    }

    fn build<'a>(
        self,
        _ctx: &GraphContext<B>,
        _factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &T,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[Handle<DescriptorSetLayout<B>>],
    ) -> Result<TriangleRenderPipeline<B>, failure::Error> {
        assert!(buffers.is_empty());
        assert!(images.is_empty());
        assert!(set_layouts.is_empty());

        Ok(TriangleRenderPipeline { vertex: None })
    }
}

impl<B, T> SimpleGraphicsPipeline<B, T> for TriangleRenderPipeline<B>
where
    B: hal::Backend,
    T: ?Sized,
{
    type Desc = TriangleRenderPipelineDesc;

    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        _set_layouts: &[Handle<DescriptorSetLayout<B>>],
        _index: usize,
        _aux: &T,
    ) -> PrepareResult {
        if self.vertex.is_none() {
            let vbuf_size = SHADER_REFLECTION.attributes_range(..).unwrap().stride as u64 * 3;

            let mut vbuf = factory
                .create_buffer(
                    BufferInfo {
                        size: vbuf_size,
                        usage: hal::buffer::Usage::VERTEX,
                    },
                    Dynamic,
                )
                .unwrap();

            unsafe {
                // Fresh buffer.
                factory
                    .upload_visible_buffer(
                        &mut vbuf,
                        0,
                        &[
                            PosColor {
                                position: [0.0, -0.5, 0.0].into(),
                                color: [1.0, 0.0, 0.0, 1.0].into(),
                            },
                            PosColor {
                                position: [0.5, 0.5, 0.0].into(),
                                color: [0.0, 1.0, 0.0, 1.0].into(),
                            },
                            PosColor {
                                position: [-0.5, 0.5, 0.0].into(),
                                color: [0.0, 0.0, 1.0, 1.0].into(),
                            },
                        ],
                    )
                    .unwrap();
            }

            self.vertex = Some(vbuf);
        }

        PrepareResult::DrawReuse
    }

    fn draw(
        &mut self,
        _layout: &B::PipelineLayout,
        mut encoder: RenderPassEncoder<'_, B>,
        _index: usize,
        _aux: &T,
    ) {
        let vbuf = self.vertex.as_ref().unwrap();
        encoder.bind_vertex_buffers(0, Some((vbuf.raw(), 0)));
        encoder.draw(0..3, 0..1);
    }

    fn dispose(self, _factory: &mut Factory<B>, _aux: &T) {}
}
