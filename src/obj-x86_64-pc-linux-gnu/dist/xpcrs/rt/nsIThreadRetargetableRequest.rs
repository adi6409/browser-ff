//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThreadRetargetableRequest.idl
//


/// `interface nsIThreadRetargetableRequest : nsISupports`
///

/// ```text
/// /**
///  * nsIThreadRetargetableRequest
///  *
///  * Should be implemented by requests that support retargeting delivery of
///  * data off the main thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThreadRetargetableRequest {
    vtable: *const nsIThreadRetargetableRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThreadRetargetableRequest.
unsafe impl XpCom for nsIThreadRetargetableRequest {
    const IID: nsIID = nsID(0x27b84c48, 0x5a73, 0x4ba4,
        [0xa8, 0xa4, 0x8b, 0x5e, 0x64, 0x9a, 0x14, 0x5e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThreadRetargetableRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThreadRetargetableRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadRetargetableRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIThreadRetargetableRequest`.
    fn coerce_from(v: &nsIThreadRetargetableRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadRetargetableRequestCoerce for nsIThreadRetargetableRequest {
    #[inline]
    fn coerce_from(v: &nsIThreadRetargetableRequest) -> &Self {
        v
    }
}

impl nsIThreadRetargetableRequest {
    /// Cast this `nsIThreadRetargetableRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadRetargetableRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThreadRetargetableRequest {
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
impl<T: nsISupportsCoerce> nsIThreadRetargetableRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadRetargetableRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThreadRetargetableRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadRetargetableRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void retargetDeliveryTo (in nsIEventTarget aNewTarget); */
    pub RetargetDeliveryTo: unsafe extern "system" fn (this: *const nsIThreadRetargetableRequest, aNewTarget: *const nsIEventTarget) -> ::nserror::nsresult,

    /* readonly attribute nsIEventTarget deliveryTarget; */
    pub GetDeliveryTarget: unsafe extern "system" fn (this: *const nsIThreadRetargetableRequest, aDeliveryTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThreadRetargetableRequest {

    /// ```text
    /// /**
    ///    * Called to retarget delivery of OnDataAvailable to another thread. Should
    ///    * only be called before AsyncOpen for nsIWebsocketChannels, or during
    ///    * OnStartRequest for nsIChannels.
    ///    * Note: For nsIChannels, OnStartRequest and OnStopRequest will still be
    ///    * delivered on the main thread.
    ///    *
    ///    * @param aNewTarget New event target, e.g. thread or threadpool.
    ///    *
    ///    * Note: no return value is given. If the retargeting cannot be handled,
    ///    * normal delivery to the main thread will continue. As such, listeners
    ///    * should be ready to deal with OnDataAvailable on either the main thread or
    ///    * the new target thread.
    ///    */
    /// ```
    ///

    /// `void retargetDeliveryTo (in nsIEventTarget aNewTarget);`
    #[inline]
    pub unsafe fn RetargetDeliveryTo(&self, aNewTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).RetargetDeliveryTo)(self, aNewTarget)
    }


    /// ```text
    /// /**
    ///    * Returns the event target where OnDataAvailable events will be dispatched.
    ///    *
    ///    * This is only valid after OnStartRequest has been called. Any time before
    ///    * that point, the value may be changed by `retargetDeliveryTo` calls.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIEventTarget deliveryTarget;`
    #[inline]
    pub unsafe fn GetDeliveryTarget(&self, aDeliveryTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetDeliveryTarget)(self, aDeliveryTarget)
    }


}


