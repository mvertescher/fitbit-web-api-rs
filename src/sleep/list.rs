//! Get sleep list for a user.

use crate::UserId;

use url::Url;

/// Generate the request URL from a user id.
pub fn url(user_id: UserId) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1.2/user/{}/sleep/list.json",
        user_id.to_string(),
    ))
    .unwrap()
}
