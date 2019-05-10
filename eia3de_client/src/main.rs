use eia3de_client::{graphics::*, windowing::*, Teardown};
use specs::prelude::*;

fn main() {
    let _ = simple_logger::init_with_level(log::Level::Warn);

    let mut world = World::new();
    let mut dispatcher = dispatcher();

    dispatcher.setup(&mut world.res);
    setup(&mut world);

    loop {
        dispatcher.dispatch(&world.res);
        world.maintain();

        if world.read_resource::<WindowLookup>().0.is_empty() {
            break;
        }
    }

    world.add_resource(Teardown);
    DestroyWindows.run_now(&mut world.res);
    DestroyGraph.run_now(&mut world.res);
}

#[rustfmt::skip]
fn dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {

    DispatcherBuilder::new()
        .with(DispatchWinitEvents,  "dispatch_winit_events",    &[])
        .with(DestroyWindows,       "destroy_windows",          &["dispatch_winit_events"])
        .with(UpdateWindowLookup,   "update_window_lookup",     &["destroy_windows"])
        .with(DestroyGraph,         "destroy_graph",            &["destroy_windows"])
        .with(BuildGraph,           "build_graph",              &["destroy_graph"])
        .with(RunGraphics,          "run_graphics",             &["build_graph"])
        .build()
}

fn setup(world: &mut World) {
    let window = {
        let event_loop = world.read_resource::<WinitEventLoop>();
        let lock = event_loop.inner.try_lock().unwrap();

        winit::WindowBuilder::new()
            .with_title("eia3de")
            .with_dimensions((200, 200).into())
            .build(&*lock)
            .unwrap()
    };

    world.create_entity().with(Window(window.into())).build();
}
