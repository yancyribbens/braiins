// Copyright (C) 2019  Braiins Systems s.r.o.
//
// This file is part of Braiins Open-Source Initiative (BOSI).
//
// BOSI is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Please, keep in mind that we may also license BOSI or any part thereof
// under a proprietary license. For more information on the terms and conditions
// of such proprietary license or if you have any other questions, please
// contact us at opensource@braiins.com.

//! Stratum version 2 top level module
pub mod error;
pub mod framing;
pub mod messages;
pub mod serialization;
pub mod types;

use self::messages::MessageType;
use crate::error::Result;
use crate::{AnyPayload, Message};

use async_trait::async_trait;
use packed_struct::prelude::*;
use std::convert::TryFrom;

use ii_logging::macros::*;

pub use self::framing::codec::Codec;
pub use self::framing::{Frame, Framing};

/// Protocol associates a custom handler with it
pub struct Protocol;
impl crate::Protocol for Protocol {
    type Handler = dyn Handler;
    type Header = framing::Header;
}

/// Specifies all messages to be visited
/// TODO document why anything implementing the Handler must be static
#[async_trait]
pub trait Handler: 'static + Send {
    async fn visit_setup_connection(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SetupConnection,
    ) {
    }

    async fn visit_setup_connection_success(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SetupConnectionSuccess,
    ) {
    }

    async fn visit_setup_connection_error(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SetupConnectionError,
    ) {
    }

    async fn visit_open_standard_mining_channel(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::OpenStandardMiningChannel,
    ) {
    }

    async fn visit_open_standard_mining_channel_success(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::OpenStandardMiningChannelSuccess,
    ) {
    }

    async fn visit_open_standard_mining_channel_error(
        &mut self,

        _header: &framing::Header,
        _payload: &messages::OpenStandardMiningChannelError,
    ) {
    }

    async fn visit_update_channel(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::UpdateChannel,
    ) {
    }

    async fn visit_update_channel_error(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::UpdateChannelError,
    ) {
    }

    async fn visit_submit_shares_standard(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SubmitSharesStandard,
    ) {
    }

    async fn visit_submit_shares_success(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SubmitSharesSuccess,
    ) {
    }

    async fn visit_submit_shares_error(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SubmitSharesError,
    ) {
    }

    async fn visit_new_mining_job(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::NewMiningJob,
    ) {
    }

    async fn visit_set_new_prev_hash(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SetNewPrevHash,
    ) {
    }

    async fn visit_set_target(
        &mut self,
        _header: &framing::Header,
        _payload: &messages::SetTarget,
    ) {
    }
}

/// Consumes `frame` and produces a Message object based on the payload type
pub fn build_message_from_frame(frame: framing::Frame) -> Result<Message<Protocol>> {
    trace!("V2: building message from frame {:x?}", frame);

    // Payload that already contains deserialized message can be returned directly
    if frame.payload.is_serializable() {
        let (header, payload) = frame.split();
        let serializable_payload = payload
            .into_serializable()
            .expect("BUG: cannot convert payload into serializable");

        return Ok(Message {
            header,
            payload: serializable_payload,
        });
    }
    // Header will be consumed by the subsequent transformation of the frame into the actual
    // payload for further handling. Therefore we create a copy for constructing a
    // Message<Protocol >
    let header = frame.header.clone();
    // Deserialize the payload;h based on its type specified in the header
    let payload = match MessageType::from_primitive(frame.header.msg_type).ok_or(
        error::ErrorKind::UnknownMessage(
            format!("Unexpected payload type, full header: {:x?}", frame.header).into(),
        ),
    )? {
        MessageType::SetupConnection => {
            Box::new(messages::SetupConnection::try_from(frame)?) as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SetupConnectionSuccess => {
            Box::new(messages::SetupConnectionSuccess::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SetupConnectionError => {
            Box::new(messages::SetupConnectionError::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::OpenStandardMiningChannel => {
            Box::new(messages::OpenStandardMiningChannel::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::OpenStandardMiningChannelSuccess => {
            Box::new(messages::OpenStandardMiningChannelSuccess::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::OpenStandardMiningChannelError => {
            Box::new(messages::OpenStandardMiningChannelError::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::NewMiningJob => {
            Box::new(messages::NewMiningJob::try_from(frame)?) as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SetNewPrevHash => {
            Box::new(messages::SetNewPrevHash::try_from(frame)?) as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SetTarget => {
            Box::new(messages::SetTarget::try_from(frame)?) as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SubmitSharesStandard => {
            Box::new(messages::SubmitSharesStandard::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SubmitSharesSuccess => {
            Box::new(messages::SubmitSharesSuccess::try_from(frame)?)
                as Box<dyn AnyPayload<Protocol>>
        }
        MessageType::SubmitSharesError => {
            Box::new(messages::SubmitSharesError::try_from(frame)?) as Box<dyn AnyPayload<Protocol>>
        }
        _ => {
            return Err(error::ErrorKind::UnknownMessage(
                format!("Unexpected payload type, full header: {:?}", frame.header).into(),
            )
            .into())
        }
    };

    Ok(Message { header, payload })
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test_utils::v2::*;

    use ii_async_compat::tokio;
    use std::convert::TryInto;

    /// This test demonstrates an actual implementation of protocol handler (aka visitor to a set of
    /// desired messsages
    /// TODO refactor this test once we have a message dispatcher in place
    #[tokio::test]
    async fn test_build_message_from_frame() {
        let message_payload = build_setup_connection();
        let frame = message_payload
            .try_into()
            .expect("Cannot create test frame");

        let message =
            build_message_from_frame(frame).expect("Message payload deserialization failed");
        message.accept(&mut TestIdentityHandler).await;
    }
}
