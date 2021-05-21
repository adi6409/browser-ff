//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/gmp/mozIGeckoMediaPluginChromeService.idl
//


/// `interface mozIGeckoMediaPluginChromeService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIGeckoMediaPluginChromeService {
    vtable: *const mozIGeckoMediaPluginChromeServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIGeckoMediaPluginChromeService.
unsafe impl XpCom for mozIGeckoMediaPluginChromeService {
    const IID: nsIID = nsID(0x32d35d21, 0x181f, 0x4630,
        [0x8c, 0xaa, 0xa4, 0x31, 0xe2, 0xeb, 0xad, 0x72]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIGeckoMediaPluginChromeService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIGeckoMediaPluginChromeService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIGeckoMediaPluginChromeServiceCoerce {
    /// Cheaply cast a value of this type from a `mozIGeckoMediaPluginChromeService`.
    fn coerce_from(v: &mozIGeckoMediaPluginChromeService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIGeckoMediaPluginChromeServiceCoerce for mozIGeckoMediaPluginChromeService {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginChromeService) -> &Self {
        v
    }
}

impl mozIGeckoMediaPluginChromeService {
    /// Cast this `mozIGeckoMediaPluginChromeService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIGeckoMediaPluginChromeServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIGeckoMediaPluginChromeService {
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
impl<T: nsISupportsCoerce> mozIGeckoMediaPluginChromeServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginChromeService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIGeckoMediaPluginChromeService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIGeckoMediaPluginChromeServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addPluginDirectory (in AString directory); */
    pub AddPluginDirectory: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginChromeService, directory: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removePluginDirectory (in AString directory); */
    pub RemovePluginDirectory: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginChromeService, directory: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer); */
    pub RemoveAndDeletePluginDirectory: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginChromeService, directory: *const ::nsstring::nsAString, defer: bool) -> ::nserror::nsresult,

    /* void forgetThisSite (in AString site, in AString aPattern); */
    pub ForgetThisSite: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginChromeService, site: *const ::nsstring::nsAString, aPattern: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* bool isPersistentStorageAllowed (in ACString nodeId); */
    pub IsPersistentStorageAllowed: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginChromeService, nodeId: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIFile getStorageDir (); */
    pub GetStorageDir: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginChromeService, _retval: *mut *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIGeckoMediaPluginChromeService {

    /// ```text
    /// /**
    ///    * Add a directory to scan for gecko media plugins.
    ///    * @note Main-thread API.
    ///    */
    /// ```
    ///

    /// `void addPluginDirectory (in AString directory);`
    #[inline]
    pub unsafe fn AddPluginDirectory(&self, directory: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddPluginDirectory)(self, directory)
    }


    /// ```text
    /// /**
    ///    * Remove a directory for gecko media plugins.
    ///    * @note Main-thread API.
    ///    */
    /// ```
    ///

    /// `void removePluginDirectory (in AString directory);`
    #[inline]
    pub unsafe fn RemovePluginDirectory(&self, directory: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemovePluginDirectory)(self, directory)
    }


    /// ```text
    /// /**
    ///    * Remove a directory for gecko media plugins and delete it from disk.
    ///    * If |defer| is true, wait until the plugin is unused before removing.
    ///    * @note Main-thread API.
    ///    */
    /// ```
    ///

    /// `void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer);`
    #[inline]
    pub unsafe fn RemoveAndDeletePluginDirectory(&self, directory: *const ::nsstring::nsAString, defer: bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAndDeletePluginDirectory)(self, directory, defer)
    }


    /// ```text
    /// /**
    ///    * Clears storage data associated with the site and the originAttributes
    ///    * pattern in JSON format.
    ///    */
    /// ```
    ///

    /// `void forgetThisSite (in AString site, in AString aPattern);`
    #[inline]
    pub unsafe fn ForgetThisSite(&self, site: *const ::nsstring::nsAString, aPattern: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ForgetThisSite)(self, site, aPattern)
    }


    /// ```text
    /// /**
    ///    * Returns true if the given node id is allowed to store things
    ///    * persistently on disk. Private Browsing and local content are not
    ///    * allowed to store persistent data.
    ///    */
    /// ```
    ///

    /// `bool isPersistentStorageAllowed (in ACString nodeId);`
    #[inline]
    pub unsafe fn IsPersistentStorageAllowed(&self, nodeId: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPersistentStorageAllowed)(self, nodeId, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the directory to use as the base for storing data about GMPs.
    ///    */
    /// ```
    ///

    /// `nsIFile getStorageDir ();`
    #[inline]
    pub unsafe fn GetStorageDir(&self, _retval: *mut *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageDir)(self, _retval)
    }


}


