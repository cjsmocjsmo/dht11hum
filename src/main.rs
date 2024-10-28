// use dht_mmap_rust::{Dht, DhtType};

// fn main() {
//     // The sensor is a DHT11 connected on pin 23
//     let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

//     // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
//     // the read succeeds.
//     // For more information, see documentation on `read()`
//     let reading = dht.read().unwrap();

//     // let temp_c = reading.temperature();
//     // let temperature_f = temp_c * 9.0 / 5.0 + 32.0;

//     println!("{:.2}", reading.humidity());
// }

use dht_mmap_rust::{Dht, DhtType};
use rusqlite::{params, Connection, Result};
use std::path::Path;

fn main() -> Result<()> {
    // Initialize the SQLite database
    let path = Path::new("/home/picam/dht/sensor_data.db");
    let conn = Connection::open(&path)?;

    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor_readings (
            id INTEGER PRIMARY KEY,
            tempc REAL NOT NULL,
            tempf REAL NOT NULL,
            humi REAL NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // The sensor is a DHT11 connected on pin 23
    let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

    // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
    // the read succeeds.
    // For more information, see documentation on `read()`
    let foo = true;
    while foo {
        let reading = dht.read().unwrap();

        let tempc = reading.temperature();
        let tempf = tempc * 9.0 / 5.0 + 32.0;
        let humi = reading.humidity();

        // Insert the data into the database
        conn.execute(
            "INSERT INTO sensor_readings (tempc, tempf, humi, timestamp) VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)",
            params![tempc, tempf, humi],
        )?;

        println!("Temperature (C): {:.2}", tempc);
        println!("Temperature (F): {:.2}", tempf);
        println!("Humidity: {:.2}", humi);

        std::thread::sleep(std::time::Duration::from_secs(180));
        }

    Ok(())
}