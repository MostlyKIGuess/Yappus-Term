use gemini_ai::{Gemini, Models, TokenLen, Kind, decode_gemini};
use serde_json::{self, Value};
use std::path::PathBuf;

pub fn get_model_from_config(config_dir: &PathBuf) -> Models<'static> {
    let config_file = config_dir.join("config.json");
    
    if let Ok(config_data) = std::fs::read_to_string(&config_file) {
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_data) {
            if let Some(model_name) = config["model"].as_str() {
                return match model_name {
                    "GEMINI_1_5_PRO_002" => Models::GEMINI_1_5_PRO_002,
                    "GEMINI_1_5_PRO" => Models::GEMINI_1_5_PRO,
                    "GEMINI_1_5_FLASH_002" => Models::GEMINI_1_5_FLASH_002,
                    "GEMINI_1_5_FLASH_8B" => Models::GEMINI_1_5_FLASH_8B,
                    "GEMINI_1_0_PRO" => Models::GEMINI_1_0_PRO,
                    _ => Models::GEMINI_1_5_FLASH,
                };
            }
        }
    }
    
    Models::GEMINI_1_5_FLASH
}

pub fn get_gemini_response(query: &str, api_key: &str, config_dir: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    std::env::set_var("GEMINI_API_KEY", api_key);
    
    // source code for models lmao:
    // pub enum Models<'model> {
    //     GEMINI_1_5_FLASH,
    //     GEMINI_1_5_PRO_002,
    //     GEMINI_1_5_PRO,
    //     GEMINI_1_5_FLASH_002,
    //     GEMINI_1_5_FLASH_8B,
    //     GEMINI_1_0_PRO,
    //     Custom(&'model str),
    // }
    let model = get_model_from_config(config_dir);

    let response = Gemini::new()
        .env("GEMINI_API_KEY")
        .model(model)
        .no_memory()
        .kind(Kind::Text)
        .instruction("You are a terminal guru answer in short based on user Query.")  // an optional instruction
        .text(query)
        .max_token(TokenLen::Default)
        .build()
        .output();

        let responses = decode_gemini(&response)?;
        // dbg!(&response); this is for debugging purposes only
    
    let json_value: Value = serde_json::from_str(&serde_json::to_string(&responses)?)?;

    if let Some(text) = json_value["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        // Return the extracted text
        return Ok(text.to_string());
    }
    
    // Fallback if we can't extract the text
    Ok(serde_json::to_string(&responses)?)
}