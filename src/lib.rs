use std::time::Duration;

use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy_tweening::*;

mod lens;
mod splash;
mod systems;

pub use lens::*;
use splash::create_splash;
pub use systems::ClearSplash;
use systems::*;

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
    pub width: Val,
    pub height: Val,
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

#[derive(Clone, Default)]
pub struct SplashScreen {
    pub brands: Vec<SplashItem>,
    pub splash_type: SplashType,
    pub wait_to_start: WaitScreenType,
    pub background_color: BackgroundColor,
}

#[derive(Event)]
pub struct SplashScreenSkipEvent;

#[derive(Default, Clone, Resource)]
pub(crate) struct SplashScreenSkipable(bool, bool);

#[derive(Default, Clone, Resource)]
pub(crate) struct SplashScreens(Vec<SplashScreen>);

pub struct SplashPlugin<S: FreelyMutableState> {
    state: S,
    next: S,
    skipable: bool,
    ignore_default_events: bool,
    screens: SplashScreens,
}

impl<S> SplashPlugin<S>
where
    S: FreelyMutableState,
{
    pub fn new(splash_state: S, next_state: S) -> Self {
        Self {
            skipable: false,
            ignore_default_events: false,
            state: splash_state,
            next: next_state,
            screens: SplashScreens::default(),
        }
    }

    pub fn skipable(mut self) -> Self {
        self.skipable = true;
        self
    }

    pub fn ignore_default_events(mut self) -> Self {
        self.ignore_default_events = true;
        self
    }

    pub fn add_screen(mut self, screen: SplashScreen) -> Self {
        self.screens.0.push(screen);
        self
    }
}

impl<S> Plugin for SplashPlugin<S>
where
    S: FreelyMutableState,
{
    fn build(&self, app: &mut App) {
        if self.screens.0.is_empty() {
            return;
        }

        app.add_plugins(TweeningPlugin)
            .add_event::<SplashScreenSkipEvent>()
            .insert_resource(self.screens.clone())
            .insert_resource(SplashScreenSkipable(
                self.skipable,
                self.ignore_default_events,
            ))
            .insert_resource(MaxScreens(
                self.screens.0.len() as u64 - 1,
                self.next.clone(),
                (self.screens.0.iter().map(|s| s.brands.len()).sum::<usize>() * 2) as u64,
            ))
            .add_systems(Startup, create_splash)
            .add_systems(
                Update,
                (
                    component_animator_system::<UiImage>.run_if(in_state(self.state.clone())),
                    update_splash::<S>.run_if(in_state(self.state.clone())),
                    splash_skip::<S>,
                ),
            );
    }
}

/// Trait to interpolate between two values.
/// Needed for color.
#[allow(dead_code)]
trait ColorLerper {
    fn lerp(&self, target: &Self, ratio: f32) -> Self;
}

#[allow(dead_code)]
impl ColorLerper for Color {
    fn lerp(&self, target: &Color, ratio: f32) -> Color {
        let mut linear = self.to_linear();
        let linear_target = target.to_linear();
        linear.red = linear.red.lerp(linear_target.red, ratio);
        linear.green = linear.green.lerp(linear_target.green, ratio);
        linear.blue = linear.blue.lerp(linear_target.blue, ratio);
        linear.alpha = linear.alpha.lerp(linear_target.alpha, ratio);
        linear.into()
    }
}
