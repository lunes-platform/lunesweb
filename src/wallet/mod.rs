use wasm_bindgen::prelude::wasm_bindgen;

/// Sign and validate signatures
pub mod signatures;
/// Generate private and public keys
pub mod wallet;

pub const ADDRESS_VERSION: [u8; 2] = [1, 11];
pub const ADDRESS_CHECKSUM_LENGTH: u8 = 4;
pub const ADDRESS_HASH_LENGTH: u8 = 20;
pub const ADDRESS_LENGTH: u8 = 1 + 1 + ADDRESS_CHECKSUM_LENGTH + ADDRESS_HASH_LENGTH;

#[wasm_bindgen]
struct Wallet {
    seed: Vec<u8>,
    nonce: u32,
    chain: u8,
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    address: Vec<u8>,
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(constructor)]
    pub fn new(
        seed: Option<String>,
        nonce: Option<u32>,
        chain: Option<u8>,
        private_key: Option<String>,
        public_key: Option<String>,
        address: Option<String>,
    ) -> Wallet {
        match seed {
            Some(seed) => from_seed(seed, nonce, chain),
            Some(private_key) => from_private_key(private_key, chain),
            Some(public_key) => from_public_key(public_key, chain),
            Some(address) => from_adress(1, address, chain),
            None => newSeed(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_seed(&self) -> Vec<u8> {
        self.seed.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }

    #[wasm_bindgen(getter)]
    pub fn get_chain(&self) -> u8 {
        if self.chain == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_address(&self) -> Vec<u8> {
        self.address.clone()
    }
}
