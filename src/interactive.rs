use rcgen::generate_simple_self_signed;
use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

const OK: &str = "\x1b[0m\x1b[35m[fluffer]\x1b[0m";
const FAIL: &str = "\x1b[31m[fluffer]\x1b[0m";
const PROMPT: &str = "\x1b[32;3m→ ";

/// Interactively generate a certificate if one doesn't exist.
pub fn gen_cert() {
    // Exit if certificate already exists
    if let (Ok(_), Ok(_)) = (File::open("cert.pem"), File::open("key.pem")) {
        return;
    }

    let stdin = stdin();
    let mut stdout = stdout();

    // Y/N
    print!(
        "\n{OK} Missing certificate files!
Expected two files: ./cert.pem and ./key.pem

\x1b[1mDo you want to generate a new certificate now?\x1b[0m [y/n]
{PROMPT}"
    );
    let _ = stdout.flush();
    let mut yorn = String::new();
    let _ = stdin.read_line(&mut yorn);

    if yorn != "y\n" {
        return;
    }

    // Prompt domain(s)
    print!(
        "\n{OK} Enter the domain name(s) you will be using.
e.g. localhost,domain.tld,domain2.tld
{PROMPT}"
    );
    let _ = stdout.flush();
    let mut dlist = String::new();
    let _ = stdin.read_line(&mut dlist);

    let domains: Vec<String> = dlist
        .split(',')
        .into_iter()
        .map(|d| d.trim().to_string())
        .collect();

    println!("\x1b[3m{domains:#?}\x1b[0m");

    let gen_pair = match generate_simple_self_signed(domains) {
        Err(e) => {
            eprintln!("{FAIL} Failed to generated key pair: {e}");
            return;
        }
        Ok(o) => o,
    };

    // Write cert
    match gen_pair.serialize_pem() {
        Ok(cert) => match File::create("cert.pem") {
            Ok(mut file) => {
                if let Err(e) = write!(file, "{}", cert) {
                    eprintln!("{FAIL} 📜 Failed to save cert.pem: {e}");
                    return;
                }
                println!("{OK} 📜 Wrote cert.pem");
            }
            Err(e) => {
                eprintln!("{FAIL} 📜 Failed to create file cert.pem: {e}");
                return;
            }
        },
        Err(e) => eprintln!("{FAIL} 📜 Failed to serialize cert.pem: {e}"),
    }

    // Write key
    match File::create("key.pem") {
        Ok(mut file) => {
            if let Err(e) = write!(file, "{}", gen_pair.serialize_private_key_pem()) {
                eprintln!("{FAIL} 🔑 Failed to save key.pem: {e}");
                return;
            }
            println!("{OK} 🔑 Wrote key.pem");
        }
        Err(e) => {
            eprintln!("{FAIL} 🔑 Failed to create file key.pem: {e}");
            return;
        }
    }
}
