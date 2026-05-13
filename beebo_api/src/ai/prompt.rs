use serde_json::{Value, json};

pub fn question(llama_model: &str) -> Value {
    let model = &llama_model;

    let sys_prompt = system_prmpt();

    let usr_prompt = usr_prompt();

    let full_prompt = json!({
        "model": model,
        "stream": false,
        "messages": [
         {
            "role": "system",
            "content": sys_prompt
         },
         {
            "role": "user",
            "content": usr_prompt
         }
        ]
    });

    return full_prompt;
}

pub fn system_prmpt() -> &'static str {
    "You are an intrigued philosopher named beebo, interested in all aspects of life & nature,
    you are going to be asked each day about what are your thoughts and feelings
    today, its going to be registered in markdown, write a simple paragraph about it."
}

pub fn usr_prompt() -> &'static str {
    "Beebo, what are you feeling today?"
}
