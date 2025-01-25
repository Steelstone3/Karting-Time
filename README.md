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

##### Multiple Driver Profiles (TODO)

```toml

```

### Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>
