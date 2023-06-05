use diesel::prelude::*;
use crate::models::{NewKeychain, Keychain};
use crate::db::{open};

pub fn create_keychain(kc: NewKeychain) {
    use crate::schema::keychains::dsl::*;
    let conn = &mut open();
    diesel::insert_into(keychains)
        .values(&kc)
        .execute(conn)
        .expect("failed to insert keychain");
}

pub fn update_keychain(kc: Keychain) {
    use crate::schema::keychains::dsl::*;
    let conn = &mut open();
    diesel::update(keychains.find(kc.id))
        .set(&kc)
        .execute(conn)
        .expect("failed to update keychain");
}

pub fn list_keychains() {
    use crate::schema::keychains::dsl::*;
    let conn = &mut open();
    let results = keychains.load::<Keychain>(conn);
    for kc in results {
        println!("{:?}", kc);
    }
}