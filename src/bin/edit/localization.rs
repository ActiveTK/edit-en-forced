// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

include!(concat!(env!("OUT_DIR"), "/i18n_edit.rs"));

/// No-op
pub fn init() {}

pub fn loc(id: LocId) -> &'static str {
    TRANSLATIONS[0][id as usize]
}
