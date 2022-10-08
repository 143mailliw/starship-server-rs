// @generated automatically by Diesel CLI.

diesel::table! {
    components (id) {
        id -> Int4,
        name -> Varchar,
        component_id -> Text,
        planet -> Text,
    }
}

diesel::table! {
    planets (id) {
        id -> Text,
        created -> Nullable<Timestamp>,
        name -> Text,
        owner -> Text,
        private -> Bool,
        follower_count -> Int4,
        components -> Array<Nullable<Text>>,
        featured -> Bool,
        verified -> Bool,
        partnered -> Bool,
        featured_description -> Text,
        banned -> Array<Nullable<Text>>,
        roles -> Array<Nullable<Text>>,
        css -> Text,
        description -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        created -> Nullable<Timestamp>,
        username -> Varchar,
        password -> Text,
        reset_token -> Nullable<Text>,
        reset_expiry -> Nullable<Timestamp>,
        email_address -> Text,
        verified -> Nullable<Bool>,
        verification_token -> Nullable<Text>,
        following -> Array<Nullable<Text>>,
        blocked -> Array<Nullable<Text>>,
        sessions -> Array<Nullable<Uuid>>,
        banned -> Bool,
        admin -> Bool,
        notification_setting -> Int4,
        cap_waived -> Bool,
        bytes_used -> Int8,
        profile_picture -> Nullable<Text>,
        profile_banner -> Nullable<Text>,
        profile_bio -> Nullable<Varchar>,
        tfa_secret -> Nullable<Text>,
        tfa_enabled -> Bool,
        tfa_backup -> Array<Nullable<Text>>,
        token_geofenced -> Bool,
        token_expires -> Bool,
        token_ip_locked -> Bool,
    }
}

diesel::joinable!(components -> planets (planet));
diesel::joinable!(planets -> users (owner));

diesel::allow_tables_to_appear_in_same_query!(
    components,
    planets,
    users,
);
