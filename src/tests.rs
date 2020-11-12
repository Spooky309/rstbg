use crate::ecs::ComponentRegistry;

fn create_registry() -> Box<ComponentRegistry> {
    Box::new(ComponentRegistry::new())
}

#[test]
fn health_test() {
    let mut component_registry = create_registry();
    let test_guid = component_registry.transform.addnew([0.0, 0.0], 0.0, [1.0, 1.0]);
    component_registry.vitals.attachnew(test_guid, 200, 100);
    assert_eq!(component_registry.vitals.get_health(test_guid), 100);
    component_registry.vitals.modify_health(test_guid, 10);
    assert_eq!(component_registry.vitals.get_health(test_guid), 110);
    component_registry.vitals.modify_health(test_guid, -20);
    assert_eq!(component_registry.vitals.get_health(test_guid), 90);
    component_registry.vitals.modify_health(test_guid, 1000); // max is 200 so adding 1000 should yield 200
    assert_eq!(component_registry.vitals.get_health(test_guid), 200);
    
}