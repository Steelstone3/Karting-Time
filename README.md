# Karting Time

Project to document all race times in a profile

## Running Karting Time

> cd ~/Karting-Time
>
> cargo build
>
> cargo run

## Testing Karting Time

> cd ~/Karting-Time
>
> cargo test

## Importing Files

To import races use the following formats

### TOML

#### Race

```toml
laptimes = [
    "48.667",
    "43.434",
    "42.564",
    "33.39",
    "34.011",
    "34.915",
    "31.127",
    "33.36",
    "30.971",
    "30.875",
    "32.708",
    "31.274",
    "36.726",
    "32.192",
    "30.264",
    "31.648",
    "30.544",
    "30.402",
    "31.014",
    "30.696",
    "30.784",
    "32.554",
    "31.242",
    "30.626",
    "30.914",
    "29.7",
    "29.922",
    "30.833",
    "30.624",
    "30.041",
    "31.061",
    "30.488",
    "31.239",
    "33.217",
    "29.931",
]
day = 18
month = 11
year = 2023
race_position = 2
track_name = "Rochdale"
session_id = 1

# Optional
track_conditions = "Indoor"
session_type = "Race"
car_used = "Sodi GT5"
championship = "Who Is Faster Round 1"
notes = "Private race event"
```

#### Driver Profile

```toml
[driver_profile]
name = "Jack Jackson"

[[driver_profile.races]]
laptimes = [
    "88.886",
    "53.356",
    "52.117",
    "52.437",
    "52.094",
    "52.436",
    "51.218",
    "51.517",
    "51.432",
    "51.251",
    "92.104",
    "71.307",
    "52.458",
    "51.948",
    "51.552",
    "52.113",
    "51.912",
    "87.159",
    "55.39",
    "52.187",
    "50.781",
    "51.99",
]
day = 23
month = 7
year = 2025
track_name = "Three Sisters"
session_id = 1
race_position = 8

# Optional
session_type = "Qualifying"
track_conditions = "Dry"
car_used = "Sodi RT10"
championship = "Team Enduro Round 5 2025"
notes = "Kart 29"

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
day = 24
month = 3
year = 2025
track_name = "Three Sisters"
session_id = 2
race_position = 6

# No Optional Fields Used
```

## Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>
