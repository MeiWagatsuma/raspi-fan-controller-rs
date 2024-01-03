use rppal::gpio::{Gpio, Mode, OutputPin};
use std::{fs, thread, time};

const CHECK_TEMP_INTERVAL: u64 = 15;
const GPIO_PIN: u8 = 12;
const MIN_TEMP: f32 = 20.0; // Minimum temperature threshold
const MAX_TEMP: f32 = 50.0; // Maximum temperature threshold
const MIN_SPEED: f32 = 0.2; // Minimum fan speed (20% duty cycle)
const MAX_SPEED: f32 = 1.0; // Maximum fan speed (100% duty cycle)

fn get_temp() -> Result<f32, std::io::Error> {
    let contents = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let temp = contents.trim().parse::<f32>().unwrap_or(0.0) / 1000.0;
    Ok(temp)
}

fn calculate_fan_speed(temp: f32) -> f32 {
    if temp < MIN_TEMP {
        MIN_SPEED
    } else if temp > MAX_TEMP {
        MAX_SPEED
    } else {
        // Proportional control: Adjust the fan speed between MIN_SPEED and MAX_SPEED
        // based on the temperature.
        MIN_SPEED + (MAX_SPEED - MIN_SPEED) * (temp - MIN_TEMP) / (MAX_TEMP - MIN_TEMP)
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(GPIO_PIN)?.into_output();

    // Initialize PWM
    pin.set_pwm_frequency(50.0, 0.1)?;

    loop {
        let temp = get_temp()?;
        let fan_speed = calculate_fan_speed(temp);
        pin.set_pwm_frequency(50.0, fan_speed.into())?;

        println!(
            "Temperature: {:.2}°C, Fan Speed: {:.0}%",
            temp,
            fan_speed * 100.0
        );
        thread::sleep(time::Duration::from_secs(CHECK_TEMP_INTERVAL));
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

