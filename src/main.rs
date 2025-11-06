mod check;
mod dns;
use dialoguer::Input;

fn main() {
    let domain: String = Input::new()
        .with_prompt("Domaine name")
        .interact_text()
        .unwrap();

    for ip in dns::FR.iter() {
        if check::is_blocked(&domain, ip) {
            println!("🚫 {} est bloqué par le DNS {}", domain, ip);
        } else {
            println!("✅ {} n'est PAS bloqué par le DNS {}", domain, ip);
        }
    }
    //recupere les servers dns et faire une boucle qui fait une boucle des pays
}