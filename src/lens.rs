use bevy::prelude::*;
use bevy_tweening::lens::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SplashImageColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SplashTextColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

impl Lens<Text> for SplashTextColorLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target
            .sections
            .iter_mut()
            .for_each(|section| section.style.color = value.into());
    }
}

impl Lens<BackgroundColor> for SplashImageColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.0 = value.into();
    }
}
