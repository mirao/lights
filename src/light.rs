pub(crate) mod houselight;
pub(crate) mod trafficlight;
mod trafficlightcolor;

pub(crate) trait Light {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &dyn std::fmt::Debug;
}
