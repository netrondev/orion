use bevy::{color::palettes::basic::*, prelude::*, winit::WinitSettings};

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
//         .insert_resource(WinitSettings::desktop_app())
//         .add_systems(Startup, setup)
//         .add_systems(Update, button_system)
//         .run();
// }

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn ui_system_update_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Record".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Record".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Record".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn ui_system_startup(mut commands: Commands, assets: Res<AssetServer>) {
    // ui camera
    commands.spawn(button(&assets)).observe(on_click_spawn_cube);
}

fn button(asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),

            position_type: PositionType::Absolute,
            top: Val::Percent(2.0),
            left: Val::Percent(2.0),

            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Record"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}

fn on_click_spawn_cube(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut num: Local<usize>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.25 + 0.55 * *num as f32, 0.0),
        ))
        // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
        .observe(on_drag_rotate);
    *num += 1;
}

fn on_drag_rotate(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target()) {
        transform.rotate_y(drag.delta.x * 0.02);
        transform.rotate_x(drag.delta.y * 0.02);
    }
}
