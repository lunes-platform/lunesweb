use super::Wallet;
use super::{crypto, word_list};
use crate::utils::base58::vec_to_b58;
use crate::utils::random::random_triple_number;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(js_name = "fromSeed")]
    pub fn from_seed(seed: String, nonce: Option<u32>, chain: Option<u8>) -> Wallet {
        let seed_len: u8 = seed.split(" ").collect::<Vec<&str>>().len() as u8;
        let nonce = nonce.unwrap_or(0);
        let chain = chain.unwrap_or(1);
        let seed_vec = crypto::hidden_seed(nonce, seed.clone());
        let private_key = crypto::to_private_key(seed_vec.clone());
        let public_key = crypto::to_public_key(private_key.clone());
        let address = crypto::to_address(1, chain, public_key.clone());

        Wallet {
            seed,
            seed_len,
            nonce,
            chain,
            private_key: vec_to_b58(private_key),
            public_key: vec_to_b58(public_key),
            address: vec_to_b58(address),
        }
    }

    #[wasm_bindgen(js_name = "newSeed")]
    pub fn new_seed(n_words: Option<u8>, nonce: Option<u32>, chain: Option<u8>) -> Wallet {
        let nonce = nonce.unwrap_or(0);
        let chain = chain.unwrap_or(1);
        let n_words = match n_words {
            Some(n) => n,
            None => 4,
        };
        let mut seed: Vec<String> = vec![];

        for _ in 0..=n_words {
            for n in random_triple_number() {
                seed.push(word_list()[n as usize].clone())
            }
        }

        Wallet::from_seed(seed.join(" "), Some(nonce), Some(chain))
    }
}

// TODO translate typescript code below to Rust Robson

//     fromPrivateKey: (
//         privateKey: string,
//         chain: WalletTypes.Chain
//     ): IAccount => {
//         const publicKey = wasm.toPublicKey(wasm.base58ToArray(privateKey))
//         const address = wasm.toAddress(1, chain, publicKey)

//         return {
//             seed: "",
//             nonce: 0,
//             chain: chain,
//             privateKey: privateKey,
//             publicKey: wasm.arrayToBase58(publicKey),
//             address: wasm.arrayToBase58(address)
//         }
//     },
