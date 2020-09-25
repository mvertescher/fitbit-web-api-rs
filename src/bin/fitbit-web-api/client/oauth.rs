//! Get an OAuth2 token from Fitbit without user interaction using the
//! [OAuth 2.0 Authorization Code Grant](https://tools.ietf.org/html/rfc6749#section-4.1) flow.
//!
//! This capture method was inspired by
//! [oauth2-rs](https://github.com/ramosbugs/oauth2-rs/tree/master/examples).

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use oauth2::TokenResponse;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, Scope, TokenUrl};

/// Get a token via the OAuth 2.0 Implicit Grant Flow
pub(crate) fn get_token(client_id: String, client_secret: String) -> String {
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://www.fitbit.com/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://api.fitbit.com/oauth2/token".to_string()).unwrap()),
    );

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("activity".to_string()))
        .add_scope(Scope::new("heartrate".to_string()))
        .add_scope(Scope::new("location".to_string()))
        .add_scope(Scope::new("nutrition".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("settings".to_string()))
        .add_scope(Scope::new("sleep".to_string()))
        .add_scope(Scope::new("social".to_string()))
        .add_scope(Scope::new("weight".to_string()))
        .url();

    opener::open(authorize_url.to_string()).expect("failed to open authorize URL");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            // Verify that the state we generated matches the one the server sent us.
            assert_eq!(
                csrf_state.secret(),
                state.secret(),
                "CSRF state mismatch. Malicious actor?"
            );

            // Exchange the code with a token.
            let token = match client.exchange_code(code).request(http_client) {
                Ok(t) => t,
                Err(e) => {
                    log::error!("OAuth2: {}", e);
                    eprintln!("Failed to exchange the code for a valid access_token.\nIncorrect client secret?");
                    std::process::exit(1);
                }
            };

            return token.access_token().secret().clone();
        }
    }

    unreachable!();
}
