use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Result, Token, TypePath, Visibility,
};

struct CallbackTypes {
    pub interface: TypePath,
    pub arg_1: TypePath,
    pub arg_2: TypePath,
}

impl Parse for CallbackTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let args: Punctuated<TypePath, Token![,]> = content.parse_terminated(TypePath::parse)?;
        input.parse::<Token![;]>()?;
        let mut args = args.into_iter();
        if let (Some(interface), Some(arg_1), Some(arg_2)) = (args.next(), args.next(), args.next())
        {
            Ok(CallbackTypes {
                interface,
                arg_1,
                arg_2,
            })
        } else {
            Err(content.error("expected (interface, arg_1, arg_2)"))
        }
    }
}

struct CallbackStruct {
    pub vis: Visibility,
    _struct_token: Token![struct],
    pub ident: Ident,
    pub args: CallbackTypes,
}

impl Parse for CallbackStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CallbackStruct {
            vis: input.parse()?,
            _struct_token: input.parse()?,
            ident: input.parse()?,
            args: input.parse()?,
        })
    }
}

/// Implement a `CompletedCallback` using the types specified as tuple struct fields.
#[proc_macro_attribute]
pub fn completed_callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CallbackStruct);
    impl_completed_callback(&ast)
}

fn impl_completed_callback(ast: &CallbackStruct) -> TokenStream {
    let vis = &ast.vis;

    let name = &ast.ident;
    let closure = get_closure(name);
    let interface = &ast.args.interface;

    let arg_1 = &ast.args.arg_1;
    let arg_2 = &ast.args.arg_2;

    let msg = match interface.path.segments.last() {
        Some(interface) => format!("Implementation of [`{}`].", interface.ident),
        None => String::from("Implementation of unknown [`completed_callback`] interface.")
    };

    let gen = quote! {
        use windows as _;

        type #closure = CompletedClosure<#arg_1, #arg_2>;

        #[doc = #msg]
        #[implement(Microsoft::Web::WebView2::Win32::#interface)]
        #vis struct #name(Option<#closure>);

        #[allow(non_snake_case)]
        impl #name {
            pub fn create(
                closure: #closure,
            ) -> #interface {
                Self(Some(closure)).into()
            }

            pub fn wait_for_async_operation(
                closure: Box<
                    dyn FnOnce(#interface) -> crate::Result<()>,
                >,
                completed: #closure,
            ) -> crate::Result<()> {
                let (tx, rx) = mpsc::channel();
                let completed: #closure =
                    Box::new(move |arg_1, arg_2| -> ::windows::Result<()> {
                        let result = completed(arg_1, arg_2).map_err(crate::Error::WindowsError);
                        tx.send(result).expect("send over mpsc channel");
                        Ok(())
                    });
                let callback = Self::create(completed);

                closure(callback)?;
                wait_with_pump(rx)?
            }

            fn Invoke<'a>(
                &mut self,
                arg_1: <#arg_1 as InvokeArg<'a>>::Input,
                arg_2: <#arg_2 as InvokeArg<'a>>::Input,
            ) -> ::windows::Result<()> {
                match self.0.take() {
                    Some(completed) => completed(
                        <#arg_1 as InvokeArg<'a>>::convert(arg_1),
                        <#arg_2 as InvokeArg<'a>>::convert(arg_2),
                    ),
                    None => Ok(()),
                }
            }
        }
    };

    gen.into()
}

/// Implement an `EventCallback` using the types specified as tuple struct fields.
#[proc_macro_attribute]
pub fn event_callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CallbackStruct);
    impl_event_callback(&ast)
}

fn impl_event_callback(ast: &CallbackStruct) -> TokenStream {
    let vis = &ast.vis;

    let name = &ast.ident;
    let closure = get_closure(name);

    let interface = &ast.args.interface;

    let arg_1 = &ast.args.arg_1;
    let arg_2 = &ast.args.arg_2;

    let msg = match interface.path.segments.last() {
        Some(interface) => format!("Implementation of [`{}`].", interface.ident),
        None => String::from("Implementation of unknown [`event_callback`] interface.")
    };

    let gen = quote! {
        type #closure = EventClosure<#arg_1, #arg_2>;

        #[doc = #msg]
        #[implement(Microsoft::Web::WebView2::Win32::#interface)]
        #vis struct #name(#closure);

        #[allow(non_snake_case)]
        impl #name {
            pub fn create(
                closure: #closure,
            ) -> #interface {
                Self(closure).into()
            }

            fn Invoke<'a>(
                &mut self,
                arg_1: <#arg_1 as InvokeArg<'a>>::Input,
                arg_2: <#arg_2 as InvokeArg<'a>>::Input,
            ) -> ::windows::Result<()> {
                self.0(
                    <#arg_1 as InvokeArg<'a>>::convert(arg_1),
                    <#arg_2 as InvokeArg<'a>>::convert(arg_2),
                )
            }
        }
    };

    gen.into()
}

fn get_closure(name: &Ident) -> Ident {
    format_ident!("{}Closure", name)
}
