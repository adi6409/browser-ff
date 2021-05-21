//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509CertValidity.idl
//


/// `interface nsIX509CertValidity : nsISupports`
///

/// ```text
/// /**
///  * Information on the validity period of a X.509 certificate.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIX509CertValidity {
    vtable: *const nsIX509CertValidityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIX509CertValidity.
unsafe impl XpCom for nsIX509CertValidity {
    const IID: nsIID = nsID(0xe701dfd8, 0x1dd1, 0x11b2,
        [0xa1, 0x72, 0xff, 0xa6, 0xcc, 0x61, 0x56, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIX509CertValidity {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIX509CertValidity.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIX509CertValidityCoerce {
    /// Cheaply cast a value of this type from a `nsIX509CertValidity`.
    fn coerce_from(v: &nsIX509CertValidity) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIX509CertValidityCoerce for nsIX509CertValidity {
    #[inline]
    fn coerce_from(v: &nsIX509CertValidity) -> &Self {
        v
    }
}

impl nsIX509CertValidity {
    /// Cast this `nsIX509CertValidity` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIX509CertValidityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIX509CertValidity {
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
impl<T: nsISupportsCoerce> nsIX509CertValidityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509CertValidity) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIX509CertValidity
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIX509CertValidityVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute PRTime notBefore; */
    pub GetNotBefore: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotBefore: *mut PRTime) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString notBeforeLocalTime; */
    pub GetNotBeforeLocalTime: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotBeforeLocalTime: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString notBeforeLocalDay; */
    pub GetNotBeforeLocalDay: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotBeforeLocalDay: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString notBeforeGMT; */
    pub GetNotBeforeGMT: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotBeforeGMT: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute PRTime notAfter; */
    pub GetNotAfter: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotAfter: *mut PRTime) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString notAfterLocalTime; */
    pub GetNotAfterLocalTime: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotAfterLocalTime: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString notAfterLocalDay; */
    pub GetNotAfterLocalDay: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotAfterLocalDay: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString notAfterGMT; */
    pub GetNotAfterGMT: unsafe extern "system" fn (this: *const nsIX509CertValidity, aNotAfterGMT: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIX509CertValidity {

    /// ```text
    /// /**
    ///    *  The earliest point in time where
    ///    *  a certificate is valid.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime notBefore;`
    #[inline]
    pub unsafe fn GetNotBefore(&self, aNotBefore: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetNotBefore)(self, aNotBefore)
    }


    /// ```text
    /// /**
    ///    *  "notBefore" attribute formatted as a time string
    ///    *  according to the environment locale,
    ///    *  according to the environment time zone.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString notBeforeLocalTime;`
    #[inline]
    pub unsafe fn GetNotBeforeLocalTime(&self, aNotBeforeLocalTime: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNotBeforeLocalTime)(self, aNotBeforeLocalTime)
    }


    /// ```text
    /// /**
    ///    *  The day portion of "notBefore" formatted as a time string
    ///    *  according to the environment locale,
    ///    *  according to the environment time zone.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString notBeforeLocalDay;`
    #[inline]
    pub unsafe fn GetNotBeforeLocalDay(&self, aNotBeforeLocalDay: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNotBeforeLocalDay)(self, aNotBeforeLocalDay)
    }


    /// ```text
    /// /**
    ///    *  "notBefore" attribute formatted as a string
    ///    *  according to the environment locale,
    ///    *  displayed as GMT / UTC.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString notBeforeGMT;`
    #[inline]
    pub unsafe fn GetNotBeforeGMT(&self, aNotBeforeGMT: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNotBeforeGMT)(self, aNotBeforeGMT)
    }


    /// ```text
    /// /**
    ///    *  The latest point in time where
    ///    *  a certificate is valid.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime notAfter;`
    #[inline]
    pub unsafe fn GetNotAfter(&self, aNotAfter: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetNotAfter)(self, aNotAfter)
    }


    /// ```text
    /// /**
    ///    *  "notAfter" attribute formatted as a time string
    ///    *  according to the environment locale,
    ///    *  according to the environment time zone.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString notAfterLocalTime;`
    #[inline]
    pub unsafe fn GetNotAfterLocalTime(&self, aNotAfterLocalTime: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNotAfterLocalTime)(self, aNotAfterLocalTime)
    }


    /// ```text
    /// /**
    ///    *  The day portion of "notAfter" formatted as a time string
    ///    *  according to the environment locale,
    ///    *  according to the environment time zone.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString notAfterLocalDay;`
    #[inline]
    pub unsafe fn GetNotAfterLocalDay(&self, aNotAfterLocalDay: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNotAfterLocalDay)(self, aNotAfterLocalDay)
    }


    /// ```text
    /// /**
    ///    *  "notAfter" attribute formatted as a time string
    ///    *  according to the environment locale,
    ///    *  displayed as GMT / UTC.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AString notAfterGMT;`
    #[inline]
    pub unsafe fn GetNotAfterGMT(&self, aNotAfterGMT: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNotAfterGMT)(self, aNotAfterGMT)
    }


}


