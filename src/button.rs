use bevy::{ecs::system::SystemId, prelude::*};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clickable_update_system);
    }
}

#[derive(Component)]
pub struct Clickable {
    pub system_id: SystemId,
    pub pressed: bool,
}

fn clickable_update_system(
    mut cmd: Commands,
    mut q: Query<(&Interaction, &mut Clickable), Changed<Interaction>>,
) {
    for (interaction, mut button) in q.iter_mut() {
        if *interaction == Interaction::Pressed {
            button.pressed = true;
        }

        if *interaction == Interaction::Hovered && button.pressed {
            button.pressed = false;
            cmd.run_system(button.system_id);
        }

        if *interaction == Interaction::None {
            button.pressed = false;
        }
    }
}

/// button!($world, $text, $system_id)
///
/// $world => &mut World | mut Commands
/// $text => &str
/// $system_id => SystemId
#[macro_export]
macro_rules! button {
    (
        $world:ident,
        $text:expr,
        $system_id:ident
    ) => {
        $world
            .spawn((
                Name::new(format!("{} Button", $text)),
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::all(Val::Px(0.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..Default::default()
                },
                $crate::button::Clickable {
                    system_id: $system_id,
                    pressed: false,
                },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Name::new(format!("{} Button Text", $text)),
                    TextBundle {
                        text: Text::from_section(
                            $text,
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        ),
                        ..default()
                    },
                ));
            })
    };
}
