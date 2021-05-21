//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpProtocolHandler.idl
//


/// `interface nsIHttpProtocolHandler : nsIProxiedProtocolHandler`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpProtocolHandler {
    vtable: *const nsIHttpProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpProtocolHandler.
unsafe impl XpCom for nsIHttpProtocolHandler {
    const IID: nsIID = nsID(0xc48126d9, 0x2ddd, 0x485b,
        [0xa5, 0x1a, 0x37, 0x8e, 0x91, 0x7e, 0x75, 0xf8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpProtocolHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpProtocolHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpProtocolHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpProtocolHandler`.
    fn coerce_from(v: &nsIHttpProtocolHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpProtocolHandlerCoerce for nsIHttpProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIHttpProtocolHandler) -> &Self {
        v
    }
}

impl nsIHttpProtocolHandler {
    /// Cast this `nsIHttpProtocolHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpProtocolHandler {
    type Target = nsIProxiedProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProxiedProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProxiedProtocolHandlerCoerce> nsIHttpProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpProtocolHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpProtocolHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProxiedProtocolHandlerVTable,

    /* [must_use] readonly attribute ACString userAgent; */
    pub GetUserAgent: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aUserAgent: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString appName; */
    pub GetAppName: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aAppName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString appVersion; */
    pub GetAppVersion: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aAppVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString platform; */
    pub GetPlatform: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aPlatform: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString oscpu; */
    pub GetOscpu: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aOscpu: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString misc; */
    pub GetMisc: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aMisc: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute Array<ACString> altSvcCacheKeys; */
    pub GetAltSvcCacheKeys: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aAltSvcCacheKeys: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [must_use] readonly attribute Array<ACString> authCacheKeys; */
    pub GetAuthCacheKeys: unsafe extern "system" fn (this: *const nsIHttpProtocolHandler, aAuthCacheKeys: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [implicit_jscontext] Promise EnsureHSTSDataReady (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub EnsureHSTSDataReady: *const ::libc::c_void,

    /* [noscript] void EnsureHSTSDataReadyNative (in HSTSDataCallbackWrapperAlreadyAddRefed aCallback); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub EnsureHSTSDataReadyNative: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpProtocolHandler {

    /// ```text
    /// /**
    ///      * Get the HTTP advertised user agent string.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString userAgent;`
    #[inline]
    pub unsafe fn GetUserAgent(&self, aUserAgent: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUserAgent)(self, aUserAgent)
    }


    /// ```text
    /// /**
    ///      * Get the application name.
    ///      *
    ///      * @return The name of this application (eg. "Mozilla").
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString appName;`
    #[inline]
    pub unsafe fn GetAppName(&self, aAppName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppName)(self, aAppName)
    }


    /// ```text
    /// /**
    ///      * Get the application version string.
    ///      *
    ///      * @return The complete version (major and minor) string. (eg. "5.0")
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString appVersion;`
    #[inline]
    pub unsafe fn GetAppVersion(&self, aAppVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppVersion)(self, aAppVersion)
    }


    /// ```text
    /// /**
    ///      * Get the current platform.
    ///      *
    ///      * @return The platform this application is running on
    ///      *         (eg. "Windows", "Macintosh", "X11")
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString platform;`
    #[inline]
    pub unsafe fn GetPlatform(&self, aPlatform: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPlatform)(self, aPlatform)
    }


    /// ```text
    /// /**
    ///      * Get the current oscpu.
    ///      *
    ///      * @return The oscpu this application is running on
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString oscpu;`
    #[inline]
    pub unsafe fn GetOscpu(&self, aOscpu: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOscpu)(self, aOscpu)
    }


    /// ```text
    /// /**
    ///      * Get the application comment misc portion.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString misc;`
    #[inline]
    pub unsafe fn GetMisc(&self, aMisc: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMisc)(self, aMisc)
    }


    /// ```text
    /// /**
    ///      * Get the Alt-Svc cache keys (used for testing).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute Array<ACString> altSvcCacheKeys;`
    #[inline]
    pub unsafe fn GetAltSvcCacheKeys(&self, aAltSvcCacheKeys: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAltSvcCacheKeys)(self, aAltSvcCacheKeys)
    }


    /// ```text
    /// /**
    ///      * Get the auth cache keys (used for testing).
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute Array<ACString> authCacheKeys;`
    #[inline]
    pub unsafe fn GetAuthCacheKeys(&self, aAuthCacheKeys: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAuthCacheKeys)(self, aAuthCacheKeys)
    }


    /// ```text
    /// /**
    ///      * This function is used to ensure HSTS data storage is ready to read after
    ///      * the returned promise is resolved.
    ///      * Note that this function should only used for testing.
    ///      * See bug 1521729 for more details.
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] Promise EnsureHSTSDataReady ();`
    const _EnsureHSTSDataReady: () = ();

    /// ```text
    /// /**
    ///      * A C++ friendly version of EnsureHSTSDataReady
    ///      */
    /// ```
    ///

    /// `[noscript] void EnsureHSTSDataReadyNative (in HSTSDataCallbackWrapperAlreadyAddRefed aCallback);`
    const _EnsureHSTSDataReadyNative: () = ();

}


