mod pages {
    pub mod create_login;
    pub mod home;
    pub mod login;
}

use yew::prelude::*;
use yew_router::prelude::*;

use pages::create_login::CreateLoginPage;
use pages::home::HomePage;
use pages::login::LoginPage;

// Define your application routes
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/create_login")]
    CreateLogin,
}

// Define the switch function to render components based on the route
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::CreateLogin => html! { <CreateLoginPage /> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
