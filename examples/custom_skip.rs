use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{
    ClearSplash, SplashAssetType, SplashItem, SplashPlugin, SplashScreen, SplashScreenSkipEvent,
    SplashTextColorLens,
};
use bevy_tweening::*;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq)]
enum ScreenStates {
    #[default]
    Splash,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ScreenStates>()
        .add_plugins(
            SplashPlugin::new(ScreenStates::Splash, ScreenStates::Menu)
                .ignore_default_events()
                .skipable()
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(
                            Text::from_sections([
                                TextSection::new(
                                    "Sergio Ribera\n",
                                    TextStyle {
                                        font_size: 76.,
                                        ..default()
                                    },
                                ),
                                TextSection::new(
                                    "presents\n",
                                    TextStyle {
                                        font_size: 38.,
                                        ..default()
                                    },
                                ),
                            ])
                            .with_justify(JustifyText::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::WHITE,
                        width: Val::Percent(40.),
                        height: Val::Px(80.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs(5),
                        is_static: false,
                    }],
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(
                            Text::from_section(
                                "Custom Skip\n",
                                TextStyle {
                                    font_size: 75.,
                                    ..default()
                                },
                            )
                            .with_justify(JustifyText::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::WHITE,
                        width: Val::Percent(35.),
                        height: Val::Px(160.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs(5),
                        is_static: false,
                    }],
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }),
        )
        .add_systems(Startup, create_scene)
        .add_systems(Update, button_system)
        .run()
}

fn create_scene(mut cmd: Commands, assets: ResMut<AssetServer>) {
    cmd.spawn(Camera2dBundle::default());

    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            direction: Direction::LeftToRight,
            align_items: AlignItems::FlexEnd,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            overflow: Overflow::clip(),
            ..default()
        },
        ..default()
    })
    .insert(ClearSplash)
    .with_children(|cmd| {
        cmd.spawn(ButtonBundle {
            style: Style {
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE.with_a(0.)),
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Press Any Key or Touch screen for skip",
                        TextStyle {
                            font_size: 50.,
                            font: assets.load("FiraSans-Bold.ttf"),
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    ..default()
                },
                Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(3),
                        SplashTextColorLens::new(vec![Color::WHITE]),
                    )
                    .with_repeat_count(RepeatCount::Infinite)
                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat),
                ),
            ));
        });
    });
}

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut send_skip: EventWriter<SplashScreenSkipEvent>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => send_skip.send(SplashScreenSkipEvent),
            _ => {}
        }
    }
}
