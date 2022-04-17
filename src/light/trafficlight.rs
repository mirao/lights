use std::fmt::Display;

use crate::light::trafficlightcolor::TrafficLightColor;

use super::Light;

#[derive(Debug)]
pub(crate) struct TrafficLight {
    color: TrafficLightColor,
}

impl TrafficLight {
    pub(crate) fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }

    pub(crate) fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }

    pub(crate) fn turn_yellow(&mut self) {
        self.color = TrafficLightColor::Yellow
    }

    pub(crate) fn turn_red(&mut self) {
        self.color = TrafficLightColor::Red
    }
}

impl Default for TrafficLight {
    fn default() -> Self {
        Self::new()
    }
}

impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.color
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffic light is {}", self.color)
    }
}
