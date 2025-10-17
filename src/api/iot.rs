use serde::Serialize;
use rand::Rng;

#[derive(Serialize)]
pub struct IoTData {
    pub power_kw: u64,
    pub energy_today_kwh: u64,
}

pub fn mock_iot_data() -> IoTData {
    let mut rng = rand::thread_rng();
    IoTData {
        power_kw: rng.gen_range(0..1000),
        energy_today_kwh: rng.gen_range(0..5000),
    }
}
