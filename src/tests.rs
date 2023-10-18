#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Name(String);
    #[derive(Debug)]
    struct Age(u8);

    #[test]
    fn create_simple_world() {
        const DOLLY: &str = "Dolly";
        const AGE: u8 = 32;
        fn f(_mask: u8, entities: Vec<&Entity<u8, u8>>) {
            for entity in entities {
                for component in &entity.components {
                    if let Some(name) = (*component).borrow().downcast_ref::<Name>() {
                        assert_eq!(name.0, DOLLY);
                    }
                    if let Some(age) = (*component).borrow().downcast_ref::<Age>() {
                        assert_eq!(age.0, AGE);
                    }
                }
            }
        }
        let world = PrimitiveWorld::new("simple world");
        PrimitiveWorld::register::<Name>();
        PrimitiveWorld::register::<Age>();
        let mut world = world
            .spawn(vec![
                RefCell::new(Box::new(Name("Andy".to_owned()))),
                RefCell::new(Box::new(Age(AGE))),
            ])
            .spawn(vec![RefCell::new(Box::new(Age(AGE)))]);

        // Run query for entities with just a 'Name' component and update the name to "Dolly"
        // This would be asserted on in the system callback
        let mask = Query::<Name>::query();
        {
            let entities = world.query(mask);
            assert_eq!(entities.len(), 1);
            let mut component = entities[0].components[0].borrow_mut();
            let component = (*component).downcast_mut::<Name>().expect("Name component");
            assert_eq!(component.0, "Andy");
            component.0 = DOLLY.to_owned();
        }
        world.add_system(mask, f);
        world.run();
    }
}
