use super::*;

pub fn event_open_init(
    connection_id: ConnectionId,
    client_id: ClientId,
    counterparty_client_id: ClientId,
) -> Event {
    Event::new(IbcEventType::OpenInitConnection.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(
            COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
            counterparty_client_id.as_str(),
        )
}