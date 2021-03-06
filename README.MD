# Raven Weather

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)

## Overview

RavenWeather is a DB Actualizer and RESTful APP using OpenWeatherMap APIs made in Rust.

### Stack

* Rust 2018
* Postgresql Database
* Libraries :
    * Diesel 
    * dotenv
    * Reqwest
    * Axum
    * Serde & serde_json

### Env. Variables

* DATABASE_URL - postgres://[username]:[password]@localhost/[database-name]
* KEY - OpenWeatherMap appKey
* LATITUDE 
* LONGITUDE
* CORS Origin for Access-Control-Allow-Origin

### Warning

Data sent as floating point needs to be divided by 100.0 at reception.

## Road Map

* Data : <br>
[X] - Current Weather <br>
[X] - Air Pollution <br>
[_] - Forecast <br>
[_] - Handle Rejection so it can send its error in case or 401 or 429 error codes.
* Database : <br>
[_] - Modify floating types in table to integer.
* Security : <br>
[X] - Implement CORS Headers logic.
[/] - Implement a Key to accept or reject REST requests. (To be resettled following migration to axum.)
* Local Implementation : <br>
[_] - Implement Feature to connect to a local weather station.

## License

```
DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                   Version 2, December 2004

Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.

           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
  TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

 0. You just DO WHAT THE FUCK YOU WANT TO.
```



