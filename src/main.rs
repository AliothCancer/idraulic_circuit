pub mod circuit;
pub mod component;
pub mod pump_builder;
pub mod tank_builder;
pub mod tube_builder;

use circuit::CircuitBuilder;
use pump_builder::PumpBuilder;
use tank_builder::TankBuilder;
use tube_builder::TubeBuilder;

fn main() {
    let circuit = CircuitBuilder::new()
        .connect(PumpBuilder::build(1.0, 0.5))
        .connect(TubeBuilder::build(0.3, 10.0))
        .connect(TankBuilder::build(10.0))
        .build();
}
