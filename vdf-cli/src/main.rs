extern crate vdf;
#[macro_use]
extern crate clap;
fn main() {
    let matches = clap_app!(vdf =>
        (version: "0.1.0")
        (author: "Block Notary <poa.networks>")
        (about: "CLI to Verifiable Delay Functions")
    ).get_matches();
}