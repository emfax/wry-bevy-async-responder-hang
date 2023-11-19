use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowCreated},
    winit::WinitWindows,
};
use crossbeam::channel::{unbounded, Receiver};
use wry::{http::Response, Rect, RequestAsyncResponder, WebViewBuilder};

#[derive(Resource)]
struct EventRx(Receiver<RequestAsyncResponder>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, webview_init.run_if(on_event::<WindowCreated>()))
        .add_systems(Update, events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn webview_init(world: &mut World) {
    let (id, _) = world.query::<(Entity, With<PrimaryWindow>)>().single(world);

    let Some(winit_windows) = world.get_non_send_resource::<WinitWindows>() else {
        warn!("could not find winit windows");

        return;
    };

    let Some(window) = winit_windows.get_window(id) else {
        warn!("could not find primary window");

        return;
    };

    let size = window.inner_size();

    let (event_tx, event_rx) = unbounded::<RequestAsyncResponder>();

    let webview = WebViewBuilder::new_as_child(window)
        .with_bounds(Rect {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        })
        .with_transparent(true)
        .with_asynchronous_custom_protocol("event".to_string(), move |_, responder| {
            let _ = event_tx.send(responder);
        })
        .with_url("http://localhost:1420")
        .unwrap()
        .build()
        .unwrap();

    world.insert_non_send_resource(webview);
    world.insert_resource(EventRx(event_rx));
}

fn events(rx: Option<Res<EventRx>>) {
    let Some(rx) = rx else {
        return;
    };

    let Ok(responder) = rx.0.try_recv() else {
        return;
    };

    let response = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .body(r#"{"kind":"event","name":"test"}"#.to_string().as_bytes().to_vec())
        .unwrap();

    responder.respond(response);
}
