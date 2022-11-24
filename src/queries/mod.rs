mod sysinfo;
mod users;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(users::UserQuery, sysinfo::SysInfoQuery);
