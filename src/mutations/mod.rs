mod components;
mod members;
mod planets;
mod users;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Mutation(
    users::UserMutation,
    planets::PlanetMutation,
    components::ComponentMutation,
    members::MemberMutation,
);
