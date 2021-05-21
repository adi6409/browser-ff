//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIHelperAppLauncherDialog.idl
//


/// `interface nsIHelperAppLauncherDialog : nsISupports`
///

/// ```text
/// /**
///  * This interface is used to display a confirmation dialog before
///  * launching a "helper app" to handle content not handled by
///  * Mozilla.
///  *
///  * Usage:  Clients (of which there is one: the nsIExternalHelperAppService
    ///  * implementation in mozilla/uriloader/exthandler) create an instance of
///  * this interface (using the contract ID) and then call the show() method.
///  *
///  * The dialog is shown non-modally.  The implementation of the dialog
///  * will access methods of the nsIHelperAppLauncher passed in to show()
///  * in order to cause a "save to disk" or "open using" action.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHelperAppLauncherDialog {
    vtable: *const nsIHelperAppLauncherDialogVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHelperAppLauncherDialog.
unsafe impl XpCom for nsIHelperAppLauncherDialog {
    const IID: nsIID = nsID(0xbfc739f3, 0x8d75, 0x4034,
        [0xa6, 0xf8, 0x10, 0x39, 0xa5, 0x99, 0x6b, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHelperAppLauncherDialog {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHelperAppLauncherDialog.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHelperAppLauncherDialogCoerce {
    /// Cheaply cast a value of this type from a `nsIHelperAppLauncherDialog`.
    fn coerce_from(v: &nsIHelperAppLauncherDialog) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHelperAppLauncherDialogCoerce for nsIHelperAppLauncherDialog {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncherDialog) -> &Self {
        v
    }
}

impl nsIHelperAppLauncherDialog {
    /// Cast this `nsIHelperAppLauncherDialog` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHelperAppLauncherDialogCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHelperAppLauncherDialog {
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
impl<T: nsISupportsCoerce> nsIHelperAppLauncherDialogCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncherDialog) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHelperAppLauncherDialog
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHelperAppLauncherDialogVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void show (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in unsigned long aReason); */
    pub Show: unsafe extern "system" fn (this: *const nsIHelperAppLauncherDialog, aLauncher: *const nsIHelperAppLauncher, aWindowContext: *const nsIInterfaceRequestor, aReason: u32) -> ::nserror::nsresult,

    /* void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt); */
    pub PromptForSaveToFileAsync: unsafe extern "system" fn (this: *const nsIHelperAppLauncherDialog, aLauncher: *const nsIHelperAppLauncher, aWindowContext: *const nsIInterfaceRequestor, aDefaultFileName: *const i16, aSuggestedFileExtension: *const i16, aForcePrompt: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHelperAppLauncherDialog {
    /// ```text
    /// /**
    ///    * This request is passed to the helper app dialog because Gecko can not
    ///    * handle content of this type.
    ///    */
    /// ```
    ///

    pub const REASON_CANTHANDLE: i64 = 0;

    /// ```text
    /// /**
    ///    * The server requested external handling.
    ///    */
    /// ```
    ///

    pub const REASON_SERVERREQUEST: i64 = 1;

    /// ```text
    /// /**
    ///    * Gecko detected that the type sent by the server (e.g. text/plain) does
    ///    * not match the actual type.
    ///    */
    /// ```
    ///

    pub const REASON_TYPESNIFFED: i64 = 2;

    /// ```text
    /// /**
    ///    * Show confirmation dialog for launching application (or "save to
        ///    * disk") for content specified by aLauncher.
    ///    *
    ///    * @param aLauncher
    ///    *        A nsIHelperAppLauncher to be invoked when a file is selected.
    ///    * @param aWindowContext
    ///    *        Window associated with action.
    ///    * @param aReason
    ///    *        One of the constants from above. It indicates why the dialog is
    ///    *        shown. Implementors should treat unknown reasons like
    ///    *        REASON_CANTHANDLE.
    ///    */
    /// ```
    ///

    /// `void show (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in unsigned long aReason);`
    #[inline]
    pub unsafe fn Show(&self, aLauncher: *const nsIHelperAppLauncher, aWindowContext: *const nsIInterfaceRequestor, aReason: u32) -> ::nserror::nsresult {
        ((*self.vtable).Show)(self, aLauncher, aWindowContext, aReason)
    }


    /// ```text
    /// /**
    ///    * Async invoke a save-to-file dialog instead of the full fledged helper app
    ///    * dialog. When the file is chosen (or the dialog is closed), the callback
    ///    * in aLauncher (aLauncher.saveDestinationAvailable) is called with the
    ///    * selected file.
    ///    *
    ///    * @param aLauncher
    ///    *        A nsIHelperAppLauncher to be invoked when a file is selected.
    ///    * @param aWindowContext
    ///    *        Window associated with action.
    ///    * @param aDefaultFileName
    ///    *        Default file name to provide (can be null)
    ///    * @param aSuggestedFileExtension
    ///    *        Sugested file extension
    ///    * @param aForcePrompt
    ///    *        Set to true to force prompting the user for thet file
    ///    *        name/location, otherwise perferences may control if the user is
    ///    *        prompted.
    ///    */
    /// ```
    ///

    /// `void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt);`
    #[inline]
    pub unsafe fn PromptForSaveToFileAsync(&self, aLauncher: *const nsIHelperAppLauncher, aWindowContext: *const nsIInterfaceRequestor, aDefaultFileName: *const i16, aSuggestedFileExtension: *const i16, aForcePrompt: bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptForSaveToFileAsync)(self, aLauncher, aWindowContext, aDefaultFileName, aSuggestedFileExtension, aForcePrompt)
    }


}


