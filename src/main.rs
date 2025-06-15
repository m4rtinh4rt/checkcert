use std::{env, fs::read_to_string, io::Error, process};

use checkcert::Host;

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: {} FILENAME", env::args().next().unwrap());
        process::exit(1);
    });

    let lines: Vec<String> = read_to_string(&filename)
        .map_err(|e| {
            eprintln!("Error: Unable to read the file '{}': {}", filename, e);
            process::exit(1);
        })
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let hosts: Vec<Host> = lines
        .iter()
        .filter_map(|line| {
            let mut tokens = line.split_whitespace();
            let domain_port = tokens.next()?.split(':').collect::<Vec<&str>>();
            let digest = tokens.next()?.to_string();

            if domain_port.len() == 2 {
                let domain = domain_port[0].to_string();
                let port = domain_port[1].parse::<u16>().ok()?;
                Some(Host::new(domain, port, digest))
            } else {
                None
            }
        })
        .collect();

    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    for host in hosts {
        match host.hello(tls_config.clone()) {
            Ok(x) => {
                if host.equal(&x) {
                    println!("ok\t{}", host.host_with_port());
                } else {
                    eprintln!("fail\t{} (found: {})", host.host_with_port(), x);
                }
            }
            Err(e) => {
                eprintln!("err\t{} ({})", host.host_with_port(), e);
            }
        }
    }
    Ok(())
}
