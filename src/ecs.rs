pub mod transforms;
pub mod vitals;
pub mod constants;

use transforms::Transforms;
use vitals::VitalsComponent;

pub struct ComponentRegistry {
    pub transform: Transforms,
    pub vitals: VitalsComponent,
}

impl ComponentRegistry {
    pub fn new() -> ComponentRegistry {
        ComponentRegistry {
            transform: Transforms::new(),
            vitals: VitalsComponent::new()
        }
    }
}