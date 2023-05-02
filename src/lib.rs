use std::time::Duration;

use bevy::{
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
};
pub use bevy_tweening::{lens::*, *};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod lens;

pub use lens::*;

#[derive(Clone, Component)]
pub enum SplashAssetType {
    /// Content and Font
    SingleText(Text, String),
    SingleImage(String),
}

#[derive(Clone, Component, Default, PartialEq, Eq)]
pub enum SplashType {
    #[default]
    List,
    Grid,
}

#[derive(Clone)]
pub struct SplashItem {
    pub asset: SplashAssetType,
    pub tint: Color,
    pub size: Size,
    pub ease_function: EaseMethod,
    pub duration: Duration,
    pub is_static: bool,
}

#[derive(Clone, Component, Default, PartialEq, Eq)]
pub enum WaitScreenType {
    #[default]
    AfterEnd,
    Specific(Duration),
}
// #[derive(Resource)]
// pub struct SplashResource {
//     screens: Vec<SplashScreen>,
// }
//
// pub struct SplashScreen {
//     pub brands: Vec<SplashItem>,
//     pub splash_type: SplashType,
// }

#[derive(Clone, Default)]
pub struct SplashScreen {
    pub brands: Vec<SplashItem>,
    pub splash_type: SplashType,
    pub wait_to_start: WaitScreenType,
    pub background_color: BackgroundColor,
}

#[derive(Default, Clone, Resource)]
struct SplashScreens(Vec<SplashScreen>);

pub struct SplashPlugin<S> {
    next: S,
    skipable: bool,
    screens: SplashScreens,
}

impl<S> SplashPlugin<S>
where
    S: States,
{
    pub fn new(next_state: S, skipable: bool) -> Self {
        Self {
            skipable,
            next: next_state,
            screens: SplashScreens::default(),
        }
    }

    pub fn add_screen(mut self, screen: SplashScreen) -> Self {
        self.screens.0.push(SplashScreen {
            brands: screen.brands.iter().map(|brand| SplashItem {
                ease_function: EaseMethod::Linear,
                ..brand.clone()
            }).collect(),
            ..screen
        });
        self
    }
}

impl<S> Plugin for SplashPlugin<S>
where
    S: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .insert_resource(self.screens.clone())
            .insert_resource(MaxScreens(
                self.screens.0.len() as u64 - 1,
                self.next.clone(),
            ))
            .add_plugin(TweeningPlugin)
            .add_startup_system(create_splash)
            .add_system(update_splash::<S>);

        if self.skipable {
            app.add_system(splash_skip::<S>);
        }
    }
}

#[derive(Component)]
struct ClearSplash;

#[derive(Resource)]
struct MaxScreens<S>(u64, S)
where
    S: States;

fn create_splash(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    screens: Res<SplashScreens>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (i_screen, screen) in screens.0.iter().enumerate() {
        let (flex_direction, flex_wrap) = match screen.splash_type {
            SplashType::List => (FlexDirection::Column, FlexWrap::NoWrap),
            SplashType::Grid => (FlexDirection::Row, FlexWrap::Wrap),
        };

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
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                overflow: Overflow::Hidden,
                ..default()
            },
            background_color: BackgroundColor(screen.background_color.0.with_a(0.)),
            ..default()
        })
        .insert(ClearSplash)
        .with_children(|cmd| {
            let max_duration = if i_screen == 0 {
                Duration::from_secs(1)
            } else if let Some(next_screen) = screens.0.get(i_screen) {
                match next_screen.wait_to_start {
                    WaitScreenType::AfterEnd => Duration::from_secs(
                        next_screen
                            .brands
                            .iter()
                            .map(|b| b.duration.as_secs())
                            .max()
                            .unwrap_or(1)
                            * 2
                            + 1,
                    ),
                    WaitScreenType::Specific(t) => t,
                }
            } else {
                Duration::from_secs(1)
            };

            for (i, brand) in screen.brands.iter().enumerate() {
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
                                    size: brand.size,
                                    ..default()
                                },
                                ..default()
                            },
                            Animator::new(
                                Tween::new(
                                    brand.ease_function,
                                    Duration::from_secs(1),
                                    SplashTextColorLens {
                                        start: brand.tint.with_a(0.),
                                        end: brand.tint.with_a(0.),
                                    },
                                )
                                .then(
                                    Delay::new(max_duration).then(
                                        Tween::new(
                                            brand.ease_function,
                                            brand.duration,
                                            SplashTextColorLens {
                                                start: brand.tint.with_a(0.),
                                                end: brand.tint,
                                            },
                                        )
                                        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                                        .with_repeat_count(RepeatCount::Finite(2))
                                        .with_completed_event(i as u64),
                                    ),
                                ),
                            ),
                        ))
                    }
                    SplashAssetType::SingleImage(handler) => {
                        let unique_material = materials.add(brand.tint.into());
                        cmd.spawn((
                            ImageBundle {
                                image: UiImage {
                                    texture: assets.load(handler),
                                    flip_x: false,
                                    flip_y: false,
                                },
                                style: Style {
                                    size: brand.size,
                                    ..default()
                                },
                                ..default()
                            },
                            unique_material.clone(),
                            AssetAnimator::new(
                                unique_material.clone(),
                                Tween::new(
                                    brand.ease_function,
                                    Duration::from_secs(1),
                                    ColorMaterialColorLens {
                                        start: brand.tint.with_a(0.),
                                        end: brand.tint.with_a(0.),
                                    },
                                )
                                .then(
                                    Delay::new(max_duration).then(
                                        Tween::new(
                                            brand.ease_function,
                                            brand.duration,
                                            ColorMaterialColorLens {
                                                start: brand.tint.with_a(0.),
                                                end: brand.tint,
                                            },
                                        )
                                        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                                        .with_repeat_count(RepeatCount::Finite(2))
                                        .with_completed_event(i_screen as u64),
                                    ),
                                ),
                            ),
                        ))
                    }
                };
            }
        });
    }
}

fn splash_end<'a, S: States>(
    mut cmd: Commands,
    next_state: S,
    brands: impl Iterator<Item = (Entity, &'a Node, &'a ClearSplash)>,
) {
    for (entity, _, _) in brands {
        cmd.entity(entity).despawn_recursive();
    }
    cmd.insert_resource(NextState(Some(next_state)));
    println!("Splash End");
}

fn update_splash<S: States>(
    cmd: Commands,
    brands: Query<(Entity, &Node, &ClearSplash)>,
    mut reader: EventReader<TweenCompleted>,
    max_screens: Res<MaxScreens<S>>,
    mut all_screens_end: Local<bool>,
) {
    let mut clear = false;
    for e in reader.iter() {
        if e.user_data == max_screens.0 && *all_screens_end {
            clear = true;
        } else if e.user_data == max_screens.0 && !*all_screens_end {
            *all_screens_end = true;
        }
        println!(
            "{} is ended == {} && {}",
            e.user_data, max_screens.0, *all_screens_end
        );
    }
    if clear {
        splash_end(cmd, max_screens.1.clone(), brands.iter());
    }
}

fn splash_skip<S: States>(
    cmd: Commands,
    mut kbd: EventReader<KeyboardInput>,
    mut mouse: EventReader<MouseButtonInput>,
    mut gamepad: EventReader<GamepadEvent>,
    mut touch: EventReader<TouchInput>,
    brands: Query<(Entity, &Node, &ClearSplash)>,
    max_screens: Res<MaxScreens<S>>,
) {
    if brands.is_empty() {
        return;
    }

    use bevy::input::{touch::TouchPhase, ButtonState};

    let mut done = false;

    for ev in kbd.iter() {
        if let ButtonState::Pressed = ev.state {
            done = true;
        }
    }

    for ev in mouse.iter() {
        if let ButtonState::Pressed = ev.state {
            done = true;
        }
    }

    for ev in gamepad.iter() {
        if let GamepadEvent::Button(_) = ev {
            done = true;
        }
    }

    for ev in touch.iter() {
        if let TouchPhase::Started = ev.phase {
            done = true;
        }
    }

    if done {
        splash_end(cmd, max_screens.1.clone(), brands.iter());
        println!("Splash End");
    }
}
