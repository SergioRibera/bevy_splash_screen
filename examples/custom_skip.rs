use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{
    ClearSplash, SplashAssetType, SplashItem, SplashPlugin, SplashScreen, SplashScreenSkipEvent,
    SplashText, SplashTextColorLens, SplashTextSection,
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
                        asset: SplashAssetType::SingleText(SplashText {
                            sections: vec![
                                SplashTextSection {
                                    text: "Sergio Ribera\n".into(),
                                    text_font: "FiraSans-Bold.ttf".to_string(),
                                    text_size: 76.,
                                    text_color: Color::WHITE.into(),
                                },
                                SplashTextSection {
                                    text: "presents\n".into(),
                                    text_font: "FiraSans-Bold.ttf".to_string(),
                                    text_size: 38.,
                                    text_color: Color::WHITE.with_alpha(0.75).into(),
                                },
                            ],
                            text_alignment: JustifyText::Center,
                        }),
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
                        asset: SplashAssetType::SingleText(SplashText {
                            sections: vec![SplashTextSection {
                                text: "Custom Skip\n".into(),
                                text_font: "FiraSans-Bold.ttf".to_string(),
                                text_size: 75.,
                                text_color: Color::WHITE.into(),
                            }],
                            text_alignment: JustifyText::Center,
                        }),
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
        .run();
}

fn create_scene(mut cmd: Commands, assets: ResMut<AssetServer>) {
    cmd.spawn(Camera2d::default());

    cmd.spawn(Node {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        align_items: AlignItems::FlexEnd,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        overflow: Overflow::clip(),
        ..default()
    })
    .insert(ClearSplash)
    .with_children(|cmd| {
        cmd.spawn((
            Button::default(),
            Node {
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::WHITE.with_alpha(0.)),
        ))
        .with_children(|cmd| {
            cmd.spawn((
                Text("Press Any Key or Touch screen for skip".into()),
                TextFont {
                    font: assets.load("FiraSans-Bold.ttf"),
                    font_size: 50.,
                    ..default()
                },
                Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(3),
                        SplashTextColorLens::new(Color::WHITE),
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
        if *interaction == Interaction::Pressed {
            send_skip.write(SplashScreenSkipEvent);
        }
    }
}
