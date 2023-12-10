use std::f32::consts::PI;

use crate::component::{Component, ComponentType};

pub struct TubeBuilder {}

impl TubeBuilder {
    pub fn build(radius: f32, lenght: f32) -> Component {
        let component_type = ComponentType::Tube;

        let section = PI * radius.powi(2);
        let volume = section * lenght;
        Component::new(
            component_type,
            volume,
            0.0,
            None,
            Some(lenght),
            Some(radius),
            Some(section),
        )
    }
}
