use napi_derive::napi;

#[napi(object)]
pub struct PlayerSteamId {
    pub steam_id64: String,
    pub steam_id32: String,
    pub account_id: u32,
}

#[napi]
pub mod localplayer {
    use super::PlayerSteamId;

    #[napi]
    pub fn get_steam_id() -> PlayerSteamId {
        let client = crate::client::get_client();
        let steam_id = client.user().steam_id();

        PlayerSteamId {
            steam_id64: steam_id.raw().to_string(),
            steam_id32: steam_id.steamid32(),
            account_id: steam_id.account_id().raw(),
        }
    }

    #[napi]
    pub fn get_name() -> String {
        let client = crate::client::get_client();
        client.friends().name()
    }

    #[napi]
    pub fn get_level() -> u32 {
        let client = crate::client::get_client();
        client.user().level()
    }

    /// @returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in, e.g. "US" or "UK".
    #[napi]
    pub fn get_ip_country() -> String {
        let client = crate::client::get_client();
        client.utils().ip_country()
    }

    #[napi]
    pub fn set_rich_presence(key: String, value: Option<String>) {
        let client = crate::client::get_client();
        client.friends().set_rich_presence(&key, value.as_deref());
    }

    #[napi]
    pub fn activate_game_overlay_to_web_page(url: String) {
        let client = crate::client::get_client();
        client.friends().activate_game_overlay_to_web_page(&url);
    }
}
