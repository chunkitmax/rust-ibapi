use std::thread;
use std::time::Duration;

use clap::{arg, ArgMatches, Command};

use ibapi::contracts::Contract;
use ibapi::market_data::realtime::{BarSize, WhatToShow};
use ibapi::Client;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = Command::new("stream_bars")
        .version("1.0")
        .author("Wil Boayue <wil@wsbsolutions.com")
        .about("Streams realtime bars")
        .arg(arg!(--connection_string <VALUE>).default_value("localhost:4002"))
        .arg(arg!(--stock <SYMBOL>))
        .arg(arg!(--futures <SYMBOL>))
        .arg(arg!(--exchange <EXCHANGE>))
        .get_matches();

    let connection_string = matches.get_one::<String>("connection_string").expect("connection_string is required");
    let contract = extract_contract(&matches).expect("error parsing --stock or --future or --exchange");

    println!("connection_string: {connection_string:?}");
    println!("contract: {contract:?}");

    let client = Client::connect("127.0.0.1:4002", 100).unwrap();

    println!("server_version: {}", client.server_version());
    println!("server_time: {:?}", client.connection_time());
    println!("next_order_id: {}", client.next_order_id());

    let bars = client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::Trades, false)?;

    for (i, bar) in bars.iter().enumerate().take(60) {
        println!("bar: {i:?} {bar:?}");
    }

    thread::sleep(Duration::from_secs(5));

    Ok(())
}

fn extract_contract(matches: &ArgMatches) -> Option<Contract> {
    if let Some(symbol) = matches.get_one::<String>("stock") {
        return Some(Contract::stock(&symbol.to_uppercase()));
    } else if let Some(local_symbol) = matches.get_one::<String>("futures") {
        if let Some(exchange) = matches.get_one::<String>("exchange") {
            return Some(Contract::futures(&local_symbol.to_uppercase(), &exchange.to_uppercase()));
        }
    }
    None
}
