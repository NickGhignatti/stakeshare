use rand::{rngs::StdRng, RngCore, SeedableRng};
use uuid::{builder::Builder, Variant, Version};

pub fn uuidv4() -> String {
    // We're using uuid crate with disabled v4 feature, because of the wasm issue
    // (rand requires stdweb / wasm-bindgen feature enabled to work on the wasm32 arch).
    // We have added rand as our direct dependency with wasm-bindgen feature enabled
    // and following is a copy & paste of the Uuid::new_v4() method to make it working.
    let mut rng = StdRng::seed_from_u64(ic_cdk::api::time());
    let mut bytes = [0; 16];

    rng.fill_bytes(&mut bytes);

    let uuid = Builder::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("could not retrieve random bytes for uuid: {}", err))
        .set_version(Version::Random)
        .set_variant(Variant::RFC4122)
        .build();

    uuid.to_hyphenated().to_string()
}
