//use std::hash::Hasher;
use hex;

use sha2::{Sha512, Digest};
pub struct Node {
    ip: String,
    port: u16,
    id: [u8; 64],
}
 
impl Node {
    // Constructor
    pub fn new(ip: String, port: u16) -> Self {
        let id = Self::my_hash(&ip);
        Node { ip, port, id }
    }
    fn my_hash(ip: &str) -> [u8; 64] {         // create super mega secret ID (sha512 of IP IP)
        let addr = format!("{}{}", ip, ip);
        let addr = addr.as_bytes();
        let mut hasher = Sha512::new();
        hasher.update(&addr);
        let result = hasher.finalize();
        let shad = format!("{:X}", result);
        let decoded = hex::decode(shad).expect("Decoding failed");
        let mut id: [u8; 64] = [0; 64];
        for i in 0..decoded.len() {
            id[i] = decoded[i];
        }
        id
    }
    // Getter methods
    pub fn get_ip(&self) -> &String {
        &self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_id(&self) -> [u8; 64] {
        self.id
    }

    // Setter methods
    fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn set_id(&mut self, id: [u8; 64]) {
        self.id = id;
    }

    pub fn print_info(&self) {
        println!("IP: {}, Port: {}, ID: {}", self.ip, self.port, hex::encode(&self.id).to_ascii_lowercase());
    }
}
