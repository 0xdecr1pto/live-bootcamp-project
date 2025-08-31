use std::collections::HashMap;

use crate::domain::{User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => if user.password == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), false);
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), false);
        user_store.users.insert("test@example.com".to_string(), user.clone());

        let result = user_store.get_user("test@example.com").await;
        assert_eq!(result, Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), false);
        user_store.users.insert("test@example.com".to_string(), user.clone());

        let result = user_store.validate_user("test@example.com", "password").await;
        assert!(result.is_ok());
        
        let result = user_store.validate_user("test@example.com", "wrong_password").await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        let result = user_store.validate_user("wrong@example.com", "password").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}