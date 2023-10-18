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
        fn f(mask: u8, entities: Vec<&Entity<u8, u8>>) {
            println!("mask: {}", mask);
            println!("entities: {}", entities.len());
            for entity in entities {
                println!("Entity {}", entity.id);
                for component in &entity.components {
                    if let Some(name) = (*component).borrow().downcast_ref::<Name>() {
                        println!("Your name is {:?}", name.0);
                    }
                    if let Some(age) = (*component).borrow().downcast_ref::<Age>() {
                        println!("Your age is {:?}", age.0);
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
                RefCell::new(Box::new(Age(50))),
            ])
            .spawn(vec![RefCell::new(Box::new(Age(32)))]);
        // The mask needs to be easier to use, if I want to query for entities with Name and Age
        // I would naturally just use the types to build the query.
        let mask = Query::<Name, Age>::query();
        let entities = world.query(mask);
        assert_eq!(entities.len(), 2);
        world.add_system(mask, f);
        world.run();
    }
}
