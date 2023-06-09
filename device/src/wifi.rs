use esp_idf_hal::{modem::Modem, peripheral::Peripheral};
// use esp_idf_svc::netif::EspNetifWait;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    wifi::{EspWifi, WifiWait},
};
use std::time::Duration;
// use std::net::Ipv4Addr;
use embedded_svc::wifi::{AccessPointConfiguration, ClientConfiguration, Configuration};

// Initializes ESP32 wifi and connects to the specified network
pub fn init_wifi(
    wifi_ssid: &str,
    wifi_pass: &str,
    modem: impl Peripheral<P = Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Box<EspWifi<'static>>> {
    let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), None)?);
    println!("wifi: created");

    let ap_infos = wifi.scan()?;

    let mut channel = None;
    let target = ap_infos.into_iter().find(|ap| ap.ssid == wifi_ssid);
    if let Some(target) = target {
        println!("wifi: ssid matched");
        channel = Some(target.channel)
    }

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: wifi_ssid.into(),
            password: wifi_pass.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    wifi.start()?;

    if !WifiWait::new(&sysloop)?
        .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
    {
        println!("wifi: device failed to start");
    }

    println!("wifi: connecting to {}", wifi_ssid);

    wifi.connect()?;

    // TODO: fix
    // if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
    //     Duration::from_secs(20),
    //     || {
    //         wifi.is_connected().unwrap()
    //             && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
    //     },
    // ) {
    //     bail!("wifi: did not connect or did not receive a DHCP lease");
    // }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    println!("wifi: dhcp info: {:?}", ip_info);

    Ok(wifi)
}
