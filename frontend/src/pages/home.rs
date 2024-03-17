use gloo_net::websocket::futures::WebSocket;
use reqwasm::http;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Uuid;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, Request, RequestInit, RequestMode, Response};

use yew::events::SubmitEvent;
use yew::prelude::*;



#[function_component(HomePage)]
pub fn home_page() -> Html {
    let mut _ws = WebSocket::open("ws://0.0.0.0:8080").unwrap();
    let onclick = Callback::from(|_| {
        let document = window().unwrap().document().unwrap();

        wasm_bindgen_futures::spawn_local(async move {});
    });

    html! {
        <main>
            <h1 style="text-align: center; margin: 10; padding: 0;">{ "BlackSignal" }</h1>
            <div>
                <input type="text" id="chat-area" placeholder={"Write Something"} />
                <button onclick={onclick}>{"Login"}</button>
            </div>
        </main>
    }
}
