use sha2::{Digest};

fn mining(record:&str)->(u32, String, String) {
    for nonce in 0..1000000000 {
        let text = format!("nonce={:x}\n{}", nonce, record);
        let hash = format!("{:x}", sha2::Sha256::digest(text.as_bytes()));
        if hash.starts_with("0000") { return (nonce, hash, text); }
    }
    return (0, "".to_string(), "".to_string());
}

fn main() {
    let (nonce, hash, text) = mining("john=>mary:$2.7\ngeorge=>john:$1.3");
    println!("nonce={:x}\nhash ={}\n=====text=======\n{}", nonce, hash, text);
}
