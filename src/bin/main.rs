// https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/wifi_embassy_dhcp.rs
//! Embassy DHCP Example
//!
//! Set SSID and PASSWORD env variable before running this example.
//!
//! This gets an ip address via DHCP then performs an HTTP get request to some "random" server
//!
//! Because of the huge task-arena size configured this won't work on ESP32-S2

//% FEATURES: embassy esp-wifi esp-wifi/wifi esp-wifi/utils esp-hal/unstable
//% CHIPS: esp32 esp32s2 esp32s3 esp32c2 esp32c3 esp32c6

#![no_std]
#![no_main]

// Core dependencies for network functionality
use core::net::Ipv4Addr; // Provides IPv4 address structure in no_std environment

// Embassy is an async runtime and network stack for embedded systems
use embassy_executor::Spawner; // Handles scheduling and running async tasks
use embassy_net::{tcp::TcpSocket, Runner, StackResources}; // Networking components
use embassy_time::{Duration, Timer}; // Time management utilities

// ESP32-specific dependencies
use esp_alloc as _; // Heap allocation support (required for dynamic memory)
use esp_backtrace as _; // Provides better error traces
use esp_hal::{clock::CpuClock, rng::Rng, timer::timg::TimerGroup}; // Hardware abstraction layer
use esp_println::println; // println! macro that works on ESP32
use esp_wifi::{
    init,
    wifi::{
        ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiStaDevice,
        WifiState,
    },
    EspWifiController,
}; // WiFi-specific functionality

// Add this line to import the functions from network module
use superviper::network::{connection, net_task};
use superviper::system::setup_heap;

// Issue a build/run command like SSID=my_ssid PASSWORD=my_password cargo run
// OR set the environment variables in your shell before running cargo run
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

// This macro creates static (global) variables safely at runtime
// Static variables are necessary because the WiFi stack needs data that lives for the entire program
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    // Initialize logging system
    esp_println::logger::init_logger_from_env();

    // Configure the ESP32 to run at maximum CPU frequency
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config); // Initialize ESP32 hardware

    setup_heap();

    // Initialize timer groups (required for various timing operations)
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    // Initialize random number generator (required for network operations)
    let mut rng = Rng::new(peripherals.RNG);

    // Initialize WiFi controller with required peripherals
    let init = &*mk_static!(
        EspWifiController<'static>,
        init(timg0.timer0, rng.clone(), peripherals.RADIO_CLK).unwrap()
    );

    // Create WiFi interface in Station (client) mode
    let wifi = peripherals.WIFI;
    let (wifi_interface, controller) =
        esp_wifi::wifi::new_with_mode(&init, wifi, WifiStaDevice).unwrap();

    // Initialize embassy time driver
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timg1.timer0);

    // Configure network stack to use DHCP for IP address acquisition
    let config = embassy_net::Config::dhcpv4(Default::default());

    // Generate random seed for network stack
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // Initialize network stack with 3 sockets maximum
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    // Spawn background tasks for WiFi connection and network management
    spawner.spawn(connection(controller, SSID, PASSWORD)).ok();
    spawner.spawn(net_task(runner)).ok();

    // Buffer sizes for TCP communication
    let mut rx_buffer = [0; 4096]; // Receive buffer
    let mut tx_buffer = [0; 4096]; // Transmit buffer

    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    loop {
        Timer::after(Duration::from_millis(1_000)).await;

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        let remote_endpoint = (Ipv4Addr::new(142, 250, 185, 115), 80);
        println!("connecting...");
        let r = socket.connect(remote_endpoint).await;
        if let Err(e) = r {
            println!("connect error: {:?}", e);
            continue;
        }
        println!("connected!");
        let mut buf = [0; 1024];
        loop {
            use embedded_io_async::Write;
            let r = socket
                .write_all(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
                .await;
            if let Err(e) = r {
                println!("write error: {:?}", e);
                break;
            }
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    println!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    println!("read error: {:?}", e);
                    break;
                }
            };
            println!("{}", core::str::from_utf8(&buf[..n]).unwrap());
        }
        Timer::after(Duration::from_millis(3000)).await;
    }
}
