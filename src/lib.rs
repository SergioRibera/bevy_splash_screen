use std::time::Duration;

use bevy::prelude::*;
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

#[derive(Clone, Default)]
pub struct SplashScreen {
    pub brands: Vec<SplashItem>,
    pub splash_type: SplashType,
    pub wait_to_start: WaitScreenType,
    pub background_color: BackgroundColor,
}

pub struct SplashScreenSkipEvent;

#[derive(Default, Clone, Resource)]
pub(crate) struct SplashScreenSkipable(bool, bool);

#[derive(Default, Clone, Resource)]
pub(crate) struct SplashScreens(Vec<SplashScreen>);

pub struct SplashPlugin<S> {
    state: S,
    next: S,
    skipable: bool,
    ignore_default_events: bool,
    screens: SplashScreens,
}

impl<S> SplashPlugin<S>
where
    S: States,
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
    S: States,
{
    fn build(&self, app: &mut App) {
        if self.screens.0.is_empty() {
            return;
        }

        app.add_plugin(TweeningPlugin)
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
            .add_startup_system(create_splash)
            .add_systems((
                component_animator_system::<BackgroundColor>.run_if(in_state(self.state.clone())),
                update_splash::<S>.run_if(in_state(self.state.clone())),
            ))
            .add_system(splash_skip::<S>);
    }
}
