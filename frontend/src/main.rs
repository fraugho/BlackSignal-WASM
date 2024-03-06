mod login;
use login::LoginPage;

fn main() {
    yew::Renderer::<LoginPage>::new().render();
}
