use std::collections::HashMap;

use bevy::{
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
};
use bevy_tweening::TweenCompleted;

use crate::{SplashScreenSkipEvent, SplashScreenSkipable};
// Internal components for system logic
#[derive(Component)]
pub struct ClearSplash;

#[derive(Resource)]
pub(crate) struct MaxScreens<S>(pub(crate) u64, pub(crate) S, pub(crate) u64)
where
    S: States;

#[derive(Component)]
pub(crate) struct SplashBackground {
    pub(super) screens: Vec<u64>,
    pub(super) screen_colors: Vec<Color>,
}

//
// Remove all nodes when splash end
//
pub(crate) fn splash_end<'a, S: States>(
    mut cmd: Commands,
    next_state: S,
    brands: impl Iterator<Item = (Entity, &'a Node, &'a ClearSplash)>,
) {
    for (entity, _, _) in brands {
        cmd.entity(entity).despawn_recursive();
    }
    cmd.insert_resource(NextState(Some(next_state)));
}

//
// Logic to end splash and change background color
//
pub(crate) fn update_splash<S: States>(
    cmd: Commands,
    brands: Query<(Entity, &Node, &ClearSplash)>,
    mut background: Query<(&Node, &mut BackgroundColor, &SplashBackground)>,
    mut reader: EventReader<TweenCompleted>,
    max_screens: Res<MaxScreens<S>>,
    mut screens_end: Local<u64>,
    // screen => count of brand show
    mut brands_showed: Local<HashMap<u64, u64>>,
) {
    let mut clear = false;
    for e in reader.read() {
        *screens_end += 1;
        clear = e.user_data == max_screens.0 && *screens_end == max_screens.2;
        _ = brands_showed
            .entry(e.user_data)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);

        for (_, mut bg, data) in background.iter_mut() {
            if let Some(v) = brands_showed.get(&e.user_data) {
                if data.screens[e.user_data as usize] == *v
                    && e.user_data as usize + 1 < data.screen_colors.len()
                {
                    bg.0 = data.screen_colors[e.user_data as usize + 1];
                }
            }
        }
    }
    if clear {
        splash_end(cmd, max_screens.1.clone(), brands.iter());
    }
}

//
// System for skip splash
//
pub(crate) fn splash_skip<S: States>(
    cmd: Commands,
    mut kbd: EventReader<KeyboardInput>,
    mut mouse: EventReader<MouseButtonInput>,
    mut gamepad: EventReader<GamepadEvent>,
    mut touch: EventReader<TouchInput>,
    dev_skip: EventReader<SplashScreenSkipEvent>,
    brands: Query<(Entity, &Node, &ClearSplash)>,
    max_screens: Res<MaxScreens<S>>,
    skipable: Res<SplashScreenSkipable>,
) {
    if brands.is_empty() || !skipable.0 {
        return;
    }

    let mut done = false;

    if !skipable.1 {
        use bevy::input::{touch::TouchPhase, ButtonState};

        done = kbd.read().any(|ev| ev.state == ButtonState::Pressed)
            || mouse.read().any(|ev| ev.state == ButtonState::Pressed)
            || touch.read().any(|ev| ev.phase == TouchPhase::Started);

        for ev in gamepad.read() {
            if let GamepadEvent::Button(_) = ev {
                done = true;
            }
        }
    }

    if done || !dev_skip.is_empty() {
        splash_end(cmd, max_screens.1.clone(), brands.iter());
    }
}
