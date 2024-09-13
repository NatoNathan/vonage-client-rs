mod user;
use crate::client::{VonageClient, VonageClientError};
pub use user::{User, UserListPage};

pub trait ConversationApi {
    /// Get Users
    /// This function gets a list of users from the Vonage API
    /// @return The list of users
    fn get_users(
        &mut self,
    ) -> impl std::future::Future<Output = Result<UserListPage, VonageClientError>>;

    /// Create User
    /// This function creates a user in the Vonage API
    /// @param user The user to create
    /// @return The response from the Vonage API
    fn create_user(
        &mut self,
        user: User,
    ) -> impl std::future::Future<Output = Result<User, VonageClientError>>;
}

impl ConversationApi for VonageClient {
    async fn get_users(&mut self) -> Result<UserListPage, VonageClientError> {
        log::debug!("Getting users");
        self.get("/v1/users")
            .await
            .map(Self::debug_response("Get Users Response".into()))
    }

    async fn create_user(&mut self, user: User) -> Result<User, VonageClientError> {
        log::debug!("Creating user: {:?}", user);
        self.post("/v1/users", user)
            .await
            .map(Self::debug_response("Create User Response".into()))
    }
}
