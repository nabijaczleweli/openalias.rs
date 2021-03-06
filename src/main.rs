extern crate openalias;

use openalias::{Options, Error};
use std::process::exit;


macro_rules! maybe_escape {
    ($val:expr, $fmt:expr, $($arg:tt)*) => {
        println!($fmt, $($arg)*, q = if $val.starts_with(" ") || $val.ends_with(" ") {"\""} else {""});
    }
}


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        eprintln!("{}", err);
        1
    } else {
        0
    }
}

fn result_main() -> Result<(), Error> {
    let opts = Options::parse();

    for addr in opts.aliases {
        if opts.verbose {
            eprintln!("Looking up {}...", addr);
        }

        if opts.raw {
            let mut raddrs = openalias::address_strings(&addr)?;
            if raddrs.is_empty() {
                println!("No records found for {}.", addr);
            } else {
                if let Some(ref currency_filter) = opts.currency_filter.as_ref() {
                    raddrs.retain(|raddr| currency_filter.iter().any(|curr| raddr[4..].starts_with(curr)));
                    if raddrs.is_empty() {
                        print!("No ");
                        for (id, ref curr) in currency_filter.iter().enumerate() {
                            if id != 0 {
                                print!(", ");
                            }
                            if currency_filter.len() != 1 && id == currency_filter.len() - 1 {
                                print!("nor ");
                            }
                            print!("{}", curr);
                        }
                        println!(" records found for {}.", addr);
                    }
                }

                if !raddrs.is_empty() {
                    println!("Addresses for {}:", addr);
                    for raddr in raddrs {
                        println!("  {}", raddr);
                    }
                }
            }
        } else {
            let mut caddrs = openalias::addresses(&addr)?;
            if caddrs.is_empty() {
                println!("No addresses found for {}.", addr);
            } else {
                if let Some(ref currency_filter) = opts.currency_filter.as_ref() {
                    caddrs.retain(|caddr| currency_filter.iter().any(|curr| caddr.cryptocurrency == *curr));
                    if caddrs.is_empty() {
                        print!("No ");
                        for (id, ref curr) in currency_filter.iter().enumerate() {
                            if id != 0 {
                                print!(", ");
                            }
                            if currency_filter.len() != 1 && id == currency_filter.len() - 1 {
                                print!("nor ");
                            }
                            print!("{}", curr);
                        }
                        println!(" addresses found for {}.", addr);
                    }
                }

                if !caddrs.is_empty() {
                    println!("Addresses of {}:", addr);
                    for caddr in caddrs {
                        println!("  {}:", caddr.cryptocurrency);
                        if let Some(recipient_name) = caddr.recipient_name.as_ref() {
                            maybe_escape!(recipient_name, "    {q}{}{q}", recipient_name);
                        }
                        maybe_escape!(caddr.address, "    {q}{}{q}", caddr.address);
                        if let Some(tx_description) = caddr.tx_description.as_ref() {
                            maybe_escape!(tx_description, "    {q}{}{q}", tx_description);
                        }
                        if let Some(tx_amount) = caddr.tx_amount.as_ref() {
                            maybe_escape!(tx_amount, "    {q}{}{q}", tx_amount);
                        }
                        if let Some(tx_payment_id) = caddr.tx_payment_id.as_ref() {
                            maybe_escape!(tx_payment_id, "    {q}{}{q}", tx_payment_id);
                        }
                        if let Some(address_signature) = caddr.address_signature.as_ref() {
                            maybe_escape!(address_signature, "    {q}{}{q}", address_signature);
                        }
                        for (key, val) in caddr.additional_values {
                            maybe_escape!(val, "    {}: {q}{}{q}", key, val);
                        }
                        if let Some(&(_, checksum_ok)) = caddr.checksum.as_ref() {
                            println!("    Checksum {}", if checksum_ok { "OK" } else { "INCORRECT" });
                        }
                    }
                }
            }
        }

        println!();
    }

    Ok(())
}
