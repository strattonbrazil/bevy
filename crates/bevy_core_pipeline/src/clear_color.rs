use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_render::{color::Color, extract_resource::ExtractResource};
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug, Default)]
#[reflect_value(Serialize, Deserialize)]
pub enum ClearColorConfig {
    #[default]
    Default,
    Custom(Color),
    None,
}

/// When used as a resource, sets the color that is used to clear the screen between frames.
///
/// This color appears as the "background" color for simple apps, when
/// there are portions of the screen with nothing rendered.
#[derive(Component, Clone, Debug, Deref, DerefMut, ExtractResource)]
pub struct ClearColor(pub Color);

impl Default for ClearColor {
    fn default() -> Self {
        Self(Color::rgb(0.4, 0.4, 0.4))
    }
}
