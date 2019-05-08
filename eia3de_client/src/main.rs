use eia3de_client::windowing;
use specs::prelude::*;

fn main() {
    let _ = simple_logger::init();

    let mut world = World::new();
    let mut dispatcher = dispatcher();

    setup(&mut world);
    dispatcher.setup(&mut world.res);

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
    world.register::<windowing::Window>();

    world.add_resource(windowing::WinitEventLoop::default());
    world.add_resource(windowing::WindowLookup::default());

    let mut wec = windowing::WinitEventChannel::default();
    world.add_resource(windowing::DestroyWindowsReader(wec.register_reader()));
    world.add_resource(wec);

    let reader = world.write_storage::<windowing::Window>().register_reader();
    world.add_resource(windowing::UpdateWindowLookupReader(reader));

    let window = {
        let event_loop = &world.read_resource::<windowing::WinitEventLoop>();
        let lock = event_loop.inner.try_lock().unwrap();

        winit::WindowBuilder::new()
            .with_title("eia3de")
            .build(&*lock)
            .unwrap()
    };

    world
        .create_entity()
        .with(windowing::Window(window))
        .build();
}
