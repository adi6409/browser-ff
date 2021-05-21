//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIPrintingPromptService.idl
//


/// `interface nsIPrintingPromptService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrintingPromptService {
    vtable: *const nsIPrintingPromptServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrintingPromptService.
unsafe impl XpCom for nsIPrintingPromptService {
    const IID: nsIID = nsID(0x72006d06, 0xa2a5, 0x4250,
        [0xae, 0x92, 0x04, 0xb2, 0xf0, 0xe2, 0xab, 0x8d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrintingPromptService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrintingPromptService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrintingPromptServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPrintingPromptService`.
    fn coerce_from(v: &nsIPrintingPromptService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrintingPromptServiceCoerce for nsIPrintingPromptService {
    #[inline]
    fn coerce_from(v: &nsIPrintingPromptService) -> &Self {
        v
    }
}

impl nsIPrintingPromptService {
    /// Cast this `nsIPrintingPromptService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrintingPromptServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrintingPromptService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIPrintingPromptServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintingPromptService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrintingPromptService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrintingPromptServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void showPrintDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings); */
    pub ShowPrintDialog: unsafe extern "system" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings) -> ::nserror::nsresult,

    /* void showPrintProgressDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen); */
    pub ShowPrintProgressDialog: unsafe extern "system" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings, openDialogObserver: *const nsIObserver, isForPrinting: bool, webProgressListener: *mut *const nsIWebProgressListener, printProgressParams: *mut *const nsIPrintProgressParams, notifyOnOpen: *mut bool) -> ::nserror::nsresult,

    /* void showPageSetupDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings); */
    pub ShowPageSetupDialog: unsafe extern "system" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrintingPromptService {

    /// ```text
    /// /**
    ///    *  This service enables embedders to implement their own Print and Progress Dialogs.
    ///    *  Each platform has a "base" or "basckstop" implementation of the service. The
    ///    *  service is automatically registered at start up.
    ///    *
    ///    *  Historically, platform toolkits with native dialogs have implemented them in the GFX layer
    ///    *  Usually they were displayed when a new DeviceContextSpec specific to that platform
    ///    *  was created.
    ///    *
    ///    *  Windows: The GFX layer no longers supports default toolkit behavior for displaying the
    ///    *           native Print Dialog.
    ///    *           If an embedder implemented service returns any error code (other than NS_ERROR_ABORT)
    ///    *           printing will terminate.
    ///    *
    ///    *           Returning NS_OK assumes that the PrintSettings object was correctly filled in and
    ///    *           if it does not have valid fields for printer name, etc. it may also terminate.
    ///    *
    ///    *           Defaults for platform service:
    ///    *             showPrintDialog           - displays a native dialog
    ///    *             showPrintProgressDialog   - displays a XUL dialog
    ///    *             showPageSetupDialog       - displays a XUL dialog
    ///    *
    ///    *           Summary for Windows Embedders:
    ///    *             Stated once again: There is no "fallback" native platform support in GFX for the
    ///    *             displaying of the native print dialog. The current default implementation for Windows
    ///    *             display a native print dialog but a XUL-based progress dialog.
    ///    *             If you wish to have a native progress dialog on Windows you will have to create and
    ///    *             register your own service.
    ///    *
    ///    *             Note: The Windows version Mozilla implements this service which is
    ///    *                   automatically built and registered for you. You can use it as an example.
    ///    *                   It is located at "widget/windows/nsPrintDialogService.cpp". That service
    ///    *                   is capable of displaying a native print dialog and a XUL progress dialog.
    ///    *
    ///    *             To fly your own dialog you may:
    ///    *
    ///    *              1) Implement this service to display at least the Print Dialog and a Print Progress Dialog
///    *                 or you may implement just one of the dialogs and pass back NS_ERROR_NOT_IMPLEMENTED
///    *                 for any of the others.
///    *
///    *              2) For the Print Dialog:
///    *                 You may stub out this service by having all the methods return NS_ERROR_NOT_IMPLEMENTED.
///    *                 You can then fly you own dialog and then properly fill in the PrintSettings object
///    *                 before calling nsIWebBrowserPrint's Print method. If you stub out this service
///    *                 you MUST set "printSilent" to true, if you do not, Printing will terminate and an
///    *                 error dialog will be displayed.
///    *
///    *  Mac: The GFX layer still supports default toolkit behavior for displaying the Print Dialog.
///    *       If an embedder implemented service returns NS_ERROR_NOT_IMPLEMENTED for "showPrintDialog"
///    *       The toolkit will display the native print dialog.
///    *
///    *       Mac OSX: showPrintDialog           - displays a native dialog
///    *                showPrintProgressDialog   - not implemented (provided by OS)
///    *                showPageSetupDialog       - displays a native dialog
///    *
///    *  GTK: There are no native dialog for GTK.
///    *
///    *       Defaults for platform service:
///    *         showPrintDialog           - displays a native dialog
///    *         showPrintProgressDialog   - displays a XUL dialog
///    *         showPageSetupDialog       - displays a native dialog
///    *
///    */
/// /**
///    *  Show the Print Dialog
///    *
///    *  @param parent - a DOM windows the dialog will be parented to (required)
///    *  @param webBrowserPrint - represents the document to be printed (required)
///    *  @param printSettings - PrintSettings for print "job" (required)
///    *
///    */
/// ```
///

/// `void showPrintDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings);`
#[inline]
pub unsafe fn ShowPrintDialog(&self, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings) -> ::nserror::nsresult {
((*self.vtable).ShowPrintDialog)(self, parent, printSettings)
}


/// ```text
/// /**
///    *  Shows the print progress dialog
///    *
///    *  @param parent - a DOM windows the dialog will be parented to
///    *  @param printSettings - PrintSettings for print "job"
///    *  @param openDialogObserver - an observer that will be notifed when the dialog is opened
///    *  @param isForPrinting - true - for printing, false for print preview
///    *  @param webProgressListener - additional listener can be registered for progress notifications
///    *  @param printProgressParams - parameter object for passing progress state
///    *  @param notifyOnOpen - this indicates that the observer will be notified when the progress
///    *                        dialog has been opened. If false is returned it means the observer
///    *                        (usually the caller) shouldn't wait
///    *                        For Print Preview Progress there is intermediate progress
///    */
/// ```
///

/// `void showPrintProgressDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen);`
#[inline]
pub unsafe fn ShowPrintProgressDialog(&self, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings, openDialogObserver: *const nsIObserver, isForPrinting: bool, webProgressListener: *mut *const nsIWebProgressListener, printProgressParams: *mut *const nsIPrintProgressParams, notifyOnOpen: *mut bool) -> ::nserror::nsresult {
((*self.vtable).ShowPrintProgressDialog)(self, parent, printSettings, openDialogObserver, isForPrinting, webProgressListener, printProgressParams, notifyOnOpen)
}


/// ```text
/// /**
///    *  Shows the print page setup dialog
///    *
///    *  @param parent - a DOM windows the dialog will be parented to (required)
///    *  @param printSettings - PrintSettings for page setup (required)
///    */
/// ```
///

/// `void showPageSetupDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings);`
#[inline]
pub unsafe fn ShowPageSetupDialog(&self, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings) -> ::nserror::nsresult {
((*self.vtable).ShowPageSetupDialog)(self, parent, printSettings)
}


}


