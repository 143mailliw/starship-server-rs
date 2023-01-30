mod planets;
mod users;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Mutation(users::UserMutation, planets::PlanetMutation);
