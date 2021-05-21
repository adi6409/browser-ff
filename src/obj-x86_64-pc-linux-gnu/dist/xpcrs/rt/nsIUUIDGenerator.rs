//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIUUIDGenerator.idl
//


/// `interface nsIUUIDGenerator : nsISupports`
///

/// ```text
/// /**
///  * nsIUUIDGenerator is implemented by a service that can generate
///  * universally unique identifiers, ideally using any platform-native
///  * method for generating UUIDs.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUUIDGenerator {
    vtable: *const nsIUUIDGeneratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUUIDGenerator.
unsafe impl XpCom for nsIUUIDGenerator {
    const IID: nsIID = nsID(0x138ad1b2, 0xc694, 0x41cc,
        [0xb2, 0x01, 0x33, 0x3c, 0xe9, 0x36, 0xd8, 0xb8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUUIDGenerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUUIDGenerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUUIDGeneratorCoerce {
    /// Cheaply cast a value of this type from a `nsIUUIDGenerator`.
    fn coerce_from(v: &nsIUUIDGenerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUUIDGeneratorCoerce for nsIUUIDGenerator {
    #[inline]
    fn coerce_from(v: &nsIUUIDGenerator) -> &Self {
        v
    }
}

impl nsIUUIDGenerator {
    /// Cast this `nsIUUIDGenerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUUIDGeneratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUUIDGenerator {
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
impl<T: nsISupportsCoerce> nsIUUIDGeneratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUUIDGenerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUUIDGenerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUUIDGeneratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIDPtr generateUUID (); */
    pub GenerateUUID: unsafe extern "system" fn (this: *const nsIUUIDGenerator, _retval: *mut *mut nsID) -> ::nserror::nsresult,

    /* [noscript] void generateUUIDInPlace (in nsNonConstIDPtr id); */
    /// Unable to generate binding because `native type nsID unsupported`
    pub GenerateUUIDInPlace: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUUIDGenerator {

    /// ```text
    /// /**
    ///    * Obtains a new UUID using appropriate platform-specific methods to
    ///    * obtain a nsID that can be considered to be globally unique.
    ///    *
    ///    * @returns an nsID filled in with a new UUID.
    ///    *
    ///    * @throws NS_ERROR_FAILURE if a UUID cannot be generated (e.g. if
        ///    * an underlying source of randomness is not available)
    ///    */
    /// ```
    ///

    /// `nsIDPtr generateUUID ();`
    #[inline]
    pub unsafe fn GenerateUUID(&self, _retval: *mut *mut nsID) -> ::nserror::nsresult {
        ((*self.vtable).GenerateUUID)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Obtain a new UUID like the generateUUID method, but place it in
    ///    * the provided nsID pointer instead of allocating a new nsID.
    ///    *
    ///    * @param id an existing nsID pointer where the UUID will be stored.
    ///    *
    ///    * @throws NS_ERROR_FAILURE if a UUID cannot be generated (e.g. if
        ///    * an underlying source of randomness is not available)
    ///    */
    /// ```
    ///

    /// `[noscript] void generateUUIDInPlace (in nsNonConstIDPtr id);`
    const _GenerateUUIDInPlace: () = ();

}


