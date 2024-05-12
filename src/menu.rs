use bevy::{ecs::system::SystemId, prelude::*};

use crate::{button, RouteState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_click_handlers);
        app.add_systems(OnEnter(RouteState::Menu), spawn_menu);
        app.add_systems(OnExit(RouteState::Menu), despawn_menu);
    }
}

#[derive(Resource)]
struct ClickHandlers {
    pub game_system_id: SystemId,
    pub settings_system_id: SystemId,
    pub quit_system_id: SystemId,
}

fn setup_click_handlers(world: &mut World) {
    let game_system_id = world.register_system(game);
    let settings_system_id = world.register_system(settings);
    let quit_system_id = world.register_system(quit);

    world.insert_resource(ClickHandlers {
        game_system_id,
        settings_system_id,
        quit_system_id,
    });
}

fn game(mut next_route: ResMut<NextState<RouteState>>) {
    next_route.set(RouteState::Game);
}

fn settings(mut next_route: ResMut<NextState<RouteState>>) {
    next_route.set(RouteState::Settings);
}

fn quit(mut next_route: ResMut<NextState<RouteState>>) {
    next_route.set(RouteState::Exit);
}

#[derive(Component)]
struct MenuMarker;

fn spawn_menu(mut cmd: Commands, systems: Res<ClickHandlers>) {
    let game_system_id = systems.game_system_id;
    let settings_system_id = systems.settings_system_id;
    let quit_system_id = systems.quit_system_id;

    cmd.spawn((
        Name::new("Menu Container"),
        MenuMarker,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|container| {
        button!(container, "Game", game_system_id);
        button!(container, "Settings", settings_system_id);
        button!(container, "Exit", quit_system_id);
    });
}

fn despawn_menu(mut cmd: Commands, menu_q: Query<Entity, With<MenuMarker>>) {
    for entity in menu_q.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}
