//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/res/nsISubstitutingProtocolHandler.idl
//


/// `interface nsISubstitutingProtocolHandler : nsIProtocolHandler`
///

/// ```text
/// /**
///  * Protocol handler superinterface for a protocol which performs substitutions
///  * from URIs of its scheme to URIs of another scheme.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISubstitutingProtocolHandler {
    vtable: *const nsISubstitutingProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISubstitutingProtocolHandler.
unsafe impl XpCom for nsISubstitutingProtocolHandler {
    const IID: nsIID = nsID(0x154c64fd, 0xa69e, 0x4105,
        [0x89, 0xf8, 0xbd, 0x7d, 0xfe, 0x62, 0x13, 0x72]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISubstitutingProtocolHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISubstitutingProtocolHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISubstitutingProtocolHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsISubstitutingProtocolHandler`.
    fn coerce_from(v: &nsISubstitutingProtocolHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISubstitutingProtocolHandlerCoerce for nsISubstitutingProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsISubstitutingProtocolHandler) -> &Self {
        v
    }
}

impl nsISubstitutingProtocolHandler {
    /// Cast this `nsISubstitutingProtocolHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISubstitutingProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISubstitutingProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProtocolHandlerCoerce> nsISubstitutingProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISubstitutingProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISubstitutingProtocolHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISubstitutingProtocolHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProtocolHandlerVTable,

    /* [must_use] void setSubstitution (in ACString root, in nsIURI baseURI); */
    pub SetSubstitution: unsafe extern "system" fn (this: *const nsISubstitutingProtocolHandler, root: *const ::nsstring::nsACString, baseURI: *const nsIURI) -> ::nserror::nsresult,

    /* [must_use] void setSubstitutionWithFlags (in ACString root, in nsIURI baseURI, in uint32_t flags); */
    pub SetSubstitutionWithFlags: unsafe extern "system" fn (this: *const nsISubstitutingProtocolHandler, root: *const ::nsstring::nsACString, baseURI: *const nsIURI, flags: uint32_t) -> ::nserror::nsresult,

    /* [must_use] nsIURI getSubstitution (in ACString root); */
    pub GetSubstitution: unsafe extern "system" fn (this: *const nsISubstitutingProtocolHandler, root: *const ::nsstring::nsACString, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] boolean hasSubstitution (in ACString root); */
    pub HasSubstitution: unsafe extern "system" fn (this: *const nsISubstitutingProtocolHandler, root: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] AUTF8String resolveURI (in nsIURI resURI); */
    pub ResolveURI: unsafe extern "system" fn (this: *const nsISubstitutingProtocolHandler, resURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISubstitutingProtocolHandler {
    /// ```text
    /// /**
    ///    * Content script may access files in this package.
    ///    */
    /// ```
    ///

    pub const ALLOW_CONTENT_ACCESS: i64 = 1;

    /// ```text
    /// /**
    ///    * This substitution exposes nsIJARURI instead of a nsIFileURL.  By default
    ///    * NewURI will always return a nsIFileURL even when the URL is jar:
    ///    */
    /// ```
    ///

    pub const RESOLVE_JAR_URI: i64 = 2;

    /// ```text
    /// /**
    ///    * Sets the substitution for the root key:
    ///    *   resource://root/path ==> baseURI.resolve(path)
    ///    *
    ///    * A null baseURI removes the specified substitution.
    ///    *
    ///    * The root key will be converted to lower-case to conform to
    ///    * case-insensitive URI hostname matching behavior.
    ///    */
    /// ```
    ///

    /// `[must_use] void setSubstitution (in ACString root, in nsIURI baseURI);`
    #[inline]
    pub unsafe fn SetSubstitution(&self, root: *const ::nsstring::nsACString, baseURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetSubstitution)(self, root, baseURI)
    }


    /// ```text
    /// /**
    ///    * Same as setSubstitution, but with specific flags.
    ///    */
    /// ```
    ///

    /// `[must_use] void setSubstitutionWithFlags (in ACString root, in nsIURI baseURI, in uint32_t flags);`
    #[inline]
    pub unsafe fn SetSubstitutionWithFlags(&self, root: *const ::nsstring::nsACString, baseURI: *const nsIURI, flags: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetSubstitutionWithFlags)(self, root, baseURI, flags)
    }


    /// ```text
    /// /**
    ///    * Gets the substitution for the root key.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE if none exists.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIURI getSubstitution (in ACString root);`
    #[inline]
    pub unsafe fn GetSubstitution(&self, root: *const ::nsstring::nsACString, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetSubstitution)(self, root, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns TRUE if the substitution exists and FALSE otherwise.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean hasSubstitution (in ACString root);`
    #[inline]
    pub unsafe fn HasSubstitution(&self, root: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasSubstitution)(self, root, _retval)
    }


    /// ```text
    /// /**
    ///    * Utility function to resolve a substituted URI.  A resolved URI is not
    ///    * guaranteed to reference a resource that exists (ie. opening a channel to
        ///    * the resolved URI may fail).
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE if resURI.host() is an unknown root key.
    ///    */
    /// ```
    ///

    /// `[must_use] AUTF8String resolveURI (in nsIURI resURI);`
    #[inline]
    pub unsafe fn ResolveURI(&self, resURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ResolveURI)(self, resURI, _retval)
    }


}


