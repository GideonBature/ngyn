use ngyn::prelude::*;
use serde::{Deserialize, Serialize};

use super::weather_gate::WeatherGate;
use super::weather_service::WeatherService;

#[derive(Dto, Serialize, Deserialize)]
pub struct WeatherDto {
    pub location: String,
    pub temperature: f32,
    pub humidity: f32,
}

#[controller(middlewares = [])]
pub struct WeatherController {
    weather_service: WeatherService,
}

#[routes]
impl WeatherController {
    #[get("/weather/<location>/<city>")]
    #[check(WeatherGate)]
    async fn get_location(&self, params: Param) -> String {
        self.weather_service
            .get_location_weather(params.get("location").unwrap().as_str())
            .await
    }

    #[post("/weather")]
    #[check(WeatherGate)]
    async fn post_location(&self, weather: WeatherDto) -> String {
        let location = weather.location;
        self.weather_service.get_location_weather(&location).await
    }
}
