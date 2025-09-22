#[tracing::instrument]
pub fn weather_to_emoji(desc: &str) -> String {
    let desc_lower = desc.to_lowercase();

    let patterns = vec![
        ("замерзающий", "🧊"),
        ("переменная", "☀️"),
        ("гроза", "⛈️"),
        ("дождь", "🌧️"),
        ("снег", "❄️"),
        ("слякоть", "🌨️"),
        ("град", "🌨️"),
        ("туман", "🌫️"),
        ("дымка", "🌫️"),
        ("ясно", "☀️"),
        ("облачно", "☁️"),
        ("пасмурно", "🌥️"),
    ];

    let mut ans = String::new();

    for (pattern, emoji) in &patterns {
        if desc_lower.contains(pattern) {
            ans += emoji;
        }
    }

    ans
}

#[cfg(test)]
mod weather_tests {
    use crate::handlers::formats_handler::weather_to_emoji;

    #[test]
    fn test_weather_emoji_one_condition_lowercase() {
        assert_eq!(weather_to_emoji("гроза").as_str(), "⛈️");
        assert_eq!(weather_to_emoji("дождь").as_str(), "🌧️");
        assert_eq!(weather_to_emoji("снег").as_str(), "❄️");
        assert_eq!(weather_to_emoji("слякоть").as_str(), "🌨️");
        assert_eq!(weather_to_emoji("град").as_str(), "🌨️");
        assert_eq!(weather_to_emoji("туман").as_str(), "🌫️");
        assert_eq!(weather_to_emoji("дымка").as_str(), "🌫️");
        assert_eq!(weather_to_emoji("ясно").as_str(), "☀️");
        assert_eq!(weather_to_emoji("облачно").as_str(), "☁️");
        assert_eq!(weather_to_emoji("пасмурно").as_str(), "🌥️");
    }

    #[test]
    fn test_weather_emoji_one_condition_uppercase() {
        assert_eq!(weather_to_emoji("Гроза"), "⛈️");
        assert_eq!(weather_to_emoji("Дождь"), "🌧️");
        assert_eq!(weather_to_emoji("Снег"), "❄️");
        assert_eq!(weather_to_emoji("Слякоть"), "🌨️");
        assert_eq!(weather_to_emoji("Град"), "🌨️");
        assert_eq!(weather_to_emoji("Туман"), "🌫️");
        assert_eq!(weather_to_emoji("Дымка"), "🌫️");
        assert_eq!(weather_to_emoji("Ясно"), "☀️");
        assert_eq!(weather_to_emoji("Облачно"), "☁️");
        assert_eq!(weather_to_emoji("Пасмурно"), "🌥️");
    }

    #[test]
    fn test_weather_emoji_one_condition_dot() {
        assert_eq!(weather_to_emoji("гроза."), "⛈️");
        assert_eq!(weather_to_emoji("дождь."), "🌧️");
        assert_eq!(weather_to_emoji("снег."), "❄️");
        assert_eq!(weather_to_emoji("слякоть."), "🌨️");
        assert_eq!(weather_to_emoji("град."), "🌨️");
        assert_eq!(weather_to_emoji("туман."), "🌫️");
        assert_eq!(weather_to_emoji("дымка."), "🌫️");
        assert_eq!(weather_to_emoji("ясно."), "☀️");
        assert_eq!(weather_to_emoji("облачно."), "☁️");
        assert_eq!(weather_to_emoji("пасмурно."), "🌥️");
    }

    #[test]
    fn test_weather_emoji_several_conditions() {
        assert_eq!(weather_to_emoji("пасмурно, снег"), "❄️🌥️");
        assert_eq!(weather_to_emoji("снег, пасмурно"), "❄️🌥️");
        assert_eq!(weather_to_emoji("ясно, замерзающий дождь"), "🧊🌧️☀️");
        assert_eq!(weather_to_emoji("переменная облачность").as_str(), "☀️☁️");
    }
}
