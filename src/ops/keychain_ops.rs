use diesel::prelude::*;
use crate::models::{NewKeychain, Keychain, UpdateKeychain};
use crate::schema::keychains::dsl::*;
use crate::db::{open};

pub fn create_keychain(kc: NewKeychain) {
    let conn = &mut open();
    diesel::insert_into(keychains)
        .values(&kc)
        .execute(conn)
        .expect("failed to insert keychain");
}

pub fn update_keychain(kc: UpdateKeychain) {
    let conn = &mut open();
    diesel::update(keychains.find(kc.id))
        .set(&kc)
        .execute(conn)
        .expect("failed to update keychain");
}

pub fn list_keychains() {
    let conn = &mut open();
    let results = keychains.load::<Keychain>(conn);
    println!("{:?}", results);
}