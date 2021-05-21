//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStandardURL.idl
//


/// `interface nsIStandardURL : nsISupports`
///

/// ```text
/// /**
///  * nsIStandardURL defines the interface to an URL with the standard
///  * file path format common to protocols like http, ftp, and file.
///  * It supports initialization from a relative path and provides
///  * some customization on how URLs are normalized.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStandardURL {
    vtable: *const nsIStandardURLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStandardURL.
unsafe impl XpCom for nsIStandardURL {
    const IID: nsIID = nsID(0xbabd6cca, 0xebe7, 0x4329,
        [0x96, 0x7c, 0xd6, 0xb9, 0xe3, 0x3c, 0xaa, 0x81]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStandardURL {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStandardURL.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStandardURLCoerce {
    /// Cheaply cast a value of this type from a `nsIStandardURL`.
    fn coerce_from(v: &nsIStandardURL) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStandardURLCoerce for nsIStandardURL {
    #[inline]
    fn coerce_from(v: &nsIStandardURL) -> &Self {
        v
    }
}

impl nsIStandardURL {
    /// Cast this `nsIStandardURL` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStandardURLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStandardURL {
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
impl<T: nsISupportsCoerce> nsIStandardURLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStandardURL) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStandardURL
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStandardURLVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStandardURL {
    /// ```text
    /// /**
    ///      * blah:foo/bar    => blah://foo/bar
    ///      * blah:/foo/bar   => blah:///foo/bar
    ///      * blah://foo/bar  => blah://foo/bar
    ///      * blah:///foo/bar => blah:///foo/bar
    ///      */
    /// ```
    ///

    pub const URLTYPE_STANDARD: i64 = 1;

    /// ```text
    /// /**
    ///      * blah:foo/bar    => blah://foo/bar
    ///      * blah:/foo/bar   => blah://foo/bar
    ///      * blah://foo/bar  => blah://foo/bar
    ///      * blah:///foo/bar => blah://foo/bar
    ///      */
    /// ```
    ///

    pub const URLTYPE_AUTHORITY: i64 = 2;

    /// ```text
    /// /**
    ///      * blah:foo/bar    => blah:///foo/bar
    ///      * blah:/foo/bar   => blah:///foo/bar
    ///      * blah://foo/bar  => blah://foo/bar
    ///      * blah:///foo/bar => blah:///foo/bar
    ///      */
    /// ```
    ///

    pub const URLTYPE_NO_AUTHORITY: i64 = 3;


}


/// `interface nsIStandardURLMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStandardURLMutator {
    vtable: *const nsIStandardURLMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStandardURLMutator.
unsafe impl XpCom for nsIStandardURLMutator {
    const IID: nsIID = nsID(0xfc894e98, 0x23a1, 0x43cd,
        [0xa7, 0xfe, 0x72, 0x87, 0x6f, 0x8e, 0xa2, 0xee]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStandardURLMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStandardURLMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStandardURLMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsIStandardURLMutator`.
    fn coerce_from(v: &nsIStandardURLMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStandardURLMutatorCoerce for nsIStandardURLMutator {
    #[inline]
    fn coerce_from(v: &nsIStandardURLMutator) -> &Self {
        v
    }
}

impl nsIStandardURLMutator {
    /// Cast this `nsIStandardURLMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStandardURLMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStandardURLMutator {
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
impl<T: nsISupportsCoerce> nsIStandardURLMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStandardURLMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStandardURLMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStandardURLMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIURIMutator init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
    pub Init: unsafe extern "system" fn (this: *const nsIStandardURLMutator, aUrlType: u32, aDefaultPort: i32, aSpec: *const ::nsstring::nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* nsIURIMutator setDefaultPort (in long aNewDefaultPort); */
    pub SetDefaultPort: unsafe extern "system" fn (this: *const nsIStandardURLMutator, aNewDefaultPort: i32, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStandardURLMutator {

    /// ```text
    /// /**
    ///      * Initialize a standard URL.
    ///      *
    ///      * @param aUrlType       - one of the URLTYPE_ flags listed above.
    ///      * @param aDefaultPort   - if the port parsed from the URL string matches
    ///      *                         this port, then the port will be removed from the
    ///      *                         canonical form of the URL.
    ///      * @param aSpec          - URL string.
    ///      * @param aOriginCharset - the charset from which this URI string
    ///      *                         originated.  this corresponds to the charset
    ///      *                         that should be used when communicating this
    ///      *                         URI to an origin server, for example.  if
    ///      *                         null, then provide aBaseURI implements this
    ///      *                         interface, the origin charset of aBaseURI will
    ///      *                         be assumed, otherwise defaulting to UTF-8 (i.e.,
        ///      *                         no charset transformation from aSpec).
    ///      * @param aBaseURI       - if null, aSpec must specify an absolute URI.
    ///      *                         otherwise, aSpec will be resolved relative
    ///      *                         to aBaseURI.
    ///      */
    /// ```
    ///

    /// `nsIURIMutator init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI);`
    #[inline]
    pub unsafe fn Init(&self, aUrlType: u32, aDefaultPort: i32, aSpec: *const ::nsstring::nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aUrlType, aDefaultPort, aSpec, aOriginCharset, aBaseURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Set the default port.
    ///      *
    ///      * Note: If this object is already using its default port (i.e. if it has
        ///      * mPort == -1), then it will now implicitly be using the new default port.
    ///      *
    ///      * @param aNewDefaultPort - if the URI has (or is later given) a port that
    ///      *                          matches this default, then we won't include a
    ///      *                          port number in the canonical form of the URL.
    ///      */
    /// ```
    ///

    /// `nsIURIMutator setDefaultPort (in long aNewDefaultPort);`
    #[inline]
    pub unsafe fn SetDefaultPort(&self, aNewDefaultPort: i32, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultPort)(self, aNewDefaultPort, _retval)
    }


}


