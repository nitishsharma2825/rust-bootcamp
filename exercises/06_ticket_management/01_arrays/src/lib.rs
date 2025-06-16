// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

#[derive(Debug)]
pub struct WeekTemperatures {
    // TODO
    temp: [WeekTemperature; 7],
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WeekTemperature {
    day: Option<Weekday>,
    temperature: Option<i32>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures {
            temp: [
                WeekTemperature {
                    day: Some(Weekday::Monday),
                    temperature: None,
                },
                WeekTemperature {
                    day: Some(Weekday::Tuesday),
                    temperature: None,
                },
                WeekTemperature {
                    day: Some(Weekday::Wednesday),
                    temperature: None,
                },
                WeekTemperature {
                    day: Some(Weekday::Thursday),
                    temperature: None,
                },
                WeekTemperature {
                    day: Some(Weekday::Friday),
                    temperature: None,
                },
                WeekTemperature {
                    day: Some(Weekday::Saturday),
                    temperature: None,
                },
                WeekTemperature {
                    day: Some(Weekday::Sunday),
                    temperature: None,
                },
            ],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        for week_temp in &self.temp {
            println!("Checking day: {:?}", week_temp.day);
            if week_temp.day == Some(day) {
                return week_temp.temperature;
            }
        }
        None
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        for week_temp in &mut self.temp {
            if week_temp.day == Some(day) {
                week_temp.temperature = Some(temperature);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
