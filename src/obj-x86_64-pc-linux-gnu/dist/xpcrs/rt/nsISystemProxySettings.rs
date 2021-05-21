//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISystemProxySettings.idl
//


/// `interface nsISystemProxySettings : nsISupports`
///

/// ```text
/// /**
///  * This interface allows the proxy code to use platform-specific proxy
///  * settings when the proxy preference is set to "automatic discovery". This service
///  * acts like a PAC parser to netwerk, but it will actually read the system settings and
///  * either return the proper proxy data from the autoconfig URL specified in the system proxy,
///  * or generate proxy data based on the system's manual proxy settings.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISystemProxySettings {
    vtable: *const nsISystemProxySettingsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISystemProxySettings.
unsafe impl XpCom for nsISystemProxySettings {
    const IID: nsIID = nsID(0x971591cd, 0x277e, 0x409a,
        [0xbb, 0xf6, 0x0a, 0x79, 0x87, 0x9c, 0xd3, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISystemProxySettings {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISystemProxySettings.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISystemProxySettingsCoerce {
    /// Cheaply cast a value of this type from a `nsISystemProxySettings`.
    fn coerce_from(v: &nsISystemProxySettings) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISystemProxySettingsCoerce for nsISystemProxySettings {
    #[inline]
    fn coerce_from(v: &nsISystemProxySettings) -> &Self {
        v
    }
}

impl nsISystemProxySettings {
    /// Cast this `nsISystemProxySettings` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISystemProxySettingsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISystemProxySettings {
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
impl<T: nsISupportsCoerce> nsISystemProxySettingsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISystemProxySettings) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISystemProxySettings
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISystemProxySettingsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute bool mainThreadOnly; */
    pub GetMainThreadOnly: unsafe extern "system" fn (this: *const nsISystemProxySettings, aMainThreadOnly: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String PACURI; */
    pub GetPACURI: unsafe extern "system" fn (this: *const nsISystemProxySettings, aPACURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort); */
    pub GetProxyForURI: unsafe extern "system" fn (this: *const nsISystemProxySettings, testSpec: *const ::nsstring::nsACString, testScheme: *const ::nsstring::nsACString, testHost: *const ::nsstring::nsACString, testPort: int32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISystemProxySettings {

    /// ```text
    /// /**
    ///      * Whether or not it is appropriate to execute getProxyForURI off the main thread.
    ///      * If that method can block (e.g. for WPAD as windows does) then it must be
    ///      * not mainThreadOnly to avoid creating main thread jank. The main thread only option is
    ///      * provided for implementations that do not block but use other main thread only
    ///      * functions such as dbus.
    ///      */
    /// ```
    ///

    /// `readonly attribute bool mainThreadOnly;`
    #[inline]
    pub unsafe fn GetMainThreadOnly(&self, aMainThreadOnly: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMainThreadOnly)(self, aMainThreadOnly)
    }


    /// ```text
    /// /**
    ///      * If non-empty, use this PAC file. If empty, call getProxyForURI instead.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String PACURI;`
    #[inline]
    pub unsafe fn GetPACURI(&self, aPACURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPACURI)(self, aPACURI)
    }


    /// ```text
    /// /**
    ///      * See ProxyAutoConfig::getProxyForURI; this function behaves similarly except
    ///      * a more relaxed return string is allowed that includes full urls instead of just
    ///      * host:port syntax. e.g. "PROXY http://www.foo.com:8080" instead of
    ///      * "PROXY www.foo.com:8080"
    ///      */
    /// ```
    ///

    /// `AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort);`
    #[inline]
    pub unsafe fn GetProxyForURI(&self, testSpec: *const ::nsstring::nsACString, testScheme: *const ::nsstring::nsACString, testHost: *const ::nsstring::nsACString, testPort: int32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyForURI)(self, testSpec, testScheme, testHost, testPort, _retval)
    }


}


