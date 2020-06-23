use sha2::{Digest};

fn mining(ctx:&str)->u32 {
    for nonce in 0..1000000000 {
        let text = format!("nonce={:x}\n{}\n\n", nonce, ctx);
        let h = format!("{:x}", sha2::Sha256::digest(text.as_bytes()));
        if h.starts_with("00000") { return nonce; }
    }
    return 0;
}

fn main() {
    let nonce = mining("john => mary : $2.7;\ngeorge => john : $1.3");
    println!("nonce={:x}", nonce);
}
