mod check;
mod dns;
use dialoguer::Input;

fn main() {
    let domain: String = Input::new()
        .with_prompt("Domaine name")
        .interact_text()
        .unwrap();

    for (country, dns_list) in dns::COUNTRIES.iter() {
        println!("DNS Server Verification {}:", country);

        for ip in dns_list.iter() {
            if check::is_blocked(&domain, ip) {
                println!("🚫 {} is blocked by DNS {}", domain, ip);
            } else {
                println!("✅ {} is NOT blocked by DNS {}", domain, ip);
            }
        }
        println!();
    }
}
