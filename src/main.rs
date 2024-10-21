use dotenv;
use std::{env, io};
use reqwest::blocking;
use serde_json::{json, Value};
use reqwest::header::{USER_AGENT, CONTENT_TYPE, AUTHORIZATION};

fn main() {

    dotenv::from_path("./.env").expect("no such file buddy");
    let api_key = env::var("OPENAI_API_KEY").expect("Api Key missing bro");
    let open_ai_url = env::var("COMPLETIONS_ENDPOINT").expect("endpoint uri missing bro");

    let mut user_prompt: String = String::new();

     match io::stdin().read_line(&mut user_prompt) {
        Err(error) => panic!("{error}"),
        _ => {},
    }

    let formatted_question_to_open_ai = json!({
    "model": "gpt-3.5-turbo",
    "messages": [
        {
            "role": "system",
            "content": "You are a helpful assistant."
         },
         {
             "role": "user",
             "content": &user_prompt
         }
      ]
    });

    let client = blocking::Client::new();
    let response_from_open_ai = client.post(open_ai_url)
        .header(USER_AGENT, "request")
        .header(CONTENT_TYPE,"application/json")
        .header(AUTHORIZATION, "Bearer ".to_string() + &api_key)
        .json(&formatted_question_to_open_ai)
        .send();

    let (s, err) = match response_from_open_ai {
        Ok(response) => {
            let status = response.status();
            let open_ai_answer = response.text().expect("Something went wrong with open ai");
            if status.is_success()
            {
                let data: Value = serde_json::from_str(&open_ai_answer).expect("Invalid response");
                let answer = &data["choices"][0]["message"]["content"];
                //(serde_json::from_str(open_ai_answer), String::from(""))
                (format!("ChatGpt Answer:\n {answer}"), "".to_string())

            } else{

                (String::from(""), format!("{status} : {open_ai_answer}"))
            }

        }
        Err(e) => {
            eprintln!("Error: {:#?}", e);
            (String::from(""), String::from(""))
        }
    };

    println!("{:#?} {}", s, err);

}
