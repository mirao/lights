use std::fmt::Display;

use super::Light;

#[derive(Debug)]
pub(crate) struct HouseLight {
    on: bool,
}

impl HouseLight {
    pub(crate) fn new() -> Self {
        Self { on: false }
    }
}

impl Default for HouseLight {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for HouseLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Houselight is {}", if self.on { "on" } else { "off" })
    }
}

impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.on
    }
}
