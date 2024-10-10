// @generated automatically by Diesel CLI.

diesel::table! {
    child (id) {
        id -> BigInt,
        parent_id -> BigInt,
    }
}

diesel::table! {
    parent (id) {
        id -> BigInt,
    }
}

diesel::joinable!(child -> parent (parent_id));

diesel::allow_tables_to_appear_in_same_query!(
    child,
    parent,
);
