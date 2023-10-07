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
    SingleImage(Handle<Image>),
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
pub struct SplashScreens(Vec<SplashScreen>);

impl SplashScreens {
    pub fn add_screen(&mut self, screen: SplashScreen) {
        self.0.push(screen);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn screens(&self) -> &[SplashScreen] {
        self.0.as_ref()
    }
}

pub struct SplashPlugin<S> {
    state: S,
    next: S,
    skipable: bool,
    ignore_default_events: bool,
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
}

impl<S> Plugin for SplashPlugin<S>
where
    S: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_event::<SplashScreenSkipEvent>()
            .insert_resource(SplashScreenSkipable(
                self.skipable,
                self.ignore_default_events,
            ))
            .insert_resource(NextStateRes(self.next.clone()))
            .add_systems(
                OnEnter(self.state.clone()),
                (set_max_screens::<S>, create_splash),
            )
            .add_systems(
                Update,
                (
                    component_animator_system::<BackgroundColor>,
                    update_splash::<S>,
                    splash_skip::<S>,
                )
                    .run_if(in_state(self.state.clone())),
            );
    }
}
