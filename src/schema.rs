use diesel::table;

table! {
    user (user_id) {
        user_id -> Unsigned<Integer>,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Nullable<Varchar>,
        failures_num -> Nullable<TinyInt>,
        first_failed_at -> Nullable<Timestamp>,
        lock_expires_at -> Nullable<Timestamp>,
        enabled -> TinyInt,
        salt -> Nullable<Varchar>,
    }
}