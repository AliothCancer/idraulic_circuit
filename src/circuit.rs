use crate::component::Component;

#[derive(Debug)]
pub struct Circuit {
    components: Vec<Component>,
    time: f64,
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            components: vec![],
            time: 0.0,
        }
    }
    pub fn list_components(&self) {
        println!("{:?}", self.components);
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Component> {
        self.components.iter()
    }
    pub fn play_simulation(self) {}
}

pub struct CircuitBuilder {
    circuit: Circuit,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        CircuitBuilder {
            circuit: Circuit::new(),
        }
    }

    pub fn connect(mut self, component: Component) -> Self {
        self.circuit.components.push(component);
        self
    }

    pub fn build(self) -> Circuit {
        self.circuit
    }
}
