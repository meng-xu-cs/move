// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

/// Semantics of an intrinsic type in the verification context
pub trait IntrinsicTypeSemantics: 'static {
    // TODO(mengxu): e.g., boogie prelude code
}

/// Semantics of an intrinsic function in the verification context
pub trait IntrinsicFunSemantics: 'static {
    // TODO(mengxu): e.g., boogie prelude code
    // TODO(mengxu): replace well_known.rs in `move-model`
}
