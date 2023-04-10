use super::constants;
use crate::entities::planet;
use crate::entities::planet_member;
use crate::entities::planet_role;
use std::collections::HashMap;

pub fn has_permission(
    permission: &str,
    planet: &planet::Model,
    member: Option<planet_member::Model>,
    roles: Option<Vec<planet_role::Model>>,
) -> bool {
    let mut calculated_permissions: HashMap<String, bool> = HashMap::new();
    let mut administrator = false;
    let mut owner = false;

    if !planet.private {
        for permission in constants::VIEWER_PERMISSIONS
            .iter()
            .map(|a| (*a).trim_start_matches('+').to_string())
        {
            calculated_permissions.insert(permission, true);
        }
    };

    if planet.private && member.is_none() {
        return false;
    }

    if let Some(member) = member {
        if !member.banned {
            if let Some(roles) = roles {
                let mut role_vec = roles;
                role_vec.sort_by_key(|r| r.position);

                for role in &role_vec {
                    if role.planet != planet.id {
                        return false;
                    }

                    for permission in &role.permissions {
                        if permission == "+administrator" {
                            administrator = true;
                        }

                        let mut permission_chars = permission.chars();
                        permission_chars.next();

                        calculated_permissions
                            .insert(permission_chars.collect(), permission.starts_with('+'));
                    }
                }
            }

            if member.planet != planet.id {
                return false;
            }

            for permission in member.permissions {
                if permission == "+administrator" {
                    administrator = true;
                }

                if permission == "+owner" {
                    owner = true;
                }

                let mut permission_chars = permission.chars();
                permission_chars.next();

                calculated_permissions
                    .insert(permission_chars.collect(), permission.starts_with('+'));
            }
        }
    }

    if administrator || owner {
        for permission in constants::ADMINISTRATOR_PERMISSIONS
            .iter()
            .chain(constants::VIEWER_PERMISSIONS.iter())
            .chain(constants::MEMBER_PERMISSIONS.iter())
            .map(|a| (*a).trim_start_matches('+').to_string())
        {
            calculated_permissions.insert(permission, true);
        }
    }

    if owner {
        for permission in constants::OWNER_PERMISSIONS
            .iter()
            .map(|a| (*a).trim_start_matches('+').to_string())
        {
            calculated_permissions.insert(permission, true);
        }
    }

    calculated_permissions
        .iter()
        .any(|p| *p.0 == permission && *p.1)
}

pub fn high_enough(
    user_roles: Option<Vec<planet_role::Model>>,
    mut origin_roles: Vec<planet_role::Model>,
    member: Option<planet_member::Model>,
) -> bool {
    if member.is_some() && member.unwrap().permissions.contains(&"+owner".to_string()) {
        return true;
    }

    let user_lowest = user_roles
        .and_then(|mut v| {
            v.sort_by_key(|role| role.position);
            v.last().map(|role| role.position)
        })
        .unwrap_or(i32::MAX);

    origin_roles.sort_by_key(|role| role.position);

    let origin_lowest = origin_roles.last().map_or(0, |role| role.position);

    user_lowest >= origin_lowest
}
