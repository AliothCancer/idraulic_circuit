#[derive(Debug)]
pub struct Component {
    // general param
    component_type: ComponentType,
    resistence: f32,
    volume: f32,
    flow: f32,

    // specific for pump
    pump_is_on: Option<bool>,

    // specific for tube
    lenght: Option<f32>,
    radius: Option<f32>,
    section: Option<f32>,
}

#[derive(Debug)]
pub enum ComponentType {
    Tank,
    Tube,
    Pump,
}

impl Component {
    pub fn new(
        component_type: ComponentType,
        volume: f32,
        flow: f32,
        pump_is_on: Option<bool>,
        lenght: Option<f32>,
        radius: Option<f32>,
        section: Option<f32>,
    ) -> Self {
        Component {
            volume,
            resistence: 0.0,
            flow,
            pump_is_on,
            component_type,
            lenght,
            radius,
            section,
        }
    }
}
