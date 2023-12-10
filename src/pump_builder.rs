use crate::component::{Component, ComponentType};

pub struct PumpBuilder {}

impl PumpBuilder {
    pub fn build(volume: f32, flow: f32) -> Component {
        let component_type = ComponentType::Pump;
        Component::new(component_type, volume, flow, Some(false), None, None, None)
    }
}
