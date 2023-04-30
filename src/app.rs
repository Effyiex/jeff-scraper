
use bevy::{
  prelude::*,
  winit::WinitWindows,
  core_pipeline::clear_color::ClearColorConfig
};

use winit::{
  dpi::{
    PhysicalSize,
    PhysicalPosition
  },
  window::WindowLevel
};

pub struct AppPlugin;
pub struct AppToggleEvent;

impl Plugin for AppPlugin {

  fn build(&self, app: &mut App) {
    
    app

      .add_event::<AppToggleEvent>()
      .add_system(await_toggle)

      .add_startup_system(setup);

  }

}

#[derive(Resource)]
struct AppResources {
  regular_font: Handle<Font>
}

fn setup(
  mut commands: Commands,
  mut windows: Query<&mut Window>,
  winit_buffer: NonSend<WinitWindows>,
  asset_server: Res<AssetServer>
) {

  for mut window in windows.iter_mut() {
    window.title = format!("{} ({})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
  }
  println!("[{}::app::setup] // Set Window-Title.", env!("CARGO_PKG_NAME"));

  for window in winit_buffer.windows.iter() {

    window.1.set_decorations(false);
    window.1.set_window_level(WindowLevel::AlwaysOnTop);
    window.1.set_visible(false);
    println!("[{}::app::setup] // Set Window-Defaults.", env!("CARGO_PKG_NAME"));

    if let Some(monitor) = window.1.current_monitor() {
      let screen_size: PhysicalSize<u32> = monitor.size();
      window.1.set_inner_size(PhysicalSize::new(screen_size.width / 3, screen_size.height));
      window.1.set_outer_position(PhysicalPosition::new(screen_size.width / 4 * 3, 0));
    }
    println!("[{}::app::setup] // Set Window-Boundaries.", env!("CARGO_PKG_NAME"));

  }

  commands.spawn(Camera2dBundle {
    camera_2d: Camera2d {
      clear_color: ClearColorConfig::Custom(Color::rgb(0.05, 0.05, 0.075)) 
    },
    ..Default::default()
  });
  println!("[{}::app::setup] // Spawned Camera.", env!("CARGO_PKG_NAME"));

  let resources: AppResources = AppResources { 
    regular_font: asset_server.load("nunito_regular.ttf")
  };

  create_menu(&mut commands, &resources);

  commands.insert_resource(resources);

}

fn await_toggle(
    mut event_reader: EventReader<AppToggleEvent>,
    winit_buffer: NonSend<WinitWindows>
) {

    if event_reader.is_empty() {
        return;
    }
    event_reader.clear();
    println!("[{}::app::await_toggle] // AppToggleEvent received.", env!("CARGO_PKG_NAME"));

    for window in winit_buffer.windows.iter() {

      window.1.set_visible(!window.1.is_visible().unwrap());

      if window.1.is_visible().unwrap() {
        println!("[{}::app::await_toggle] // Visible.", env!("CARGO_PKG_NAME"));
      } else {
        println!("[{}::app::await_toggle] // Invisible.", env!("CARGO_PKG_NAME"));
      }

    }

}

fn create_menu(
  commands: &mut Commands,
  resources: &AppResources
)  {
  
  commands.spawn(ImageBundle {
    style: Style {
      position_type: PositionType::Absolute,
      size: Size::all(Val::Percent(100.0)),
      align_items: AlignItems::Center,
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::Start,
      ..Default::default()
    },
    background_color: Color::rgb(0.05, 0.05, 0.075).into(),
    ..Default::default()
  })
    .with_children(|parent| {

      parent.spawn(ImageBundle {
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect::top(Val::Px(0.0)),
          size: Size::new(Val::Percent(100.0), Val::Px(64.0)),
          ..Default::default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.25).into(),
        ..Default::default()
      })
        .with_children(|parent| {

          parent.spawn(TextBundle {
            style: Style {
              margin: UiRect { 
                left: Val::Px(16.0), 
                right: Val::Auto, 
                top: Val::Auto, 
                bottom: Val::Auto 
              },
              ..Default::default()
            },
            text: Text::from_section(
              format!("{} ({})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
              TextStyle {
                font: resources.regular_font.clone(),
                font_size: 48.0,
                color: Color::WHITE
              }
            ),
            ..Default::default()
          });

        });

    });

  println!("[{}::app::create_menu] // Menu created.", env!("CARGO_PKG_NAME"));

}