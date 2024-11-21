use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, 
Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use rand::{Rng, thread_rng};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use json::json;
use std::net::TcpStream;

const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);

fn createJsonPeripherial(peripherial: Peripheral) -> JsonValue::Object{
    let peripheralJson = object!{
        // quotes on keys are optional
        "address": peripheral.address(),
        "id": peripheral.id(),
    }
}

fn jsonToString(JsonValue::Object) -> String{
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Start app");

    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();
    
    let mut stream = TcpStream::connect("127.0.0.1:34254");

    loop{
        // start scanning for devices

        println!("Scan start");
        central.start_scan(ScanFilter::default()).await?;

        // wait 5 seconds scan
        time::sleep(Duration::from_secs(5)).await;

        let peripherals = central.peripherals().await.unwrap();

        central.stop_scan().await?;
        println!("Scan stop");

        println!("Peripherals founded {}", peripherals.len());

        for peripheral in peripherals{
            let addr = peripheral.address();
            let id = peripheral.id();

            println!("Bluetooth id {} address {}", id, addr);

            auto jsonPeripheral = createJsonPeripherial(peripheral);
        }
    }
    
    println!("End app");

    Ok(())
}

async fn find_light(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("LEDBlue"))
        {
            return Some(p);
        }
    }
    None
}
