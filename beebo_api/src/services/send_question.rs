use reqwest::{Client, Error};

use crate::{ai::question, model::LlamaResponse};

pub async fn ask_beebo(
    client: &Client,
    llama_url: &str,
    llama_model: &str,
) -> Result<String, Error> {
    let client_req_body = question(llama_model);

    let res = client
        .post(llama_url)
        .json(&client_req_body)
        .send()
        .await?
        .error_for_status()?;

    let parse_res: LlamaResponse = res.json().await?;

    Ok(format!(
        "{}\n",
        parse_res.choices[0].message.content.clone()
    ))
}
