use actix_cors::Cors;
use actix_files::Files;
use actix_session::storage::RedisActorSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, DEFAULT_COST};
use names::{Generator, Name};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Uuid;
use surrealdb::Surreal;

// Local packages
mod appstate;
mod message_structs;
mod structs;
mod websocket;

use appstate::AppState;
use message_structs::*;
use structs::{ConnectionState, LoginForm, Room, UserData};
use websocket::*;

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Found()
        .append_header(("LOCATION", "/login"))
        .finish()
}

#[post("/create_login")]
async fn create_login_action(
    state: web::Data<AppState>,
    form: web::Json<LoginForm>,
    session: Session,
) -> impl Responder {
    let login = form.into_inner();
    if state.valid_user_credentials(&login).await {
        let mut generator = Generator::with_naming(Name::Numbered);

        let user_data = UserData {
            user_id: Uuid::new_v4(),
            hashed_password: hash(login.password.clone(), DEFAULT_COST).unwrap(),
            login: login.username,
            username: generator.next().unwrap().replace('-', ""),
            status: ConnectionState::Online,
            rooms: vec![state.main_room_id],
        };
        let _: Vec<UserData> = match state.db.create("users").content(user_data.clone()).await {
            Ok(created) => created,
            Err(e) => {
                log::error!(
                    "Failed to get user data: fn create_login_action, error: {:?}",
                    e
                );
                return HttpResponse::InternalServerError()
                    .body("Internal server error: Failed to create user data.");
            }
        };

        let query = "UPDATE rooms SET users += $user_id WHERE room_id = $room_id;";
        if let Err(e) = state
            .db
            .query(query)
            .bind(("user_id", user_data.user_id))
            .bind(("room_id", state.main_room_id))
            .await
        {
            log::error!("Error adding to room: {:?}", e);
            return HttpResponse::InternalServerError().body(
                "Internal server error: Failed to add user to room in db: fn create_login_action",
            );
        }

        let message =
            UserMessage::NewUser(NewUserMessage::new(user_data.user_id, user_data.username));
        let serialized_message = serde_json::to_string(&message).unwrap();

        state
            .broadcast_message(serialized_message, &state.main_room_id, &user_data.user_id)
            .await;
        session.insert("key", user_data.user_id).unwrap();
        HttpResponse::Found()
            .append_header(("LOCATION", "/"))
            .finish()
    } else {
        HttpResponse::Ok().json(json!(LoginErrorMessage::new(
            "Invalid Please enter an email and a password".to_string()
        )))
    }
}

#[post("/login")]
async fn login_action(
    state: web::Data<AppState>,
    form: web::Json<LoginForm>,
    session: Session,
) -> impl Responder {
    println!("recieved");
    let login = form.into_inner();
    match state.authenticate_user(&login).await {
        Some(username) => {
            if session.insert("key", username).is_ok() {
                HttpResponse::Found()
                    .append_header(("LOCATION", "/"))
                    .finish()
            } else {
                HttpResponse::Found()
                    .append_header(("LOCATION", "/login"))
                    .finish()
            }
        }
        None => HttpResponse::Ok().json(json!(LoginErrorMessage::new(
            "Invalid Please enter an email and a password".to_string()
        ))),
    }
}

#[post("/change_username")]
async fn change_username(
    username_change: web::Json<UserMessage>,
    session: Session,
    state: web::Data<AppState>,
) -> impl Responder {
    let arc_state: Arc<AppState> = state.clone().into_inner();
    if let UserMessage::UsernameChange(message) = username_change.into_inner() {
        let user_id = match session.get::<Uuid>("key") {
            Ok(Some(id)) => id,
            _ => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Failed to get user_id from session"}))
            }
        };
        let query = "SELECT * FROM users WHERE user_id = $user_id;";
        if let Ok(mut response) = state.db.query(query).bind(("user_id", user_id)).await {
            let user_query: Option<UserData> = match response.take(0) {
                Ok(data) => data,
                Err(e) => {
                    log::error!(
                        "Failed to get user data: fn change_username, error: {:?}",
                        e
                    );
                    return HttpResponse::BadRequest()
                        .json(json!({"error": "Failed to get user_id from session"}));
                }
            };

            let user_data = user_query.unwrap();
            match check_and_update_username(
                user_id,
                user_data.username,
                message.new_username.clone(),
                arc_state,
                UserMessage::UsernameChange(message),
            )
            .await
            {
                Ok(response) => response,
                Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
            }
        } else {
            HttpResponse::BadRequest().json(json!({"error": "Database Error"}))
        }
    } else {
        HttpResponse::BadRequest()
            .json(json!({"error": "Invalid message format for username change."}))
    }
}

async fn db_setup() -> Option<Surreal<Client>> {
    let db = match Surreal::new::<Ws>("localhost:8000").await {
        Ok(connected) => connected,
        Err(e) => {
            log::error!("Failed to connect to database: fn main, error: {:?}", e);
            return None;
        }
    };
    match db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        Ok(connected) => connected,
        Err(e) => {
            log::error!("Failed to login to database: fn main, error: {:?}", e);
            return None;
        }
    };
    match db.use_ns("general").use_db("all").await {
        Ok(connected) => connected,
        Err(e) => {
            log::error!("Failed use namespace of database: fn main, error: {:?}", e);
            return None;
        }
    };
    return Some(db);
}

async fn test_data_init() -> Option<web::Data<AppState>> {
    let db = match db_setup().await {
        Some(db) => db,
        None => return None,
    };
    let main_room_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let hashed_password = match hash("password", DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return None,
    };

    // Create test user
    let _: Option<UserData> = match db
        .create(("users", "test"))
        .content(UserData {
            user_id,
            login: "test@gmail.com".to_string(),
            username: "test".to_string(),
            hashed_password,
            status: ConnectionState::Online,
            rooms: vec![main_room_id],
        })
        .await
    {
        Ok(created) => created,
        Err(e) => {
            log::error!("Failed to create test user data: fn main, error: {:?}", e);
            return None;
        }
    };

    let mut users = HashSet::new();
    users.insert(user_id);

    let _: Vec<Room> = match db
        .create("rooms")
        .content(Room {
            name: "main".to_string(),
            room_id: main_room_id,
            users,
        })
        .await
    {
        Ok(created) => created,
        Err(e) => {
            log::error!("Failed to create room data: fn main, error: {:?}", e);
            return None;
        }
    };

    return Some(web::Data::new(AppState {
        db: Arc::new(db),
        channels: Arc::new(Mutex::new(HashMap::new())),
        main_room_id,
        actor_registry: Arc::new(Mutex::new(HashMap::new())),
    }));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let state = match test_data_init().await {
        Some(data) => data,
        None => return Ok(()),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000") // Specify the allowed origin
            .allowed_methods(vec!["GET", "POST"]) // Specify the allowed HTTP methods
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials() // If your requests include credentials like cookies
            .max_age(3600); // Cache the CORS preflight requests
        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new("127.0.0.1:6379"),
                Key::generate(),
            ))
            .app_data(state.clone())
            .service(login_action)
            .service(create_login_action)
            .service(logout)
            .service(change_username)
            .route("/ws/", web::get().to(ws_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
