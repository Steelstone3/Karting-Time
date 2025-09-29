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

pub fn format_laptimes(laptimes: Vec<Lap>) -> Vec<String> {
    let mut formatted_laptimes: Vec<String> = vec![];

    for laptime in laptimes {
        formatted_laptimes.push(format_laptime(laptime.time));
    }

    formatted_laptimes
}

#[cfg(test)]
mod format_laptime_should {
    use crate::controllers::driver_profile::time_parser::format_laptime;
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
}
