extern crate clap;
extern crate rand;
extern crate ripemd160;
extern crate secp256k1;
extern crate sha2;
extern crate rayon;
extern crate bloomfilter;

use base58::ToBase58;
use clap::{App, Arg};
use rand::Rng;
use ripemd160::{Digest as RipemdDigest, Ripemd160};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest as ShaDigest};
use std::sync::{Arc};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use bloomfilter::Bloom;

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn public_key_to_address(pubkey: &[u8], sha256_hasher: &mut Sha256, ripemd160_hasher: &mut Ripemd160) -> String {
    sha256_hasher.update(pubkey);
    let sha256_hash = sha256_hasher.finalize_reset();
    ripemd160_hasher.update(&sha256_hash);
    let ripemd160_hash = ripemd160_hasher.finalize_reset();

    let mut address_bytes = [0u8; 25];
    address_bytes[0] = 0x00; // Prefixo para endereços Bitcoin
    address_bytes[1..21].copy_from_slice(&ripemd160_hash);
    let checksum = Sha256::digest(&Sha256::digest(&address_bytes[..21]));
    address_bytes[21..25].copy_from_slice(&checksum[..4]);
    address_bytes.to_base58()
}

fn generate_key(private_key_bytes: &[u8], secp: &Secp256k1<secp256k1::All>, sha256_hasher: &mut Sha256, ripemd160_hasher: &mut Ripemd160) -> Option<String> {
    let mut extended_key = [0u8; 32];
    extended_key[16..].copy_from_slice(private_key_bytes);

    if let Ok(private_key) = SecretKey::from_slice(&extended_key) {
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let serialized_pubkey = public_key.serialize();
        return Some(public_key_to_address(&serialized_pubkey, sha256_hasher, ripemd160_hasher));
    }

    None
}

fn generate_keys_and_addresses(start: u128, end: u128, target_address: Arc<String>, random: bool, verbose: bool, counter: Arc<AtomicU64>, found: Arc<AtomicBool>) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    let mut bloom = if random && (end - start > 0) {
        Some(Bloom::new_for_fp_rate((end - start) as usize, 0.01))
    } else {
        None
    };

    let mut sha256_hasher = Sha256::new();
    let mut ripemd160_hasher = Ripemd160::new();
    let mut local_count = 0;

    for current in start..=end {
        if found.load(Ordering::SeqCst) {
            break;
        }

        let private_key_bytes = if random {
            loop {
                let random_value = rng.gen_range(start..=end);
                let random_bytes = random_value.to_be_bytes().to_vec();

                if let Some(ref mut bloom) = bloom {
                    if !bloom.check_and_set(&random_bytes) {
                        break random_bytes;
                    }
                } else {
                    break random_bytes;
                }
            }
        } else {
            current.to_be_bytes().to_vec()
        };

        if let Some(address) = generate_key(&private_key_bytes, &secp, &mut sha256_hasher, &mut ripemd160_hasher) {
            let private_key_hex = bytes_to_hex(&private_key_bytes);
            if verbose {
                println!("Testando chave: {} -> Endereço: {}", private_key_hex, address);
            }
            if address == *target_address {
                println!("\n==============================\nKEY FOUND: {} \nBitcoin: {}\n==============================", &private_key_hex, &address);
                found.store(true, Ordering::SeqCst);
                break;
            }
        }

        local_count += 1;
        if local_count % 1000 == 0 {
            counter.fetch_add(1000, Ordering::SeqCst);
        }
    }
}

fn main() {
    println!("==============================\n      BTC-Cracker v1.0        \n==============================\n");

    let matches = App::new("BTC-Cracker")
        .version("1.0")
        .author("Seu Nome <seuemail@example.com>")
        .about("Gera chaves e endereços Bitcoin")
        .arg(Arg::new("range")
            .short('r')
            .long("range")
            .value_name("RANGE")
            .help("Intervalo de chaves em hexadecimal (ex: 1a838b13505b26861,2832ed74f2b5e35ff)")
            .takes_value(true)
            .required(false))
        .arg(Arg::new("random_range")
            .short('R')
            .long("random-range")
            .value_name("RANGE")
            .help("Intervalo de chaves em hexadecimal para percorrer de maneira aleatória (ex: 1a838b13505b26861,2832ed74f2b5e35ff)")
            .takes_value(true)
            .required(false))
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .value_name("TARGET")
            .help("Endereço Bitcoin alvo")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("threads")
            .short('T')
            .long("threads")
            .value_name("THREADS")
            .help("Número de threads a serem usadas (padrão é 8)")
            .takes_value(true)
            .default_value("8"))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Exibe as chaves privadas testadas e os endereços BTC correspondentes"))
        .get_matches();

    let (range, random) = if let Some(range) = matches.value_of("range") {
        (range, false)
    } else if let Some(range) = matches.value_of("random_range") {
        (range, true)
    } else {
        eprintln!("Erro: Você deve fornecer um intervalo com -r ou -R.");
        std::process::exit(1);
    };

    let parts: Vec<&str> = range.split(',').collect();

    if parts.len() != 2 {
        eprintln!("Erro: O intervalo deve conter dois valores hexadecimais separados por vírgula.");
        std::process::exit(1);
    }

    let start = u128::from_str_radix(parts[0], 16).expect("Intervalo inicial inválido");
    let end = u128::from_str_radix(parts[1], 16).expect("Intervalo final inválido");

    // Dividir o trabalho em partes menores
    let num_threads: usize = matches.value_of_t("threads").unwrap_or(8);

    // Calcular o tamanho do chunk
    let chunk_size = (end - start + 1) / num_threads as u128;

    // Usar Rayon para paralelizar o trabalho
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    // Criar variáveis compartilhadas
    let target_address = Arc::new(matches.value_of("target").unwrap().to_string());
    let counter = Arc::new(AtomicU64::new(0));
    let found = Arc::new(AtomicBool::new(false));

    // Thread para exibir progresso
    {
        let counter_clone = Arc::clone(&counter);
        let found_clone = Arc::clone(&found);

        thread::spawn(move || {
            let mut last_count = 0;
            loop {                                                                                      thread::sleep(Duration::from_secs(30));
                let count = counter_clone.load(Ordering::SeqCst);
                let tested_in_last_30_seconds = count - last_count;
                last_count = count;                                                     
                print!("\rTotal: {} keys, {} keys/s", count, tested_in_last_30_seconds / 30);
                io::stdout().flush().unwrap();                                          
                if found_clone.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
    }

    rayon::scope(|s| {
        for i in 0..num_threads {
            // Calcular o início e o fim do chunk
            let start_chunk = start + i as u128 * chunk_size;
            // O último chunk pode ser maior se o número total não for divisível pelo número de threads
            let end_chunk = if i == num_threads - 1 { end } else { start_chunk + chunk_size - 1 };

            // Clonar variáveis para uso dentro da thread
            let target_address_clone = Arc::clone(&target_address);
            let counter_clone = Arc::clone(&counter);
            let found_clone = Arc::clone(&found);

            s.spawn(move |_| {
                generate_keys_and_addresses(start_chunk, end_chunk, target_address_clone, random, false, counter_clone, found_clone);
            });                                                                                 }
    });

    while !found.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
    }

    println!("\nProcesso concluído. Verifique o resultado acima para a chave encontrada.");
}
