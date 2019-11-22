use neovim_lib::Value;
use super::eventhandler::EventHandler;

pub struct State;

impl State {
    pub fn son_state(e: &EventHandler) -> Value {
        use std::convert::TryInto;
        Value::Map(vec![
            (
                "connected".try_into().unwrap(),
                e.client.auth_token.is_some().try_into().unwrap(),
            ),
            (
                "username".try_into().unwrap(),
                e.client.un.clone().unwrap_or("<Unknown>".into()).try_into().unwrap(),
            ),
            (
                "url".try_into().unwrap(),
                e.client.url.clone().unwrap_or("<No URL set>".into()).try_into().unwrap(),
            ),
        ])
    }
}
