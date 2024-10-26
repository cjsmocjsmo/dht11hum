use dht_mmap_rust::{Dht, DhtType};

fn main() {
    // The sensor is a DHT11 connected on pin 23
    let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

    // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
    // the read succeeds.
    // For more information, see documentation on `read()`
    let reading = dht.read().unwrap();

    // let temperature_c = reading.temperature();
    // let temperature_f = temperature_c * 9.0 / 5.0 + 32.0;

    println!("{:.2}", reading.humidity());
}

