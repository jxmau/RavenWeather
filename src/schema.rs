table! {
    air (id) {
        id -> Int4,
        dt -> Int8,
        aqi -> Int2,
        co -> Float4,
        no -> Float4,
        no2 -> Float4,
        o3 -> Float4,
        so2 -> Float4,
        pm2_5 -> Float4,
        pm10 -> Float4,
        nh3 -> Float4,
    }
}

table! {
    weather (id) {
        id -> Int4,
        dt -> Int8,
        wind_speed -> Float4,
        wind_direction -> Int2,
        temp -> Float4,
        feels_like -> Float4,
        temp_min -> Float4,
        temp_max -> Float4,
        pressure -> Int2,
        humidity -> Int2,
        weather_id -> Int2,
    }
}

allow_tables_to_appear_in_same_query!(
    air,
    weather,
);
