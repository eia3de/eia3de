//! Windowing

use specs::prelude::*;

/// Resource
#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
pub struct WinitEventLoop(
    #[derivative(Default(value = "winit::EventsLoop::new()"))] pub winit::EventsLoop,
);

unsafe impl Send for WinitEventLoop {}
unsafe impl Sync for WinitEventLoop {}

/// Resource
pub type WinitEventChannel = shrev::EventChannel<winit::Event>;

/// System
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DispatchWinitEvents;

impl<'a> System<'a> for DispatchWinitEvents {
    type SystemData = (Write<'a, WinitEventLoop>, Write<'a, WinitEventChannel>);

    fn run(&mut self, (mut wel, mut wec): Self::SystemData) {
        wel.0.poll_events(|event| wec.single_write(event))
    }
}

/// Component
#[derive(Debug, specs_derive::Component)]
#[storage(FlaggedStorage)]
pub struct Window(pub winit::Window);

/// Resource
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WindowLookup(pub std::collections::HashMap<winit::WindowId, Entity>);

/// System
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct UpdateWindowLookup;

impl<'a> System<'a> for UpdateWindowLookup {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Window>,
        Write<'a, WindowLookup>,
        WriteExpect<'a, UpdateWindowLookupReader>,
    );

    fn run(&mut self, (entities, windows, mut lookup, mut reader): Self::SystemData) {
        let mut insertions = BitSet::new();
        let mut removals = BitSet::new();

        for event in windows.channel().read(&mut reader.0) {
            match *event {
                ComponentEvent::Inserted(id) => {
                    insertions.add(id);
                }
                ComponentEvent::Removed(id) => {
                    removals.add(id);
                }
                _ => {}
            }
        }

        lookup.0.extend(
            (&entities, &windows, &insertions)
                .join()
                .map(|(entity, window, _)| (window.0.id(), entity)),
        );

        let removed_entities = (&entities, &removals)
            .join()
            .map(|(entity, _)| entity)
            .collect::<std::collections::HashSet<_>>();

        lookup.0.retain(|_, v| !removed_entities.contains(v))
    }
}

/// Resource
#[derive(Debug)]
pub struct UpdateWindowLookupReader(pub shrev::ReaderId<ComponentEvent>);

/// System
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DestroyWindows;

impl<'a> System<'a> for DestroyWindows {
    type SystemData = (
        Read<'a, WindowLookup>,
        Read<'a, WinitEventChannel>,
        WriteExpect<'a, DestroyWindowsReader>,
        WriteStorage<'a, Window>,
    );

    fn run(&mut self, (lookup, channel, mut reader, mut windows): Self::SystemData) {
        for event in channel.read(&mut reader.0) {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    window_id,
                }
                | winit::Event::WindowEvent {
                    event: winit::WindowEvent::Destroyed,
                    window_id,
                } => {
                    lookup
                        .0
                        .get(&window_id)
                        .map(|&entity| windows.remove(entity));
                }
                _ => {}
            }
        }
    }
}

/// Resource
#[derive(Debug)]
pub struct DestroyWindowsReader(pub shrev::ReaderId<winit::Event>);
