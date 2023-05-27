pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash_sha256(&self) -> [u8; 32] {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
            .try_into()
            .unwrap()
    }
}
impl Hashable for u32 {
    fn bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for u64 {
    fn bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for u128 {
    fn bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl Hashable for String {
    fn bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
impl Hashable for Vec<u8> {
    fn bytes(&self) -> Vec<u8> {
        self.clone()
    }
}
