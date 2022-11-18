pub mod account;
pub mod message;

#[cfg(feature = "rest")]
#[derive(schemars::JsonSchema)]
pub enum RootSchema {
    Account(account::Account),
    CreateAccount(account::CreateAccount),
    Message(message::CreateMessage),
    ListMessages(message::ListMessages),
    CreateMessage(message::CreateMessage),
    MessageWithCreator(message::MessageWithCreator),
}

impl RootSchema {
    pub fn build() -> schemars::schema::RootSchema {
        schemars::schema_for!(Self)
    }
}
