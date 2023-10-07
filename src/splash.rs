use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

use crate::{
    systems::{ClearSplash, SplashBackground},
    InstanceLens, SplashAssetType, SplashImageColorLens, SplashItem, SplashScreens,
    SplashTextColorLens, SplashType, WaitScreenType,
};

fn get_max_duration(screens: SplashScreens, curr_screen: usize) -> Duration {
    if curr_screen == 0 {
        return Duration::from_secs(1);
    }
    let next_screen = screens.0.get(curr_screen - 1).unwrap();

    return match next_screen.wait_to_start {
        WaitScreenType::AfterEnd => Duration::from_secs(
            next_screen
                .brands
                .iter()
                .map(|b| b.duration.as_secs())
                .max()
                .unwrap_or(1)
                * 2
                * curr_screen as u64
                + 1,
        ),
        WaitScreenType::Specific(t) => t,
    };
}

pub(crate) fn create_splash(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    screens: Res<SplashScreens>,
) {
    // Background
    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            overflow: Overflow::clip(),
            ..default()
        },
        background_color: BackgroundColor(screens.0[0].background_color.0),
        ..default()
    })
    .insert(ClearSplash)
    .insert(SplashBackground {
        screens: screens
            .0
            .iter()
            .map(|s| s.brands.len() as u64 * 2)
            .collect(),
        screen_colors: screens.0.iter().map(|s| s.background_color.0).collect(),
    });

    // Create each screen
    for (i_screen, screen) in screens.0.iter().enumerate() {
        let (flex_direction, flex_wrap) = match screen.splash_type {
            SplashType::List => (FlexDirection::Column, FlexWrap::NoWrap),
            SplashType::Grid => (FlexDirection::Row, FlexWrap::Wrap),
        };
        let max_duration = get_max_duration(screens.clone(), i_screen);

        // Parent of screen content
        // Contains brands
        cmd.spawn(NodeBundle {
            style: Style {
                flex_wrap,
                flex_direction,
                display: Display::Flex,
                position_type: PositionType::Absolute,
                direction: Direction::LeftToRight,
                align_items: AlignItems::Center,
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
            for brand in screen.brands.iter() {
                match &brand.asset {
                    SplashAssetType::SingleText(text, font) => {
                        let text = Text::from_sections(text.sections.iter().map(|s| TextSection {
                            value: s.value.clone(),
                            style: TextStyle {
                                font: assets.load(font),
                                ..s.style
                            },
                        }))
                        .with_alignment(text.alignment);
                        cmd.spawn((
                            TextBundle {
                                text: text.clone(),
                                style: Style {
                                    flex_direction,
                                    flex_wrap,
                                    width: brand.width,
                                    height: brand.height,
                                    ..default()
                                },
                                ..default()
                            },
                            create_animator::<Text, SplashTextColorLens>(
                                brand,
                                max_duration,
                                i_screen,
                            ),
                        ))
                    }
                    SplashAssetType::SingleImage(handler) => cmd.spawn((
                        ImageBundle {
                            image: UiImage {
                                texture: handler.clone(),
                                flip_x: false,
                                flip_y: false,
                            },
                            style: Style {
                                width: brand.width,
                                height: brand.height,
                                ..default()
                            },
                            ..default()
                        },
                        create_animator::<BackgroundColor, SplashImageColorLens>(
                            brand,
                            max_duration,
                            i_screen,
                        ),
                    )),
                };
            }
        });
    }
}

fn create_animator<C, L>(
    brand: &SplashItem,
    max_duration: Duration,
    curr_screen: usize,
) -> Animator<C>
where
    C: Component,
    L: Lens<C> + InstanceLens + Send + Sync + 'static,
{
    Animator::new(
        Tween::new(
            brand.ease_function,
            Duration::from_secs(1),
            L::create(brand.tint.with_a(0.), brand.tint.with_a(0.)),
        )
        .then(
            Delay::new(max_duration).then(
                Tween::new(
                    brand.ease_function,
                    brand.duration,
                    L::create(brand.tint.with_a(0.), brand.tint),
                )
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                .with_repeat_count(RepeatCount::Finite(2))
                .with_completed_event(curr_screen as u64),
            ),
        ),
    )
}
