//! Graphics

use crate::{
    windowing::{Window, WinitEventChannel},
    ManualSetup, ManualSetupHandler, Teardown,
};
use rendy::{
    command::Families,
    factory::{Config, Factory},
    graph::{Graph, GraphBuilder},
    vulkan::Backend,
};
use specs::prelude::*;

/// Resource
#[derive(Debug)]
pub struct Rendy {
    pub graph: Option<Graph<Backend, ()>>,
    pub families: Families<Backend>,
    pub factory: Factory<Backend>,
}

impl Rendy {
    fn split_mut<'a>(
        &'a mut self,
    ) -> (
        &'a mut Factory<Backend>,
        &'a mut Families<Backend>,
        Option<&'a mut Graph<Backend, ()>>,
    ) {
        (&mut self.factory, &mut self.families, self.graph.as_mut())
    }
}

impl ManualSetup for Rendy {
    fn setup(res: &mut Resources) {
        let config: Config = Default::default();
        let (factory, families) = rendy::factory::init(config).expect("failed to init rendy");

        res.insert(Self {
            factory,
            families,
            graph: None,
        });
    }
}

/// System
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct DestroyGraph;

impl<'a> System<'a> for DestroyGraph {
    type SystemData = (
        Option<Read<'a, Teardown>>,
        Read<'a, WinitEventChannel>,
        ReadStorage<'a, Window>,
        Write<'a, DestroyGraphReader, ManualSetupHandler>,
        Write<'a, Rendy, ManualSetupHandler>,
    );

    fn run(&mut self, (teardown, wec, windows, mut reader, mut rendy): Self::SystemData) {
        let mut windows_resized = false;
        for event in wec.read(&mut reader.winit) {
            match *event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Resized(_),
                    ..
                } => windows_resized = true,
                _ => {}
            }
        }

        let windows_changed = windows.channel().read(&mut reader.window_storage).count() > 0;
        let should_destroy =
            rendy.graph.is_some() && (teardown.is_some() || windows_resized || windows_changed);

        if should_destroy {
            let graph = rendy.graph.take().unwrap();

            graph.dispose(&mut rendy.factory, &mut ());
            //                                  XXX
        }
    }
}

/// Resource
#[derive(Debug)]
pub struct DestroyGraphReader {
    pub window_storage: ReaderId<ComponentEvent>,
    pub winit: ReaderId<winit::Event>,
}

impl ManualSetup for DestroyGraphReader {
    fn setup(res: &mut Resources) {
        WriteStorage::<Window>::setup(res);
        Write::<WinitEventChannel>::setup(res);

        let window_storage = WriteStorage::<Window>::fetch(&res).register_reader();
        let winit = Write::<WinitEventChannel>::fetch(&res).register_reader();

        res.insert(Self {
            window_storage,
            winit,
        });
    }
}

/// System
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct BuildGraph;

impl<'a> System<'a> for BuildGraph {
    type SystemData = (
        ReadStorage<'a, Window>,
        Write<'a, Rendy, ManualSetupHandler>,
    );

    fn run(&mut self, (windows, mut rendy): Self::SystemData) {
        let at_least_one_window = windows.join().count() > 0;
        let should_build = rendy.graph.is_none() && at_least_one_window;

        if should_build {
            let mut builder = GraphBuilder::new();

            for (window,) in (&windows,).join() {
                let surface = rendy.factory.create_surface(window.0.clone());

                /*
                // XXX
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
                // XXX
                */
            }

            let (mut factory, mut families, _) = rendy.split_mut();
            let graph = builder.build(&mut factory, &mut families, &mut ()).unwrap();
            //                                                      XXX

            rendy.graph = Some(graph);
        }
    }
}

/// System
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct RunGraphics;

impl<'a> System<'a> for RunGraphics {
    type SystemData = (Write<'a, Rendy, ManualSetupHandler>,);

    fn run(&mut self, (mut rendy,): Self::SystemData) {
        let (factory, families, graph) = rendy.split_mut();

        factory.maintain(families);

        if let Some(graph) = graph {
            graph.run(factory, families, &mut ());
            //                              XXX
        }
    }
}
