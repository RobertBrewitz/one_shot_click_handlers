use bevy::prelude::*;

use crate::RouteState;

pub struct GamePlugin;

#[derive(Debug, Default, States, Hash, Clone, PartialEq, Eq, Reflect)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_systems(OnEnter(RouteState::Game), enter_game_system);
        app.add_systems(
            Update,
            update_game_system.run_if(in_state(RouteState::Game)),
        );

        app.add_systems(OnExit(RouteState::Game), exit_game_system);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct GameMarker(pub Timer);

fn enter_game_system(mut cmd: Commands) {
    cmd.spawn((
        Name::new("Game"),
        GameMarker(Timer::from_seconds(1.0, TimerMode::Once)),
    ));
}

fn exit_game_system(mut cmd: Commands, query: Query<Entity, With<GameMarker>>) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}

fn update_game_system(
    time: Res<Time>,
    mut query: Query<&mut GameMarker>,
    mut next_route: ResMut<NextState<RouteState>>,
) {
    for mut game_marker in query.iter_mut() {
        game_marker.tick(time.delta());

        if game_marker.finished() {
            next_route.set(RouteState::Menu);
        }
    }
}
