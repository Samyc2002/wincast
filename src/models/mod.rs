use native_db::*;
use once_cell::sync::Lazy;

pub mod data {
    use native_db::{native_db, ToKey};
    use native_model::{native_model, Model};
    use serde::{Deserialize, Serialize};

    pub type Data = v1::Data;

    pub mod v1 {
        use super::*;

        #[derive(Serialize, Deserialize, Debug)]
        #[native_model(id=1, version=1)]
        #[native_db]
        pub struct Data {
            #[primary_key]
            // pub id: i32,
            pub name: String,
            pub path: String,
            pub icon: String,
            pub search_type: String,
        }
    }
}

pub static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<data::v1::Data>().unwrap();
    return models;
});
