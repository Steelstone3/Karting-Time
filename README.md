# Karting Time

Project to document all race times in a profile

## Running Karting Time

> cd ~/Karting-Time
>
> cargo build
>
> cargo run

### Testing Karting Time

> cd ~/Karting-Time
>
> cargo test

### Importing Files

To import races use the following formats

#### TOML

##### Race

```toml
laptimes = [
    53.59,
    35.68,
    35.34,
    34.63,
    34.65,
]

[race_information]
track_name = "Trafford Park"
session_id = 3
race_position = 6

[race_information.date]
day = 27
month = 1
year = 2025
```

##### Driver Profile

```toml
[driver_profile]
driver_id = 12
name = "Jack Jackson"

[[driver_profile.races]]
track_name = "Three Sisters"
date = "31/12/2025"
session_id = 1
position = 1

[[driver_profile.races.laptimes]]
lap_number = 1
time = 50.4

[[driver_profile.races.laptimes]]
lap_number = 2
time = 55.5



[[driver_profile.races]]
track_name = "Llandow"
date = "01/01/2025"
session_id = 1
position = 1

[[driver_profile.races.laptimes]]
lap_number = 1
time = 50.4

[[driver_profile.races.laptimes]]
lap_number = 2
time = 55.5
```

### Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>
