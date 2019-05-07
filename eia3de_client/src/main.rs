use eia3de_client::windowing;
use specs::prelude::*;

fn main() {
    let _ = simple_logger::init();

    let mut world = World::new();

    world.register::<windowing::Window>();

    world.add_resource(windowing::WinitEventLoop::default());
    world.add_resource(windowing::WindowLookup::default());

    let mut wec = windowing::WinitEventChannel::default();
    world.add_resource(windowing::DestroyWindowsReader(wec.register_reader()));
    world.add_resource(wec);

    let reader = world.write_storage::<windowing::Window>().register_reader();
    world.add_resource(windowing::UpdateWindowLookupReader(reader));

    let window = {
        let event_loop = &world.system_data::<Read<windowing::WinitEventLoop>>().0;

        winit::WindowBuilder::new()
            .with_title("eia3de")
            .build(event_loop)
            .expect("window creation")
    };

    world
        .create_entity()
        .with(windowing::Window(window))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(windowing::DispatchWinitEvents, "dispatch_winit_events", &[])
        .with(
            windowing::DestroyWindows,
            "destroy_windows",
            &["dispatch_winit_events"],
        )
        .with(
            windowing::UpdateWindowLookup,
            "update_window_lookup",
            &["dispatch_winit_events", "destroy_windows"],
        )
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
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
