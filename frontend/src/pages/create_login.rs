use gloo_net::http;
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

#[function_component(CreateLoginPage)]
pub fn create_login_page() -> Html {
    let onclick = Callback::from(|_| {
        let document = window().unwrap().document().unwrap();

        let username = document
            .get_element_by_id("username")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let password = document
            .get_element_by_id("password")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let data = LoginForm { username, password };

        let serialized_data = serde_json::to_string(&data).unwrap_or_else(|err| {
            web_sys::console::log_1(&format!("Error serializing data: {:?}", err).into());
            "".into()
        });

        println!("{}", serialized_data);

        wasm_bindgen_futures::spawn_local(async move {
            create_login(&serialized_data).await;
        });
    });

    html! {
        <main>
            <h1 style="text-align: center; margin: 10; padding: 0;">{ "BlackSignal" }</h1>
            <div>
                <h2>{"Create Login"}</h2>
                <input type="text" id="username" placeholder={"Username"} />
                <input type="password" id="password" placeholder={"Password"} />
                <button onclick={onclick}>{"Login"}</button>
            </div>
        </main>
    }
}

pub async fn create_login(login: &str) -> Result<String, String> {
    let request = match http::Request::post("http://0.0.0.0:8080/create_login")
        .header("Content-Type", "application/json")
        .body(login)
    {
        Ok(ok) => ok,
        Err(e) => return Err("Failed to make request".to_string()),
    };

    match request.send().await {
        Ok(res) => res,
        Err(_) => return Err("Failed to send request".to_string()),
    };
    Ok("cool".to_string())
}
