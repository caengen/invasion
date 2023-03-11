use bevy::prelude::*;
use rand::{prelude::SmallRng, SeedableRng};

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Random(SmallRng);

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<Random>(Random(SmallRng::from_entropy()));
    }
}

impl FromWorld for Random {
    fn from_world(world: &mut World) -> Self {
        let rng = world
            .get_resource_mut::<Random>()
            .expect("Random resource not found");
        Random(SmallRng::from_rng(rng.clone()).unwrap())
    }
}
