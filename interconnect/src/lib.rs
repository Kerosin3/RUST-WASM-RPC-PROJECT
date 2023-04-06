pub mod serdes {
    extern crate serde;
    use self::serde::{Deserialize, Serialize};
    //simple struct to pass to wasm module and calc hash inside
    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct PersonSend {
        pub first_name: String,
    }
    // recv struct
    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct PersonHashedRecv {
        pub first_name: String,
        pub hash: u64,
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub enum Operation {
        One,
        Two,
        Three,
    }
    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct StructSend {
        pub payload: String,
        pub id: i32,
        pub oper: Operation,
    }
    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct StructRecv {
        pub payload_back: String,
    }
}
