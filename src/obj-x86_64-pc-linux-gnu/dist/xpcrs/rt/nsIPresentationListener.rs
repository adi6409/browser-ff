//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationListener.idl
//


/// `interface nsIPresentationAvailabilityListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationAvailabilityListener {
    vtable: *const nsIPresentationAvailabilityListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationAvailabilityListener.
unsafe impl XpCom for nsIPresentationAvailabilityListener {
    const IID: nsIID = nsID(0x0105f837, 0x4279, 0x4715,
        [0x9d, 0x5b, 0x2d, 0xc3, 0xf8, 0xb6, 0x53, 0x53]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationAvailabilityListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationAvailabilityListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationAvailabilityListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationAvailabilityListener`.
    fn coerce_from(v: &nsIPresentationAvailabilityListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationAvailabilityListenerCoerce for nsIPresentationAvailabilityListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationAvailabilityListener) -> &Self {
        v
    }
}

impl nsIPresentationAvailabilityListener {
    /// Cast this `nsIPresentationAvailabilityListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationAvailabilityListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationAvailabilityListener {
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
impl<T: nsISupportsCoerce> nsIPresentationAvailabilityListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationAvailabilityListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationAvailabilityListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationAvailabilityListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void notifyAvailableChange (in URLArrayRef urls, in bool available); */
    /// Unable to generate binding because `native type const nsTArray<nsString> unsupported`
    pub NotifyAvailableChange: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationAvailabilityListener {


    /// `[noscript] void notifyAvailableChange (in URLArrayRef urls, in bool available);`
    const _NotifyAvailableChange: () = ();

}


/// `interface nsIPresentationSessionListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationSessionListener {
    vtable: *const nsIPresentationSessionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationSessionListener.
unsafe impl XpCom for nsIPresentationSessionListener {
    const IID: nsIID = nsID(0x7dd48df8, 0x8f8c, 0x48c7,
        [0xac, 0x37, 0x7b, 0x9f, 0xd1, 0xac, 0xf2, 0xf8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationSessionListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationSessionListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationSessionListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationSessionListener`.
    fn coerce_from(v: &nsIPresentationSessionListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationSessionListenerCoerce for nsIPresentationSessionListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionListener) -> &Self {
        v
    }
}

impl nsIPresentationSessionListener {
    /// Cast this `nsIPresentationSessionListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationSessionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationSessionListener {
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
impl<T: nsISupportsCoerce> nsIPresentationSessionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationSessionListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationSessionListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notifyStateChange (in AString sessionId, in unsigned short state, in nsresult reason); */
    pub NotifyStateChange: unsafe extern "system" fn (this: *const nsIPresentationSessionListener, sessionId: *const ::nsstring::nsAString, state: u16, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void notifyMessage (in AString sessionId, in ACString data, in boolean isBinary); */
    pub NotifyMessage: unsafe extern "system" fn (this: *const nsIPresentationSessionListener, sessionId: *const ::nsstring::nsAString, data: *const ::nsstring::nsACString, isBinary: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationSessionListener {

    pub const STATE_CONNECTING: i64 = 0;


    pub const STATE_CONNECTED: i64 = 1;


    pub const STATE_CLOSED: i64 = 2;


    pub const STATE_TERMINATED: i64 = 3;


    /// `void notifyStateChange (in AString sessionId, in unsigned short state, in nsresult reason);`
    #[inline]
    pub unsafe fn NotifyStateChange(&self, sessionId: *const ::nsstring::nsAString, state: u16, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).NotifyStateChange)(self, sessionId, state, reason)
    }



    /// `void notifyMessage (in AString sessionId, in ACString data, in boolean isBinary);`
    #[inline]
    pub unsafe fn NotifyMessage(&self, sessionId: *const ::nsstring::nsAString, data: *const ::nsstring::nsACString, isBinary: bool) -> ::nserror::nsresult {
        ((*self.vtable).NotifyMessage)(self, sessionId, data, isBinary)
    }


}


/// `interface nsIPresentationRespondingListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationRespondingListener {
    vtable: *const nsIPresentationRespondingListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationRespondingListener.
unsafe impl XpCom for nsIPresentationRespondingListener {
    const IID: nsIID = nsID(0x27f101d7, 0x9ed1, 0x429e,
        [0xb4, 0xf8, 0x43, 0xb0, 0x0e, 0x8e, 0x11, 0x1c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationRespondingListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationRespondingListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationRespondingListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationRespondingListener`.
    fn coerce_from(v: &nsIPresentationRespondingListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationRespondingListenerCoerce for nsIPresentationRespondingListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationRespondingListener) -> &Self {
        v
    }
}

impl nsIPresentationRespondingListener {
    /// Cast this `nsIPresentationRespondingListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationRespondingListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationRespondingListener {
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
impl<T: nsISupportsCoerce> nsIPresentationRespondingListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationRespondingListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationRespondingListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationRespondingListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notifySessionConnect (in unsigned long long windowId, in AString sessionId); */
    pub NotifySessionConnect: unsafe extern "system" fn (this: *const nsIPresentationRespondingListener, windowId: u64, sessionId: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationRespondingListener {


    /// `void notifySessionConnect (in unsigned long long windowId, in AString sessionId);`
    #[inline]
    pub unsafe fn NotifySessionConnect(&self, windowId: u64, sessionId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).NotifySessionConnect)(self, windowId, sessionId)
    }


}


