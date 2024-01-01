// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0 or MIT

use crate::common::crypto_callback::FAKE_HMAC;
use crate::common::device_io::{FakeSpdmDeviceIoReceve, SharedBuffer};
use crate::common::secret_callback::*;
use crate::common::transport::PciDoeTransportEncap;
use crate::common::util::create_info;
use codec::{Codec, Writer};
use spdmlib::common::session::{SpdmSession, SpdmSessionState};
use spdmlib::common::SpdmCodec;
use spdmlib::message::*;
use spdmlib::protocol::*;
use spdmlib::{crypto, responder, secret};
use spin::Mutex;
extern crate alloc;
use alloc::sync::Arc;

#[test]
#[cfg(not(feature = "hashed-transcript-data"))]
fn test_case0_handle_spdm_psk_finish() {
    let future = async {
        let (config_info, provision_info) = create_info();
        let pcidoe_transport_encap = Arc::new(Mutex::new(PciDoeTransportEncap {}));
        let shared_buffer = SharedBuffer::new();
        let socket_io_transport = Arc::new(Mutex::new(FakeSpdmDeviceIoReceve::new(Arc::new(
            shared_buffer,
        ))));

        secret::psk::register(SECRET_PSK_IMPL_INSTANCE.clone());
        crypto::hmac::register(FAKE_HMAC.clone());

        let mut context = responder::ResponderContext::new(
            socket_io_transport,
            pcidoe_transport_encap,
            config_info,
            provision_info,
        );
        context.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        context.common.negotiate_info.base_asym_sel = SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;
        context.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        context.common.session = gen_array_clone(SpdmSession::new(), 4);
        context.common.session[0].setup(4294901758).unwrap();
        context.common.session[0].set_crypto_param(
            SpdmBaseHashAlgo::TPM_ALG_SHA_384,
            SpdmDheAlgo::SECP_384_R1,
            SpdmAeadAlgo::AES_256_GCM,
            SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE,
        );
        context.common.session[0].set_session_state(SpdmSessionState::SpdmSessionEstablished);

        let spdm_message_header = &mut [0u8; 1024];
        let mut writer = Writer::init(spdm_message_header);
        let value = SpdmMessageHeader {
            version: SpdmVersion::SpdmVersion10,
            request_response_code: SpdmRequestResponseCode::SpdmRequestChallenge,
        };
        let _ = value.encode(&mut writer);

        let psk_finish = &mut [0u8; 1024];
        let mut writer = Writer::init(psk_finish);
        let value = SpdmPskFinishRequestPayload {
            verify_data: SpdmDigestStruct {
                data_size: 48,
                data: Box::new([100u8; SPDM_MAX_HASH_SIZE]),
            },
        };
        let _ = value.spdm_encode(&mut context.common, &mut writer);

        let bytes = &mut [0u8; 1024];
        bytes.copy_from_slice(&spdm_message_header[0..]);
        bytes[2..].copy_from_slice(&psk_finish[0..1022]);
        let mut response_buffer = [0u8; spdmlib::config::MAX_SPDM_MSG_SIZE];
        let mut writer = Writer::init(&mut response_buffer);
        let (status, send_buffer) = context.handle_spdm_psk_finish(4294901758, bytes, &mut writer);
    };
    executor::block_on(future);
}

#[test]
#[cfg(not(feature = "hashed-transcript-data"))]
fn test_case1_handle_spdm_psk_finish() {
    let future = async {
        let (config_info, provision_info) = create_info();
        let pcidoe_transport_encap = Arc::new(Mutex::new(PciDoeTransportEncap {}));
        let shared_buffer = SharedBuffer::new();
        let socket_io_transport = Arc::new(Mutex::new(FakeSpdmDeviceIoReceve::new(Arc::new(
            shared_buffer,
        ))));

        secret::psk::register(SECRET_PSK_IMPL_INSTANCE.clone());
        crypto::hmac::register(FAKE_HMAC.clone());

        let mut context = responder::ResponderContext::new(
            socket_io_transport,
            pcidoe_transport_encap,
            config_info,
            provision_info,
        );
        context.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        context.common.negotiate_info.base_asym_sel = SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;
        context.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        context.common.session = gen_array_clone(SpdmSession::new(), 4);
        context.common.session[0].setup(4294901758).unwrap();
        context.common.session[0].set_crypto_param(
            SpdmBaseHashAlgo::TPM_ALG_SHA_384,
            SpdmDheAlgo::SECP_384_R1,
            SpdmAeadAlgo::AES_256_GCM,
            SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE,
        );
        context.common.session[0].set_session_state(SpdmSessionState::SpdmSessionEstablished);

        let spdm_message_header = &mut [0u8; 1024];
        let mut writer = Writer::init(spdm_message_header);
        let value = SpdmMessageHeader {
            version: SpdmVersion::SpdmVersion10,
            request_response_code: SpdmRequestResponseCode::SpdmRequestChallenge,
        };
        let _ = value.encode(&mut writer);

        let psk_finish = &mut [0u8; 1024];
        let mut writer = Writer::init(psk_finish);
        let value = SpdmPskFinishRequestPayload {
            verify_data: SpdmDigestStruct {
                data_size: 48,
                data: Box::new([100u8; SPDM_MAX_HASH_SIZE]),
            },
        };
        let _ = value.spdm_encode(&mut context.common, &mut writer);

        let bytes = &mut [0u8; 1024];
        bytes.copy_from_slice(&spdm_message_header[0..]);
        bytes[2..].copy_from_slice(&psk_finish[0..1022]);
        let mut response_buffer = [0u8; spdmlib::config::MAX_SPDM_MSG_SIZE];
        let mut writer = Writer::init(&mut response_buffer);
        let (status, send_buffer) = context.handle_spdm_psk_finish(4294901758, bytes, &mut writer);

        for session in context.common.session.iter() {
            assert_eq!(
                session.get_session_id(),
                spdmlib::common::INVALID_SESSION_ID
            );
        }
    };
    executor::block_on(future);
}
