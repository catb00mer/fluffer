use rcgen::generate_simple_self_signed;
use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

use crate::err::AppErr;

const OK: &str = "\x1b[0m\x1b[33m[fluffer]\x1b[0m";
const FAIL: &str = "\x1b[31m[fluffer]\x1b[0m";
const PROMPT: &str = "\x1b[33;3m→ ";

/// Interactively generate a certificate if one doesn't exist.
pub fn gen_cert(cert: &str, key: &str) -> Result<(), AppErr> {
    match (File::open(cert).is_ok(), File::open(key).is_ok()) {
        (false, true) => {
            eprintln!("{FAIL} Missing certificate: [{cert}].");
            eprintln!(
                "{FAIL} You can move [{key}] to generate a new keypair, or relocate the missing file."
            );
            return Err(AppErr::RcGenStop);
        }
        (true, false) => {
            eprintln!("{FAIL} Missing private key: [{key}].");
            eprintln!(
                "{FAIL} You can move [{cert}] to generate a new keypair, or relocate the missing file."
            );
            return Err(AppErr::RcGenStop);
        }
        (true, true) => return Ok(()),
        (false, false) => (),
    }

    let stdin = stdin();
    let mut stdout = stdout();

    // Y/N
    print!(
        "\n{OK} Missing certificate files!
Expected two files: [{cert}] and [{key}].

\x1b[1mDo you want to generate a new certificate now?\x1b[0m [y/n]
{PROMPT}"
    );
    let _ = stdout.flush();
    let mut yorn = String::new();
    let _ = stdin.read_line(&mut yorn);

    if yorn != "y\n" {
        return Err(AppErr::RcGenStop);
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

    let domains: Vec<String> = dlist.split(',').map(|d| d.trim().to_string()).collect();

    println!("\x1b[3m{domains:#?}\x1b[0m");

    let gen_pair = match generate_simple_self_signed(domains) {
        Err(e) => {
            eprintln!("{FAIL} Failed to generated key pair: {e}");
            panic!();
        }
        Ok(o) => o,
    };

    // Write cert
    let pem = gen_pair.serialize_pem()?;
    let mut file = File::create(&cert)?;
    write!(file, "{pem}")?;
    println!("{OK} 📜 Wrote cert.pem");

    // Write key
    let mut file = File::create("key.pem")?;
    write!(file, "{}", gen_pair.serialize_private_key_pem())?;
    println!("{OK} 🔑 Wrote {key}");

    Ok(())
}
