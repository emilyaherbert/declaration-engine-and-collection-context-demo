use crate::{
    declaration_engine::DeclarationEngine,
    language::{
        ResolvedExpression, ResolvedStructExpressionField, TypedExpression,
        TypedStructExpressionField,
    },
};

pub(super) fn resolve_expression(
    declaration_engine: &mut DeclarationEngine,
    expression: TypedExpression,
) -> ResolvedExpression {
    match expression {
        TypedExpression::Literal { value } => ResolvedExpression::Literal { value },
        TypedExpression::Variable { name } => ResolvedExpression::Variable { name },
        TypedExpression::FunctionApplication { name, arguments } => {
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
            ResolvedExpression::FunctionApplication {
                name,
                function_declaration,
                arguments: new_arguments,
            }
        }
        TypedExpression::Struct {
            struct_name,
            fields,
        } => {
            let new_fields = fields
                .into_iter()
                .map(|field| resolve_struct_expression_field(declaration_engine, field))
                .collect::<Vec<_>>();
            ResolvedExpression::Struct {
                struct_name,
                fields: new_fields,
            }
        }
        TypedExpression::Enum {
            enum_name,
            variant_name,
            value,
        } => {
            let new_value = resolve_expression(declaration_engine, *value);
            ResolvedExpression::Enum {
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
