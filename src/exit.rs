use bevy::{app::AppExit, prelude::*};

pub struct ExitPlugin<S: States> {
    pub run_state: S,
}

impl<S: States> Plugin for ExitPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.run_state.clone()), exit_system);
    }
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}
