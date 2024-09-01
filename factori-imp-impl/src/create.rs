use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Ident, Token};

use super::{ident_builder, ident_mixins_enum};

/// e.g. create!(ty, :mixin1, :mixin2, field1: value1, field2: value2)
///
/// ... becomes:
///
/// Create {
///   ty: 'ty',
///   mixins: vec!['mixin1', 'mixin2'],
///   fields: vec!['field1', 'field2'],
///   values: vec!['value1', 'value2'],
/// }
///
/// fields and values can also be the transient ones
struct Create {
  ty: Ident,
  mixins: Vec<Ident>,
  fields: Vec<Ident>,
  values: Vec<Expr>,
}

impl Create {
  /// Parses the rest of the create macro input
  ///
  /// This helps use it in other macros like
  /// create!(ty, <input>);
  /// create_vec!(ty, count, <input>);
  ///
  /// ty is extracted and input is the rest of the token stream
  /// in effect it parses everything after `ty,` or `ty, count,`
  fn build_after_type(ty: Ident, input: ParseStream) -> Result<Self> {
    if input.peek(Token![,]) {
      input.parse::<Token![,]>()?;
    }

    let mut mixins = Vec::new();
    while input.peek(Token![:]) {
      input.parse::<Token![:]>()?;
      mixins.push(input.parse()?);

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    let mut fields = Vec::new();
    let mut values = Vec::new();
    loop {
      if input.is_empty() {
        break;
      }

      fields.push(input.parse()?);
      input.parse::<Token![:]>()?;
      values.push(input.parse()?);

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Create {
      ty,
      mixins,
      fields,
      values,
    })
  }

  /// Generates the code for its create!(...) call
  fn generate_code(&self) -> proc_macro2::TokenStream {
    let Self {
      ty,
      mixins,
      fields,
      values,
    } = self;

    let ident_builder = ident_builder(ty);
    let ident_mixins_enum = ident_mixins_enum(ty);

    let mut mixins = mixins.iter();
    let value = if let Some(mixin) = mixins.next() {
      let initial = quote! {
          factori::Mixin::default(#ident_mixins_enum::#mixin)
      };
      mixins.fold(initial, |acc, mixin| {
        quote! {
            factori::Mixin::extend(#ident_mixins_enum::#mixin, #acc)
        }
      })
    } else {
      quote! { factori::Default::default () }
    };

    let quoted = quote! {
        factori::Builder::build(
          #[allow(clippy::needless_update)]
          #ident_builder {
            #(
                #fields: #values,
            )*
            .. #value
        })
    };

    quoted
  }
}

impl Parse for Create {
  fn parse(input: ParseStream) -> Result<Self> {
    let ty = input.parse()?;

    Self::build_after_type(ty, input)
  }
}

pub fn create_macro(input: TokenStream) -> TokenStream {
  let create: Create = parse_macro_input!(input);
  create.generate_code().into()
}

/// e.g. create_vec!(ty, 3, :mixin1, :mixin2, field1: value1, field2: value2)
///
/// ... becomes:
///
/// CreateVec {
///   ty: 'ty',
///   count: 3,
///   create: Create {
///     ty: 'ty',
///     mixins: vec!['mixin1', 'mixin2'],
///     fields: vec!['field1', 'field2'],
///     values: vec!['value1', 'value2'],
///   }
/// }
struct CreateVec {
  ty: Ident,
  count: Expr,
  create: Create,
}

impl Parse for CreateVec {
  fn parse(input: ParseStream) -> Result<Self> {
    let ty: Ident = input.parse()?;

    input.parse::<Token![,]>()?;
    let count = input.parse()?;

    let create = Create::build_after_type(ty.clone(), input)?;

    Ok(CreateVec { ty, count, create })
  }
}

/// Generates the code for a vec of count the factory
///
/// ```
/// // we basically want from
/// let users = create_vec!(User, 4, :mixin, name: "blah");
/// // to generate the following code
/// let users = (0..4).iter()
///   .map(|_| code_from_create_generate_code)
///   .collect<Vec<User>>();
/// ```
pub fn create_vec_macro(input: TokenStream) -> TokenStream {
  let CreateVec { ty, count, create } = parse_macro_input!(input);

  let create_code = create.generate_code();

  let quoted = quote! {
    (0..#count).map(|_| #create_code).collect::<Vec<#ty>>()
  };

  quoted.into()
}
