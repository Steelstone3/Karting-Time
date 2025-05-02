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
car_used = "Biz Evo 3"

[race_information.date]
day = 27
month = 1
year = 2025
```

##### Driver Profile

```toml
[driver_profile]
name = "Jack Jackson"

[[driver_profile.races]]
laptimes = [
    "85.984",
    "53.024",
    "54.996",
    "52.514",
    "51.889",
    "52.492",
    "51.887",
    "53.747",
    "53.628",
    "52.208",
    "51.984",
    "52.011",
    "52.179",
    "52.115",
    "51.995",
    "52.456",
    "56.41",
    "52.672",
    "52.32",
    "52.784",
    "52.555",
    "58.763",
]

[driver_profile.races.race_information]
track_name = "Three Sisters"
session_id = 1
race_position = 7
car_used = "Sodi RT10"

[driver_profile.races.race_information.date]
day = 24
month = 3
year = 2025

[[driver_profile.races]]
laptimes = [
    "75.821",
    "53.664",
    "53.965",
    "53.838",
    "52.649",
    "52.463",
    "51.665",
    "51.775",
    "52.162",
    "56.097",
    "51.735",
    "52.64",
    "51.847",
    "52.84",
    "51.479",
    "51.477",
    "51.448",
    "51.817",
    "51.863",
    "52.123",
    "53.488",
    "52.455",
    "52.261",
]

[driver_profile.races.race_information]
track_name = "Three Sisters"
session_id = 2
race_position = 6
car_used = "Sodi RT10"

[driver_profile.races.race_information.date]
day = 24
month = 3
year = 2025
```

### Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>
