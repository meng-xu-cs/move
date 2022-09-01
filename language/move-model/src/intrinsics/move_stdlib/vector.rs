// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    intrinsics::{IntrinsicFunDecl, IntrinsicTypeDecl, ModuleMemberIdent, RefinableType},
    model::{AbilityConstraint, AbilitySet},
};

fn type_vector_identity() -> ModuleMemberIdent {
    ModuleMemberIdent::default_for_intrinsic("Vector")
}
fn type_vector_type_params() -> Vec<AbilityConstraint> {
    vec![AbilityConstraint(AbilitySet::EMPTY)]
}
fn type_vector_refinable_type() -> RefinableType {
    RefinableType::Intrinsic(
        type_vector_identity(),
        vec![RefinableType::TypeParameter(0)],
    )
}

pub(crate) fn collect_intrinsics(
    type_decls: &mut Vec<IntrinsicTypeDecl>,
    fun_decls: &mut Vec<IntrinsicFunDecl>,
) {
    // type: Vector<E>, refines from vector<E>
    type_decls.push(IntrinsicTypeDecl {
        identifier: type_vector_identity(),
        abilities: AbilitySet::VECTOR,
        type_params: type_vector_type_params(),
        refines_from: vec![RefinableType::Vector(Box::new(
            RefinableType::TypeParameter(0),
        ))],
    });

    // fun: empty<E>() -> Vector<E> { .. }
    fun_decls.push(IntrinsicFunDecl {
        identifier: ModuleMemberIdent::default_for_intrinsic("empty"),
        type_params: type_vector_type_params(),
        parameters: vec![],
        return_type: type_vector_refinable_type(),
    });
    // fun: length<E>(&Vector<E>) -> num { .. }
    fun_decls.push(IntrinsicFunDecl {
        identifier: ModuleMemberIdent::default_for_intrinsic("length"),
        type_params: type_vector_type_params(),
        parameters: vec![RefinableType::ImmRef(
            Box::new(type_vector_refinable_type()),
        )],
        return_type: RefinableType::Num,
    });
    // fun: borrow<E>(&Vector<E>, num) -> &E { .. }
    fun_decls.push(IntrinsicFunDecl {
        identifier: ModuleMemberIdent::default_for_intrinsic("borrow"),
        type_params: type_vector_type_params(),
        parameters: vec![
            RefinableType::ImmRef(Box::new(type_vector_refinable_type())),
            RefinableType::Num,
        ],
        return_type: RefinableType::ImmRef(Box::new(RefinableType::TypeParameter(0))),
    });
}
