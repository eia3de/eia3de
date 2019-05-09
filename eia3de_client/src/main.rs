use specs::prelude::*;

fn main() {
    let _ = simple_logger::init();

    let mut world = World::new();
    let mut dispatcher = dispatcher();

    dispatcher.setup(&mut world.res);
    setup(&mut world);

    loop {
        dispatcher.dispatch(&world.res);
        world.maintain();

        use eia3de_client::windowing::WindowLookup;
        if world.read_resource::<WindowLookup>().0.is_empty() {
            break;
        }
    }
}

#[rustfmt::skip]
fn dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    use eia3de_client::{windowing::*, graphics::*};

    DispatcherBuilder::new()
        .with(DispatchWinitEvents,  "dispatch_winit_events",    &[])
        .with(DestroyWindows,       "destroy_windows",          &["dispatch_winit_events"])
        .with(UpdateWindowLookup,   "update_window_lookup",     &["destroy_windows"])
        .with(ManageGraph,          "manage_graph",             &["destroy_windows"])
        .with(RunGraphics,          "run_graphics",             &["manage_graph"])
        .build()
}

fn setup(world: &mut World) {
    use eia3de_client::windowing::*;

    for _ in 0..5 {
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
}
