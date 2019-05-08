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

        if world
            .read_resource::<windowing::WindowLookup>()
            .0
            .is_empty()
        {
            break;
        }
    }
}

#[rustfmt::skip]
fn dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    use eia3de_client::windowing::*;

    DispatcherBuilder::new()
        .with(DispatchWinitEvents,  "dispatch_winit_events",    &[])
        .with(DestroyWindows,       "destroy_windows",          &["dispatch_winit_events"])
        .with(UpdateWindowLookup,   "update_window_lookup",     &["destroy_windows"])
        .build()
}

fn setup(world: &mut World) {
    use eia3de_client::windowing::*;

    let window = {
        let event_loop = world.read_resource::<WinitEventLoop>();
        let lock = event_loop.inner.try_lock().unwrap();

        winit::WindowBuilder::new()
            .with_title("eia3de")
            .build(&*lock)
            .unwrap()
    };

    world.create_entity().with(Window(window)).build();
}
