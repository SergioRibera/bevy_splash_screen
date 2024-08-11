use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::Targetable;

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
    fn lerp(&mut self, target: &mut dyn Targetable<Text>, ratio: f32) {
        target
            .sections
            .iter_mut()
            .enumerate()
            .for_each(|(i, section)| {
                use crate::ColorLerper as _;
                let value = self.0[i].with_alpha(0.).lerp(&self.0[i], ratio);
                section.style.color = value;
            });
    }
}

impl InstanceLens for SplashImageColorLens {
    fn create(start: Color, end: Color) -> Self {
        Self { start, end }
    }
}

impl Lens<UiImage> for SplashImageColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<UiImage>, ratio: f32) {
        use crate::ColorLerper as _;
        let value = self.start.lerp(&self.end, ratio);
        target.color = value;
    }
}
