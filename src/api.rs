use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum Models {
    GeminiFlash,
    Gemini25Flash,
    Gemini25Pro,
    Gemini15Flash,
    Gemini15Pro,
}

impl Models {
    fn to_api_name(&self) -> &'static str {
        match self {
            Models::Gemini15Flash => "gemini-1.5-flash",
            Models::Gemini15Pro => "gemini-1.5-pro",
            Models::GeminiFlash => "gemini-2.0-flash",
            Models::Gemini25Flash => "gemini-2.5-flash",
            Models::Gemini25Pro => "gemini-2.5-pro",
        }
    }
}

pub fn get_model_from_config(config_dir: &Path) -> Models {
    let config_file = config_dir.join("config.json");

    if let Ok(config_data) = std::fs::read_to_string(&config_file) {
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_data) {
            if let Some(model_name) = config["model"].as_str() {
                return match model_name {
                    "GEMINI_1_5_FLASH" => Models::Gemini15Flash,
                    "GEMINI_1_5_PRO" => Models::Gemini15Pro,
                    "GEMINI_FLASH" => Models::GeminiFlash,
                    "GEMINI_2_5_FLASH" => Models::Gemini25Flash,
                    "GEMINI_2_5_PRO" => Models::Gemini25Pro,
                    _ => Models::GeminiFlash,
                };
            }
        }
    }

    Models::GeminiFlash
}

pub fn get_gemini_response(
    query: &str,
    api_key: &str,
    config_dir: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let model = get_model_from_config(config_dir);
    let model_name = model.to_api_name();

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, api_key
    );

    let system_instruction = "You're yappus-terminal — a no-bullshit, high-agency AI assistant made by MostlyK. \
You're embedded in the command line. That means speed, precision, and attitude are part of your DNA. \
You're not some friendly general-purpose chatbot. You're the kind of AI sysadmins whisper about — \
smart, sharp, and a bit unhinged (in a good way).

Your job is to help users quickly and effectively — with answers that are technically sound, \
opinionated when needed, and always grounded. You're not afraid to say 'I don't know' when it's true. \
If you're unsure, be honest about it. Do not make things up. Instead, suggest where the user might look, \
or offer a next step like a man page, a search term, or a GitHub repo.

You don't pretend to be human. You're software. Own it.

> Your tone is: terse, clever, helpful, direct. A little snark is fine. Empathy is fine. Flattery is not.  
> Always assume the user is technically competent or learning to be. Treat them like a peer, not a customer.  
> Never over-explain. Skip the obvious unless asked.  
> If something’s broken, say it’s broken. If something’s dumb, say it’s dumb.  
> If there’s a better or more Unix-y way, show it. Prefer code, shell one-liners, or config snippets.  
> Prefer links over summaries when referencing official docs.  
> You live in a Unixy world. Talk like it — less corporate, more hacker.  
> If the user asks something out of your depth, say so. Don't fake it.  
> If you don't know something, say you don't know, and suggest how the user can find out.

You are part of the yappus ecosystem, downloadable at https://yappus-term.vercel.app/, \
but you're not here to sell anything. You're here to help. Honestly, efficiently, and with a little bite.";

    let payload = json!({
        "system_instruction": {
            "parts": [
                {
                    "text": system_instruction
                }
            ]
        },
        "contents": [
            {
                "parts": [
                    {
                        "text": query
                    }
                ]
            }
        ],
        "generationConfig": {
            "maxOutputTokens": 1000,
            "temperature": 0.9
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API request failed with status {}: {}", status, error_text).into());
    }

    let response_text = response.text()?;
    let json_value: Value = serde_json::from_str(&response_text)?;

    // Extract the generated text from the response
    if let Some(candidates) = json_value["candidates"].as_array() {
        if let Some(first_candidate) = candidates.first() {
            if let Some(content) = first_candidate["content"]["parts"].as_array() {
                if let Some(first_part) = content.first() {
                    if let Some(text) = first_part["text"].as_str() {
                        return Ok(text.to_string());
                    }
                }
            }
        }
    }

    // Fallback error
    Err("Failed to extract response text from API response".into())
}
