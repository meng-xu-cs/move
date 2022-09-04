// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    ast::PropertyValue,
    builder::{model_builder::StructEntry, module_builder::SpecBlockContext},
    pragmas::INTRINSIC_PRAGMA,
    Loc, ModelBuilder,
};

pub(crate) fn check_intrinsic_declaration(
    builder: &ModelBuilder,
    context: &SpecBlockContext,
    loc: &Loc,
    prop: &PropertyValue,
) {
    match context {
        SpecBlockContext::Struct(symbol) => {
            let target = builder.struct_table.get(symbol).expect("struct entry");
            check_intrinsic_declaration_in_struct(builder, target, loc, prop);
        }
        SpecBlockContext::Function(symbol) => {
            builder.fun_table.get(symbol).expect("function entry");
            // TODO(mengxu): add checking there
        }
        SpecBlockContext::Schema(..)
        | SpecBlockContext::Module
        | SpecBlockContext::FunctionCode(..) => {
            unreachable!(
                "\"pragma {}\" is not allowed on context {}",
                INTRINSIC_PRAGMA, context
            );
        }
    }
}

fn check_intrinsic_declaration_in_struct(
    builder: &ModelBuilder,
    _target: &StructEntry,
    loc: &Loc,
    prop: &PropertyValue,
) {
    match prop {
        PropertyValue::Value(_) | PropertyValue::Symbol(_) => {
            builder.error(loc, "Invalid intrinsic declaration");
        }
        PropertyValue::QualifiedSymbol(symbol) => match builder.spec_struct_table.get(symbol) {
            None => {
                builder.error(
                    loc,
                    &format!(
                        "No such intrinsic type: {}",
                        symbol.display(builder.env.symbol_pool())
                    ),
                );
            }
            Some(_) => {}
        },
    }
    // TODO(mengxu): incomplete
}
