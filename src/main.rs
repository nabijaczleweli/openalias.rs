extern crate openalias;

use std::str::FromStr;


fn main() {
    println!("{:#?}",
             openalias::CryptoAddress::from_str("oa1:btc recipient_address=1MoSyGZp3SKpoiXPXfZDFK7cDUFCVtEDeS; \
             	                                   recipient_name=\"nabijaczleweli; free dicks for all\";tx_description=Donation for nabijaczleweli:\\ ; \
             	                                   tx_amount=0.1;checksum=2BE79557;"));
}
