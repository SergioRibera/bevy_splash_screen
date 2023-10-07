use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen, SplashScreens};
use bevy_tweening::EaseFunction;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq)]
enum ScreenStates {
    #[default]
    Splash,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<ScreenStates>()
        .add_plugins(SplashPlugin::new(ScreenStates::Splash, ScreenStates::Menu).skipable())
        .add_systems(Startup, create_scene)
        .run()
}

fn create_scene(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle::default());
    let bevy_logo = asset_server.load("bevy_logo.png");

    // Create screen resource
    let mut screens = SplashScreens::default();
    screens.add_screen(SplashScreen {
        brands: vec![
            SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_sections([
                        TextSection::new(
                            "Simple Test\n",
                            TextStyle {
                                font_size: 40.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "by\n",
                            TextStyle {
                                font_size: 24.,
                                color: Color::WHITE.with_a(0.75),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Sergio Ribera",
                            TextStyle {
                                font_size: 32.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                    ])
                    .with_alignment(TextAlignment::Center),
                    "FiraSans-Bold.ttf".to_string(),
                ),
                tint: Color::SEA_GREEN,
                width: Val::Percent(30.),
                height: Val::Px(150.),
                ease_function: EaseFunction::QuarticInOut.into(),
                duration: Duration::from_secs_f32(5.),
                is_static: false,
            },
            SplashItem {
                asset: SplashAssetType::SingleImage(bevy_logo.clone()),
                tint: Color::WHITE,
                width: Val::Percent(60.),
                height: Val::Px(150.),
                ease_function: EaseFunction::QuinticInOut.into(),
                duration: Duration::from_secs_f32(5.),
                is_static: true,
            },
        ],
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });

    // add screen
    cmd.insert_resource(screens);
}
