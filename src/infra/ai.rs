use crate::models::types::Res;

pub trait AiProvider {
    async fn process(weathe: String) -> Res<String>;
}