// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Integer,
        #[max_length = 64]
        name -> Char,
    }
}
