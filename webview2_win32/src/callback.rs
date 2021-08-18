use std::sync::mpsc;

use windows::{implement, HRESULT};

use bindings::{
    Microsoft::{self, Web::WebView2::Win32 as WebView2},
    Windows::{self, Win32::Foundation::PWSTR},
};

use super::{pwstr::string_from_pwstr, wait_with_pump};

/// Generic closure signature for [`CompletedCallback`].
pub type CompletedClosure<Arg1, Arg2> = Box<dyn FnOnce(Arg1, Arg2) -> ::windows::Result<()>>;

/// Generic closure signature for [`EventCallback`].
pub type EventClosure<Arg1, Arg2> = Box<dyn FnMut(Arg1, Arg2) -> windows::Result<()>>;

#[completed_callback]
pub struct CreateCoreWebView2EnvironmentCompletedHandler(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    HRESULT,
    WebView2::ICoreWebView2Environment,
);

#[completed_callback]
pub struct CreateCoreWebView2ControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    HRESULT,
    WebView2::ICoreWebView2Controller,
);

#[event_callback]
pub struct WebMessageReceivedEventHandler(
    ICoreWebView2WebMessageReceivedEventHandler,
    WebView2::ICoreWebView2,
    WebView2::ICoreWebView2WebMessageReceivedEventArgs,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    ICoreWebView2WebResourceRequestedEventHandler,
    WebView2::ICoreWebView2,
    WebView2::ICoreWebView2WebResourceRequestedEventArgs,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    ICoreWebView2PermissionRequestedEventHandler,
    WebView2::ICoreWebView2,
    WebView2::ICoreWebView2PermissionRequestedEventArgs,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    ICoreWebView2NavigationCompletedEventHandler,
    WebView2::ICoreWebView2,
    WebView2::ICoreWebView2NavigationCompletedEventArgs,
);

#[string_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
);

#[string_callback]
pub struct ExecuteScriptCompletedHandler(ICoreWebView2ExecuteScriptCompletedHandler);
