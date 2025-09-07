use iced::widget::canvas::path::lyon_path::geom::euclid::num::Floor;

// TODO AH consider hours
pub fn format_laptime(time_in_seconds: f32) -> String {
    if time_in_seconds < 60.0 {
        return format!("{time_in_seconds:.2}");
    }

    let minutes = (time_in_seconds as u32 / 60).floor();
    let seconds = time_in_seconds - (minutes * 60) as f32;

    // add leading zero
    if seconds < 10.0 {
        let mut formatted_seconds = format!("{seconds:02.2}");
        formatted_seconds.insert(0, '0');

        format!("{minutes}:{formatted_seconds}")
    } else {
        format!("{minutes}:{seconds:02.2}")
    }
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
    fn form_laptime(#[case] time_in_seconds: f32, #[case] expected_formatted_time: String) {
        // When
        let formatted_time = format_laptime(time_in_seconds);

        // Then
        assert_eq!(expected_formatted_time, formatted_time);
    }
}
