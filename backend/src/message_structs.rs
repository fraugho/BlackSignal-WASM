use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Uuid;

// UserInfo Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub ws_id: Uuid,
    pub username: String,
}

impl UserInfo {
    pub fn new(user_id: Uuid, ws_id: Uuid, username: String) -> Self {
        UserInfo { user_id, ws_id, username }
    }
}

// InitMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct InitMessage {
    pub user_id: Uuid,
    pub ws_id: Uuid,
    pub username: String,
    pub user_map: HashMap<Uuid, String>,
}

impl InitMessage {
    pub fn new(user_id: Uuid, ws_id: Uuid, username: String, user_map: HashMap<Uuid, String>) -> Self {
        InitMessage { user_id, ws_id, username, user_map }
    }
}

// Message Enum
#[derive(Serialize, Deserialize, Clone)]
pub enum UserMessage {
    Basic(BasicMessage),
    TSBasic(TSBasicMessage),
    Image(ImageMessage),
    Notification(NotificationMessage),
    Typing(TypingMessage),
    UserRemoval(UserRemovalMessage),
    UserAddition(UserAdditionMessage),
    NewUser(NewUserMessage),
    ChangeRoom(ChangeRoomMessage),
    UsernameChange(UsernameChangeMessage),
    CreateRoomChange(CreateRoomChangeMessage),
    Initialization(InitMessage),
    Deletion(DeletionMessage)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeletionMessage {
    pub sender_id: String,
    pub message_id: String,
}

// BasicMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct BasicMessage {
    pub content: String,
    pub sender_id: Uuid,
    pub timestamp: u64,
    pub message_id: Uuid,
    pub room_id: Uuid,
    pub ws_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TSBasicMessage {
    pub content: String,
}

// ImageMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct ImageMessage {
    pub image_url: String,
    pub sender_id: String,
}

// NotificationMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationMessage {
    pub sender_id: String,
}

// TypingMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct TypingMessage {
    pub sender_id: String,
}

// UserRemovalMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct UserRemovalMessage {
    pub removed_user: String,
    pub room_id: Uuid,
    pub sender_id: Uuid,
}

// UserAdditionMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct UserAdditionMessage {
    pub user_id: Uuid,
    pub username: String,
}

impl UserAdditionMessage {
    pub fn new(user_id: Uuid, username: String) -> Self {
        UserAdditionMessage { user_id, username }
    }
}

// NewUserMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct NewUserMessage {
    pub user_id: Uuid,
    pub username: String,
}

impl NewUserMessage {
    pub fn new(user_id: Uuid, username: String) -> Self {
        NewUserMessage { user_id, username }
    }
}

// ChangeRoomMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct ChangeRoomMessage {
    pub room_id: Uuid,
    pub sender_id: Uuid,
}

// UsernameChangeMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct UsernameChangeMessage {
    pub new_username: String,
    pub sender_id: Uuid,
}

impl UsernameChangeMessage {
    pub fn new(sender_id: Uuid, new_username: String) -> Self {
        UsernameChangeMessage { sender_id, new_username }
    }
}

// CreateRoomChangeMessage Struct
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateRoomChangeMessage {
    pub room_name: String,
    pub sender_id: Uuid,
}

impl CreateRoomChangeMessage {
    pub fn new(sender_id: Uuid, room_name: String) -> Self {
        CreateRoomChangeMessage { sender_id, room_name }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginErrorMessage {
    pub message: String,
}

impl LoginErrorMessage {
    pub fn new(message: String) -> Self {
        LoginErrorMessage { message }
    }
}