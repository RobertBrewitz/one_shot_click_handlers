use bevy::{ecs::system::SystemId, prelude::*};

use crate::{button, RouteState};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_click_handlers);
        app.add_systems(OnEnter(RouteState::Settings), spawn_menu);
        app.add_systems(OnExit(RouteState::Settings), despawn_menu);
    }
}

#[derive(Resource)]
struct ClickHandlers {
    pub back_system_id: SystemId,
}

fn setup_click_handlers(world: &mut World) {
    let back_system_id = world.register_system(back);

    world.insert_resource(ClickHandlers { back_system_id });
}

fn back(mut next_route: ResMut<NextState<RouteState>>) {
    next_route.set(RouteState::Menu);
}

#[derive(Component)]
struct SettingsMarker;

fn spawn_menu(mut cmd: Commands, systems: Res<ClickHandlers>) {
    let back_system_id = systems.back_system_id;

    cmd.spawn((
        Name::new("Settings Container"),
        SettingsMarker,
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
        button!(container, "Back", back_system_id);
    });
}

fn despawn_menu(mut cmd: Commands, menu_q: Query<Entity, With<SettingsMarker>>) {
    for entity in menu_q.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}
