// This file is copied from:
//     https://github.com/szeged/blurz/blob/master/src/bluetooth_event.rs
//
// Copyright (c) 2016, University of Szeged
// Copyright (c) 2016, Attila Dusnoki <adusnoki@inf.u-szeged.hu>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use dbus::{arg::cast, arg::RefArg, arg::TypeMismatchError, arg::Variant, Message};
use std::collections::HashMap;

use crate::bluetooth::{AdapterId, CharacteristicId, DeviceId};

#[derive(Clone, Debug)]
pub enum BluetoothEvent {
    Powered {
        adapter: AdapterId,
        powered: bool,
    },
    Discovering {
        adapter: AdapterId,
        discovering: bool,
    },
    Connected {
        device: DeviceId,
        connected: bool,
    },
    ServicesResolved {
        device: DeviceId,
        services_resolved: bool,
    },
    Value {
        characteristic: CharacteristicId,
        value: Box<[u8]>,
    },
    RSSI {
        device: DeviceId,
        rssi: i16,
    },
    None,
}

impl BluetoothEvent {
    pub fn from(conn_msg: Message) -> Option<BluetoothEvent> {
        #[allow(clippy::type_complexity)]
        let result: Result<
            (&str, HashMap<String, Variant<Box<dyn RefArg>>>),
            TypeMismatchError,
        > = conn_msg.read2();

        match result {
            Ok((_, properties)) => {
                let object_path = conn_msg.path().unwrap().to_string();

                if let Some(value) = properties.get("Powered") {
                    if let Some(powered) = cast::<bool>(&value.0) {
                        let event = BluetoothEvent::Powered {
                            adapter: AdapterId { object_path },
                            powered: *powered,
                        };

                        return Some(event);
                    }
                }

                if let Some(value) = properties.get("Discovering") {
                    if let Some(discovering) = cast::<bool>(&value.0) {
                        let event = BluetoothEvent::Discovering {
                            adapter: AdapterId { object_path },
                            discovering: *discovering,
                        };

                        return Some(event);
                    }
                }

                if let Some(value) = properties.get("Connected") {
                    if let Some(connected) = cast::<bool>(&value.0) {
                        let event = BluetoothEvent::Connected {
                            device: DeviceId { object_path },
                            connected: *connected,
                        };

                        return Some(event);
                    }
                }

                if let Some(value) = properties.get("ServicesResolved") {
                    if let Some(services_resolved) = cast::<bool>(&value.0) {
                        let event = BluetoothEvent::ServicesResolved {
                            device: DeviceId { object_path },
                            services_resolved: *services_resolved,
                        };

                        return Some(event);
                    }
                }

                if let Some(value) = properties.get("Value") {
                    if let Some(value) = cast::<Vec<u8>>(&value.0) {
                        let event = BluetoothEvent::Value {
                            characteristic: CharacteristicId { object_path },
                            value: value.clone().into_boxed_slice(),
                        };

                        return Some(event);
                    }
                }

                if let Some(value) = properties.get("RSSI") {
                    if let Some(rssi) = cast::<i16>(&value.0) {
                        let event = BluetoothEvent::RSSI {
                            device: DeviceId { object_path },
                            rssi: *rssi,
                        };

                        return Some(event);
                    }
                }

                Some(BluetoothEvent::None)
            }
            Err(_err) => None,
        }
    }
}
