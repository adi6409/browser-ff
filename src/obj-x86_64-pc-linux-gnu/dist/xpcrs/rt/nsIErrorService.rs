//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIErrorService.idl
//


/// `interface nsIErrorService : nsISupports`
///

/// ```text
/// /**
///  * nsIErrorService: This is an interim service that allows nsresult codes to be mapped to
///  * string bundles that can be used to look up error messages. String bundle keys can also
///  * be mapped.
///  *
///  * This service will eventually get replaced by extending xpidl to allow errors to be defined.
///  * (http://bugzilla.mozilla.org/show_bug.cgi?id=13423).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIErrorService {
    vtable: *const nsIErrorServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIErrorService.
unsafe impl XpCom for nsIErrorService {
    const IID: nsIID = nsID(0xafe1f190, 0xa3c2, 0x11e3,
        [0xa5, 0xe2, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIErrorService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIErrorService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIErrorServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIErrorService`.
    fn coerce_from(v: &nsIErrorService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIErrorServiceCoerce for nsIErrorService {
    #[inline]
    fn coerce_from(v: &nsIErrorService) -> &Self {
        v
    }
}

impl nsIErrorService {
    /// Cast this `nsIErrorService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIErrorServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIErrorService {
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
impl<T: nsISupportsCoerce> nsIErrorServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIErrorService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIErrorService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIErrorServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void registerErrorStringBundle (in short errorModule, in string stringBundleURL); */
    pub RegisterErrorStringBundle: unsafe extern "system" fn (this: *const nsIErrorService, errorModule: i16, stringBundleURL: *const libc::c_char) -> ::nserror::nsresult,

    /* void unregisterErrorStringBundle (in short errorModule); */
    pub UnregisterErrorStringBundle: unsafe extern "system" fn (this: *const nsIErrorService, errorModule: i16) -> ::nserror::nsresult,

    /* string getErrorStringBundle (in short errorModule); */
    pub GetErrorStringBundle: unsafe extern "system" fn (this: *const nsIErrorService, errorModule: i16, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIErrorService {

    /// ```text
    /// /**
    ///      * Registers a string bundle URL for an error module. Error modules are obtained from
    ///      * nsresult code with NS_ERROR_GET_MODULE.
    ///      */
    /// ```
    ///

    /// `void registerErrorStringBundle (in short errorModule, in string stringBundleURL);`
    #[inline]
    pub unsafe fn RegisterErrorStringBundle(&self, errorModule: i16, stringBundleURL: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RegisterErrorStringBundle)(self, errorModule, stringBundleURL)
    }


    /// ```text
    /// /**
    ///      * Unregisters a string bundle URL for an error module.
    ///      */
    /// ```
    ///

    /// `void unregisterErrorStringBundle (in short errorModule);`
    #[inline]
    pub unsafe fn UnregisterErrorStringBundle(&self, errorModule: i16) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterErrorStringBundle)(self, errorModule)
    }


    /// ```text
    /// /**
    ///      * Retrieves a string bundle URL for an error module.
    ///      */
    /// ```
    ///

    /// `string getErrorStringBundle (in short errorModule);`
    #[inline]
    pub unsafe fn GetErrorStringBundle(&self, errorModule: i16, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorStringBundle)(self, errorModule, _retval)
    }


}


