// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::intrinsics::{IntrinsicFunDecl, IntrinsicTypeDecl};

mod vector;

pub(crate) fn collect_intrinsics(
    type_decls: &mut Vec<IntrinsicTypeDecl>,
    fun_decls: &mut Vec<IntrinsicFunDecl>,
) {
    vector::collect_intrinsics(type_decls, fun_decls);
}
