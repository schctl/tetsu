//! Mojang user information.

use crate::crypto;

use serde::{Deserialize, Serialize};

/// Mojang authentication server.
pub const AUTH_SERVER: &str = "https://authserver.mojang.com/authenticate";
/// Server join request session-server.
pub const JOIN_SERVER: &str = "https://sessionserver.mojang.com/session/minecraft/join";

// ----- Mojang sent info -----

/// User preferences.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserProperty {
    /// Name of the property.
    pub name: String,
    /// Value of the property.
    pub value: String,
}

/// Information of a Mojang user.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserInfo {
    /// User's username - usually email.
    pub username: String,
    /// Other user properties like preferences, language, etc.
    pub properties: Vec<UserProperty>,
    /// User's ID (?)
    pub id: String,
}

/// A single user profile.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserProfile {
    /// Profile's username.
    pub name: String,
    /// Profile's UUID.
    pub id: String,
}

// ----- Info to be sent ------

/// Information of the client.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserAgent {
    /// Game name - just "Minecraft" here.
    pub name: String,
    /// Major version of the game? - version 1 here.
    pub version: u32,
}

impl Default for UserAgent {
    fn default() -> Self {
        Self {
            name: String::from("Minecraft"),
            version: 1,
        }
    }
}

/// Information required by /authenticate.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserAuthentication {
    pub agent: UserAgent,
    /// Mojang user username.
    pub username: String,
    /// Mojang user password.
    password: String,
    /// Client identifier.
    pub client_token: String,
}

/// Information required by /join
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JoinServer {
    /// Session specific user authentication token.
    access_token: String,
    /// Currently selected user profile.
    pub selected_profile: String,
    /// Unique server ID.
    pub server_id: String,
}

// ------ Mojang Profile ------

/// Structure representing a Mojang user.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User object.
    pub user: Option<UserInfo>,
    /// Client identifier.
    pub client_token: String,
    /// Session specific user authentication token.
    access_token: String,
    /// ADoesn't do anything for now.
    pub available_profiles: Vec<UserProfile>,
    /// Currently selected user profile.
    pub selected_profile: UserProfile,
}

impl User {
    /// Authenticate with the Mojang authentication servers.
    /// Returns a new User.
    pub fn authenticate(username: String, password: String) -> Self {
        let user_auth = UserAuthentication {
            agent: Default::default(),
            username,
            password,
            client_token: String::from("dufc231fhufbcuibeacda42323dsc"),
        };
        let auth_request = serde_json::to_string(&user_auth).unwrap();
        let res = ureq::post(AUTH_SERVER)
            .set("content-type", "application/json")
            .send_string(&auth_request[..]);

        let res = match res {
            Ok(r) => r,
            Err(ureq::Error::Status(code, response)) => {
                panic!(
                    "Failed to log in [[{}] bad-status]: {}",
                    code,
                    response.into_string().unwrap()
                )
            }
            _ => panic!("Failed to log in [unknown-response]"),
        };

        let res = res.into_string().unwrap();

        serde_json::from_str(&res[..]).unwrap()
    }

    /// Send a server join request to Mojang.
    pub fn join_server(&self, server_id: &str, shared_key: &[u8], public_key: &[u8]) {
        let hash_str;
        {
            let mut hasher = crypto::Sha1::new();
            hasher.update(server_id.as_bytes());
            hasher.update(shared_key);
            hasher.update(public_key);

            hash_str = crypto::hexdigest(hasher);
        }

        let join_info = JoinServer {
            access_token: self.access_token.clone(),
            selected_profile: self.selected_profile.id.clone(),
            server_id: hash_str,
        };

        let res = ureq::post(JOIN_SERVER)
            .set("content-type", "application/json")
            .send_string(&serde_json::to_string(&join_info).unwrap()[..])
            .unwrap();

        if res.status() != 204 {
            panic!(
                "Failed to authenticate with server [[{}] bad-status].",
                res.status()
            );
        }
    }
}
