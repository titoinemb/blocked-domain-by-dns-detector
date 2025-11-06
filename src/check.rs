use std::net::IpAddr;
use trust_dns_resolver::config::{
    NameServerConfig, Protocol, ResolverConfig, ResolverOpts,
};
use trust_dns_resolver::Resolver;

pub fn is_blocked(domain: &str, dns_server: &str) -> bool {
    let dns_ip: IpAddr = match dns_server.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Adresse DNS invalide : {}", dns_server);
            return false;
        }
    };

    let name_server: NameServerConfig = NameServerConfig {
        socket_addr: (dns_ip, 53).into(),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: true,
        bind_addr: None,
    };

    let resolver_config: ResolverConfig = ResolverConfig::from_parts(None, vec![], vec![name_server]);
    let resolver_opts: ResolverOpts = ResolverOpts::default();

    let resolver: Resolver = match Resolver::new(resolver_config, resolver_opts) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erreur d'initialisation du résolveur DNS : {}", e);
            return false;
        }
    };

    match resolver.lookup_ip(domain) {
        Ok(lookup) => {
            let ips: Vec<IpAddr> = lookup.iter().collect();
            println!("Résolution réussie : {:?}", ips);
            false
        }
        Err(e) => {
            println!("Erreur de résolution : {}", e);
            true
        }
    }
}
