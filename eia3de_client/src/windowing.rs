//! Windowing

use crate::{ManualSetup, ManualSetupHandler};
use shrev::EventChannel;
use specs::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Resource
#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
pub struct WinitEventLoop {
    #[derivative(Default(value = "Mutex::new(winit::EventsLoop::new())"))]
    pub inner: Mutex<winit::EventsLoop>,
}

/// # Safety
/// winit::EventsLoop is not Send, we wrap it in a std::sync::Mutex and its
/// only access pattern is through the ECS, it *should* be safe.
unsafe impl Send for WinitEventLoop {}

/// # Safety
/// winit::EventsLoop is not Sync, we wrap it in a std::sync::Mutex and its
/// only access pattern is through the ECS, it *should* be safe.
unsafe impl Sync for WinitEventLoop {}

/// Resource
pub type WinitEventChannel = EventChannel<winit::Event>;

/// System
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DispatchWinitEvents;

impl<'a> System<'a> for DispatchWinitEvents {
    type SystemData = (Write<'a, WinitEventLoop>, Write<'a, WinitEventChannel>);

    fn run(&mut self, (wel, mut wec): Self::SystemData) {
        let mut lock = wel
            .inner
            .try_lock()
            .expect("WinitEventLoop must not be shared");

        lock.poll_events(|event| wec.single_write(event));
    }
}

/// Component
#[derive(Debug, specs_derive::Component)]
#[storage(FlaggedStorage)]
pub struct Window(pub Arc<winit::Window>);

/// Resource
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WindowLookup(pub HashMap<winit::WindowId, Entity>);

/// System
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct UpdateWindowLookup;

impl<'a> System<'a> for UpdateWindowLookup {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Window>,
        Write<'a, WindowLookup>,
        Write<'a, UpdateWindowLookupReader, ManualSetupHandler>,
    );

    fn run(&mut self, (entities, windows, mut lookup, mut reader): Self::SystemData) {
        let mut insertions = BitSet::new();
        let mut removals = BitSet::new();

        for event in windows.channel().read(&mut reader.0) {
            match *event {
                ComponentEvent::Inserted(id) => {
                    let _ = insertions.add(id);
                }
                ComponentEvent::Removed(id) => {
                    let _ = removals.add(id);
                }
                _ => {}
            }
        }

        lookup.0.retain(|_, v| !removals.contains(v.id()));

        lookup.0.extend(
            (&entities, &windows, &insertions)
                .join()
                .map(|(entity, window, _)| (window.0.id(), entity)),
        );
    }
}

/// Resource
#[derive(Debug)]
pub struct UpdateWindowLookupReader(pub ReaderId<ComponentEvent>);

impl ManualSetup for UpdateWindowLookupReader {
    fn setup(res: &mut Resources) {
        WriteStorage::<Window>::setup(res);
        let reader = WriteStorage::<Window>::fetch(&res).register_reader();
        res.insert(Self(reader))
    }
}

/// System
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DestroyWindows;

impl<'a> System<'a> for DestroyWindows {
    type SystemData = (
        Read<'a, WindowLookup>,
        Read<'a, WinitEventChannel>,
        Write<'a, DestroyWindowsReader, ManualSetupHandler>,
        WriteStorage<'a, Window>,
    );

    fn run(&mut self, (lookup, channel, mut reader, mut windows): Self::SystemData) {
        use winit::{
            Event::WindowEvent,
            WindowEvent::{CloseRequested, Destroyed},
        };

        for event in channel.read(&mut reader.0) {
            match event {
                WindowEvent {
                    event: CloseRequested,
                    window_id,
                }
                | WindowEvent {
                    event: Destroyed,
                    window_id,
                } => {
                    if let Some(&entity) = lookup.0.get(&window_id) {
                        log::warn!("destroying window component: {:?} {:?}", entity, window_id);
                        let _ = windows.remove(entity);
                    };
                }
                _ => {}
            }
        }
    }
}

/// Resource
#[derive(Debug)]
pub struct DestroyWindowsReader(pub ReaderId<winit::Event>);

impl ManualSetup for DestroyWindowsReader {
    fn setup(res: &mut Resources) {
        Write::<WinitEventChannel>::setup(res);
        let reader = Write::<WinitEventChannel>::fetch(&res).register_reader();
        res.insert(Self(reader))
    }
}
