use crate::{core_impl::BindgenArgType, ImplItemMethodInfo, MethodType};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::ReturnType;

use super::TypeRegistry;

impl ImplItemMethodInfo {
    /// Generates metadata struct for this function.
    ///
    /// # Example:
    /// The following function:
    /// ```ignore
    /// fn f3(&mut self, arg0: FancyStruct, arg1: u64) -> Result<IsOk, Error> { }
    /// ```
    /// will produce this struct:
    /// ```ignore
    /// near_sdk::FunctionMetadata {
    ///     name: "f3".to_string(),
    ///     is_view: false,
    ///     is_init: false,
    ///     args: {
    ///         #[derive(borsh::BorshSchema)]
    ///         #[derive(serde :: Deserialize, serde :: Serialize)]
    ///         struct Input {
    ///             arg0: FancyStruct,
    ///             arg1: u64,
    ///         }
    ///         Some(Input::schema_container())
    ///     },
    ///     callbacks: vec![],
    ///     callbacks_vec: None,
    ///     result: Some(Result < IsOk, Error > ::schema_container())
    /// }
    /// ```
    /// If args are serialized with Borsh it will not include `#[derive(borsh::BorshSchema)]`.
    pub fn metadata_struct(&self, registry: &mut TypeRegistry) -> TokenStream2 {
        let method_name_str = self.attr_signature_info.ident.to_string();
        let is_view = matches!(&self.attr_signature_info.method_type, &MethodType::View);
        let is_init = matches!(
            &self.attr_signature_info.method_type,
            &MethodType::Init | &MethodType::InitIgnoreState
        );
        let params: Vec<TokenStream2> = self
            .attr_signature_info
            .input_args()
            .map(|arg| {
                let type_id = registry.register_type(Box::new(arg.ty.clone()));
                let serialization_type = arg.serializer_ty.to_abi_serializer_type();
                quote! {
                    near_sdk::AbiParameter {
                        type_id: #type_id,
                        serialization_type: #serialization_type,
                    }
                }
            })
            .collect();
        let callbacks: Vec<TokenStream2> = self
            .attr_signature_info
            .args
            .iter()
            .filter(|arg| matches!(arg.bindgen_ty, BindgenArgType::CallbackArg))
            .map(|arg| {
                let type_id = registry.register_type(Box::new(arg.ty.clone()));
                let serialization_type = arg.serializer_ty.to_abi_serializer_type();
                quote! {
                    near_sdk::AbiParameter {
                        type_id: #type_id,
                        serialization_type: #serialization_type,
                    }
                }
            })
            .collect();
        let callback_vec = self
            .attr_signature_info
            .args
            .iter()
            .filter(|arg| matches!(arg.bindgen_ty, BindgenArgType::CallbackArgVec))
            .collect::<Vec<_>>();
        if callback_vec.len() > 1 {
            return TokenStream2::from(
                syn::Error::new(
                    Span::call_site(),
                    "A function can only have one #[callback_vec] parameter.",
                )
                .to_compile_error(),
            );
        }
        let callback_vec = match callback_vec.last() {
            Some(arg) => {
                let type_id = registry.register_type(Box::new(arg.ty.clone()));
                let serialization_type = arg.serializer_ty.to_abi_serializer_type();
                quote! {
                    Some(
                        near_sdk::AbiParameter {
                            type_id: #type_id,
                            serialization_type: #serialization_type,
                        }
                    )
                }
            }
            None => {
                quote! { None }
            }
        };
        let result = match &self.attr_signature_info.returns {
            ReturnType::Default => {
                quote! {
                    None
                }
            }
            ReturnType::Type(_, ty) => {
                let type_id = registry.register_type(ty.clone());
                let serialization_type =
                    &self.attr_signature_info.result_serializer.to_abi_serializer_type();
                quote! {
                    Some(
                        near_sdk::AbiParameter {
                            type_id: #type_id,
                            serialization_type: #serialization_type,
                        }
                    )
                }
            }
        };

        quote! {
             near_sdk::AbiFunction {
                 name: #method_name_str.to_string(),
                 is_view: #is_view,
                 is_init: #is_init,
                 params: vec![#(#params),*],
                 callbacks: vec![#(#callbacks),*],
                 callbacks_vec: #callback_vec,
                 result: #result
             }
        }
    }
}
