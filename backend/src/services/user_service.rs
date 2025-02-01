use crate::models::user::User;
use mongodb::{bson::doc, Client, Collection};
use bcrypt::{hash, verify, DEFAULT_COST};
use futures::stream::StreamExt;

#[derive(Clone)]
pub struct UserService {
    collection: Collection<User>,
}

impl UserService {
    pub fn new(client: Client) -> Self {
        let db = client.database("my_database");
        let collection = db.collection::<User>("users");
        UserService { collection }
    }

    pub async fn create_user(&self, user: User) -> mongodb::error::Result<()> {
        let hashed_password = hash(user.password, DEFAULT_COST).unwrap();
        let user_to_insert = User {
            password: hashed_password,
            is_admin: Some(false), // Valor padrão para is_admin
            ..user
        };
        self.collection.insert_one(user_to_insert, None).await?;
        Ok(())
    }

    pub async fn list_users(&self) -> mongodb::error::Result<Vec<User>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursor.next().await {
            users.push(user?);
        }
        Ok(users)
    }

    pub async fn login(&self, email: String, password: String) -> mongodb::error::Result<Option<User>> {
        let user = self.collection.find_one(Some(doc! { "email": email }), None).await?;
        if let Some(user) = user {
            if verify(password.as_str(), user.password.as_str()).unwrap() {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        } else { 
            Ok(None) 
        }
    }
}