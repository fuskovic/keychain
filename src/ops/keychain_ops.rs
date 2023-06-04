use models::{NewKeychain, Keychain};
use db::{open};

pub fn create_keychain(kc: NewKeychain) {
    let conn = open();
    diesel.insert_into(keychains)
        .values(&kc)
        .execute(conn)
        .expect("failed to insert keychain")
}

pub fn update_keychain(kc: Keychain) {
    let conn = open();
    diesel.update(keychains.find(kc.id))
        .set(&kc)
        .execute(conn)
        .expect("failed to update keychain")
}

pub fn list_keychains() {
    let conn = open();
    let results = keychains.load::<Keychain>(&conn);
    for kc in results {
        println!("{:?}", kc)
    }
}