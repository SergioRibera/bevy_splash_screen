use bevy::prelude::*;
use bevy_tweening::lens::*;

pub trait InstanceLens {
    fn create(start: Color, end: Color) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SplashImageColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

#[derive(Debug, Clone, PartialEq)]
/// Lens for interpolating Bevy Text sections. The single parameter is a reference to the color of each section.
pub struct SplashTextColorLens(Vec<Color>);

impl SplashTextColorLens {
    /// Create instance of Text Lens
    ///
    /// * `colors`: Each color refers to a section and is placed in order.
    pub fn new(colors: Vec<Color>) -> Self {
        Self(colors)
    }
}

impl Lens<Text> for SplashTextColorLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        target.sections.iter_mut().enumerate().for_each(|(i, section)| {
            let start: Vec4 = self.0[i].with_a(0.).into();
            let end: Vec4 = self.0[i].into();
            let value = start.lerp(end, ratio);
            section.style.color = value.into();
        });
    }
}

impl InstanceLens for SplashImageColorLens {
    fn create(start: Color, end: Color) -> Self {
        Self { start, end }
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
