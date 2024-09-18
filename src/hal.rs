use rand::Rng;

pub trait Sensor {
    fn read(&self) -> f64;
}

pub trait Actuator {
    fn write(&mut self, value: f64);
}

pub struct MotorController {
    current_speed: f64,
}

impl Actuator for MotorController {
    fn write(&mut self, value: f64) {
        self.current_speed = value.clamp(-1.0, 1.0);
        println!("Motor speed set to: {}", self.current_speed);
    }
}

pub struct DistanceSensor {
    // Add fields for sensor configuration
}

impl Sensor for DistanceSensor {
    fn read(&self) -> f64 {
        // Simulate reading from a distance sensor
        rand::thread_rng().gen_range(0.0..10.0)
    }
}

pub struct HAL {
    pub motor: MotorController,
    pub distance_sensor: DistanceSensor,
}

impl HAL {
    pub fn new() -> Self {
        HAL {
            motor: MotorController { current_speed: 0.0 },
            distance_sensor: DistanceSensor {},
        }
    }
}