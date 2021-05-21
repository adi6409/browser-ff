//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIEventSourceEventService.idl
//


/// `interface nsIEventSourceEventListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEventSourceEventListener {
    vtable: *const nsIEventSourceEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEventSourceEventListener.
unsafe impl XpCom for nsIEventSourceEventListener {
    const IID: nsIID = nsID(0xd2cc6222, 0xb7f2, 0x490d,
        [0xad, 0xc2, 0x49, 0x7d, 0x89, 0x87, 0x8f, 0xa2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEventSourceEventListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEventSourceEventListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEventSourceEventListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIEventSourceEventListener`.
    fn coerce_from(v: &nsIEventSourceEventListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEventSourceEventListenerCoerce for nsIEventSourceEventListener {
    #[inline]
    fn coerce_from(v: &nsIEventSourceEventListener) -> &Self {
        v
    }
}

impl nsIEventSourceEventListener {
    /// Cast this `nsIEventSourceEventListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEventSourceEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEventSourceEventListener {
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
impl<T: nsISupportsCoerce> nsIEventSourceEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventSourceEventListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEventSourceEventListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEventSourceEventListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void eventSourceConnectionOpened (in uint64_t aHttpChannelId); */
    pub EventSourceConnectionOpened: unsafe extern "system" fn (this: *const nsIEventSourceEventListener, aHttpChannelId: uint64_t) -> ::nserror::nsresult,

    /* [must_use] void eventSourceConnectionClosed (in uint64_t aHttpChannelId); */
    pub EventSourceConnectionClosed: unsafe extern "system" fn (this: *const nsIEventSourceEventListener, aHttpChannelId: uint64_t) -> ::nserror::nsresult,

    /* [must_use] void eventReceived (in uint64_t aHttpChannelId, in AString aEventName, in AString aLastEventID, in AString aData, in uint32_t aRetry, in DOMHighResTimeStamp aTimeStamp); */
    pub EventReceived: unsafe extern "system" fn (this: *const nsIEventSourceEventListener, aHttpChannelId: uint64_t, aEventName: *const ::nsstring::nsAString, aLastEventID: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aRetry: uint32_t, aTimeStamp: DOMHighResTimeStamp) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEventSourceEventListener {


    /// `[must_use] void eventSourceConnectionOpened (in uint64_t aHttpChannelId);`
    #[inline]
    pub unsafe fn EventSourceConnectionOpened(&self, aHttpChannelId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).EventSourceConnectionOpened)(self, aHttpChannelId)
    }



    /// `[must_use] void eventSourceConnectionClosed (in uint64_t aHttpChannelId);`
    #[inline]
    pub unsafe fn EventSourceConnectionClosed(&self, aHttpChannelId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).EventSourceConnectionClosed)(self, aHttpChannelId)
    }



    /// `[must_use] void eventReceived (in uint64_t aHttpChannelId, in AString aEventName, in AString aLastEventID, in AString aData, in uint32_t aRetry, in DOMHighResTimeStamp aTimeStamp);`
    #[inline]
    pub unsafe fn EventReceived(&self, aHttpChannelId: uint64_t, aEventName: *const ::nsstring::nsAString, aLastEventID: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aRetry: uint32_t, aTimeStamp: DOMHighResTimeStamp) -> ::nserror::nsresult {
        ((*self.vtable).EventReceived)(self, aHttpChannelId, aEventName, aLastEventID, aData, aRetry, aTimeStamp)
    }


}


/// `interface nsIEventSourceEventService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEventSourceEventService {
    vtable: *const nsIEventSourceEventServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEventSourceEventService.
unsafe impl XpCom for nsIEventSourceEventService {
    const IID: nsIID = nsID(0xc0378840, 0x8a74, 0x4b0a,
        [0x92, 0x25, 0xc3, 0xa0, 0xac, 0x1f, 0xac, 0x41]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEventSourceEventService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEventSourceEventService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEventSourceEventServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIEventSourceEventService`.
    fn coerce_from(v: &nsIEventSourceEventService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEventSourceEventServiceCoerce for nsIEventSourceEventService {
    #[inline]
    fn coerce_from(v: &nsIEventSourceEventService) -> &Self {
        v
    }
}

impl nsIEventSourceEventService {
    /// Cast this `nsIEventSourceEventService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEventSourceEventServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEventSourceEventService {
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
impl<T: nsISupportsCoerce> nsIEventSourceEventServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventSourceEventService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEventSourceEventService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEventSourceEventServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIEventSourceEventService, aInnerWindowID: u64, aListener: *const nsIEventSourceEventListener) -> ::nserror::nsresult,

    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIEventSourceEventService, aInnerWindowID: u64, aListener: *const nsIEventSourceEventListener) -> ::nserror::nsresult,

    /* [must_use] bool hasListenerFor (in unsigned long long aInnerWindowID); */
    pub HasListenerFor: unsafe extern "system" fn (this: *const nsIEventSourceEventService, aInnerWindowID: u64, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEventSourceEventService {


    /// `[must_use] void addListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener);`
    #[inline]
    pub unsafe fn AddListener(&self, aInnerWindowID: u64, aListener: *const nsIEventSourceEventListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, aInnerWindowID, aListener)
    }



    /// `[must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, aInnerWindowID: u64, aListener: *const nsIEventSourceEventListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, aInnerWindowID, aListener)
    }



    /// `[must_use] bool hasListenerFor (in unsigned long long aInnerWindowID);`
    #[inline]
    pub unsafe fn HasListenerFor(&self, aInnerWindowID: u64, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasListenerFor)(self, aInnerWindowID, _retval)
    }


}


