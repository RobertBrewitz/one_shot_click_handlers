use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use bevy::asset::AssetMetaCheck;

use button::ButtonPlugin;
use camera::MainCameraPlugin;
use exit::ExitPlugin;
use game::GamePlugin;
use menu::MenuPlugin;
use settings::SettingsPlugin;

mod button;
mod camera;
mod exit;
mod game;
mod menu;
mod settings;

#[derive(Debug, Default, States, Hash, Clone, PartialEq, Eq, Reflect)]
pub enum RouteState {
    #[default]
    Menu,
    Game,
    Settings,
    Exit,
}

fn main() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.insert_resource(AssetMetaCheck::Never);

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
                    ..default()
                }),
                ..Default::default()
            }),
    );

    app.init_state::<RouteState>();

    app.add_plugins((
        MainCameraPlugin,
        MenuPlugin,
        SettingsPlugin,
        GamePlugin,
        ButtonPlugin,
        ExitPlugin {
            run_state: RouteState::Exit,
        },
    ));

    app.run();
}
