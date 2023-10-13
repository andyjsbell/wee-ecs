#[cfg(test)]
mod tests {
    use crate::prelude::*;

    struct Name(String);
    struct Age(u8);

    // Box<dyn Any> = Age(50)::new()
    #[test]
    fn create_simple_world() {
        fn f(mask: u8, entities: Vec<&Entity<u8, u8>>) {
            println!("mask: {}", mask);
            println!("entities: {}", entities.len());
        }

        let world = PrimitiveWorld::new("simple world");
        PrimitiveWorld::register::<Name>();
        PrimitiveWorld::register::<Age>();
        let mut world = world.spawn(vec![Box::new(Name("Andy".to_owned())), Box::new(Age(50))]);
        let entities = world.query(3);
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        assert_eq!(entity.components.len(), 2);
        world.add_system(3, f);
        world.run();
    }
}
