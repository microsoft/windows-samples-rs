use std::sync::mpsc;

use windows::{implement, IUnknown, Interface, HRESULT};

use bindings::{
    Microsoft::{self, Web::WebView2::Win32::*},
    Windows::{
        self,
        Win32::{
            Foundation::{BOOL, PWSTR},
            Storage::StructuredStorage::IStream,
        },
    },
};

use super::{pwstr::string_from_pwstr, wait_with_pump};

pub trait ClosureArg {
    type Output: Sized;
}

pub trait InvokeArg<'a> {
    type Input: 'a;

    fn convert(input: Self::Input) -> <Self as ClosureArg>::Output
    where
        Self: ClosureArg;
}

impl ClosureArg for HRESULT {
    type Output = windows::Result<()>;
}

impl<'a> InvokeArg<'a> for HRESULT {
    type Input = Self;

    fn convert(input: HRESULT) -> windows::Result<()> {
        input.ok()
    }
}

impl ClosureArg for BOOL {
    type Output = Self;
}

impl<'a> InvokeArg<'a> for BOOL {
    type Input = Self;

    fn convert(input: BOOL) -> BOOL {
        input
    }
}

impl ClosureArg for PWSTR {
    type Output = String;
}

impl<'a> InvokeArg<'a> for PWSTR {
    type Input = Self;

    fn convert(input: PWSTR) -> String {
        string_from_pwstr(input)
    }
}

impl<I: Interface> ClosureArg for Option<I> {
    type Output = Self;
}

impl<'a, I: 'a + Interface> InvokeArg<'a> for Option<I> {
    type Input = &'a Self;

    fn convert(input: &'a Self) -> <Self as ClosureArg>::Output {
        input.clone()
    }
}

/// Generic closure signature for [`completed_callback`].
pub type CompletedClosure<Arg1, Arg2> = Box<
    dyn FnOnce(<Arg1 as ClosureArg>::Output, <Arg2 as ClosureArg>::Output) -> ::windows::Result<()>,
>;

/// Generic closure signature for [`event_callback`].
pub type EventClosure<Arg1, Arg2> = Box<
    dyn FnMut(<Arg1 as ClosureArg>::Output, <Arg2 as ClosureArg>::Output) -> windows::Result<()>,
>;

#[completed_callback]
pub struct CreateCoreWebView2EnvironmentCompletedHandler(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    HRESULT,
    Option<ICoreWebView2Environment>,
);

#[completed_callback]
pub struct CreateCoreWebView2ControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    HRESULT,
    Option<ICoreWebView2Controller>,
);

#[event_callback]
pub struct NewBrowserVersionAvailableEventHandler(
    ICoreWebView2NewBrowserVersionAvailableEventHandler,
    Option<ICoreWebView2Environment>,
    Option<IUnknown>,
);

#[completed_callback]
pub struct CreateCoreWebView2CompositionControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2CompositionControllerCompletedHandler,
    HRESULT,
    Option<ICoreWebView2CompositionController>,
);

#[event_callback]
pub struct ZoomFactorChangedEventHandler(
    ICoreWebView2ZoomFactorChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct MoveFocusRequestedEventHandler(
    ICoreWebView2MoveFocusRequestedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<ICoreWebView2MoveFocusRequestedEventArgs>,
);

#[event_callback]
pub struct FocusChangedEventHandler(
    ICoreWebView2FocusChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct AcceleratorKeyPressedEventHandler(
    ICoreWebView2AcceleratorKeyPressedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<ICoreWebView2AcceleratorKeyPressedEventArgs>,
);

#[event_callback]
pub struct RasterizationScaleChangedEventHandler(
    ICoreWebView2RasterizationScaleChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct NavigationStartingEventHandler(
    ICoreWebView2NavigationStartingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationStartingEventArgs>,
);

#[event_callback]
pub struct ContentLoadingEventHandler(
    ICoreWebView2ContentLoadingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ContentLoadingEventArgs>,
);

#[event_callback]
pub struct SourceChangedEventHandler(
    ICoreWebView2SourceChangedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2SourceChangedEventArgs>,
);

#[event_callback]
pub struct HistoryChangedEventHandler(
    ICoreWebView2HistoryChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    ICoreWebView2NavigationCompletedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationCompletedEventArgs>,
);

#[event_callback]
pub struct ScriptDialogOpeningEventHandler(
    ICoreWebView2ScriptDialogOpeningEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ScriptDialogOpeningEventArgs>,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    ICoreWebView2PermissionRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2PermissionRequestedEventArgs>,
);

#[event_callback]
pub struct ProcessFailedEventHandler(
    ICoreWebView2ProcessFailedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ProcessFailedEventArgs>,
);

#[completed_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    HRESULT,
    PWSTR,
);

#[completed_callback]
pub struct ExecuteScriptCompletedHandler(
    ICoreWebView2ExecuteScriptCompletedHandler,
    HRESULT,
    PWSTR,
);

type CapturePreviewCompletedHandlerClosure =
    Box<dyn FnOnce(<HRESULT as ClosureArg>::Output) -> ::windows::Result<()>>;

/// Implementation of [`ICoreWebView2CapturePreviewCompletedHandler`].
///
/// This interface is unique in that it only takes 1 parameter, which is why
/// it does not use the [`completed_callback`] macro for its implementation.
#[implement(Microsoft::Web::WebView2::Win32::ICoreWebView2CapturePreviewCompletedHandler)]
pub struct CapturePreviewCompletedHandler(Option<CapturePreviewCompletedHandlerClosure>);

#[allow(non_snake_case)]
impl CapturePreviewCompletedHandler {
    pub fn create(
        closure: CapturePreviewCompletedHandlerClosure,
    ) -> ICoreWebView2CapturePreviewCompletedHandler {
        Self(Some(closure)).into()
    }

    pub fn wait_for_async_operation(
        closure: Box<dyn FnOnce(ICoreWebView2CapturePreviewCompletedHandler) -> crate::Result<()>>,
        completed: CapturePreviewCompletedHandlerClosure,
    ) -> crate::Result<()> {
        let (tx, rx) = mpsc::channel();
        let completed: CapturePreviewCompletedHandlerClosure =
            Box::new(move |arg_1| -> ::windows::Result<()> {
                let result = completed(arg_1).map_err(crate::Error::WindowsError);
                tx.send(result).expect("send over mpsc channel");
                Ok(())
            });
        let callback = Self::create(completed);

        closure(callback)?;
        wait_with_pump(rx)?
    }

    fn Invoke<'a>(&mut self, arg_1: <HRESULT as InvokeArg<'a>>::Input) -> ::windows::Result<()> {
        match self.0.take() {
            Some(completed) => completed(<HRESULT as InvokeArg<'a>>::convert(arg_1)),
            None => Ok(()),
        }
    }
}

#[event_callback]
pub struct WebMessageReceivedEventHandler(
    ICoreWebView2WebMessageReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebMessageReceivedEventArgs>,
);

#[completed_callback]
pub struct CallDevToolsProtocolMethodCompletedHandler(
    ICoreWebView2CallDevToolsProtocolMethodCompletedHandler,
    HRESULT,
    PWSTR,
);

#[event_callback]
pub struct NewWindowRequestedEventHandler(
    ICoreWebView2NewWindowRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NewWindowRequestedEventArgs>,
);

#[event_callback]
pub struct DocumentTitleChangedEventHandler(
    ICoreWebView2DocumentTitleChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct ContainsFullScreenElementChangedEventHandler(
    ICoreWebView2ContainsFullScreenElementChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    ICoreWebView2WebResourceRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebResourceRequestedEventArgs>,
);

#[event_callback]
pub struct WebResourceResponseReceivedEventHandler(
    ICoreWebView2WebResourceResponseReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebResourceResponseReceivedEventArgs>,
);

#[completed_callback]
pub struct WebResourceResponseViewGetContentCompletedHandler(
    ICoreWebView2WebResourceResponseViewGetContentCompletedHandler,
    HRESULT,
    Option<IStream>,
);

#[event_callback]
pub struct WindowCloseRequestedEventHandler(
    ICoreWebView2WindowCloseRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct DownloadStartingEventHandler(
    ICoreWebView2DownloadStartingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2DownloadStartingEventArgs>,
);

#[event_callback]
pub struct BytesReceivedChangedEventHandler(
    ICoreWebView2BytesReceivedChangedEventHandler,
    Option<ICoreWebView2DownloadOperation>,
    Option<IUnknown>,
);

#[event_callback]
pub struct EstimatedEndTimeChangedEventHandler(
    ICoreWebView2EstimatedEndTimeChangedEventHandler,
    Option<ICoreWebView2DownloadOperation>,
    Option<IUnknown>,
);

#[event_callback]
pub struct StateChangedEventHandler(
    ICoreWebView2StateChangedEventHandler,
    Option<ICoreWebView2DownloadOperation>,
    Option<IUnknown>,
);

#[event_callback]
pub struct DevToolsProtocolEventReceivedEventHandler(
    ICoreWebView2DevToolsProtocolEventReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2DevToolsProtocolEventReceivedEventArgs>,
);

#[event_callback]
pub struct FrameCreatedEventHandler(
    ICoreWebView2FrameCreatedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2FrameCreatedEventArgs>,
);

#[event_callback]
pub struct FrameDestroyedEventHandler(
    ICoreWebView2FrameDestroyedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<IUnknown>,
);

#[event_callback]
pub struct FrameNameChangedEventHandler(
    ICoreWebView2FrameNameChangedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<IUnknown>,
);

#[completed_callback]
pub struct GetCookiesCompletedHandler(
    ICoreWebView2GetCookiesCompletedHandler,
    HRESULT,
    Option<ICoreWebView2CookieList>,
);

#[completed_callback]
pub struct TrySuspendCompletedHandler(ICoreWebView2TrySuspendCompletedHandler, HRESULT, BOOL);
