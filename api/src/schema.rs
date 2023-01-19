// @generated automatically by Diesel CLI.

diesel::table! {
    credential_refresh (id) {
        id -> Integer,
        credential_id -> Integer,
        token -> Text,
        user_agent -> Text,
        created_at -> Timestamp,
        used_at -> Timestamp,
    }
}

diesel::table! {
    credentials (id) {
        id -> Integer,
        user_name -> Text,
        pass -> Nullable<Text>,
        recovery_key -> Nullable<Text>,
        recovery_expires -> Nullable<Timestamp>,
    }
}

diesel::table! {
    devices (id) {
        id -> Integer,
        adr -> Integer,
        endpoint_count -> Integer,
    }
}

diesel::table! {
    points (id) {
        id -> Integer,
        device_id -> Integer,
        device_position -> Integer,
        val -> Integer,
        width -> Float,
        height -> Float,
        x -> Float,
        y -> Float,
        rotation -> Float,
        watts -> Float,
        active -> Bool,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    preset_items (id) {
        id -> Integer,
        preset_id -> Integer,
        point_id -> Integer,
        val -> Integer,
    }
}

diesel::table! {
    presets (id) {
        id -> Integer,
        user_id -> Integer,
        preset_name -> Text,
        favorite -> Bool,
        active -> Bool,
        icon -> Nullable<Text>,
    }
}

diesel::joinable!(credential_refresh -> credentials (credential_id));
diesel::joinable!(points -> devices (device_id));
diesel::joinable!(preset_items -> points (point_id));
diesel::joinable!(preset_items -> presets (preset_id));
diesel::joinable!(presets -> credentials (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    credential_refresh,
    credentials,
    devices,
    points,
    preset_items,
    presets,
);
