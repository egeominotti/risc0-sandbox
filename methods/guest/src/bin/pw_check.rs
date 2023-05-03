#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
risc0_zkvm::guest::entry!(main);

pub fn main() {
    let pw: String = env::read();
    let mut is_ok: bool = false;

    for ch in pw.chars() {
        if ch.is_ascii_punctuation() {
            is_ok = true;
            break;
        }
    }

    if !is_ok {
        panic!();
    }

    let digest = Impl::hash_bytes(pw.as_bytes());

    env::commit(digest);
}
