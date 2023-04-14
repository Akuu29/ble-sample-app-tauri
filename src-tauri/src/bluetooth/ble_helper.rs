use btleplug::platform::Peripheral;

#[derive(Debug)]
pub struct BleHelper {
    pub peripherals: Option<Vec<Peripheral>>,
}
