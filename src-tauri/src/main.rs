#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    // this is where you pass in your custom commands.
    .invoke_handler(tauri::generate_handler![hash_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use std::{fs, io};
use hex::encode_upper;
use digest::Digest;
use md5::{Md5};
use sha1::{Sha1};
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use ripemd::{Ripemd160, Ripemd320};
use blake2::{Blake2b512, Blake2s256};
use whirlpool::{Whirlpool};
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use streebog::*;
use tiger::Tiger;
use sm3::{Sm3};
use groestl::{Groestl256};
use gost94::{Gost94CryptoPro};
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};

#[tauri::command(async)]
async fn hash_command(path: String, algo: String) -> String {
  match &algo as &str {
    "MD5" => return hash_file(path, Md5::default()),
    "SHA1" => return hash_file(path, Sha1::default()),
    "SHA256" => return hash_file(path, Sha256::default()),
    "SHA384" => return hash_file(path, Sha384::default()),
    "SHA512" => return hash_file(path, Sha512::default()),
    "SHA3-224" => return hash_file(path, Sha3_224::new()),
    "SHA3-256" => return hash_file(path, Sha3_256::new()),
    "SHA3-384" => return hash_file(path, Sha3_384::new()),
    "SHA3-512" => return hash_file(path, Sha3_512::new()),
    "RIPEMD160" => return hash_file(path, Ripemd160::default()),
    "RIPEMD320" => return hash_file(path, Ripemd320::default()),
    "BLAKE2S" => return hash_file(path, Blake2s256::default()),
    "BLAKE2B" => return hash_file(path, Blake2b512::default()),
    "WHIRLPOOL" => return hash_file(path, Whirlpool::default()),
    "SHABAL192" => return hash_file(path, Shabal192::new()),
    "SHABAL224" => return hash_file(path, Shabal224::new()),
    "SHABAL256" => return hash_file(path, Shabal256::new()),
    "SHABAL384" => return hash_file(path, Shabal384::new()),
    "SHABAL512" => return hash_file(path, Shabal512::new()),
    "STREEBOG256" => return hash_file(path, Streebog256::new()),
    "STREEBOG512" => return hash_file(path, Streebog512::new()),
    "TIGER" => return hash_file(path, Tiger::default()),
    "SM3" => return hash_file(path, Sm3::default()),
    "GROESTL" => return hash_file(path, Groestl256::default()), 
    "GOST" => return hash_file(path, Gost94CryptoPro::default()),
    "FSB-160" => return hash_file(path, Fsb160::default()),
    "FSB-224" => return hash_file(path, Fsb224::default()),
    "FSB-256" => return hash_file(path, Fsb256::default()),
    "FSB-384" => return hash_file(path, Fsb384::default()),
    "FSB-512" => return hash_file(path, Fsb512::default()),
    _ => return hash_file(path, Sha256::default()),
  }
}

fn hash_file<T>(path: String, mut hasher: T) -> String
where
  T: Digest,
  T: io::Write, 
{
    let mut input = fs::File::open(path).unwrap();
    io::copy(&mut input, &mut hasher).unwrap();
    return encode_upper(hasher.finalize());
}