use std::str::FromStr;
use std::time::Duration;

pub mod setup;

use common::ibc::core::ics24_host::identifier::ClientId;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::traits::AnyTypes;
use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::to_binary;

use cosmwasm_std::Addr;
use cosmwasm_std::ContractResult;

use cosmwasm_std::SystemResult;
use cosmwasm_std::WasmQuery;

use cw_common::get_address_storage_prefix;

use cw_common::raw_types::connection::RawMsgConnectionOpenInit;

use cw_ibc_core::context::CwIbcCoreContext;
use cw_ibc_core::conversions::to_ibc_client_id;
use cw_ibc_core::conversions::to_ibc_connection_id;
use cw_ibc_core::conversions::to_ibc_height;
use cw_ibc_core::ics03_connection::event::create_connection_event;
use cw_ibc_core::ics03_connection::event::CLIENT_ID_ATTRIBUTE_KEY;
use cw_ibc_core::ics03_connection::event::CONN_ID_ATTRIBUTE_KEY;
use cw_ibc_core::ics03_connection::event::COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY;
use cw_ibc_core::ics03_connection::event::COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY;

//use cw_ibc_core::ics03_connection::event::create_open_init_event;

use common::ibc::core::ics03_connection::connection::Counterparty;
use common::ibc::core::ics03_connection::connection::State;
use common::ibc::core::ics03_connection::version::get_compatible_versions;
use common::ibc::core::ics03_connection::version::Version;
use common::ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use common::ibc::core::ics24_host::identifier::ConnectionId;
use common::ibc::events::IbcEventType;
use cw_common::ibc_types::IbcClientId;
use cw_ibc_core::light_client::light_client::LightClient;
use cw_ibc_core::ConnectionEnd;
use prost::Message;
use setup::*;

#[test]
fn test_set_connection() {
    let mut deps = deps();
    let conn_end = ConnectionEnd::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();
    let result = contract
        .connection_end(deps.as_ref().storage, &conn_id)
        .unwrap();

    assert_eq!(conn_end, result)
}

#[test]
fn test_get_connection() {
    let mut deps = deps();
    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = Counterparty::new(IbcClientId::default(), None, ss.unwrap());
    let conn_end = ConnectionEnd::new(
        State::Open,
        IbcClientId::default(),
        counter_party,
        vec![Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();
    let result = contract
        .connection_end(deps.as_ref().storage, &conn_id)
        .unwrap();

    assert_eq!(conn_end, result)
}

#[test]
fn test_connection_seq_on_a() {
    let mut store = deps();
    let contract = CwIbcCoreContext::new();
    contract
        .connection_next_sequence_init(store.as_mut().storage, u64::default())
        .unwrap();
    let result = contract.connection_counter(store.as_ref().storage).unwrap();

    assert_eq!(0, result);

    let increment_seq_on_a = contract
        .increase_connection_counter(store.as_mut().storage)
        .unwrap();
    assert_eq!(1, increment_seq_on_a);
}

#[test]
fn test_client_connection() {
    let mut deps = deps();
    let client_id = ClientId::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();

    contract
        .store_connection_to_client(deps.as_mut().storage, &client_id, &conn_id)
        .unwrap();

    let result = contract
        .client_connection(deps.as_ref().storage, &client_id)
        .unwrap();

    assert_eq!(conn_id, result)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_get_connection_fail() {
    let deps = deps();

    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();

    contract
        .connection_end(deps.as_ref().storage, &conn_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_set_connection_fail() {
    let deps = deps();
    let conn_id = ConnectionId::new(0);
    let contract = CwIbcCoreContext::new();
    contract
        .connection_end(deps.as_ref().storage, &conn_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_connection_seq_on_a_fail() {
    let store = deps();
    let contract = CwIbcCoreContext::new();
    contract.connection_counter(store.as_ref().storage).unwrap();
}

#[test]
#[should_panic(
    expected = "Std(NotFound { kind: \"common::ibc::core::ics24_host::identifier::ConnectionId\" })"
)]
fn test_client_connection_fail() {
    let deps = deps();
    let client_id = ClientId::default();

    let contract = CwIbcCoreContext::new();

    contract
        .client_connection(deps.as_ref().storage, &client_id)
        .unwrap();
}

#[test]
fn test_commitment_prefix() {
    let contract = CwIbcCoreContext::new();
    let env = get_mock_env();
    let prefix = get_address_storage_prefix(
        "archway19d4lkjwk2wnf4fzraw4gwspvevlqa9kwu2nasl",
        "commitments",
    );
    let expected = CommitmentPrefix::try_from(prefix).unwrap_or_default();
    let result = contract.commitment_prefix(mock_dependencies().as_ref(), &env);
    assert_eq!(result, expected);
}

#[test]
fn connection_open_init() {
    let mut deps = deps();

    let res_msg = RawMsgConnectionOpenInit {
        client_id: "iconclient-0".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };
    let client_id = to_ibc_client_id(&res_msg.client_id).unwrap();

    let contract = CwIbcCoreContext::new();
    let client_state: ClientState = get_dummy_client_state();
    contract
        .store_client_implementations(
            deps.as_mut().storage,
            &ClientId::from_str("iconclient-0").unwrap(),
            LightClient::new("lightclientaddress".to_string()),
        )
        .unwrap();

    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => SystemResult::Ok(ContractResult::Ok(to_binary(&vec![0, 1, 2, 3]).unwrap())),
        _ => todo!(),
    });

    let cl = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    contract
        .client_state(&mut deps.storage, &client_id)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    let res = contract.connection_open_init(deps.as_mut(), res_msg);

    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_validate_open_init_connection_fail() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let message = RawMsgConnectionOpenInit {
        client_id: "client_id_on_a".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };

    contract
        .connection_open_init(deps.as_mut(), message)
        .unwrap();
}

#[test]
fn create_connection_open_init_event() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_connection_event(
        IbcEventType::OpenInitConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        None,
    )
    .unwrap();
    assert_eq!(IbcEventType::OpenInitConnection.as_str(), event.ty);
    assert_eq!("connection-10", event.attributes[0].value);
    assert_eq!("default-0", event.attributes[1].value);
    assert_eq!("default-0", event.attributes[2].value);
}

#[test]
fn create_connection_open_ack_event() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let counterparty_connection_id = ConnectionId::new(20);

    let event = create_connection_event(
        IbcEventType::OpenAckConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        Some(counterparty_connection_id),
    )
    .unwrap();
    assert_eq!(IbcEventType::OpenAckConnection.as_str(), event.ty);
    assert_eq!("connection-10", event.attributes[0].value);
    assert_eq!("default-0", event.attributes[1].value);
    assert_eq!("connection-20", event.attributes[3].value);
}

#[test]
fn create_connection_open_try_event() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let counterparty_connection_id = ConnectionId::new(20);

    let event = create_connection_event(
        IbcEventType::OpenTryConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        Some(counterparty_connection_id),
    )
    .unwrap();
    assert_eq!(IbcEventType::OpenTryConnection.as_str(), event.ty);
}

#[test]
fn create_conection_open_confirm_event() {
    let connection_id_on_b = ConnectionId::new(10);
    let client_id_on_b = ClientId::default();
    let counterparty_connection_id_on_a = ConnectionId::new(2);
    let counterparty_client_id_on_a = ClientId::default();
    let event = create_connection_event(
        IbcEventType::OpenConfirmConnection,
        &connection_id_on_b,
        &client_id_on_b,
        &counterparty_client_id_on_a,
        Some(counterparty_connection_id_on_a),
    )
    .unwrap();

    assert_eq!(IbcEventType::OpenConfirmConnection.as_str(), event.ty);
    assert_eq!("connection-10", event.attributes[0].value);
}

#[test]
fn test_get_compatible_versions() {
    let versions = get_compatible_versions();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0], Version::default());
}

#[test]
fn connection_to_verify_correct_connection_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_connection_event(
        IbcEventType::OpenInitConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        None,
    )
    .unwrap();
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == CONN_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "connection-10");
}

#[test]
fn connection_to_verify_correct_client_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_connection_event(
        IbcEventType::OpenInitConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        None,
    )
    .unwrap();
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == CLIENT_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "default-0");
}

#[test]
fn connection_to_verify_correct_counterparty_client_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_connection_event(
        IbcEventType::OpenInitConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        None,
    )
    .unwrap();
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "default-0");
}

#[test]
fn connection_to_verify_correct_counterparty_conn_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let counterparty_conn_id = ConnectionId::new(1);
    let event = create_connection_event(
        IbcEventType::OpenAckConnection,
        &connection_id,
        &client_id,
        &counterparty_client_id,
        Some(counterparty_conn_id),
    )
    .unwrap();
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "connection-1");
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn connection_open_ack_validate_fail() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_ack(10, 10);

    let _connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let client_id = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &client_id.clone(), light_client)
        .unwrap();

    let client_state_bytes_any = client_state.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            client_state_bytes_any,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &client_id,
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();
    contract
        .connection_open_ack(deps.as_mut(), info, env, message)
        .unwrap();
}

#[test]
fn connection_open_ack_validate() {
    let mut deps = deps();
    let mut env = get_mock_env();
    env.contract.address =
        Addr::unchecked("archway17upmkapj64vcmc554gn8kqhkeaj79nsflaee44u8z6vtwwt9nkgswkx0j9");
    let info = create_mock_info("alice", "umlg", 2000);

    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_ack(10, 10);
    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let client_id = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let light_client = LightClient::new("lightclient".to_string());

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        None,
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Init,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();

    let client_state_bytes = client_state.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let res = contract.connection_open_ack(deps.as_mut(), info, env, message);
    assert!(res.is_ok())
}

#[test]
fn connection_validate_delay() {
    let mut deps = deps();
    let env = get_mock_env();
    let packet_proof_height = common::ibc::core::ics02_client::height::Height::new(1, 1).unwrap();
    let conn_end = ConnectionEnd::default();
    let contract = CwIbcCoreContext::new();
    contract
        .store_last_processed_on(deps.as_mut().storage, &env, conn_end.client_id())
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let result =
        contract.verify_connection_delay_passed(&deps.storage, env, packet_proof_height, conn_end);
    assert!(result.is_ok())
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"cw_ibc_core::ics24_host::LastProcessedOn\" })")]
fn connection_validate_delay_fails() {
    let deps = deps();
    let _env = get_mock_env();
    let packet_proof_height = common::ibc::core::ics02_client::height::Height::new(1, 1).unwrap();
    let conn_end = ConnectionEnd::default();
    let contract = CwIbcCoreContext::new();
    let env = get_mock_env();
    contract
        .verify_connection_delay_passed(&deps.storage, env, packet_proof_height, conn_end)
        .unwrap();
}

#[test]
fn test_block_delay() {
    let mut deps = deps();
    let env = get_mock_env();
    let delay_time = Duration::new(1, 1);
    let contract = CwIbcCoreContext::new();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    let result = contract.calc_block_delay(&delay_time);
    assert_eq!(1, result)
}

#[test]
fn connection_open_try_validate() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_try(10, 10);

    // res_msg.client_id_on_b = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let cl = client_state.to_any().encode_to_vec();
    let message_client_id = to_ibc_client_id(&message.client_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &message_client_id,
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &message_client_id,
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();

    let res = contract.connection_open_try(deps.as_mut(), info, env, message);
    assert!(res.is_ok());
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientNotFound { client_id: ClientId(\"default-0\") } }"
)]
fn open_try_validate_fails() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_try(10, 10);

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let client_state_bytes = client_state.to_any().encode_to_vec();
    let message_client_id = to_ibc_client_id(&message.client_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &message_client_id,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &message_client_id,
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();
    contract
        .connection_open_try(deps.as_mut(), info, env, message)
        .unwrap();
}
#[test]
fn connection_open_confirm_validate() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_confirm();
    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        connection_id.clone().into(),
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::TryOpen,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let cl = client_state.to_any().encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &conn_end.client_id().clone(),
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let res = contract.connection_open_confirm(deps.as_mut(), env, info, message);
    assert!(res.is_ok())
}

#[test]
#[should_panic(expected = "ConnectionMismatch")]
fn connection_open_confirm_validate_fails_of_connection_state_mismatch() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let _env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_confirm();
    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let client_id = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();
    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        connection_id.clone().into(),
        counterparty_prefix,
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Init,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &client_id.clone(), light_client)
        .unwrap();

    let cl = client_state.to_any().encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &client_id,
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    contract
        .connection_open_confirm(deps.as_mut(), get_mock_env(), info, message)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn connection_check_open_init_validate_fails() {
    let mut deps = deps();

    let message = RawMsgConnectionOpenInit {
        client_id: "client_id_on_a".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };
    let client_id = to_ibc_client_id(&message.client_id).unwrap();

    let contract = CwIbcCoreContext::new();

    contract
        .client_state(&mut deps.storage, &client_id)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    contract
        .connection_open_init(deps.as_mut(), message)
        .unwrap();
}

#[test]
fn connection_open_init_fails_of_clientstate() {
    let mut deps = deps();

    let message = RawMsgConnectionOpenInit {
        client_id: "client_id_on_a".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };

    let client_id = ClientId::default();
    let contract = CwIbcCoreContext::new();
    let client_state: ClientState = get_dummy_client_state();

    let client_state_bytes = client_state.encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    let res = contract.connection_open_init(deps.as_mut(), message);
    assert!(res.is_err());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn connection_open_init_validate_invalid_client_id() {
    let mut deps = deps();

    let message = RawMsgConnectionOpenInit {
        client_id: "client_id_on_a".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };
    let seq_on_a: u64 = 24;

    let client_id = ClientId::default();
    let client_on_a = to_ibc_client_id(&message.client_id).unwrap();
    let contract = CwIbcCoreContext::new();
    let client_state: ClientState = get_dummy_client_state();

    let client_state_bytes = client_state.encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_on_a,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    contract
        .client_state(&mut deps.storage, &client_id)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, seq_on_a)
        .unwrap();
    contract
        .connection_open_init(deps.as_mut(), message)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn query_get_connection_fails() {
    let deps = deps();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .connection_end(deps.as_ref().storage, &conn_id)
        .unwrap();
}

#[test]
fn test_update_connection_commitment() {
    let mut deps = deps();
    let conn_id = ConnectionId::new(1);
    let conn_end = ConnectionEnd::default();

    let contract = CwIbcCoreContext::new();
    let res = contract.update_connection_commitment(&mut deps.storage, &conn_id, &conn_end);
    assert!(res.is_ok())
}

#[test]
fn test_check_connection() {
    let mut deps = deps();
    let commitment_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".to_string().as_bytes().to_vec(),
        );
    let counter_party = Counterparty::new(IbcClientId::default(), None, commitment_prefix.unwrap());
    let conn_end = ConnectionEnd::new(
        State::Open,
        IbcClientId::default(),
        counter_party,
        vec![Version::default()],
        Duration::default(),
    );
    let client_id = ClientId::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();
    contract
        .connection_end(deps.as_ref().storage, &conn_id)
        .unwrap();
    let res = contract.check_for_connection(&mut deps.storage, &client_id);
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_connection_seq_on_a_fails_without_initialising() {
    let mut store = deps();
    let contract = CwIbcCoreContext::new();
    contract.connection_counter(store.as_ref().storage).unwrap();
    contract
        .increase_connection_counter(store.as_mut().storage)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn connection_open_init_fails() {
    let mut deps = deps();

    let message = RawMsgConnectionOpenInit {
        client_id: "client_id_on_a".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };

    let contract = CwIbcCoreContext::new();
    let client_state: ClientState = get_dummy_client_state();
    let client_id = to_ibc_client_id(&message.client_id).unwrap();
    let cl = client_state.encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    contract
        .connection_open_init(deps.as_mut(), message)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn connection_open_ack_validate_fails_of_consensus_state() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_ack(10, 10);

    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let _proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let client_id = IbcClientId::default();
    let client_state: ClientState = get_dummy_client_state();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &client_id.clone(), light_client)
        .unwrap();

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        None,
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Init,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();

    let client_state_bytes = client_state.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();

    contract
        .connection_open_ack(deps.as_mut(), info, env, message)
        .unwrap();
}

#[test]
#[should_panic(expected = "ConnectionMismatch")]
fn connection_open_ack_validate_fails_of_connection_mismatch() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_ack(10, 10);

    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let client_id = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &client_id.clone(), light_client)
        .unwrap();

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        None,
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();

    let client_state_bytes = client_state.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();

    contract
        .connection_open_ack(deps.as_mut(), info, env, message)
        .unwrap();
}
