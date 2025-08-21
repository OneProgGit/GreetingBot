#[derive(Debug, Clone)]
pub struct WeatherModel {
    pub temp_c: String,
    pub feels_like_c: String,
    pub wind_speed_kmph: String,
    pub min_temp_c: String,
    pub max_temp_c: String,
    pub status: String,
}
