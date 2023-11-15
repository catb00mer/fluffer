use rcgen::{CertificateParams, DistinguishedName, DnType, DnValue};
use std::{
    fs::{File, Permissions},
    io::{stdin, stdout, Write},
    os::unix::prelude::PermissionsExt,
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
    stdout.flush()?;
    let mut yorn = String::new();
    stdin.read_line(&mut yorn)?;

    if yorn != "y\n" {
        return Err(AppErr::RcGenStop);
    }

    // Prompt domain(s)
    print!(
        "\n{OK} Enter the domain name(s) you will be using.
e.g. localhost, *.localhost, sub.example.com
{PROMPT}"
    );
    stdout.flush()?;

    let mut domains = String::new();
    stdin.read_line(&mut domains)?;

    let mut domains: Vec<String> = domains.split(',').map(|d| d.trim().to_string()).collect();

    if domains.iter().all(|x| x.is_empty()) {
        return Err(AppErr::RcGenNoDomains);
    }

    // Preview domains
    println!("\x1b[3m{domains:#?}\x1b[0m");

    // Use the first domain as the subject name
    let subject_name = domains.get(0).ok_or(AppErr::RcGenNoDomains)?.clone();

    // Remove subject name from domains, and use the remaining ones as
    // alt names
    domains.reverse();
    domains.pop();
    let subject_alt_names = domains;

    // Create params
    let mut params = CertificateParams::new(subject_alt_names);
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, DnValue::Utf8String(subject_name));

    // Generate keypair
    let gen_pair = rcgen::Certificate::from_params(params)?;

    // Write cert
    let pem = gen_pair.serialize_pem()?;
    let mut file = File::create(&cert)?;
    write!(file, "{pem}")?;
    println!("{OK} 📜 Wrote cert.pem");

    // Write key
    let mut file = File::create("key.pem")?;
    file.set_permissions(Permissions::from_mode(0o600))?;
    write!(file, "{}", gen_pair.serialize_private_key_pem())?;
    println!("{OK} 🔑 Wrote {key}");

    Ok(())
}
