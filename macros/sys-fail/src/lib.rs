use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

const QUICK_MSG: &str = "#[sys_fail] systems have no return types.";

#[proc_macro_attribute]
pub fn sys_fail(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as syn::ItemFn);
    match sys_fail_impl(function) {
        Ok(token_stream) => token_stream.into(),
        Err(syn_error) => syn_error.into_compile_error().into(),
    }
}

fn sys_fail_impl(mut function: syn::ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    if !matches!(function.sig.output, syn::ReturnType::Default) {
        return Err(syn::Error::new_spanned(function.sig.output, QUICK_MSG));
    }

    let body = &function.block.stmts;
    let vis = &function.vis;
    let fn_ident = &function.sig.ident;

    // Add comma at end so that we can add the #extra_params
    if !function.sig.inputs.is_empty() && !function.sig.inputs.trailing_punct() {
        function.sig.inputs.push_punct(syn::token::Comma::default());
    }

    let params = &function.sig.inputs;
    let params_gen = &function.sig.generics.params;
    let where_gen = &function.sig.generics.where_clause;
    let attrs = &function.attrs;

    let extra_param =
        quote!(__sysfail_params: bevy::ecs::system::StaticSystemParam<bevy::prelude::Commands>);
    let commands = quote!(let mut commands = __sysfail_params.into_inner(););

    Ok(quote! {
        #(#attrs)*
        #vis fn #fn_ident <#params_gen> (#params #extra_param) #where_gen {
            use exit::ExitCommands;
            let mut inner_system = move || -> anyhow::Result<()> {
                #(#body)*
                return ::core::result::Result::Ok(());
            };
            if let Err(err) = inner_system() {
                #commands
                commands.exit_error(err);
            }
        }
    })
}
