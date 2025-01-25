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

#### JSON

##### Single JSON

```json
"track_name": "Three Sisters",
"date": "31/12/2025",
"session_id": 1,
"laptimes": [
    {
        "lap_number": 1,
        "time": 50.4
    },
    {
        "lap_number": 2,
        "time": 54.6
    },
    {
        "lap_number": 3,
        "time": 52.8
    },
    {
        "lap_number": 4,
        "time": 55.1
    },
]
```

##### Multiple JSON

```json
"driver_profile": {
    "driver_id": 12,
    "name": "Jack Jackson",
    "races": [{
        "track_name": "Three Sisters",
        "date": "12/12/2025",
        "session_id": 1,
        "laptimes": [
            {
                "lap_number": 1,
                "time": 50.4
            },
            {
                "lap_number": 2,
                "time": 55.5
            }
        ]
    }]
}
```

#### TOML

##### Single TOML

```toml
[races]
track_name = "Three Sisters"
date = "31/12/25"
session_id = 1

[[races.laptimes]]
lap_number = 1
time = 50.4

[[races.laptimes]]
lap_number = 1
time = 55.4
```

##### Multiple

```toml
[driver_profile]
driver_id = 12
name = "Jack Jackson"

[[driver_profile.races]]
track_name = "Three Sisters"
date = "31/12/2025"
session_id = 1

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
