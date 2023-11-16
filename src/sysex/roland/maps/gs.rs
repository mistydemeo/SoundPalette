//! Roland GS.
//!
//! References:
//! - Roland SC-55 Owner's Manual.
//! - Roland SC-55mkII Owner's Manual.
//! - Roland SC-7 Owner's Manual (not a GS device, only has a tiny subset).

use super::{
    param_enum, param_range, param_simple, AddressBlockMap, ModelInfo, ParameterAddressMap,
};

/// Roland GS.
pub const GS: ModelInfo = ModelInfo {
    model_id: &[0x42],
    name: "Roland GS",
    default_device_id: 0x10, // SC-55 and SC-7 respond to this, at least
    address_size: 3,
    address_block_map: GS_ABM,
};

const GS_ABM: AddressBlockMap = &[
    (&[0x40, 0x00], "System Parameters", GS_PAM_SYSTEM),
    // TODO: Patch parameters, Drum setup parameters, Bulk dump
];

const GS_PAM_SYSTEM: ParameterAddressMap = &[
    // TODO: MASTER TUNE ("nibblized data" support missing)
    param_simple(&[0x04], 0x01, "MASTER VOLUME", None),
    param_range(
        &[0x05],
        0x01,
        "MASTER KEY-SHIFT",
        0x28..=0x58,
        Some(0x40),
        -24.0..=24.0,
        "semitones",
    ),
    // TODO: how to accomodate zero value for panning? There is no "unit".
    param_simple(&[0x06], 0x01, "MASTER PAN", Some(0x01..=0x7F)),
    param_enum(
        &[0x7F],
        0x01,
        // SC-55mkII name. Called "RESET TO THE GSstandard MODE" in the SC-55
        // manual.
        "MODE SET",
        // SC-55mkII manual also mentions 0x7F as valid data, but it does not
        // explain what it would do, and "MODE SET" is marked as "(Rx. only)",
        // so it can't be for querying. Perhaps that extra byte was an error,
        // it isn't in the original SC-55 manual.
        0x00..=0x00,
        &[(&[0x00], "GS Reset")],
    ),
];

// TODO: Voice Reserve (non-single-byte parameter support missing)