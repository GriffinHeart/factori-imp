use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parse_macro_input, Expr, Token, Type};

use super::{ident_builder, ident_mixins_enum};

struct DefaultBlock {
  fields: Vec<Ident>,
  types: Vec<Option<Type>>,
  values: Vec<Expr>,
}

impl Parse for DefaultBlock {
  fn parse(input: ParseStream) -> Result<Self> {
    let inner;
    braced!(inner in input);

    let mut fields = Vec::new();
    let mut types = Vec::new();
    let mut values = Vec::new();

    loop {
      if inner.is_empty() {
        break;
      }

      fields.push(inner.parse()?);

      // Optional type. If it's specified for one field it needs to be specified for all.
      // Should be specified only if there is a builder {} block.
      // This is enforced in Definition::validate().
      if inner.peek(Token![:]) {
        inner.parse::<Token![:]>()?;
        types.push(Some(inner.parse()?));
      } else {
        types.push(None);
      }

      inner.parse::<Token![=]>()?;
      values.push(inner.parse()?);

      if inner.peek(Token![,]) {
        inner.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      fields,
      types,
      values,
    })
  }
}

struct MixinBlock {
  name: Ident,
  fields: Vec<Ident>,
  values: Vec<Expr>,
}

impl Parse for MixinBlock {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse()?;

    let inner;
    braced!(inner in input);

    let mut fields = Vec::new();
    let mut values = Vec::new();

    loop {
      if inner.is_empty() {
        break;
      }

      fields.push(inner.parse()?);
      inner.parse::<Token![=]>()?;
      values.push(inner.parse()?);

      if inner.peek(Token![,]) {
        inner.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      name,
      fields,
      values,
    })
  }
}

struct TransientBlock {
  fields: Vec<Ident>,
  values: Vec<Expr>,
  types: Vec<Type>,
}

impl Parse for TransientBlock {
  fn parse(input: ParseStream) -> Result<Self> {
    let inner;
    braced!(inner in input);

    let mut fields = Vec::new();
    let mut values = Vec::new();
    let mut types = Vec::new();

    loop {
      if inner.is_empty() {
        break;
      }

      // parse a: type = value and  take ending , if there
      fields.push(inner.parse()?); // a
      inner.parse::<Token![:]>()?; // :
      types.push(inner.parse()?); // type
      inner.parse::<Token![=]>()?; // =
      values.push(inner.parse()?); // value
      if inner.peek(Token![,]) {
        // maybe ,
        inner.parse::<Token![,]>()?;
      }
    }

    Ok(Self {
      fields,
      values,
      types,
    })
  }
}

struct Definition {
  ty: Ident,

  default: DefaultBlock,
  transient: Option<TransientBlock>,
  builder: Option<TokenTree>,
  mixins: Vec<MixinBlock>,
}

impl Parse for Definition {
  fn parse(input: ParseStream) -> Result<Self> {
    let ty = input.parse()?;
    input.parse::<Token![,]>()?;

    let inner;
    braced!(inner in input);

    let mut default: Option<DefaultBlock> = None;
    let mut transient: Option<TransientBlock> = None;
    let mut builder = None;
    let mut mixins = Vec::new();

    loop {
      if inner.is_empty() {
        break;
      }

      let key: Ident = inner.parse()?;
      if key == "default" {
        if default.is_some() {
          return Err(inner.error("default {} block defined twice"));
        }
        default = Some(inner.parse()?);
      } else if key == "builder" {
        if builder.is_some() {
          return Err(inner.error("builder {} block is defined twice"));
        }
        builder = Some(inner.parse()?);
      } else if key == "mixin" {
        mixins.push(inner.parse()?);
      } else if key == "transient" {
        if transient.is_some() {
          return Err(inner.error("transient {} block defined twice"));
        }
        transient = Some(inner.parse()?);
      }
    }

    let default = default.ok_or_else(|| inner.error("missing default {} block"))?;

    Ok(Self {
      ty,
      default,
      builder,
      mixins,
      transient,
    })
  }
}

impl Definition {
  fn validate(&self) -> Option<TokenStream> {
    let missing_type = self
      .default
      .fields
      .iter()
      .zip(&self.default.types)
      .filter(|(_, ty)| ty.is_none())
      .next();

    if let Some((name, _)) = missing_type {
      if self.builder.is_some() {
        let error = syn::Error::new(
          name.span(),
          "Type must be specified if using a custom `builder {}` block.",
        )
        .to_compile_error();

        return Some(error);
      }
    }

    None
  }

  fn generate_transient_parts(&self) -> (TokenStream, TokenStream, TokenStream) {
    if let Some(transient) = &self.transient {
      let trans_fields = &transient.fields;
      let trans_types = &transient.types;
      let trans_values = &transient.values;

      (
        quote! {
          #( pub #trans_fields: #trans_types ),*
        },
        quote! {
          #( #trans_fields: #trans_values ),*
        },
        quote! {
          #(
            #[allow(unused_variable)]
            let #trans_fields = self.#trans_fields;
          )*
        },
      )
    } else {
      (quote! {}, quote! {}, quote! {})
    }
  }

  fn generate_builder(&self) -> TokenStream {
    let ident_builder = ident_builder(&self.ty);

    let ty = &self.ty;
    let fields = &self.default.fields;
    let types = &self.default.types;
    let values = &self.default.values;

    let (transient_field_decl, transient_default_values, transient_build_group) =
      self.generate_transient_parts();

    match &self.builder {
      None => {
        quote! {
            #[allow(non_camel_case_types)]
            pub type #ident_builder = #ty;

            impl factori::Default for #ident_builder {
                fn default() -> Self {
                    #ty {
                        #( #fields: #values ),*
                    }
                }
            }

            impl factori::Builder for #ident_builder {
                type Ty = #ty;

                fn build(self) -> Self::Ty {
                    self
                }
            }
        }
      }

      Some(builder) => {
        quote! {
            #[allow(non_camel_case_types, dead_code)]
            pub struct #ident_builder {
                #( pub #fields: #types ),*
                ,
                #transient_field_decl
            }

            impl factori::Default for #ident_builder {
                fn default() -> Self {
                    #ident_builder {
                        #( #fields: #values ),*
                        ,
                        #transient_default_values
                    }
                }
            }

            impl factori::Builder for #ident_builder {
                type Ty = #ty;

                fn build(self) -> Self::Ty {
                    #(
                        #[allow(unused_variable)]
                        let #fields = self.#fields;
                    )*
                    #transient_build_group

                    #builder
                }
            }
        }
      }
    }
  }

  fn generate_mixins(&self) -> TokenStream {
    let ident_builder = ident_builder(&self.ty);
    let ident_mixins_enum = ident_mixins_enum(&self.ty);

    let idents_builder = &ident_builder;
    let idents_mixins_enum = &ident_mixins_enum;

    let mixin_names: Vec<_> = self.mixins.iter().map(|mixin| &mixin.name).collect();
    let mixin_fields: Vec<_> = self.mixins.iter().map(|mixin| &mixin.fields).collect();
    let mixin_values: Vec<_> = self.mixins.iter().map(|mixin| &mixin.values).collect();

    quote! {
        #[allow(non_camel_case_types)]
        pub enum #ident_mixins_enum {
            #( #mixin_names ),*
        }

        impl factori::Mixin<#ident_builder> for #ident_mixins_enum {
            fn default(self) -> #ident_builder {
                self.extend(factori::Default::default())
            }

            #[allow(unused_variable)]
            fn extend(self, other: #ident_builder) -> #ident_builder {
                match self {
                    #(
                        #idents_mixins_enum::#mixin_names => {
                            #idents_builder {
                                #(
                                    #mixin_fields: #mixin_values
                                ),* ,
                                .. other
                            }
                        }
                    ),*
                }
            }
        }
    }
  }

  fn into_token_stream(&self) -> TokenStream {
    let builder = self.generate_builder();
    let mixins = self.generate_mixins();

    quote! {
        #builder
        #mixins
    }
  }
}

struct MultipleDefinition {
  definitions: Vec<Definition>,
}

impl Parse for MultipleDefinition {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut definitions = Vec::new();

    loop {
      if input.is_empty() {
        break;
      }
      definitions.push(input.parse()?);
    }

    Ok(Self { definitions })
  }
}

pub fn define_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let MultipleDefinition { definitions } = parse_macro_input!(input);

  let mut stream = TokenStream::new();
  for definition in definitions {
    if let Some(error) = definition.validate() {
      return error.into();
    }
    stream.extend(definition.into_token_stream());
  }

  stream.into()
}
