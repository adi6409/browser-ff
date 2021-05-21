//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIJARURI.idl
//


/// `interface nsIJARURI : nsIURL`
///

/// ```text
/// /**
///  * JAR URLs have the following syntax
///  *
///  * jar:<jar-file-uri>!/<jar-entry>
///  *
///  * EXAMPLE: jar:http://www.big.com/blue.jar!/ocean.html
///  *
///  * The nsIURL methods operate on the <jar-entry> part of the spec.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJARURI {
    vtable: *const nsIJARURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJARURI.
unsafe impl XpCom for nsIJARURI {
    const IID: nsIID = nsID(0x646a508c, 0xf786, 0x4e14,
        [0xbe, 0x6d, 0x8d, 0xda, 0x2a, 0x63, 0x3c, 0x60]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJARURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJARURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJARURICoerce {
    /// Cheaply cast a value of this type from a `nsIJARURI`.
    fn coerce_from(v: &nsIJARURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJARURICoerce for nsIJARURI {
    #[inline]
    fn coerce_from(v: &nsIJARURI) -> &Self {
        v
    }
}

impl nsIJARURI {
    /// Cast this `nsIJARURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJARURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJARURI {
    type Target = nsIURL;
    #[inline]
    fn deref(&self) -> &nsIURL {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIURLCoerce> nsIJARURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJARURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJARURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJARURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIURLVTable,

    /* readonly attribute nsIURI JARFile; */
    pub GetJARFile: unsafe extern "system" fn (this: *const nsIJARURI, aJARFile: *mut *const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String JAREntry; */
    pub GetJAREntry: unsafe extern "system" fn (this: *const nsIJARURI, aJAREntry: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJARURI {

    /// ```text
    /// /**
    ///      * Returns the root URI (the one for the actual JAR file) for this JAR
    ///      * (e.g., http://www.big.com/blue.jar).
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIURI JARFile;`
    #[inline]
    pub unsafe fn GetJARFile(&self, aJARFile: *mut *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetJARFile)(self, aJARFile)
    }


    /// ```text
    /// /**
    ///      * Returns the entry specified for this JAR URI (e.g., "ocean.html").  This
    ///      * value may contain %-escaped byte sequences.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String JAREntry;`
    #[inline]
    pub unsafe fn GetJAREntry(&self, aJAREntry: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetJAREntry)(self, aJAREntry)
    }


}


/// `interface nsIJARURIMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJARURIMutator {
    vtable: *const nsIJARURIMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJARURIMutator.
unsafe impl XpCom for nsIJARURIMutator {
    const IID: nsIID = nsID(0xd66df117, 0xeda7, 0x4324,
        [0xb4, 0xe4, 0x1f, 0x67, 0x0f, 0xf6, 0x71, 0x8e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJARURIMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJARURIMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJARURIMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsIJARURIMutator`.
    fn coerce_from(v: &nsIJARURIMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJARURIMutatorCoerce for nsIJARURIMutator {
    #[inline]
    fn coerce_from(v: &nsIJARURIMutator) -> &Self {
        v
    }
}

impl nsIJARURIMutator {
    /// Cast this `nsIJARURIMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJARURIMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJARURIMutator {
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
impl<T: nsISupportsCoerce> nsIJARURIMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJARURIMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJARURIMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJARURIMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setSpecBaseCharset (in AUTF8String aSpec, in nsIURI aBase, in string aCharset); */
    pub SetSpecBaseCharset: unsafe extern "system" fn (this: *const nsIJARURIMutator, aSpec: *const ::nsstring::nsACString, aBase: *const nsIURI, aCharset: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJARURIMutator {

    /// ```text
    /// /**
    ///      * Will initalize a URI using the passed spec, baseURI and charset.
    ///      */
    /// ```
    ///

    /// `void setSpecBaseCharset (in AUTF8String aSpec, in nsIURI aBase, in string aCharset);`
    #[inline]
    pub unsafe fn SetSpecBaseCharset(&self, aSpec: *const ::nsstring::nsACString, aBase: *const nsIURI, aCharset: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetSpecBaseCharset)(self, aSpec, aBase, aCharset)
    }


}


