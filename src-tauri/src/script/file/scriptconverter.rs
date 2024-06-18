use anyhow::{bail, Result};
use sha3::Digest;

use crate::script::data::script::Script;

use super::dat::{cipher_to_text, text_to_cipher};

const SCRIPT_DAT_HASH:&str = "d18f3a643bee62db6870b35b1a1781bcc4067bd7409fa620168e16054ddc7ce645463b59e06d0768d87eff9ad9bdc1f0efd04dbc498d2e5de73d5a863a692a90";
const SCRIPT_DAT_EN_HASH:&str = "146e1b6e9e63ed22fb84b3c38f4d25a0723b07fe3fefe9395af68d6eeaa3b1108b288847ec50114efff4e7600afccc68a983d681b94cbb55a507b21f45d52db7";

pub fn read_script_dat(file: &[u8]) -> Result<Script> {
    if !is_valid_script_dat(file) {
        bail!("Invalid script.dat file");
    }
    let txt = cipher_to_text(file);
    Script::parse(&txt)
}

pub fn build_script_dat(script: &Script) -> Vec<u8> {
    let txt = script.stringify();
    text_to_cipher(&txt)
}

pub fn is_valid_script_dat(file: &[u8]) -> bool {
    let script_dat_hash = sha3::Sha3_512::digest(file).to_vec();
    script_dat_hash == hex::decode(SCRIPT_DAT_HASH).unwrap()
        || script_dat_hash == hex::decode(SCRIPT_DAT_EN_HASH).unwrap()
}
