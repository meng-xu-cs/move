// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::file_format::TypeParameterIndex;
use move_core_types::{account_address::AccountAddress, identifier::Identifier};

use crate::model::{AbilityConstraint, AbilitySet};

/// The default address for intrinsic members
pub const DEFAULT_INTRINSICS_ADDRESS: AccountAddress = AccountAddress::ZERO;

/// The default module name for intrinsic members
pub const DEFAULT_INTRINSICS_MODULE: &str = "intrinsics";

/// A unique identifier for a member item in a module
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ModuleMemberIdent {
    pub address: AccountAddress,
    pub module: Identifier,
    pub member: Identifier,
}

impl ModuleMemberIdent {
    pub fn new(address: AccountAddress, module: &str, member: &str) -> ModuleMemberIdent {
        ModuleMemberIdent {
            address,
            module: Identifier::new(module).expect("Invalid module name"),
            member: Identifier::new(member).expect("Invalid module member identifier"),
        }
    }

    pub fn default_for_intrinsic(member: &str) -> ModuleMemberIdent {
        ModuleMemberIdent::new(
            DEFAULT_INTRINSICS_ADDRESS,
            DEFAULT_INTRINSICS_MODULE,
            member,
        )
    }
}

/// A fat-type that captures either a regular type or an instantiation of an intrinsic type
/// (i.e., a concrete type that is mapped to an intrinsic type).
pub enum RefinableType {
    // primitives
    Bool,
    U8,
    U64,
    U128,
    Num, // refinable to any integer type
    Address,
    Signer,
    // collections
    Vector(Box<RefinableType>),
    Struct(ModuleMemberIdent, Vec<RefinableType>),
    // type parameters
    TypeParameter(TypeParameterIndex),
    // references
    ImmRef(Box<RefinableType>),
    MutRef(Box<RefinableType>), // refinable to immutable reference
    // intrinsics
    Intrinsic(ModuleMemberIdent, Vec<RefinableType>),
}

/// Typing semantics of an intrinsic type
pub struct IntrinsicTypeDecl {
    pub identifier: ModuleMemberIdent,
    pub abilities: AbilitySet,
    pub type_params: Vec<AbilityConstraint>,
    pub refines_from: Vec<RefinableType>,
}

/// Typing semantics of an intrinsic function
pub struct IntrinsicFunDecl {
    pub identifier: ModuleMemberIdent,
    pub type_params: Vec<AbilityConstraint>,
    pub parameters: Vec<RefinableType>,
    pub return_type: RefinableType,
}

// TODO(mengxu): ideally, the rest of the modules need to be placed in the code of each individual
// crate and only supply the list of intrinsics to move-model via a `get_intrinsics()` call. This
// aligns with the treatment of pluggable native functions w.r.t MoveVM.

mod move_stdlib;

/// Expose all intrinsic functions registered into move-model
pub(crate) fn all_intrinsics() -> (Vec<IntrinsicTypeDecl>, Vec<IntrinsicFunDecl>) {
    let mut type_decls = vec![];
    let mut fun_decls = vec![];

    move_stdlib::collect_intrinsics(&mut type_decls, &mut fun_decls);

    (type_decls, fun_decls)
}
