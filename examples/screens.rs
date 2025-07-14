use bevy::color::palettes;
use bevy::prelude::*;
use bevy_splash_screen::{
    SplashAssetType, SplashItem, SplashPlugin, SplashScreen, SplashText, SplashTextSection,
};
use std::time::Duration;

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
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(SplashText {
                            sections: vec![
                                SplashTextSection {
                                    text: "Simple Test\n".into(),
                                    text_font: "FiraSans-Bold.ttf".to_string(),
                                    text_size: 40.,
                                    text_color: Color::WHITE.into(),
                                },
                                SplashTextSection {
                                    text: "by\n".into(),
                                    text_font: "FiraSans-Bold.ttf".to_string(),
                                    text_size: 24.,
                                    text_color: Color::WHITE.with_alpha(0.75).into(),
                                },
                                SplashTextSection {
                                    text: "Sergio Ribera\n".into(),
                                    text_font: "FiraSans-Bold.ttf".to_string(),
                                    text_size: 32.,
                                    text_color: Srgba::WHITE.into(),
                                },
                            ],
                            text_alignment: JustifyText::Center,
                        }),
                        tint: palettes::css::SEA_GREEN.into(),
                        width: Val::Percent(30.),
                        height: Val::Px(150.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs_f32(5.),
                        is_static: false,
                    }],
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(SplashText {
                            sections: vec![SplashTextSection {
                                text: "With Bevy Engine".into(),
                                text_font: "FiraSans-Bold.ttf".to_string(),
                                text_size: 32.,
                                text_color: Color::WHITE.into(),
                            }],
                            text_alignment: JustifyText::Center,
                        }),
                        tint: Color::WHITE,
                        width: Val::Percent(30.),
                        height: Val::Px(150.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs_f32(5.),
                        is_static: false,
                    }],
                    wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(SplashText {
                            sections: vec![SplashTextSection {
                                text: "With Love <3".into(),
                                text_font: "FiraSans-Bold.ttf".to_string(),
                                text_size: 32.,
                                text_color: Color::WHITE.into(),
                            }],
                            text_alignment: JustifyText::Center,
                        }),
                        tint: Srgba::RED.into(),
                        width: Val::Percent(30.),
                        height: Val::Px(150.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs_f32(5.),
                        is_static: false,
                    }],
                    wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }),
        )
        .add_systems(Startup, create_scene)
        .run();
}

fn create_scene(mut cmd: Commands) {
    cmd.spawn(Camera2d::default());
}
