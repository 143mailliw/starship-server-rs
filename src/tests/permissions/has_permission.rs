use crate::entities::planet;
use crate::entities::planet_member;
use crate::entities::planet_role;
use crate::permissions::checks::has_permission;

#[cfg(test)]
#[actix_web::test]
async fn public_permissions() {
    let planet = create_planet(false);

    let check = has_permission("planet.view", &planet, None, None);

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn private_permissions() {
    let planet = create_planet(true);

    let check = has_permission("planet.view", &planet, None, None);

    assert!(!check, "planet.view allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn member_with_permission() {
    let planet = create_planet(true);
    let member = create_member(vec!["+planet.view".to_string()]);

    let check = has_permission("planet.view", &planet, Some(member), None);

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn member_without_permission() {
    let planet = create_planet(false);
    let member = create_member(vec!["-planet.view".to_string()]);

    let check = has_permission("planet.view", &planet, Some(member), None);

    assert!(!check, "planet.view allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn role_with_permission() {
    let planet = create_planet(true);
    let member = create_member(vec![]);
    let role = create_role(vec!["+planet.view".to_string()], 0);

    let check = has_permission("planet.view", &planet, Some(member), Some(vec![role]));

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn role_without_permission() {
    let planet = create_planet(false);
    let member = create_member(vec![]);
    let role = create_role(vec!["-planet.view".to_string()], 0);

    let check = has_permission("planet.view", &planet, Some(member), Some(vec![role]));

    assert!(!check, "planet.view allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn member_supersedes_role() {
    let planet = create_planet(false);
    let member = create_member(vec!["+planet.view".to_string()]);
    let role = create_role(vec!["-planet.view".to_string()], 0);

    let check = has_permission("planet.view", &planet, Some(member), Some(vec![role]));

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn correct_role_order() {
    let planet = create_planet(false);
    let member = create_member(vec![]);
    let role1 = create_role(vec!["-planet.view".to_string()], 0);
    let role2 = create_role(vec!["+planet.view".to_string()], 1);

    let check = has_permission(
        "planet.view",
        &planet,
        Some(member),
        Some(vec![role2, role1]),
    );

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn admin_overrides() {
    let planet = create_planet(false);
    let member = create_member(vec![]);
    let role1 = create_role(vec!["+administrator".to_string()], 0);
    let role2 = create_role(vec!["-planet.view".to_string()], 1);

    let check = has_permission(
        "planet.view",
        &planet,
        Some(member),
        Some(vec![role1, role2]),
    );

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn owner_overrides() {
    let planet = create_planet(false);
    let member = create_member(vec!["+owner".to_string()]);
    let role = create_role(vec!["-planet.view".to_string()], 0);

    let check = has_permission("planet.view", &planet, Some(member), Some(vec![role]));

    assert!(check, "planet.view not allowed");
}

#[cfg(test)]
#[actix_web::test]
async fn member_planet_locked() {
    let planet = create_planet(false);
    let mut member = create_member(vec![]);
    member.planet = "different".to_string();

    let check = has_permission("planet.view", &planet, Some(member), None);

    assert!(!check, "permission check did not detect incorrect planet");
}

#[cfg(test)]
#[actix_web::test]
async fn role_planet_locked() {
    let planet = create_planet(false);
    let member = create_member(vec![]);
    let mut role = create_role(vec![], 0);
    role.planet = "different".to_string();

    let check = has_permission("planet.view", &planet, Some(member), Some(vec![role]));

    assert!(!check, "permission check did not detect incorrect planet");
}

fn create_planet(private: bool) -> planet::Model {
    planet::Model {
        private,
        // all other fields are unimportant for this test
        id: "irrelevant".to_string(),
        name: "irrelevant".to_string(),
        owner: "irrelevant".to_string(),
        featured: false,
        member_count: 0,
        verified: false,
        partnered: false,
        featured_description: "irrelevant".to_string(),
        created: chrono::offset::Utc::now().naive_utc(),
        banned: vec!["irrelevant".to_string()],
        css: "irrelevant".to_string(),
        description: Some("irrelevant".to_string()),
        home: Some("irrelevant".to_string()),
    }
}

fn create_member(permissions: Vec<String>) -> planet_member::Model {
    planet_member::Model {
        permissions,
        // all other fields are unimportant for this test
        id: "irrelevant".to_string(),
        planet: "irrelevant".to_string(),
        user: "irrelevant".to_string(),
        roles: vec!["irrelevant".to_string()],
        created: chrono::offset::Utc::now().naive_utc(),
    }
}

fn create_role(permissions: Vec<String>, position: i32) -> planet_role::Model {
    planet_role::Model {
        position,
        permissions,
        // all other fields are unimportant for this test
        id: "irrelevant".to_string(),
        planet: "irrelevant".to_string(),
        default: false,
        name: "irrelevant".to_string(),
        color: "#FFFFFF".to_string(),
    }
}
