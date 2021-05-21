//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/captivedetect/nsICaptivePortalDetector.idl
//


/// `interface nsICaptivePortalCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICaptivePortalCallback {
    vtable: *const nsICaptivePortalCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICaptivePortalCallback.
unsafe impl XpCom for nsICaptivePortalCallback {
    const IID: nsIID = nsID(0x593fdeec, 0x6284, 0x4de8,
        [0xb4, 0x16, 0x8e, 0x63, 0xcb, 0xdc, 0x69, 0x5e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICaptivePortalCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICaptivePortalCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICaptivePortalCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsICaptivePortalCallback`.
    fn coerce_from(v: &nsICaptivePortalCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICaptivePortalCallbackCoerce for nsICaptivePortalCallback {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalCallback) -> &Self {
        v
    }
}

impl nsICaptivePortalCallback {
    /// Cast this `nsICaptivePortalCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICaptivePortalCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICaptivePortalCallback {
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
impl<T: nsISupportsCoerce> nsICaptivePortalCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICaptivePortalCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICaptivePortalCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void prepare (); */
    pub Prepare: unsafe extern "system" fn (this: *const nsICaptivePortalCallback) -> ::nserror::nsresult,

    /* void complete (in bool success); */
    pub Complete: unsafe extern "system" fn (this: *const nsICaptivePortalCallback, success: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICaptivePortalCallback {

    /// ```text
    /// /**
    ///    * Preparation for network interface before captive portal detection started.
    ///    */
    /// ```
    ///

    /// `void prepare ();`
    #[inline]
    pub unsafe fn Prepare(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Prepare)(self, )
    }


    /// ```text
    /// /**
    ///    * Invoke callbacks after captive portal detection finished.
    ///    */
    /// ```
    ///

    /// `void complete (in bool success);`
    #[inline]
    pub unsafe fn Complete(&self, success: bool) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, success)
    }


}


/// `interface nsICaptivePortalDetector : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICaptivePortalDetector {
    vtable: *const nsICaptivePortalDetectorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICaptivePortalDetector.
unsafe impl XpCom for nsICaptivePortalDetector {
    const IID: nsIID = nsID(0x2f827c5a, 0xf551, 0x477f,
        [0xaf, 0x09, 0x71, 0xad, 0xbf, 0xbd, 0x85, 0x4a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICaptivePortalDetector {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICaptivePortalDetector.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICaptivePortalDetectorCoerce {
    /// Cheaply cast a value of this type from a `nsICaptivePortalDetector`.
    fn coerce_from(v: &nsICaptivePortalDetector) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICaptivePortalDetectorCoerce for nsICaptivePortalDetector {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalDetector) -> &Self {
        v
    }
}

impl nsICaptivePortalDetector {
    /// Cast this `nsICaptivePortalDetector` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICaptivePortalDetectorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICaptivePortalDetector {
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
impl<T: nsISupportsCoerce> nsICaptivePortalDetectorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalDetector) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICaptivePortalDetector
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICaptivePortalDetectorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void checkCaptivePortal (in AString ifname, in nsICaptivePortalCallback callback); */
    pub CheckCaptivePortal: unsafe extern "system" fn (this: *const nsICaptivePortalDetector, ifname: *const ::nsstring::nsAString, callback: *const nsICaptivePortalCallback) -> ::nserror::nsresult,

    /* void abort (in AString ifname); */
    pub Abort: unsafe extern "system" fn (this: *const nsICaptivePortalDetector, ifname: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void cancelLogin (in AString eventId); */
    pub CancelLogin: unsafe extern "system" fn (this: *const nsICaptivePortalDetector, eventId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void finishPreparation (in AString ifname); */
    pub FinishPreparation: unsafe extern "system" fn (this: *const nsICaptivePortalDetector, ifname: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICaptivePortalDetector {

    /// ```text
    /// /**
    ///    * Perform captive portal detection on specific network interface.
    ///    * @param ifname The name of network interface, exception will be thrwon
    ///    *               if the same interface has unfinished request.
    ///    * @param callback Callbacks when detection procedure starts and finishes.
    ///    */
    /// ```
    ///

    /// `void checkCaptivePortal (in AString ifname, in nsICaptivePortalCallback callback);`
    #[inline]
    pub unsafe fn CheckCaptivePortal(&self, ifname: *const ::nsstring::nsAString, callback: *const nsICaptivePortalCallback) -> ::nserror::nsresult {
        ((*self.vtable).CheckCaptivePortal)(self, ifname, callback)
    }


    /// ```text
    /// /**
    ///    * Abort captive portal detection for specific network interface
    ///    * due to system failure, callback will not be invoked.
    ///    * @param ifname The name of network interface.
    ///    */
    /// ```
    ///

    /// `void abort (in AString ifname);`
    #[inline]
    pub unsafe fn Abort(&self, ifname: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Abort)(self, ifname)
    }


    /// ```text
    /// /**
    ///    * Cancel captive portal login procedure by user, callback will be invoked.
    ///    * @param eventId Login event id provided in |captive-portal-login| event.
    ///    */
    /// ```
    ///

    /// `void cancelLogin (in AString eventId);`
    #[inline]
    pub unsafe fn CancelLogin(&self, eventId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).CancelLogin)(self, eventId)
    }


    /// ```text
    /// /**
    ///    * Notify prepare phase is finished, routing and dns must be ready for sending
    ///    * out XMLHttpRequest. this is callback for CaptivePortalDetector API user.
    ///    * @param ifname The name of network interface, must be unique.
    ///    */
    /// ```
    ///

    /// `void finishPreparation (in AString ifname);`
    #[inline]
    pub unsafe fn FinishPreparation(&self, ifname: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FinishPreparation)(self, ifname)
    }


}


