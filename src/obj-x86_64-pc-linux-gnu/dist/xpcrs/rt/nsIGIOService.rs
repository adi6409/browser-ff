//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGIOService.idl
//


/// `interface nsIGIOMimeApp : nsIHandlerApp`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGIOMimeApp {
    vtable: *const nsIGIOMimeAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGIOMimeApp.
unsafe impl XpCom for nsIGIOMimeApp {
    const IID: nsIID = nsID(0xca6bad0c, 0x8a48, 0x48ac,
        [0x82, 0xc7, 0x27, 0xbb, 0x8f, 0x51, 0x0f, 0xbe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGIOMimeApp {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGIOMimeApp.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGIOMimeAppCoerce {
    /// Cheaply cast a value of this type from a `nsIGIOMimeApp`.
    fn coerce_from(v: &nsIGIOMimeApp) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGIOMimeAppCoerce for nsIGIOMimeApp {
    #[inline]
    fn coerce_from(v: &nsIGIOMimeApp) -> &Self {
        v
    }
}

impl nsIGIOMimeApp {
    /// Cast this `nsIGIOMimeApp` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGIOMimeAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGIOMimeApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHandlerAppCoerce> nsIGIOMimeAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGIOMimeApp) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGIOMimeApp
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGIOMimeAppVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHandlerAppVTable,

    /* readonly attribute AUTF8String id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIGIOMimeApp, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String command; */
    pub GetCommand: unsafe extern "system" fn (this: *const nsIGIOMimeApp, aCommand: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long expectsURIs; */
    pub GetExpectsURIs: unsafe extern "system" fn (this: *const nsIGIOMimeApp, aExpectsURIs: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute nsIUTF8StringEnumerator supportedURISchemes; */
    pub GetSupportedURISchemes: unsafe extern "system" fn (this: *const nsIGIOMimeApp, aSupportedURISchemes: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,

    /* void setAsDefaultForMimeType (in AUTF8String mimeType); */
    pub SetAsDefaultForMimeType: unsafe extern "system" fn (this: *const nsIGIOMimeApp, mimeType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setAsDefaultForFileExtensions (in AUTF8String extensions); */
    pub SetAsDefaultForFileExtensions: unsafe extern "system" fn (this: *const nsIGIOMimeApp, extensions: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setAsDefaultForURIScheme (in AUTF8String uriScheme); */
    pub SetAsDefaultForURIScheme: unsafe extern "system" fn (this: *const nsIGIOMimeApp, uriScheme: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGIOMimeApp {

    pub const EXPECTS_URIS: i64 = 0;


    pub const EXPECTS_PATHS: i64 = 1;


    pub const EXPECTS_URIS_FOR_NON_FILES: i64 = 2;


    /// `readonly attribute AUTF8String id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute AUTF8String command;`
    #[inline]
    pub unsafe fn GetCommand(&self, aCommand: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCommand)(self, aCommand)
    }



    /// `readonly attribute long expectsURIs;`
    #[inline]
    pub unsafe fn GetExpectsURIs(&self, aExpectsURIs: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetExpectsURIs)(self, aExpectsURIs)
    }



    /// `readonly attribute nsIUTF8StringEnumerator supportedURISchemes;`
    #[inline]
    pub unsafe fn GetSupportedURISchemes(&self, aSupportedURISchemes: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportedURISchemes)(self, aSupportedURISchemes)
    }



    /// `void setAsDefaultForMimeType (in AUTF8String mimeType);`
    #[inline]
    pub unsafe fn SetAsDefaultForMimeType(&self, mimeType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAsDefaultForMimeType)(self, mimeType)
    }



    /// `void setAsDefaultForFileExtensions (in AUTF8String extensions);`
    #[inline]
    pub unsafe fn SetAsDefaultForFileExtensions(&self, extensions: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAsDefaultForFileExtensions)(self, extensions)
    }



    /// `void setAsDefaultForURIScheme (in AUTF8String uriScheme);`
    #[inline]
    pub unsafe fn SetAsDefaultForURIScheme(&self, uriScheme: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAsDefaultForURIScheme)(self, uriScheme)
    }


}


/// `interface nsIGIOService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGIOService {
    vtable: *const nsIGIOServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGIOService.
unsafe impl XpCom for nsIGIOService {
    const IID: nsIID = nsID(0xeda22a30, 0x84e1, 0x4e16,
        [0x9c, 0xa0, 0xcd, 0x15, 0x53, 0xc2, 0xb3, 0x4a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGIOService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGIOService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGIOServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIGIOService`.
    fn coerce_from(v: &nsIGIOService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGIOServiceCoerce for nsIGIOService {
    #[inline]
    fn coerce_from(v: &nsIGIOService) -> &Self {
        v
    }
}

impl nsIGIOService {
    /// Cast this `nsIGIOService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGIOServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGIOService {
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
impl<T: nsISupportsCoerce> nsIGIOServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGIOService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGIOService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGIOServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AUTF8String getMimeTypeFromExtension (in AUTF8String extension); */
    pub GetMimeTypeFromExtension: unsafe extern "system" fn (this: *const nsIGIOService, extension: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIHandlerApp getAppForURIScheme (in AUTF8String aURIScheme); */
    pub GetAppForURIScheme: unsafe extern "system" fn (this: *const nsIGIOService, aURIScheme: *const ::nsstring::nsACString, _retval: *mut *const nsIHandlerApp) -> ::nserror::nsresult,

    /* nsIMutableArray getAppsForURIScheme (in AUTF8String aURIScheme); */
    pub GetAppsForURIScheme: unsafe extern "system" fn (this: *const nsIGIOService, aURIScheme: *const ::nsstring::nsACString, _retval: *mut*const nsIMutableArray) -> ::nserror::nsresult,

    /* nsIHandlerApp getAppForMimeType (in AUTF8String mimeType); */
    pub GetAppForMimeType: unsafe extern "system" fn (this: *const nsIGIOService, mimeType: *const ::nsstring::nsACString, _retval: *mut *const nsIHandlerApp) -> ::nserror::nsresult,

    /* nsIGIOMimeApp createAppFromCommand (in AUTF8String cmd, in AUTF8String appName); */
    pub CreateAppFromCommand: unsafe extern "system" fn (this: *const nsIGIOService, cmd: *const ::nsstring::nsACString, appName: *const ::nsstring::nsACString, _retval: *mut *const nsIGIOMimeApp) -> ::nserror::nsresult,

    /* nsIGIOMimeApp findAppFromCommand (in AUTF8String cmd); */
    pub FindAppFromCommand: unsafe extern "system" fn (this: *const nsIGIOService, cmd: *const ::nsstring::nsACString, _retval: *mut *const nsIGIOMimeApp) -> ::nserror::nsresult,

    /* AUTF8String getDescriptionForMimeType (in AUTF8String mimeType); */
    pub GetDescriptionForMimeType: unsafe extern "system" fn (this: *const nsIGIOService, mimeType: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void showURI (in nsIURI uri); */
    pub ShowURI: unsafe extern "system" fn (this: *const nsIGIOService, uri: *const nsIURI) -> ::nserror::nsresult,

    /* [noscript] void showURIForInput (in ACString uri); */
    pub ShowURIForInput: unsafe extern "system" fn (this: *const nsIGIOService, uri: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void orgFreedesktopFileManager1ShowItems (in ACString path); */
    pub OrgFreedesktopFileManager1ShowItems: unsafe extern "system" fn (this: *const nsIGIOService, path: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] bool shouldUseFlatpakPortal (); */
    pub ShouldUseFlatpakPortal: unsafe extern "system" fn (this: *const nsIGIOService, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGIOService {

    /// ```text
    /// /*** MIME registry methods ***/
    /// ```
    ///

    /// `AUTF8String getMimeTypeFromExtension (in AUTF8String extension);`
    #[inline]
    pub unsafe fn GetMimeTypeFromExtension(&self, extension: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMimeTypeFromExtension)(self, extension, _retval)
    }



    /// `nsIHandlerApp getAppForURIScheme (in AUTF8String aURIScheme);`
    #[inline]
    pub unsafe fn GetAppForURIScheme(&self, aURIScheme: *const ::nsstring::nsACString, _retval: *mut *const nsIHandlerApp) -> ::nserror::nsresult {
        ((*self.vtable).GetAppForURIScheme)(self, aURIScheme, _retval)
    }



    /// `nsIMutableArray getAppsForURIScheme (in AUTF8String aURIScheme);`
    #[inline]
    pub unsafe fn GetAppsForURIScheme(&self, aURIScheme: *const ::nsstring::nsACString, _retval: *mut*const nsIMutableArray) -> ::nserror::nsresult {
        ((*self.vtable).GetAppsForURIScheme)(self, aURIScheme, _retval)
    }



    /// `nsIHandlerApp getAppForMimeType (in AUTF8String mimeType);`
    #[inline]
    pub unsafe fn GetAppForMimeType(&self, mimeType: *const ::nsstring::nsACString, _retval: *mut *const nsIHandlerApp) -> ::nserror::nsresult {
        ((*self.vtable).GetAppForMimeType)(self, mimeType, _retval)
    }



    /// `nsIGIOMimeApp createAppFromCommand (in AUTF8String cmd, in AUTF8String appName);`
    #[inline]
    pub unsafe fn CreateAppFromCommand(&self, cmd: *const ::nsstring::nsACString, appName: *const ::nsstring::nsACString, _retval: *mut *const nsIGIOMimeApp) -> ::nserror::nsresult {
        ((*self.vtable).CreateAppFromCommand)(self, cmd, appName, _retval)
    }



    /// `nsIGIOMimeApp findAppFromCommand (in AUTF8String cmd);`
    #[inline]
    pub unsafe fn FindAppFromCommand(&self, cmd: *const ::nsstring::nsACString, _retval: *mut *const nsIGIOMimeApp) -> ::nserror::nsresult {
        ((*self.vtable).FindAppFromCommand)(self, cmd, _retval)
    }



    /// `AUTF8String getDescriptionForMimeType (in AUTF8String mimeType);`
    #[inline]
    pub unsafe fn GetDescriptionForMimeType(&self, mimeType: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescriptionForMimeType)(self, mimeType, _retval)
    }


    /// ```text
    /// /*** Misc. methods ***/
    /// ```
    ///

    /// `void showURI (in nsIURI uri);`
    #[inline]
    pub unsafe fn ShowURI(&self, uri: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).ShowURI)(self, uri)
    }



    /// `[noscript] void showURIForInput (in ACString uri);`
    #[inline]
    pub unsafe fn ShowURIForInput(&self, uri: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ShowURIForInput)(self, uri)
    }



    /// `[noscript] void orgFreedesktopFileManager1ShowItems (in ACString path);`
    #[inline]
    pub unsafe fn OrgFreedesktopFileManager1ShowItems(&self, path: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OrgFreedesktopFileManager1ShowItems)(self, path)
    }



    /// `[noscript] bool shouldUseFlatpakPortal ();`
    #[inline]
    pub unsafe fn ShouldUseFlatpakPortal(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ShouldUseFlatpakPortal)(self, _retval)
    }


}


