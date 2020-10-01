table! {
    groups (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    migration (version) {
        version -> Varchar,
        apply_time -> Nullable<Int4>,
    }
}

table! {
    teachers (id) {
        id -> Int4,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    groups,
    migration,
    teachers,
);
