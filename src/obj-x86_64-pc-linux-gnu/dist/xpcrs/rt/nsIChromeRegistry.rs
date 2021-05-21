//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/chrome/nsIChromeRegistry.idl
//


/// `interface nsIChromeRegistry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIChromeRegistry {
    vtable: *const nsIChromeRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIChromeRegistry.
unsafe impl XpCom for nsIChromeRegistry {
    const IID: nsIID = nsID(0x249fb5ad, 0xae29, 0x4e2c,
        [0xa7, 0x28, 0xba, 0x5c, 0xf4, 0x64, 0xd1, 0x88]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIChromeRegistry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIChromeRegistry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIChromeRegistryCoerce {
    /// Cheaply cast a value of this type from a `nsIChromeRegistry`.
    fn coerce_from(v: &nsIChromeRegistry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIChromeRegistryCoerce for nsIChromeRegistry {
    #[inline]
    fn coerce_from(v: &nsIChromeRegistry) -> &Self {
        v
    }
}

impl nsIChromeRegistry {
    /// Cast this `nsIChromeRegistry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIChromeRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIChromeRegistry {
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
impl<T: nsISupportsCoerce> nsIChromeRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChromeRegistry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIChromeRegistry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIChromeRegistryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIURI convertChromeURL (in nsIURI aChromeURL); */
    pub ConvertChromeURL: unsafe extern "system" fn (this: *const nsIChromeRegistry, aChromeURL: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* void checkForNewChrome (); */
    pub CheckForNewChrome: unsafe extern "system" fn (this: *const nsIChromeRegistry) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIChromeRegistry {

    pub const NONE: i64 = 0;


    pub const PARTIAL: i64 = 1;


    pub const FULL: i64 = 2;

    /// ```text
    /// /**
    ///    * Resolve a chrome URL to an loadable URI using the information in the
    ///    * registry. Does not modify aChromeURL.
    ///    *
    ///    * Chrome URLs are allowed to be specified in "shorthand", leaving the
    ///    * "file" portion off. In that case, the URL is expanded to:
    ///    *
    ///    *   chrome://package/provider/package.ext
    ///    *
    ///    * where "ext" is:
    ///    *
    ///    *   "xul" for a "content" package,
    ///    *   "css" for a "skin" package, and
    ///    *   "dtd" for a "locale" package.
    ///    *
    ///    * @param aChromeURL the URL that is to be converted.
    ///    */
    /// ```
    ///

    /// `nsIURI convertChromeURL (in nsIURI aChromeURL);`
    #[inline]
    pub unsafe fn ConvertChromeURL(&self, aChromeURL: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).ConvertChromeURL)(self, aChromeURL, _retval)
    }


    /// ```text
    /// /**
    ///    * refresh the chrome list at runtime, looking for new packages/etc
    ///    */
    /// ```
    ///

    /// `void checkForNewChrome ();`
    #[inline]
    pub unsafe fn CheckForNewChrome(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CheckForNewChrome)(self, )
    }


}


/// `interface nsIXULChromeRegistry : nsIChromeRegistry`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXULChromeRegistry {
    vtable: *const nsIXULChromeRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXULChromeRegistry.
unsafe impl XpCom for nsIXULChromeRegistry {
    const IID: nsIID = nsID(0x93251ddf, 0x5e85, 0x4172,
        [0xac, 0x2a, 0x31, 0x78, 0x05, 0x62, 0x97, 0x4f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXULChromeRegistry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXULChromeRegistry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXULChromeRegistryCoerce {
    /// Cheaply cast a value of this type from a `nsIXULChromeRegistry`.
    fn coerce_from(v: &nsIXULChromeRegistry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXULChromeRegistryCoerce for nsIXULChromeRegistry {
    #[inline]
    fn coerce_from(v: &nsIXULChromeRegistry) -> &Self {
        v
    }
}

impl nsIXULChromeRegistry {
    /// Cast this `nsIXULChromeRegistry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXULChromeRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXULChromeRegistry {
    type Target = nsIChromeRegistry;
    #[inline]
    fn deref(&self) -> &nsIChromeRegistry {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIChromeRegistryCoerce> nsIXULChromeRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULChromeRegistry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXULChromeRegistry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXULChromeRegistryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIChromeRegistryVTable,

    /* boolean isLocaleRTL (in ACString package); */
    pub IsLocaleRTL: unsafe extern "system" fn (this: *const nsIXULChromeRegistry, package: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean allowScriptsForPackage (in nsIURI url); */
    pub AllowScriptsForPackage: unsafe extern "system" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean allowContentToAccess (in nsIURI url); */
    pub AllowContentToAccess: unsafe extern "system" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean canLoadURLRemotely (in nsIURI url); */
    pub CanLoadURLRemotely: unsafe extern "system" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean mustLoadURLRemotely (in nsIURI url); */
    pub MustLoadURLRemotely: unsafe extern "system" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXULChromeRegistry {


    /// `boolean isLocaleRTL (in ACString package);`
    #[inline]
    pub unsafe fn IsLocaleRTL(&self, package: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsLocaleRTL)(self, package, _retval)
    }


    /// ```text
    /// /**
    ///    * Installable skin XBL is not always granted the same privileges as other
    ///    * chrome. This asks the chrome registry whether scripts are allowed to be
    ///    * run for a particular chrome URI. Do not pass non-chrome URIs to this
    ///    * method.
    ///    */
    /// ```
    ///

    /// `boolean allowScriptsForPackage (in nsIURI url);`
    #[inline]
    pub unsafe fn AllowScriptsForPackage(&self, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AllowScriptsForPackage)(self, url, _retval)
    }


    /// ```text
    /// /**
    ///    * Content should only be allowed to load chrome JS from certain packages.
    ///    * This method reflects the contentaccessible flag on packages.
    ///    * Do not pass non-chrome URIs to this method.
    ///    */
    /// ```
    ///

    /// `boolean allowContentToAccess (in nsIURI url);`
    #[inline]
    pub unsafe fn AllowContentToAccess(&self, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AllowContentToAccess)(self, url, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the passed chrome URL is allowed to be loaded in a remote
    ///    * process. This reflects the remoteenabled flag on packages.
    ///    * Do not pass non-chrome URIs to this method.
    ///    */
    /// ```
    ///

    /// `boolean canLoadURLRemotely (in nsIURI url);`
    #[inline]
    pub unsafe fn CanLoadURLRemotely(&self, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanLoadURLRemotely)(self, url, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the passed chrome URL must be loaded in a remote process.
    ///    * This reflects the remoterequired flag on packages.
    ///    * Do not pass non-chrome URIs to this method.
    ///    */
    /// ```
    ///

    /// `boolean mustLoadURLRemotely (in nsIURI url);`
    #[inline]
    pub unsafe fn MustLoadURLRemotely(&self, url: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).MustLoadURLRemotely)(self, url, _retval)
    }


}


