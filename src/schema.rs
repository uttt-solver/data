// @generated automatically by Diesel CLI.

diesel::table! {
    branches (state) {
        #[max_length = 30]
        state -> Bpchar,
        distance -> Int4,
        done -> Bool,
    }
}

diesel::table! {
    branches_next (current, next) {
        #[max_length = 30]
        current -> Bpchar,
        #[max_length = 30]
        next -> Bpchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    branches,
    branches_next,
);
