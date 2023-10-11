// @generated automatically by Diesel CLI.

diesel::table! {
    branches (state) {
        state -> Varchar,
        distance -> Int4,
        done -> Bool,
    }
}

diesel::table! {
    branches_next (current, next) {
        current -> Varchar,
        next -> Varchar,
    }
}

diesel::table! {
    branches_prev (current, prev) {
        current -> Varchar,
        prev -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    branches,
    branches_next,
    branches_prev,
);
