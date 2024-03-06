use reqwasm::http;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Uuid;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, Request, RequestInit, RequestMode, Response};
use yew::events::SubmitEvent;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let onclick = Callback::from(|_| {
        let document = window().unwrap().document().unwrap();

        wasm_bindgen_futures::spawn_local(async move {
        });
    });

    html! {
        <main>
            <h1 style="text-align: center; margin: 10; padding: 0;">{ "BlackSignal" }</h1>
            <div>
                <input type="text" id="username" placeholder={"Username"} />
                <input type="password" id="password" placeholder={"Password"} />
                <button onclick={onclick}>{"Login"}</button>
            </div>
        </main>
    }
}

pub async fn login(login: &str) -> Result<String, String> {
    let response = match http::Request::post("http://0.0.0.0:8080/create_login")
        .header("Content-Type", "application/json")
        .body(login)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };
    Ok("cool".to_string())
}
