use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn gen_permutation(input: TokenStream) -> TokenStream {
  let n = parse_macro_input!(input as LitInt)
    .base10_parse::<usize>()
    .expect("Please provide an integer for the tuple size");

  let type_idents: Vec<_> = (0..n).map(|i| format_ident!("A{}", i)).collect();
  let var_idents: Vec<_> = (0..n).map(|i| format_ident!("a{}", i)).collect();
  let fn_name = format_ident!("permutation{}", n);

  let mut all_permutations = Vec::new();

  for p in (0..n).permutations(n) {
    let mut block = quote! {
        return Ok((remaining, (#( #var_idents ),*)));
    };

    for &idx in p.iter().rev() {
      let current_var = &var_idents[idx];
      let parser_idx = syn::Index::from(idx);
      block = quote! {
          if let Ok((remaining, #current_var)) = self.parsers.#parser_idx.parse(remaining) {
              #block
          }
      };
    }
    all_permutations.push(block);
  }

  let expanded = quote! {
      impl<#(#type_idents),*> Parser for Permutation<(#(#type_idents,)*)>
      where
          #(#type_idents: Parser),*
      {
          type Dest = (#(#type_idents::Dest,)*);

          fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
              let remaining = input;

              #( #all_permutations )*

              Err(())
          }
      }

      pub fn #fn_name <#(#type_idents: Parser),*>(
          #( #var_idents: #type_idents ),*
      ) -> Permutation<(#(#type_idents,)*)> {
          Permutation { parsers: (#(#var_idents,)*) }
      }
  };

  TokenStream::from(expanded)
}
