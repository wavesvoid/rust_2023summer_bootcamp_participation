use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Result, Token,
};


struct Bmt {
    pairs: Vec<(Expr, Expr)>,
}

impl Parse for Bmt {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut pairs = vec![];

        loop {
            if input.is_empty() {
                break;
            }

            let (key, __arrow, val, __trailing_comma) = (
                input.parse::<Expr>()?,
                input.parse::<Token![=>]>()?,
                input.parse::<Expr>()?,
                input.parse::<Token![,]>(),
            );

            pairs.push((key, val));
        }

        Ok(Self { pairs })
    }
}

#[proc_macro]
pub fn btm(ts: TokenStream) -> TokenStream {
    let (qd1, qd2): (Vec<_>, Vec<_>) = parse_macro_input!(ts as Bmt)
        .pairs
        .into_iter()
        .unzip();

    quote! {
        {
            let mut bt = ::std::collections::BTreeMap::new();
            #( bt.insert(#qd1, #qd2); )*
            bt
        }
    }
    .into()
}
