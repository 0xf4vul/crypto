
use crate::aes::generic::ExpandedKey128;


// 6.4 The Output Feedback Mode, (Page-20)
// https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf
#[derive(Debug, Clone)]
pub struct AesOfb128 {
    iv: [u8; Self::BLOCK_LEN],
    cipher: ExpandedKey128,
}

impl AesOfb128 {
    pub const BLOCK_LEN: usize = ExpandedKey128::BLOCK_LEN;
    pub const KEY_LEN: usize   = ExpandedKey128::KEY_LEN;
    pub const NONCE_LEN: usize = ExpandedKey128::BLOCK_LEN;
    pub const B: usize = Self::BLOCK_LEN * 8; // The block size, in bits.

    pub fn new(key: &[u8], nonce: &[u8]) -> Self {
        assert_eq!(key.len(), Self::KEY_LEN);
        assert_eq!(nonce.len(), Self::NONCE_LEN);

        let cipher = ExpandedKey128::new(key);
        let mut iv = [0u8; Self::BLOCK_LEN];
        iv[..Self::BLOCK_LEN].copy_from_slice(nonce);
        
        Self { cipher, iv }
    }

    pub fn encrypt(&mut self, blocks: &mut [u8]) {
        assert_eq!(blocks.len() % Self::BLOCK_LEN, 0);

        let mut last_input_block = self.iv.clone();
        let n = blocks.len() / Self::BLOCK_LEN;

        for j in 0..n {
            let output_block = self.cipher.encrypt(&last_input_block);
            
            for i in 0..Self::BLOCK_LEN {
                blocks[j * Self::BLOCK_LEN + i] ^= output_block[i];
            }

            last_input_block = output_block;
        }
        
        // The number of bits in the last plaintext or ciphertext block.
        // let u = Self::BLOCK_LEN * 8;
        // // Last block
        // let output_block = self.cipher.encrypt(&last_input_block);
    }

    pub fn decrypt(&mut self, blocks: &mut [u8]) {
        assert_eq!(blocks.len() % Self::BLOCK_LEN, 0);

        let mut last_input_block = self.iv.clone();

        let n = blocks.len() / Self::BLOCK_LEN;

        for j in 0..n {
            let output_block = self.cipher.encrypt(&last_input_block);

            for i in 0..Self::BLOCK_LEN {
                blocks[j * Self::BLOCK_LEN + i] ^= output_block[i];
            }

            last_input_block = output_block;
        }
    }
}


#[test]
fn test_aes128_ofb_enc() {
    // F.4.1  OFB-AES128.Encrypt, (Page-59)
    // https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf
    let key   = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap();
    let nonce = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();

    let mut cipher = AesOfb128::new(&key, &nonce);

    let plaintext = hex::decode("\
6bc1bee22e409f96e93d7e117393172a\
ae2d8a571e03ac9c9eb76fac45af8e51\
30c81c46a35ce411e5fbc1191a0a52ef\
f69f2445df4f9b17ad2b417be66c3710").unwrap();

    let mut ciphertext = plaintext.clone();
    cipher.encrypt(&mut ciphertext);

    assert_eq!(&ciphertext[..], &hex::decode("\
3b3fd92eb72dad20333449f8e83cfb4a\
7789508d16918f03f53c52dac54ed825\
9740051e9c5fecf64344f7a82260edcc\
304c6528f659c77866a510d9c1d6ae5e").unwrap()[..]);
}

#[test]
fn test_aes128_ofb_dec() {
    // F.4.2  OFB-AES128.Decrypt, (Page-60)
    // https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf
    let key   = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap();
    let nonce = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();

    let mut cipher = AesOfb128::new(&key, &nonce);

    let ciphertext = hex::decode("\
3b3fd92eb72dad20333449f8e83cfb4a\
7789508d16918f03f53c52dac54ed825\
9740051e9c5fecf64344f7a82260edcc\
304c6528f659c77866a510d9c1d6ae5e").unwrap();

    let mut plaintext = ciphertext.clone();
    cipher.decrypt(&mut plaintext);

    assert_eq!(&plaintext[..], &hex::decode("\
6bc1bee22e409f96e93d7e117393172a\
ae2d8a571e03ac9c9eb76fac45af8e51\
30c81c46a35ce411e5fbc1191a0a52ef\
f69f2445df4f9b17ad2b417be66c3710").unwrap()[..]);
}