//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalURLHandlerService.idl
//


/// `interface nsIExternalURLHandlerService : nsISupports`
///

/// ```text
/// /**
///  * The external URL handler service is used for finding
///  * platform-specific applications for handling particular URLs.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIExternalURLHandlerService {
    vtable: *const nsIExternalURLHandlerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIExternalURLHandlerService.
unsafe impl XpCom for nsIExternalURLHandlerService {
    const IID: nsIID = nsID(0x56c5c7d3, 0x6fd3, 0x43f8,
        [0x94, 0x29, 0x43, 0x97, 0xe1, 0x11, 0x45, 0x3a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIExternalURLHandlerService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIExternalURLHandlerService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExternalURLHandlerServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIExternalURLHandlerService`.
    fn coerce_from(v: &nsIExternalURLHandlerService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExternalURLHandlerServiceCoerce for nsIExternalURLHandlerService {
    #[inline]
    fn coerce_from(v: &nsIExternalURLHandlerService) -> &Self {
        v
    }
}

impl nsIExternalURLHandlerService {
    /// Cast this `nsIExternalURLHandlerService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExternalURLHandlerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIExternalURLHandlerService {
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
impl<T: nsISupportsCoerce> nsIExternalURLHandlerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalURLHandlerService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIExternalURLHandlerService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExternalURLHandlerServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound); */
    pub GetURLHandlerInfoFromOS: unsafe extern "system" fn (this: *const nsIExternalURLHandlerService, aURL: *const nsIURI, aFound: *mut bool, _retval: *mut *const nsIHandlerInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIExternalURLHandlerService {

    /// ```text
    /// /**
    ///    * Given a URL, looks up the handler info from the OS. This should be
    ///    * overridden by each OS's implementation.
    ///    *
    ///    * @param aURL The URL we are looking for.
    ///    * @param aFound  Was an OS default handler for this URL found?
    ///    * @return  An nsIHanderInfo for the protocol.
    ///    */
    /// ```
    ///

    /// `nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound);`
    #[inline]
    pub unsafe fn GetURLHandlerInfoFromOS(&self, aURL: *const nsIURI, aFound: *mut bool, _retval: *mut *const nsIHandlerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetURLHandlerInfoFromOS)(self, aURL, aFound, _retval)
    }


}


