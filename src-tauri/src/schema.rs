// @generated automatically by Diesel CLI.

diesel::table! {
    grant (id) {
        id -> Nullable<Integer>,
        name -> Text,
        funder -> Text,
        call_name -> Nullable<Text>,
        status -> Text,
        amount_requested -> Nullable<Integer>,
        amount_received -> Nullable<Integer>,
        currency -> Text,
        deadline -> Nullable<Text>,
        submitted_at -> Nullable<Text>,
        decision_at -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    grant_manuscript (grant_id, manuscript_id) {
        grant_id -> Integer,
        manuscript_id -> Integer,
    }
}

diesel::table! {
    manuscript (id) {
        id -> Nullable<Integer>,
        title -> Text,
        short_name -> Nullable<Text>,
        journal -> Nullable<Text>,
        status -> Text,
        next_action -> Nullable<Text>,
        submitted_at -> Nullable<Text>,
        decision_at -> Nullable<Text>,
        published_at -> Nullable<Text>,
        doi -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(grant_manuscript -> grant (grant_id));
diesel::joinable!(grant_manuscript -> manuscript (manuscript_id));

diesel::allow_tables_to_appear_in_same_query!(grant, grant_manuscript, manuscript,);
