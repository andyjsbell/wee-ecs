#[cfg(test)]
mod tests {
    use crate::prelude::*;

    struct Name(String);
    struct Age(u8);

    #[test]
    fn create_simple_world() {
        fn f(mask: u8, entities: Vec<&Entity<u8, u8>>) {
            println!("mask: {}", mask);
            println!("entities: {}", entities.len());
            if let Some(age) = entities[0].components[0].downcast_ref::<Age>() {
                println!("Your age is {:?}", (*age).0);
            }
        }
        let world = PrimitiveWorld::new("simple world");
        PrimitiveWorld::register::<Name>();
        PrimitiveWorld::register::<Age>();
        let mut world = world
            .spawn(vec![Box::new(Name("Andy".to_owned())), Box::new(Age(50))])
            .spawn(vec![Box::new(Age(32))]);
        // The mask needs to be easier to use, if I want to query for entities with Name and Age
        // I would naturally just use the types to build the query.
        let mask = PrimitiveWorld::mask::<Name>() + PrimitiveWorld::mask::<Age>();
        let entities = world.query(mask);
        assert_eq!(entities.len(), 2);
        let entity = entities[0];
        assert_eq!(entity.components.len(), 1);
        world.add_system(mask, f);
        world.run();
    }
}
