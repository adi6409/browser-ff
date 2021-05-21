//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIContentSniffer.idl
//


/// `interface nsIContentSniffer : nsISupports`
///

/// ```text
/// /**
///  * Content sniffer interface. Components implementing this interface can
///  * determine a MIME type from a chunk of bytes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentSniffer {
    vtable: *const nsIContentSnifferVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentSniffer.
unsafe impl XpCom for nsIContentSniffer {
    const IID: nsIID = nsID(0xa5772d1b, 0xfc63, 0x495e,
        [0xa1, 0x69, 0x96, 0xe8, 0xd3, 0x31, 0x1a, 0xf0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentSniffer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentSniffer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentSnifferCoerce {
    /// Cheaply cast a value of this type from a `nsIContentSniffer`.
    fn coerce_from(v: &nsIContentSniffer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentSnifferCoerce for nsIContentSniffer {
    #[inline]
    fn coerce_from(v: &nsIContentSniffer) -> &Self {
        v
    }
}

impl nsIContentSniffer {
    /// Cast this `nsIContentSniffer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentSnifferCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentSniffer {
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
impl<T: nsISupportsCoerce> nsIContentSnifferCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSniffer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentSniffer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentSnifferVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getMIMETypeFromContent (in nsIRequest aRequest, [array, size_is (aLength), const] in octet aData, in unsigned long aLength); */
    pub GetMIMETypeFromContent: unsafe extern "system" fn (this: *const nsIContentSniffer, aRequest: *const nsIRequest, aData: *const u8, aLength: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentSniffer {

    /// ```text
    /// /**
    ///    * Given a chunk of data, determines a MIME type. Information from the given
    ///    * request may be used in order to make a better decision.
    ///    *
    ///    * @param aRequest The request where this data came from. May be null.
    ///    * @param aData Data to check
    ///    * @param aLength Length of the data
    ///    *
    ///    * @return The content type
    ///    *
    ///    * @throw NS_ERROR_NOT_AVAILABLE if no MIME type could be determined.
    ///    *
    ///    * @note Implementations should consider the request read-only. Especially,
    ///    * they should not attempt to set the content type property that subclasses of
    ///    * nsIRequest might offer.
    ///    */
    /// ```
    ///

    /// `ACString getMIMETypeFromContent (in nsIRequest aRequest, [array, size_is (aLength), const] in octet aData, in unsigned long aLength);`
    #[inline]
    pub unsafe fn GetMIMETypeFromContent(&self, aRequest: *const nsIRequest, aData: *const u8, aLength: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMIMETypeFromContent)(self, aRequest, aData, aLength, _retval)
    }


}


