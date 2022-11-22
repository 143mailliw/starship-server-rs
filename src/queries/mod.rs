use async_graphql::MergedObject;

mod sysinfo;
mod users;

#[derive(MergedObject, Default)]
pub struct Query(users::UserQuery, sysinfo::SysInfoQuery);
