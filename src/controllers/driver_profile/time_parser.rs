use iced::widget::canvas::path::lyon_path::geom::euclid::num::Floor;

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
