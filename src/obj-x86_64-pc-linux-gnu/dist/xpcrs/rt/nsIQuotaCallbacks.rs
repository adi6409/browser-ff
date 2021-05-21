//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaCallbacks.idl
//


/// `interface nsIQuotaUsageCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaUsageCallback {
    vtable: *const nsIQuotaUsageCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaUsageCallback.
unsafe impl XpCom for nsIQuotaUsageCallback {
    const IID: nsIID = nsID(0xc8a21a2a, 0x17b9, 0x4b63,
        [0xad, 0x95, 0xe0, 0xfb, 0xcf, 0xf5, 0xde, 0x18]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaUsageCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaUsageCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaUsageCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaUsageCallback`.
    fn coerce_from(v: &nsIQuotaUsageCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaUsageCallbackCoerce for nsIQuotaUsageCallback {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageCallback) -> &Self {
        v
    }
}

impl nsIQuotaUsageCallback {
    /// Cast this `nsIQuotaUsageCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaUsageCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaUsageCallback {
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
impl<T: nsISupportsCoerce> nsIQuotaUsageCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaUsageCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaUsageCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onUsageResult (in nsIQuotaUsageRequest aRequest); */
    pub OnUsageResult: unsafe extern "system" fn (this: *const nsIQuotaUsageCallback, aRequest: *const nsIQuotaUsageRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaUsageCallback {


    /// `void onUsageResult (in nsIQuotaUsageRequest aRequest);`
    #[inline]
    pub unsafe fn OnUsageResult(&self, aRequest: *const nsIQuotaUsageRequest) -> ::nserror::nsresult {
        ((*self.vtable).OnUsageResult)(self, aRequest)
    }


}


/// `interface nsIQuotaCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaCallback {
    vtable: *const nsIQuotaCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaCallback.
unsafe impl XpCom for nsIQuotaCallback {
    const IID: nsIID = nsID(0xa08a28e2, 0x5a74, 0x4c84,
        [0x80, 0x70, 0xed, 0x45, 0xa0, 0x7e, 0xb0, 0x13]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaCallback`.
    fn coerce_from(v: &nsIQuotaCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaCallbackCoerce for nsIQuotaCallback {
    #[inline]
    fn coerce_from(v: &nsIQuotaCallback) -> &Self {
        v
    }
}

impl nsIQuotaCallback {
    /// Cast this `nsIQuotaCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaCallback {
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
impl<T: nsISupportsCoerce> nsIQuotaCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onComplete (in nsIQuotaRequest aRequest); */
    pub OnComplete: unsafe extern "system" fn (this: *const nsIQuotaCallback, aRequest: *const nsIQuotaRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaCallback {


    /// `void onComplete (in nsIQuotaRequest aRequest);`
    #[inline]
    pub unsafe fn OnComplete(&self, aRequest: *const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).OnComplete)(self, aRequest)
    }


}


