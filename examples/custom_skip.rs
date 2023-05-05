use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{
    Animator, ClearSplash, EaseFunction, RepeatCount, RepeatStrategy, SplashAssetType, SplashItem,
    SplashPlugin, SplashScreen, SplashScreenSkipEvent, SplashTextColorLens, Tween,
};

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
        .add_plugin(
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
                            .with_alignment(TextAlignment::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::WHITE,
                        size: Size::new(Val::Percent(40.), Val::Px(80.)),
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
                            .with_alignment(TextAlignment::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::WHITE,
                        size: Size::new(Val::Percent(35.), Val::Px(160.)),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs(5),
                        is_static: false,
                    }],
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }),
        )
        .add_startup_system(create_scene)
        .add_system(button_system)
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
            size: Size {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
            },
            overflow: Overflow::Hidden,
            ..default()
        },
        ..default()
    })
    .insert(ClearSplash)
    .with_children(|cmd| {
        cmd.spawn(ButtonBundle {
            style: Style {
                size: Size::height(Val::Px(65.0)),
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
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(3),
                        SplashTextColorLens {
                            start: Color::WHITE,
                            end: Color::WHITE.with_a(0.),
                        },
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
            Interaction::Clicked => send_skip.send(SplashScreenSkipEvent),
            _ => {}
        }
    }
}
