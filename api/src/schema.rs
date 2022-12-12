// @generated automatically by Diesel CLI.

diesel::table! {
    credential_refresh (id) {
        id -> Int4,
        credential_id -> Int4,
        token -> Text,
        user_agent -> Text,
        created_at -> Timestamp,
        used_at -> Timestamp,
    }
}

diesel::table! {
    credentials (id) {
        id -> Int4,
        user_name -> Text,
        pass -> Nullable<Text>,
        recovery_key -> Nullable<Text>,
        recovery_expires -> Nullable<Timestamp>,
    }
}

diesel::table! {
    devices (id) {
        id -> Int4,
        adr -> Int4,
        pairs_of -> Int4,
        endpoint_count -> Int4,
    }
}

diesel::table! {
    points (id) {
        id -> Int4,
        device_id -> Int4,
        val -> Int4,
        width -> Float4,
        height -> Float4,
        x -> Float4,
        y -> Float4,
        rotation -> Float4,
        watts -> Float4,
        active -> Bool,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    preset_items (id) {
        id -> Int4,
        preset_id -> Int4,
        point_id -> Int4,
        val -> Int4,
    }
}

diesel::table! {
    presets (id) {
        id -> Int4,
        user_id -> Int4,
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
