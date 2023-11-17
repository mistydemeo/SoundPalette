//! Roland GS.
//!
//! References:
//! - Roland SC-55 Owner's Manual.
//! - Roland SC-55mkII Owner's Manual.
//! - Roland SC-7 Owner's Manual (not a GS device, only has a tiny subset).

use super::{
    param_bool, param_enum, param_range, param_signed, param_unsigned, AddressBlockMap, ModelInfo,
    ParameterAddressMap,
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
    (&[0x40, 0x00], "System parameters", GS_PAM_SYSTEM),
    (
        &[0x40, 0x01],
        "Patch parameters, Patch common",
        GS_PAM_PATCH_COMMON,
    ),
    (&[0x40, 0x10], "Patch parameters, Part 10", GS_PAM_PATCH),
    (&[0x40, 0x11], "Patch parameters, Part 1", GS_PAM_PATCH),
    (&[0x40, 0x12], "Patch parameters, Part 2", GS_PAM_PATCH),
    (&[0x40, 0x13], "Patch parameters, Part 3", GS_PAM_PATCH),
    (&[0x40, 0x14], "Patch parameters, Part 4", GS_PAM_PATCH),
    (&[0x40, 0x15], "Patch parameters, Part 5", GS_PAM_PATCH),
    (&[0x40, 0x16], "Patch parameters, Part 6", GS_PAM_PATCH),
    (&[0x40, 0x17], "Patch parameters, Part 7", GS_PAM_PATCH),
    (&[0x40, 0x18], "Patch parameters, Part 8", GS_PAM_PATCH),
    (&[0x40, 0x19], "Patch parameters, Part 9", GS_PAM_PATCH),
    (&[0x40, 0x1A], "Patch parameters, Part 11", GS_PAM_PATCH),
    (&[0x40, 0x1B], "Patch parameters, Part 12", GS_PAM_PATCH),
    (&[0x40, 0x1C], "Patch parameters, Part 13", GS_PAM_PATCH),
    (&[0x40, 0x1D], "Patch parameters, Part 14", GS_PAM_PATCH),
    (&[0x40, 0x1E], "Patch parameters, Part 15", GS_PAM_PATCH),
    (&[0x40, 0x1F], "Patch parameters, Part 16", GS_PAM_PATCH),
    // TODO: Drum setup parameters, Bulk dump
];

const GS_PAM_SYSTEM: ParameterAddressMap = &[
    // TODO: MASTER TUNE ("nibblized data" support missing)
    param_unsigned(&[0x04], 0x01, "MASTER VOLUME", 0x00..=0x7F),
    param_range(
        &[0x05],
        0x01,
        "MASTER KEY-SHIFT",
        0x28..=0x58,
        0x40,
        -24.0..=24.0,
        "semitones",
    ),
    param_signed(&[0x06], 0x01, "MASTER PAN", 0x01..=0x7F, 0x40),
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

const GS_PAM_PATCH_COMMON: ParameterAddressMap = &[
    // TODO: Patch Name (non-single-byte parameter support missing)
    // TODO: Voice Reserve (non-single-byte parameter support missing)
    param_enum(
        &[0x30],
        0x01,
        "REVERB MACRO",
        0x00..=0x07,
        &[
            (&[0x00], "Room 1"),
            (&[0x01], "Room 2"),
            (&[0x02], "Room 3"),
            (&[0x03], "Hall 1"),
            (&[0x04], "Hall 2"),
            (&[0x05], "Plate"),
            (&[0x06], "Delay"),
            (&[0x07], "Panning Delay"),
        ],
    ),
    param_unsigned(&[0x31], 0x01, "REVERB CHARACTER", 0x00..=0x07),
    param_unsigned(&[0x32], 0x01, "REVERB PRE-LPF", 0x00..=0x07),
    param_unsigned(&[0x33], 0x01, "REVERB LEVEL", 0x00..=0x7F),
    param_unsigned(&[0x34], 0x01, "REVERB TIME", 0x00..=0x7F),
    param_unsigned(&[0x35], 0x01, "REVERB DELAY FEEDBACK", 0x00..=0x7F),
    param_unsigned(&[0x36], 0x01, "REVERB SEND LEVEL TO CHORUS", 0x00..=0x7F),
    // 37h is unoccupied!
    param_enum(
        &[0x38],
        0x01,
        "CHORUS MACRO",
        0x00..=0x07,
        &[
            (&[0x00], "Chorus 1"),
            (&[0x01], "Chorus 2"),
            (&[0x02], "Chorus 3"),
            (&[0x03], "Chorus 4"),
            (&[0x04], "Feedback Chorus"),
            (&[0x05], "Flanger"),
            (&[0x06], "Short Delay"),
            (&[0x07], "Short Delay (FB)"),
        ],
    ),
    param_unsigned(&[0x39], 0x01, "CHORUS PRE-LPF", 0x00..=0x07),
    param_unsigned(&[0x3A], 0x01, "CHORUS LEVEL", 0x00..=0x7F),
    param_unsigned(&[0x3B], 0x01, "CHORUS FEEDBACK", 0x00..=0x7F),
    param_unsigned(&[0x3C], 0x01, "CHORUS DELAY", 0x00..=0x7F),
    param_unsigned(&[0x3D], 0x01, "CHORUS RATE", 0x00..=0x7F),
    param_unsigned(&[0x3E], 0x01, "CHORUS DEPTH", 0x00..=0x7F),
    param_unsigned(&[0x3F], 0x01, "CHORUS SEND LEVEL TO REVERB", 0x00..=0x7F),
];

const GS_PAM_PATCH: ParameterAddressMap = &[
    // TODO: TONE NUMBER (non-single-byte parameter support missing)
    param_enum(
        &[0x02],
        0x01,
        "Rx. CHANNEL",
        0x00..=0x10,
        &[
            (&[0x00], "Channel 1"),
            (&[0x01], "Channel 2"),
            (&[0x02], "Channel 3"),
            (&[0x03], "Channel 4"),
            (&[0x04], "Channel 5"),
            (&[0x05], "Channel 6"),
            (&[0x06], "Channel 7"),
            (&[0x07], "Channel 8"),
            (&[0x08], "Channel 9"),
            (&[0x09], "Channel 10"),
            (&[0x0A], "Channel 11"),
            (&[0x0B], "Channel 12"),
            (&[0x0C], "Channel 13"),
            (&[0x0D], "Channel 14"),
            (&[0x0E], "Channel 15"),
            (&[0x0F], "Channel 16"),
            (&[0x10], "OFF"),
        ],
    ),
    param_bool(&[0x03], "Rx. PITCH BEND"),
    param_bool(&[0x04], "Rx. CH PRESSURE (CAf)"),
    param_bool(&[0x05], "Rx. PROGRAM CHANGE"),
    param_bool(&[0x06], "Rx. CONTROL CHANGE"),
    param_bool(&[0x07], "Rx. POLY PRESSURE (PAf)"),
    param_bool(&[0x08], "Rx. NOTE MESSAGE"),
    param_bool(&[0x09], "Rx. RPN"),
    param_bool(&[0x0A], "Rx. NRPN"),
    param_bool(&[0x0B], "Rx. MODULATION"),
    param_bool(&[0x0C], "Rx. VOLUME"),
    param_bool(&[0x0D], "Rx. PANPOT"),
    param_bool(&[0x0E], "Rx. EXPRESSION"),
    param_bool(&[0x0F], "Rx. HOLD1"),
    param_bool(&[0x10], "Rx. PORTAMENTO"),
    param_bool(&[0x11], "Rx. SOSTENUTO"),
    param_bool(&[0x12], "Rx. SOFT"),
    param_enum(
        &[0x13],
        0x01,
        "MONO/POLY MODE",
        0x00..=0x01,
        &[(&[0x00], "Mono"), (&[0x01], "Poly")],
    ),
    param_enum(
        &[0x14],
        0x01,
        "ASSIGN MODE",
        0x00..=0x02,
        &[
            (&[0x00], "SINGLE"),
            (&[0x01], "LIMITED - MULTI"),
            (&[0x02], "FULL - MULTI"),
        ],
    ),
    param_enum(
        &[0x15],
        0x01,
        "USE FOR RHYTHM PART",
        0x00..=0x02,
        &[(&[0x00], "OFF"), (&[0x01], "MAP1"), (&[0x02], "MAP2")],
    ),
    param_range(
        &[0x16],
        0x01,
        "PITCH KEY SHIFT",
        0x28..=0x58,
        0x40,
        -24.0..=24.0,
        "semitones",
    ),
    // TODO: PITCH OFFSET FINE ("nibblized data" support missing)
    param_unsigned(&[0x19], 0x01, "PART LEVEL", 0x00..=0x7F),
    param_unsigned(&[0x1A], 0x01, "VELOCITY SENSE DEPTH", 0x00..=0x7F),
    param_unsigned(&[0x1B], 0x01, "VELOCITY SENSE OFFSET", 0x00..=0x7F),
    // TODO: how to accomodate special "Random" value (-64) for panning?
    param_signed(&[0x1C], 0x01, "PART PANPOT", 0x00..=0x7F, 0x40),
    // TODO: MIDI note number list for these two? (share with JS?)
    param_unsigned(&[0x1D], 0x01, "KEY RANGE LOW", 0x00..=0x7F),
    param_unsigned(&[0x1E], 0x01, "KEY RANGE HIGH", 0x00..=0x7F),
    // TODO: MIDI controller number list for these two? (share with JS?)
    param_unsigned(&[0x1F], 0x01, "CC1 CONTROLLER NUMBER", 0x00..=0x5F),
    param_unsigned(&[0x20], 0x01, "CC2 CONTROLLER NUMBER", 0x00..=0x5F),
    param_unsigned(&[0x21], 0x01, "CHORUS SEND LEVEL", 0x00..=0x7F),
    param_unsigned(&[0x22], 0x01, "REVERB SEND LEVEL", 0x00..=0x7F),
    // SC-55 manual does not mention this, but SC-55mkII does. Probably added
    // with the General MIDI support? (GM mode disables bank select receive.)
    param_bool(&[0x23], "Rx. BANK SELECT [SC-55mkII+]"),
    param_signed(
        &[0x30],
        0x01,
        "TONE MODIFY 1, Vibrato rate",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x31],
        0x01,
        "TONE MODIFY 2, Vibrato depth",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x32],
        0x01,
        "TONE MODIFY 3, TVF cutoff freq.",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x33],
        0x01,
        "TONE MODIFY 4, TVF resonance",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x34],
        0x01,
        "TONE MODIFY 5, TVF & TVA Env. attack",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x35],
        0x01,
        "TONE MODIFY 6, TVF & TVA Env. decay",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x36],
        0x01,
        "TONE MODIFY 7, TVF & TVA Env. release",
        0x0E..=0x72,
        0x40,
    ),
    param_signed(
        &[0x37],
        0x01,
        "TONE MODIFY 8, Vibrato delay",
        0x0E..=0x72,
        0x40,
    ),
    // TODO: SCALE TUNING (non-single-byte parameter support missing)
    // TODO: MOD/BEND/CAf/PAf/CC1/CC2 controls (as a separate block?)
];
