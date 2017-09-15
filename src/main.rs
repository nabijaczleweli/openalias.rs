extern crate openalias;

use std::str::FromStr;
use openalias::Options;


fn main() {
    let opts = Options::parse();
    println!("{:#?}", opts);

    println!("{:#?}", openalias::alias_to_fqdn("nabijaczleweli.xyz"));
    println!("{:#?}", openalias::alias_to_fqdn("nabijaczleweli xyz"));
    println!("{:#?}", openalias::alias_to_fqdn("donate@nabijaczleweli.xyz"));
    println!("{:#?}", openalias::address_strings("nabijaczleweli"));
    println!("{:#?}", openalias::address_strings("nabijaczleweli.xyz"));
    println!("{:#?}", openalias::addresses("donate.getmonero.org."));
    println!("{:#?}",
             openalias::CryptoAddress::from_str("oa1:btc recipient_address=1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS; \
             	                                   recipient_name=\"nabijaczleweli; free dicks for all\";tx_description=Donation for nabijaczleweli:\\ ; \
             	                                   tx_amount=0.1;checksum=2BE79557; communism=yass;"));
}
