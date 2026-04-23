use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
};

pub struct AddrMapAttr {
    pub path: LitStr,
    pub _comma2: Option<Token![,]>,
    pub _extern: Option<Token![extern]>,
    pub default_abi: Option<LitStr>,
}

impl Parse for AddrMapAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AddrMapAttr {
            path: input.parse()?,
            _comma2: input.parse().ok(),
            _extern: input.parse().ok(),
            default_abi: input.parse().ok(),
        })
    }
}
