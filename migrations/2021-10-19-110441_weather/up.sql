CREATE TABLE weather (
    id SERIAL PRIMARY KEY,
    dt BIGINT NOT NULL,
    wind_speed REAL NOT NULL,
    wind_direction SMALLINT NOT NULL,
    temp REAL NOT NULL,
    feels_like REAL NOT NULL,
    temp_min REAL NOT NULL,
    temp_max REAL NOT NULL,
    pressure SMALLINT NOT NULL,
    humidity SMALLINT NOT NULL,
    weather_id SMALLINT NOT NULL
)