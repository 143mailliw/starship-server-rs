mod members;
mod planets;
mod roles;
mod sysinfo;
mod users;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(
    users::UserQuery,
    sysinfo::SysInfoQuery,
    planets::PlanetQuery,
    members::MemberQuery,
    roles::RoleQuery,
);
