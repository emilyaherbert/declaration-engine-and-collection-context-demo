use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::{
        resolved::resolved_expression::{
            ResolvedExpression, ResolvedExpressionVariant, ResolvedStructExpressionField,
        },
        typed::typed_expression::{
            TypedExpression, TypedExpressionVariant, TypedStructExpressionField,
        },
    },
};

pub(super) fn resolve_expression(
    declaration_engine: &mut DeclarationEngine,
    expression: TypedExpression,
) -> ResolvedExpression {
    let variant = resolve_expression_variant(declaration_engine, expression.variant);
    ResolvedExpression {
        variant,
        type_id: expression.type_id,
    }
}

fn resolve_expression_variant(
    declaration_engine: &mut DeclarationEngine,
    variant: TypedExpressionVariant,
) -> ResolvedExpressionVariant {
    match variant {
        TypedExpressionVariant::Literal { value } => ResolvedExpressionVariant::Literal { value },
        TypedExpressionVariant::Variable { name } => ResolvedExpressionVariant::Variable { name },
        TypedExpressionVariant::FunctionApplication { name, arguments } => {
            // TODO: check to see that it exists
            // TODO: monomorphize it
            let function_declaration = declaration_engine
                .get_function(name.clone())
                .cloned()
                .unwrap();
            let new_arguments = arguments
                .into_iter()
                .map(|argument| resolve_expression(declaration_engine, argument))
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::FunctionApplication {
                name,
                function_declaration,
                arguments: new_arguments,
            }
        }
        TypedExpressionVariant::Struct {
            struct_name,
            fields,
        } => {
            let new_fields = fields
                .into_iter()
                .map(|field| resolve_struct_expression_field(declaration_engine, field))
                .collect::<Vec<_>>();
            ResolvedExpressionVariant::Struct {
                struct_name,
                fields: new_fields,
            }
        }
        TypedExpressionVariant::Enum {
            enum_name,
            variant_name,
            value,
        } => {
            let new_value = resolve_expression(declaration_engine, *value);
            ResolvedExpressionVariant::Enum {
                enum_name,
                variant_name,
                value: Box::new(new_value),
            }
        }
    }
}

fn resolve_struct_expression_field(
    declaration_engine: &mut DeclarationEngine,
    struct_expression_field: TypedStructExpressionField,
) -> ResolvedStructExpressionField {
    let new_value = resolve_expression(declaration_engine, struct_expression_field.value);
    ResolvedStructExpressionField {
        name: struct_expression_field.name,
        value: new_value,
    }
}
