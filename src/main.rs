pub mod ecs;
#[cfg(test)] pub mod tests;
use ecs::ComponentRegistry;

fn main() {
    let mut component_registry = Box::new(ComponentRegistry::new());
    let player_guid = component_registry.transform.addnew([0.0, 0.0], 0.0, [1.0, 1.0]);
    component_registry.vitals.attachnew(player_guid, 100, 100);
    component_registry.transform.update_transforms(0);
}
