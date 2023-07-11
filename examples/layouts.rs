use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen, SplashType};
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
        .add_plugins(
            SplashPlugin::new(ScreenStates::Splash, ScreenStates::Menu)
                .skipable()
                .add_screen(SplashScreen {
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
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "With Bevy",
                                    TextStyle {
                                        font_size: 32.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )])
                                .with_alignment(TextAlignment::Center),
                                "FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: Color::WHITE,
                            width: Val::Percent(30.),
                            height: Val::Px(150.),
                            ease_function: EaseFunction::QuarticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: false,
                        },
                    ],
                    splash_type: SplashType::List,
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .add_screen(SplashScreen {
                    brands: vec![
                        SplashItem {
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "Hola Hola Hola",
                                    TextStyle {
                                        font_size: 32.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )])
                                .with_alignment(TextAlignment::Center),
                                "FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: Color::YELLOW,
                            width: Val::Percent(30.),
                            height: Val::Px(150.),
                            ease_function: EaseFunction::QuarticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: false,
                        },
                        SplashItem {
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "Hello Hello Hello",
                                    TextStyle {
                                        font_size: 32.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )])
                                .with_alignment(TextAlignment::Center),
                                "FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: Color::BLUE,
                            width: Val::Percent(30.),
                            height: Val::Px(150.),
                            ease_function: EaseFunction::QuarticInOut.into(),
                            duration: Duration::from_secs_f32(12.),
                            is_static: false,
                        },
                        SplashItem {
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "Test Test Test",
                                    TextStyle {
                                        font_size: 32.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )])
                                .with_alignment(TextAlignment::Center),
                                "FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: Color::WHITE,
                            width: Val::Percent(30.),
                            height: Val::Px(150.),
                            ease_function: EaseFunction::QuarticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: false,
                        },
                        SplashItem {
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "Bevy Bevy Bevy",
                                    TextStyle {
                                        font_size: 32.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )])
                                .with_alignment(TextAlignment::Center),
                                "FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: Color::PURPLE,
                            width: Val::Percent(30.),
                            height: Val::Px(150.),
                            ease_function: EaseFunction::QuarticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: false,
                        },
                    ],
                    splash_type: SplashType::Grid,
                    wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }),
        )
        .add_systems(Startup, create_scene)
        .run()
}

fn create_scene(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
