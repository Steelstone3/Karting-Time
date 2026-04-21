use crate::models::driver::session_information::lap::Lap;

pub fn format_laptime(time_in_seconds: f32) -> String {
    if time_in_seconds == 0.0 {
        return "".to_string();
    }

    if time_in_seconds < 60.0 {
        return format!("{:.2}", time_in_seconds);
    }

    let total_milliseconds = (time_in_seconds.fract() * 100.0).round() as u32;

    let minutes = (time_in_seconds / 60.0).floor() as u32;
    let seconds = (time_in_seconds % 60.0) as u32;

    if minutes < 60 {
        return format!("{}:{:02}.{:02}", minutes, seconds, total_milliseconds);
    }

    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;

    format!(
        "{}:{:02}:{:02}.{:02}",
        hours, remaining_minutes, seconds, total_milliseconds
    )
}

pub fn convert_laps_to_string_laps(laptimes: Vec<Lap>) -> Vec<String> {
    let mut formatted_laptimes: Vec<String> = vec![];

    for laptime in laptimes {
        formatted_laptimes.push(format_laptime(laptime.time));
    }

    formatted_laptimes
}

// TODO test
pub fn convert_string_laps_to_laps(laptimes: Vec<String>) -> Vec<Lap> {
    let mut formatted_laptimes: Vec<Lap> = vec![];

    for lap in laptimes.iter().enumerate() {
        let trimmed_lap = lap.1.trim();

        if trimmed_lap.contains(':') {
            let parts: Vec<&str> = trimmed_lap.split(':').collect();

            let minutes = parts[0].parse::<f32>();

            if let Ok(minutes) = minutes {
                let seconds = parts[1].parse::<f32>();

                if let Ok(seconds) = seconds {
                    formatted_laptimes.push(Lap::new((lap.0 + 1) as u32, minutes * 60.0 + seconds))
                };
            }
        } else {
            let time = lap.1.trim().parse::<f32>().ok();
            formatted_laptimes.push(Lap::new((lap.0 + 1) as u32, time.unwrap_or_default()));
        }
    }

    formatted_laptimes
}

#[cfg(test)]
mod format_laptime_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, "1.00".to_string())]
    #[case(59.0, "59.00".to_string())]
    #[case(59.9, "59.90".to_string())]
    #[case(59.99, "59.99".to_string())]
    #[case(60.0, "1:00.00".to_string())]
    #[case(61.0, "1:01.00".to_string())]
    #[case(61.90, "1:01.90".to_string())]
    #[case(61.99, "1:01.99".to_string())]
    #[case(120.0, "2:00.00".to_string())]
    #[case(121.90, "2:01.90".to_string())]
    #[case(121.99, "2:01.99".to_string())]
    #[case(3600.0, "1:00:00.00".to_string())]
    #[case(3601.0, "1:00:01.00".to_string())]
    #[case(3601.9, "1:00:01.90".to_string())]
    #[case(3601.99, "1:00:01.99".to_string())]
    #[case(3660.0, "1:01:00.00".to_string())]
    #[case(3660.0, "1:01:00.00".to_string())]
    #[case(3661.0, "1:01:01.00".to_string())]
    #[case(3661.9, "1:01:01.90".to_string())]
    #[case(3661.99, "1:01:01.99".to_string())]
    #[case(8661.99, "2:24:21.99".to_string())]
    fn form_laptime(#[case] time_in_seconds: f32, #[case] expected_formatted_time: String) {
        // When
        let formatted_time = format_laptime(time_in_seconds);

        // Then
        pretty_assertions::assert_eq!(expected_formatted_time, formatted_time);
    }

    #[test]
    fn be_able_to_convert_laps_to_string_laps() {
        // Given
        let laps = vec![
            Lap::new(1, 120.6),
            Lap::new(2, 120.7),
            Lap::new(3, 120.8),
            Lap::new(4, 120.9),
        ];
        let expected_string_laps = vec![
            "2:00.60".to_string(),
            "2:00.70".to_string(),
            "2:00.80".to_string(),
            "2:00.90".to_string(),
        ];

        // When
        let actual_laps = convert_laps_to_string_laps(laps);

        // Then
        pretty_assertions::assert_eq!(expected_string_laps, actual_laps);
    }

    #[test]
    fn be_able_to_convert_string_laps_to_laps() {
        // Given
        let string_laps = vec![
            "2:00.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string(),
        ];
        let expected_laps = vec![
            Lap::new(1, 120.6),
            Lap::new(2, 120.7),
            Lap::new(3, 120.8),
            Lap::new(4, 120.9),
        ];

        // When
        let actual_laps = convert_string_laps_to_laps(string_laps);

        // Then
        pretty_assertions::assert_eq!(expected_laps, actual_laps);
    }
}
