// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use move_binary_format::file_format::TypeParameterIndex;
use move_core_types::identifier::Identifier;

use crate::model::{AbilityConstraint, AbilitySet};

/// A unique identifier for an member item in a module
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ModuleMemberIdent {
    pub address: Identifier,
    pub module: Identifier,
    pub member: Identifier,
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
    MutRef(Box<RefinableType>),
    // intrinsics
    Intrinsic(ModuleMemberIdent, Vec<RefinableType>),
}

/// Typing semantics of an intrinsic type
pub struct IntrinsicTypeDecl {
    pub identifier: ModuleMemberIdent,
    pub abilities: AbilitySet,
    pub type_params: Vec<AbilityConstraint>,
}

/// Typing semantics of an intrinsic function
pub struct IntrinsicFunDecl {
    pub identifier: ModuleMemberIdent,
    pub type_params: Vec<AbilityConstraint>,
    pub parameters: Vec<RefinableType>,
    pub return_type: RefinableType,
}

/// A pluggable list of intrinsic types to be supplied to the move model
pub type IntrinsicTypeList = Vec<(IntrinsicTypeDecl, Box<dyn Any>)>;

/// A pluggable list of intrinsic functions to be supplied to the move model
pub type IntrinsicFunList = Vec<(IntrinsicFunDecl, Box<dyn Any>)>;
