use std::collections::HashMap;

use bevy::{
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
};
use bevy_tweening::TweenCompleted;
// Internal components for system logic
#[derive(Component)]
pub(crate) struct ClearSplash;

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
    for e in reader.iter() {
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
