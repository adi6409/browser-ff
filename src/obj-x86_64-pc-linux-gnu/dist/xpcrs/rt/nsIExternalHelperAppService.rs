//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalHelperAppService.idl
//


/// `interface nsIExternalHelperAppService : nsISupports`
///

/// ```text
/// /**
///  * The external helper app service is used for finding and launching
///  * platform specific external applications for a given mime content type.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIExternalHelperAppService {
    vtable: *const nsIExternalHelperAppServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIExternalHelperAppService.
unsafe impl XpCom for nsIExternalHelperAppService {
    const IID: nsIID = nsID(0x1e4f3ae1, 0xb737, 0x431f,
        [0xa9, 0x5d, 0x31, 0xfa, 0x8d, 0xa7, 0x01, 0x99]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIExternalHelperAppService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIExternalHelperAppService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExternalHelperAppServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIExternalHelperAppService`.
    fn coerce_from(v: &nsIExternalHelperAppService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExternalHelperAppServiceCoerce for nsIExternalHelperAppService {
    #[inline]
    fn coerce_from(v: &nsIExternalHelperAppService) -> &Self {
        v
    }
}

impl nsIExternalHelperAppService {
    /// Cast this `nsIExternalHelperAppService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExternalHelperAppServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIExternalHelperAppService {
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
impl<T: nsISupportsCoerce> nsIExternalHelperAppServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalHelperAppService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIExternalHelperAppService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExternalHelperAppServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub DoContent: unsafe extern "system" fn (this: *const nsIExternalHelperAppService, aMimeContentType: *const ::nsstring::nsACString, aRequest: *const nsIRequest, aContentContext: *const nsIInterfaceRequestor, aForceSave: bool, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult,

    /* nsIStreamListener createListener (in ACString aMimeContentType, in nsIRequest aRequest, in BrowsingContext aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub CreateListener: unsafe extern "system" fn (this: *const nsIExternalHelperAppService, aMimeContentType: *const ::nsstring::nsACString, aRequest: *const nsIRequest, aContentContext: *const libc::c_void, aForceSave: bool, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult,

    /* boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType); */
    pub ApplyDecodingForExtension: unsafe extern "system" fn (this: *const nsIExternalHelperAppService, aExtension: *const ::nsstring::nsACString, aEncodingType: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIExternalHelperAppService {

    /// ```text
    /// /**
    ///    * Binds an external helper application to a stream listener. The caller
    ///    * should pump data into the returned stream listener. When the OnStopRequest
    ///    * is issued, the stream listener implementation will launch the helper app
    ///    * with this data.
    ///    * @param aMimeContentType The content type of the incoming data
    ///    * @param aRequest The request corresponding to the incoming data
    ///    * @param aContentContext Used in processing content document refresh
    ///    *  headers after target content is downloaded.
    ///    * @param aForceSave True to always save this content to disk, regardless of
    ///    *  nsIMIMEInfo and other such influences.
    ///    * @param aWindowContext Used in parenting helper app dialogs, usually
    ///    *  points to the parent browser window. This parameter may be null,
    ///    *  in which case dialogs will be parented to aContentContext.
    ///    * @return A nsIStreamListener which the caller should pump the data into.
    ///    */
    /// ```
    ///

    /// `nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext);`
    #[inline]
    pub unsafe fn DoContent(&self, aMimeContentType: *const ::nsstring::nsACString, aRequest: *const nsIRequest, aContentContext: *const nsIInterfaceRequestor, aForceSave: bool, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).DoContent)(self, aMimeContentType, aRequest, aContentContext, aForceSave, aWindowContext, _retval)
    }


    /// ```text
    /// /**
    ///    * Binds an external helper application to a stream listener. The caller
    ///    * should pump data into the returned stream listener. When the OnStopRequest
    ///    * is issued, the stream listener implementation will launch the helper app
    ///    * with this data.
    ///    * Replaces doContent for native code, and uses BrowsingContext.
    ///    *
    ///    * @param aMimeContentType The content type of the incoming data
    ///    * @param aRequest The request corresponding to the incoming data
    ///    * @param aContentContext The BrowsingContext that the request was initiated
    ///    *  by. Used for closing the window if we opened one specifically for this download.
    ///    * @param aForceSave True to always save this content to disk, regardless of
    ///    *  nsIMIMEInfo and other such influences.
    ///    * @param aWindowContext Used in parenting helper app dialogs, usually
    ///    *  points to the parent browser window. This parameter may be null,
    ///    *  in which case dialogs will be parented to aContentContext.
    ///    * @return A nsIStreamListener which the caller should pump the data into.
    ///    */
    /// ```
    ///

    /// `nsIStreamListener createListener (in ACString aMimeContentType, in nsIRequest aRequest, in BrowsingContext aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext);`
    #[inline]
    pub unsafe fn CreateListener(&self, aMimeContentType: *const ::nsstring::nsACString, aRequest: *const nsIRequest, aContentContext: *const libc::c_void, aForceSave: bool, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).CreateListener)(self, aMimeContentType, aRequest, aContentContext, aForceSave, aWindowContext, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if data from a URL with this extension combination
    ///    * is to be decoded from aEncodingType prior to saving or passing
    ///    * off to helper apps, false otherwise.
    ///    */
    /// ```
    ///

    /// `boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType);`
    #[inline]
    pub unsafe fn ApplyDecodingForExtension(&self, aExtension: *const ::nsstring::nsACString, aEncodingType: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ApplyDecodingForExtension)(self, aExtension, aEncodingType, _retval)
    }


}


/// `interface nsPIExternalAppLauncher : nsISupports`
///

/// ```text
/// /**
///  * This is a private interface shared between external app handlers and the platform specific
///  * external helper app service
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsPIExternalAppLauncher {
    vtable: *const nsPIExternalAppLauncherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsPIExternalAppLauncher.
unsafe impl XpCom for nsPIExternalAppLauncher {
    const IID: nsIID = nsID(0x6613e2e7, 0xfeab, 0x4e3a,
        [0xbb, 0x1f, 0xb0, 0x32, 0x00, 0xd5, 0x44, 0xec]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsPIExternalAppLauncher {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsPIExternalAppLauncher.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsPIExternalAppLauncherCoerce {
    /// Cheaply cast a value of this type from a `nsPIExternalAppLauncher`.
    fn coerce_from(v: &nsPIExternalAppLauncher) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsPIExternalAppLauncherCoerce for nsPIExternalAppLauncher {
    #[inline]
    fn coerce_from(v: &nsPIExternalAppLauncher) -> &Self {
        v
    }
}

impl nsPIExternalAppLauncher {
    /// Cast this `nsPIExternalAppLauncher` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsPIExternalAppLauncherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsPIExternalAppLauncher {
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
impl<T: nsISupportsCoerce> nsPIExternalAppLauncherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIExternalAppLauncher) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsPIExternalAppLauncher
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsPIExternalAppLauncherVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile); */
    pub DeleteTemporaryFileOnExit: unsafe extern "system" fn (this: *const nsPIExternalAppLauncher, aTemporaryFile: *const nsIFile) -> ::nserror::nsresult,

    /* void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile); */
    pub DeleteTemporaryPrivateFileWhenPossible: unsafe extern "system" fn (this: *const nsPIExternalAppLauncher, aTemporaryFile: *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsPIExternalAppLauncher {

    /// ```text
    /// /**
    ///    * mscott --> eventually I should move this into a new service so other
    ///    * consumers can add temporary files they want deleted on exit.
    ///    * @param aTemporaryFile A temporary file we should delete on exit.
    ///    */
    /// ```
    ///

    /// `void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile);`
    #[inline]
    pub unsafe fn DeleteTemporaryFileOnExit(&self, aTemporaryFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTemporaryFileOnExit)(self, aTemporaryFile)
    }


    /// ```text
    /// /**
    ///    * Delete a temporary file created inside private browsing mode when
    ///    * the private browsing mode has ended.
    ///    */
    /// ```
    ///

    /// `void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile);`
    #[inline]
    pub unsafe fn DeleteTemporaryPrivateFileWhenPossible(&self, aTemporaryFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).DeleteTemporaryPrivateFileWhenPossible)(self, aTemporaryFile)
    }


}


/// `interface nsIHelperAppLauncher : nsICancelable`
///

/// ```text
/// /**
///  * A helper app launcher is a small object created to handle the launching
///  * of an external application.
///  *
///  * Note that cancelling the load via the nsICancelable interface will release
///  * the reference to the launcher dialog.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHelperAppLauncher {
    vtable: *const nsIHelperAppLauncherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHelperAppLauncher.
unsafe impl XpCom for nsIHelperAppLauncher {
    const IID: nsIID = nsID(0xacf2a516, 0x7d7f, 0x4771,
        [0x8b, 0x22, 0x6c, 0x4a, 0x55, 0x9c, 0x08, 0x8e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHelperAppLauncher {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHelperAppLauncher.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHelperAppLauncherCoerce {
    /// Cheaply cast a value of this type from a `nsIHelperAppLauncher`.
    fn coerce_from(v: &nsIHelperAppLauncher) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHelperAppLauncherCoerce for nsIHelperAppLauncher {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncher) -> &Self {
        v
    }
}

impl nsIHelperAppLauncher {
    /// Cast this `nsIHelperAppLauncher` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHelperAppLauncherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHelperAppLauncher {
    type Target = nsICancelable;
    #[inline]
    fn deref(&self) -> &nsICancelable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICancelableCoerce> nsIHelperAppLauncherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncher) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHelperAppLauncher
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHelperAppLauncherVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICancelableVTable,

    /* readonly attribute nsIMIMEInfo MIMEInfo; */
    pub GetMIMEInfo: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aMIMEInfo: *mut*const nsIMIMEInfo) -> ::nserror::nsresult,

    /* readonly attribute nsIURI source; */
    pub GetSource: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aSource: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute AString suggestedFileName; */
    pub GetSuggestedFileName: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aSuggestedFileName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void promptForSaveDestination (); */
    pub PromptForSaveDestination: unsafe extern "system" fn (this: *const nsIHelperAppLauncher) -> ::nserror::nsresult,

    /* void launchWithApplication (in boolean aHandleInternally); */
    pub LaunchWithApplication: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aHandleInternally: bool) -> ::nserror::nsresult,

    /* void saveDestinationAvailable (in nsIFile aFile); */
    pub SaveDestinationAvailable: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener); */
    pub SetWebProgressListener: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aWebProgressListener: *const nsIWebProgressListener2) -> ::nserror::nsresult,

    /* readonly attribute nsIFile targetFile; */
    pub GetTargetFile: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aTargetFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute boolean targetFileIsExecutable; */
    pub GetTargetFileIsExecutable: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aTargetFileIsExecutable: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute PRTime timeDownloadStarted; */
    pub GetTimeDownloadStarted: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aTimeDownloadStarted: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute int64_t contentLength; */
    pub GetContentLength: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aContentLength: *mut int64_t) -> ::nserror::nsresult,

    /* readonly attribute uint64_t browsingContextId; */
    pub GetBrowsingContextId: unsafe extern "system" fn (this: *const nsIHelperAppLauncher, aBrowsingContextId: *mut uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHelperAppLauncher {

    /// ```text
    /// /**
    ///    * The mime info object associated with the content type this helper app
    ///    * launcher is currently attempting to load
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIMIMEInfo MIMEInfo;`
    #[inline]
    pub unsafe fn GetMIMEInfo(&self, aMIMEInfo: *mut*const nsIMIMEInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetMIMEInfo)(self, aMIMEInfo)
    }


    /// ```text
    /// /**
    ///    * The source uri
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI source;`
    #[inline]
    pub unsafe fn GetSource(&self, aSource: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetSource)(self, aSource)
    }


    /// ```text
    /// /**
    ///    * The suggested name for this file
    ///    */
    /// ```
    ///

    /// `readonly attribute AString suggestedFileName;`
    #[inline]
    pub unsafe fn GetSuggestedFileName(&self, aSuggestedFileName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSuggestedFileName)(self, aSuggestedFileName)
    }


    /// ```text
    /// /**
    ///    * Saves the final destination of the file.
    ///    * NOTE: This will release the reference to the nsIHelperAppLauncherDialog.
    ///    */
    /// ```
    ///

    /// `void promptForSaveDestination ();`
    #[inline]
    pub unsafe fn PromptForSaveDestination(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).PromptForSaveDestination)(self, )
    }


    /// ```text
    /// /**
    ///    * Tell the launcher that we will want to open the file.
    ///    * NOTE: This will release the reference to the nsIHelperAppLauncherDialog.
    ///    * @param aHandleInternally TRUE if we should handle opening this internally.
    ///    */
    /// ```
    ///

    /// `void launchWithApplication (in boolean aHandleInternally);`
    #[inline]
    pub unsafe fn LaunchWithApplication(&self, aHandleInternally: bool) -> ::nserror::nsresult {
        ((*self.vtable).LaunchWithApplication)(self, aHandleInternally)
    }


    /// ```text
    /// /**
    ///    * Callback invoked by nsIHelperAppLauncherDialog::promptForSaveToFileAsync
    ///    * after the user has chosen a file through the File Picker (or dismissed it).
    ///    * @param aFile The file that was chosen by the user (or null if dialog was dismissed).
    ///    */
    /// ```
    ///

    /// `void saveDestinationAvailable (in nsIFile aFile);`
    #[inline]
    pub unsafe fn SaveDestinationAvailable(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SaveDestinationAvailable)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * The following methods are used by the progress dialog to get or set
    ///    * information on the current helper app launcher download.
    ///    * This reference will be released when the download is finished (after the
        ///    * listener receives the STATE_STOP notification).
    ///    */
    /// ```
    ///

    /// `void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener);`
    #[inline]
    pub unsafe fn SetWebProgressListener(&self, aWebProgressListener: *const nsIWebProgressListener2) -> ::nserror::nsresult {
        ((*self.vtable).SetWebProgressListener)(self, aWebProgressListener)
    }


    /// ```text
    /// /**
    ///    * The file we are saving to
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIFile targetFile;`
    #[inline]
    pub unsafe fn GetTargetFile(&self, aTargetFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetFile)(self, aTargetFile)
    }


    /// ```text
    /// /**
    ///    * The executable-ness of the target file
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean targetFileIsExecutable;`
    #[inline]
    pub unsafe fn GetTargetFileIsExecutable(&self, aTargetFileIsExecutable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetFileIsExecutable)(self, aTargetFileIsExecutable)
    }


    /// ```text
    /// /**
    ///    * Time when the download started
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime timeDownloadStarted;`
    #[inline]
    pub unsafe fn GetTimeDownloadStarted(&self, aTimeDownloadStarted: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeDownloadStarted)(self, aTimeDownloadStarted)
    }


    /// ```text
    /// /**
    ///    * The download content length, or -1 if the length is not available.
    ///    */
    /// ```
    ///

    /// `readonly attribute int64_t contentLength;`
    #[inline]
    pub unsafe fn GetContentLength(&self, aContentLength: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetContentLength)(self, aContentLength)
    }


    /// ```text
    /// /**
    ///    * The browsingContext ID of the launcher's source
    ///    */
    /// ```
    ///

    /// `readonly attribute uint64_t browsingContextId;`
    #[inline]
    pub unsafe fn GetBrowsingContextId(&self, aBrowsingContextId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowsingContextId)(self, aBrowsingContextId)
    }


}


