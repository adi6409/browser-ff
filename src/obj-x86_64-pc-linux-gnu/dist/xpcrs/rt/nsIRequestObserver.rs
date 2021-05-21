//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestObserver.idl
//


/// `interface nsIRequestObserver : nsISupports`
///

/// ```text
/// /**
///  * nsIRequestObserver
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRequestObserver {
    vtable: *const nsIRequestObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRequestObserver.
unsafe impl XpCom for nsIRequestObserver {
    const IID: nsIID = nsID(0xfd91e2e0, 0x1481, 0x11d3,
        [0x93, 0x33, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRequestObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRequestObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRequestObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIRequestObserver`.
    fn coerce_from(v: &nsIRequestObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRequestObserverCoerce for nsIRequestObserver {
    #[inline]
    fn coerce_from(v: &nsIRequestObserver) -> &Self {
        v
    }
}

impl nsIRequestObserver {
    /// Cast this `nsIRequestObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRequestObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRequestObserver {
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
impl<T: nsISupportsCoerce> nsIRequestObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRequestObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRequestObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onStartRequest (in nsIRequest aRequest); */
    pub OnStartRequest: unsafe extern "system" fn (this: *const nsIRequestObserver, aRequest: *const nsIRequest) -> ::nserror::nsresult,

    /* void onStopRequest (in nsIRequest aRequest, in nsresult aStatusCode); */
    pub OnStopRequest: unsafe extern "system" fn (this: *const nsIRequestObserver, aRequest: *const nsIRequest, aStatusCode: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRequestObserver {

    /// ```text
    /// /**
    ///      * Called to signify the beginning of an asynchronous request.
    ///      *
    ///      * @param aRequest request being observed
    ///      *
    ///      * An exception thrown from onStartRequest has the side-effect of
    ///      * causing the request to be canceled.
    ///      */
    /// ```
    ///

    /// `void onStartRequest (in nsIRequest aRequest);`
    #[inline]
    pub unsafe fn OnStartRequest(&self, aRequest: *const nsIRequest) -> ::nserror::nsresult {
        ((*self.vtable).OnStartRequest)(self, aRequest)
    }


    /// ```text
    /// /**
    ///      * Called to signify the end of an asynchronous request.  This
    ///      * call is always preceded by a call to onStartRequest.
    ///      *
    ///      * @param aRequest request being observed
    ///      * @param aStatusCode reason for stopping (NS_OK if completed successfully)
    ///      *
    ///      * An exception thrown from onStopRequest is generally ignored.
    ///      */
    /// ```
    ///

    /// `void onStopRequest (in nsIRequest aRequest, in nsresult aStatusCode);`
    #[inline]
    pub unsafe fn OnStopRequest(&self, aRequest: *const nsIRequest, aStatusCode: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnStopRequest)(self, aRequest, aStatusCode)
    }


}


