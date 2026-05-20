#![no_std]

#[cfg(feature = "alloc-hooks")]
pub mod alloc;


use defmt::println;
use embassy_net::{dns::DnsSocket, tcp::client::TcpClient};
use embassy_time::{Duration, Timer};
use esp_hal::{peripherals::{SW_INTERRUPT, TIMG0}, rng::Rng};
use reqwless::{client::{HttpClient, TlsConfig, TlsVerify}, request::Method};

pub fn init_heap_and_timers(tim: TIMG0<'static>, sw_int: SW_INTERRUPT<'static>) {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 36 * 1024);
    let timg0 = esp_hal::timer::timg::TimerGroup::new(tim);
    let sw_int = esp_hal::interrupt::software::SoftwareInterruptControl::new(sw_int);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
}

pub async fn create_client_and_connect<const N: usize, const TX_SZ: usize, const RX_SZ: usize>(
    tcp_client: &TcpClient<'static, N, TX_SZ, RX_SZ>,
    dns_client: &DnsSocket<'static>,
    link: &str
) {
    let mut client = HttpClient::new(tcp_client, dns_client);
    let mut rx_buf = [0u8; 4096];

    let mut builder = client
        .request(Method::GET, link)
        .await
        .unwrap();

    //let mut builder = builder.headers(&[("Host", "httpbin.org"), ("Connection", "close")]);

    let response = builder.send(&mut rx_buf).await.unwrap();

    match response.body().read_to_end().await {
        Ok(data) => {
            if let Ok(st) = core::str::from_utf8(data) {
                println!("Body: {}", st);
            }
        }
        Err(e) => println!("Body error: {:?}", e),
    }
}


pub async fn create_client_and_connect_tls<const N: usize, const TX_SZ: usize, const RX_SZ: usize>(
    tcp_client: &TcpClient<'static, N, TX_SZ, RX_SZ>,
    dns_client: &DnsSocket<'static>,
    link: &str,
){
    
    let rng = Rng::new();

    loop {
        Timer::after(Duration::from_millis(1000)).await;

        let mut tls_read_buf = [0; 17000];
        let mut tls_write_buf = [0; 17000];
        let seed = (rng.random() as u64) << 32 | rng.random() as u64;
        let tls_config = TlsConfig::new(seed, &mut tls_read_buf, &mut tls_write_buf, TlsVerify::None);

        let mut client = HttpClient::new_with_tls(&tcp_client, &dns_client, tls_config);
        let mut rx_buf = [0u8; 4096];

        let mut builder = client
            .request(Method::GET, link)
            .await
            .unwrap();

        let response = builder.send(&mut rx_buf).await.unwrap();

        
        match response.body().read_to_end().await {
            Ok(data) => {
                if let Ok(st) = core::str::from_utf8(data) {
                    println!("Body: {}", st);
                }
            }
            Err(e) => println!("Body error: {:?}", e),
        }
        
    }
}
