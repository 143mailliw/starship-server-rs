// we're keeping all GQL objects seperate from their direct database representations
// this is for two reasons:
// - the database data is not a direct representation of what we're sending
// - if needed, we can just regenerate the entity files

use chrono::NaiveDateTime;

pub struct User {
    id: String,
    username: String,
    admin: bool,
    profile_picture: Option<String>,
    profile_banner: Option<String>,
    profile_bio: Option<String>,
    banned: bool,
    following: Vec<String>,
    created_at: NaiveDateTime,
    used_bytes: f64,
    cap_waived: bool,
    tfa_enabled: bool,
    blocked_users: Vec<String>,
    online: bool,
    notification_setting: u8
}