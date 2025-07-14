use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

use crate::{
    systems::{ClearSplash, SplashBackground},
    InstanceLens, SplashAssetType, SplashImageColorLens, SplashItem, SplashScreens,
    SplashTextColorLens, SplashTextSection, SplashType, WaitScreenType,
};

fn get_max_duration(screens: SplashScreens, curr_screen: usize) -> Duration {
    if curr_screen == 0 {
        return Duration::from_secs(1);
    }
    let next_screen = screens.0.get(curr_screen - 1).unwrap();

    match next_screen.wait_to_start {
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
    }
}

fn create_text_color_animator(
    brand: &SplashItem,
    text_section: &SplashTextSection,
    i_screen: usize,
    max_duration: Duration,
) -> Animator<TextColor> {
    Animator::new(
        Tween::new(
            brand.ease_function,
            Duration::from_secs(1),
            SplashTextColorLens::new(text_section.text_color.0.with_alpha(0.)),
        )
        .then(
            Delay::new(max_duration).then(
                Tween::new(
                    brand.ease_function,
                    brand.duration,
                    SplashTextColorLens::new(text_section.text_color.0.with_alpha(1.)),
                )
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                .with_repeat_count(RepeatCount::Finite(2))
                .with_completed_event(i_screen as u64),
            ),
        ),
    )
}

pub(crate) fn create_splash(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    screens: Res<SplashScreens>,
) {
    // Background
    cmd.spawn((
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(screens.0[0].background_color.0),
    ))
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
        cmd.spawn(Node {
            flex_wrap,
            flex_direction,
            display: Display::Flex,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            overflow: Overflow::clip(),
            ..default()
        })
        .insert(ClearSplash)
        .with_children(|cmd| {
            for brand in screen.brands.iter() {
                match &brand.asset {
                    SplashAssetType::SingleText(splash_text) => {
                        if let Some((first_section, remaining_sections)) =
                            splash_text.sections.split_last()
                        {
                            let mut parent_text = cmd.spawn((
                                first_section.text.clone(),
                                TextFont {
                                    font: assets.load(first_section.text_font.clone()),
                                    font_size: first_section.text_size,
                                    ..default()
                                },
                                first_section.text_color,
                                Node {
                                    flex_direction,
                                    flex_wrap,
                                    width: brand.width,
                                    height: brand.height,
                                    ..default()
                                },
                                TextLayout {
                                    justify: splash_text.text_alignment,
                                    ..default()
                                },
                                create_text_color_animator(
                                    brand,
                                    first_section,
                                    i_screen,
                                    max_duration,
                                ),
                            ));

                            for section in remaining_sections.iter().rev() {
                                parent_text.with_children(|cmd| {
                                    cmd.spawn((
                                        TextSpan(section.text.to_string()),
                                        TextFont {
                                            font: assets.load(section.text_font.clone()),
                                            font_size: section.text_size,
                                            ..default()
                                        },
                                        section.text_color,
                                        Node {
                                            flex_direction,
                                            flex_wrap,
                                            width: brand.width,
                                            height: brand.height,
                                            ..default()
                                        },
                                        create_text_color_animator(
                                            brand,
                                            section,
                                            i_screen,
                                            max_duration,
                                        ),
                                    ));
                                });
                            }
                        }
                    }
                    SplashAssetType::SingleImage(handler) => {
                        cmd.spawn((
                            ImageNode {
                                image: assets.load(handler),
                                flip_x: false,
                                flip_y: false,
                                ..default()
                            },
                            Node {
                                width: brand.width,
                                height: brand.height,
                                ..default()
                            },
                            create_animator::<ImageNode, SplashImageColorLens>(
                                brand,
                                max_duration,
                                i_screen,
                            ),
                        ));
                    }
                }
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
            L::create(brand.tint.with_alpha(0.), brand.tint.with_alpha(0.)),
        )
        .then(
            Delay::new(max_duration).then(
                Tween::new(
                    brand.ease_function,
                    brand.duration,
                    L::create(brand.tint.with_alpha(0.), brand.tint),
                )
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                .with_repeat_count(RepeatCount::Finite(2))
                .with_completed_event(curr_screen as u64),
            ),
        ),
    )
}
