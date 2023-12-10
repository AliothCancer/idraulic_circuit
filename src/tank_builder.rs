use crate::component::{Component, ComponentType};

pub struct TankBuilder {}

impl TankBuilder {
    pub fn build(volume: f32) -> Component {
        let component_type = ComponentType::Tank;
        Component::new(component_type, volume, 0.0, None, None, None, None)
    }
}
