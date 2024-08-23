/// App authentication params
#[derive(Clone)]
pub struct AppAuthParams {
    /// Flag that indicates route should allow anonymous authentication
    /// See [validators](crate::auth::validators)
    pub allow_anonymous: bool,
}
