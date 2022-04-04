use crate::{core_impl::BindgenArgType, ImplItemMethodInfo, MethodType};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ReturnType;

use super::TypeRegistry;

impl ImplItemMethodInfo {
    /// Generates metadata struct for this method.
    ///
    /// # Example:
    /// The following method:
    /// ```ignore
    /// fn f3(&mut self, arg0: FancyStruct, arg1: u64) -> Result<IsOk, Error> { }
    /// ```
    /// will produce this struct:
    /// ```ignore
    /// near_sdk::MethodMetadata {
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
        let args: Vec<TokenStream2> = self
            .attr_signature_info
            .input_args()
            .map(|arg| {
                let type_id = registry.register_type(Box::new(arg.ty.clone()));
                quote! { #type_id }
            })
            .collect();
        let callbacks: Vec<TokenStream2> = self
            .attr_signature_info
            .args
            .iter()
            .filter(|arg| matches!(arg.bindgen_ty, BindgenArgType::CallbackArg))
            .map(|arg| {
                let type_id = registry.register_type(Box::new(arg.ty.clone()));
                quote! { #type_id }
            })
            .collect();
        let callbacks_vec: Vec<TokenStream2> = self
            .attr_signature_info
            .args
            .iter()
            .filter(|arg| matches!(arg.bindgen_ty, BindgenArgType::CallbackArgVec))
            .map(|arg| {
                let type_id = registry.register_type(Box::new(arg.ty.clone()));
                quote! { #type_id }
            })
            .collect();
        let result = match &self.attr_signature_info.returns {
            ReturnType::Default => {
                quote! {
                    None
                }
            }
            ReturnType::Type(_, ty) => {
                let type_id = registry.register_type(ty.clone());
                quote! {
                    Some(#type_id)
                }
            }
        };

        quote! {
             near_sdk::MethodMetadata {
                 name: #method_name_str.to_string(),
                 is_view: #is_view,
                 is_init: #is_init,
                 args: vec![#(#args),*],
                 callbacks: vec![#(#callbacks),*],
                 callbacks_vec: vec![#(#callbacks_vec),*],
                 result: #result
             }
        }
    }
}
