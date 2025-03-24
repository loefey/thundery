# Thundery

Thundery is a command-line application that fetches and displays weather information from the OpenWeatherMap API.

![thundery](https://pub-772556e86c514a789d81677bd605749d.r2.dev/thundery.jpg)

Thundery's idea, structure and design is based off [Rainy](https://github.com/liveslol/rainy) but writen in Rust instead of Python making it 25% faster!!

![thunder vs rainy](https://pub-772556e86c514a789d81677bd605749d.r2.dev/thunderyandrainy.jpg)

## Installation

### From source:

1. Clone the repository:

   ```sh
   git clone https://github.com/loefey/thundery.git
   cd thundery
   ```

2. Build the project using Cargo:
   ```sh
   cargo build --release
   ```
**I suggest running** `cargo install --path .` **to install thundery instead of just building it**

### Package managers:

**AUR**:
Thundery is on the AUR! You can now use your favourite AUR helper to download Thundery.

https://aur.archlinux.org/packages/thundery

## Configuration

The application uses a configuration file located at `~/.config/thundery/thundery.toml` (or `C:\Users\<username>\AppData\Roaming\thundery\thundery.toml` on Windows). If the configuration file does not exist, a default one will be created.

### Configuration Options

- `api_key`: Your OpenWeatherMap API key.
- `city`: The city for which to fetch weather data.
- `units`: Units for temperature and wind speed (`metric` or `imperial`).
- `timeplus`: Hours to add to sunrise and sunset times.
- `timeminus`: Hours to subtract from sunrise and sunset times.
- `showcityname`: Whether to display the city name (`true` or `false`).
- `showdate`: Whether to display the current date (`true` or `false`).
- `timeformat`: Time format for sunrise and sunset times (`24` or `12`).
- `use_colors`: Enables and disabled text colors (`true` or `false`).

Example configuration:

```toml
api_key = "your_api_key"
city = "London"
units = "metric"
timeplus = 0
timeminus = 0
showcityname = true
showdate = true
timeformat = "24"
use_colors = false
```
