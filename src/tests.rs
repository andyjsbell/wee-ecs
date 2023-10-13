#[cfg(test)]
mod tests {
    use crate::prelude::*;

    struct Name(String);
    const NAME_ID: u8 = 0;
    impl Component for Name {
        fn id(&self) -> u8 {
            NAME_ID
        }
    }

    #[test]
    fn create_simple_world() {
        fn f(mask: u8, entities: Vec<&Entity<u8, u8>>) {
            println!("mask: {}", mask);
            println!("entities: {}", entities.len());
        }
        let world = PrimitiveWorld::new("simple world");
        let mut world = world.spawn(vec![Box::new(Name("Andy".to_owned()))]);
        let entities = world.query(BitSet::mask_for(NAME_ID));
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        assert_eq!(entity.components.len(), 1);
        assert_eq!(entity.components[0].id(), NAME_ID);
        world.add_system(BitSet::mask_for(NAME_ID), f);
        world.run();
        // let world = world.spawn(vec![Box::new(Name("Andy".to_owned()))]);
        // world.despawn(entity);
        // let entities = world.query(u8::MAX);
        // assert_eq!(entities.len(), 0);
    }
}
