//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICaptivePortalService.idl
//


/// `interface nsICaptivePortalServiceCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICaptivePortalServiceCallback {
    vtable: *const nsICaptivePortalServiceCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICaptivePortalServiceCallback.
unsafe impl XpCom for nsICaptivePortalServiceCallback {
    const IID: nsIID = nsID(0xb5fd5629, 0xd04c, 0x4138,
        [0x95, 0x29, 0x93, 0x11, 0xf2, 0x91, 0xec, 0xd4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICaptivePortalServiceCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICaptivePortalServiceCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICaptivePortalServiceCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsICaptivePortalServiceCallback`.
    fn coerce_from(v: &nsICaptivePortalServiceCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICaptivePortalServiceCallbackCoerce for nsICaptivePortalServiceCallback {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalServiceCallback) -> &Self {
        v
    }
}

impl nsICaptivePortalServiceCallback {
    /// Cast this `nsICaptivePortalServiceCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICaptivePortalServiceCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICaptivePortalServiceCallback {
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
impl<T: nsISupportsCoerce> nsICaptivePortalServiceCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalServiceCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICaptivePortalServiceCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICaptivePortalServiceCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in bool success, in nsresult error); */
    pub Complete: unsafe extern "system" fn (this: *const nsICaptivePortalServiceCallback, success: bool, error: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICaptivePortalServiceCallback {

    /// ```text
    /// /**
    ///    * Invoke callbacks after captive portal detection finished.
    ///    */
    /// ```
    ///

    /// `void complete (in bool success, in nsresult error);`
    #[inline]
    pub unsafe fn Complete(&self, success: bool, error: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, success, error)
    }


}


/// `interface nsICaptivePortalService : nsISupports`
///

/// ```text
/// /**
///  * Service used for captive portal detection.
///  * The service is only active in the main process. It is also available in the
///  * content process, but only to mirror the captive portal state from the main
///  * process.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICaptivePortalService {
    vtable: *const nsICaptivePortalServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICaptivePortalService.
unsafe impl XpCom for nsICaptivePortalService {
    const IID: nsIID = nsID(0xbdbe0555, 0xfc3d, 0x4f7b,
        [0x92, 0x05, 0xc3, 0x09, 0xce, 0xb2, 0xd6, 0x41]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICaptivePortalService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICaptivePortalService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICaptivePortalServiceCoerce {
    /// Cheaply cast a value of this type from a `nsICaptivePortalService`.
    fn coerce_from(v: &nsICaptivePortalService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICaptivePortalServiceCoerce for nsICaptivePortalService {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalService) -> &Self {
        v
    }
}

impl nsICaptivePortalService {
    /// Cast this `nsICaptivePortalService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICaptivePortalServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICaptivePortalService {
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
impl<T: nsISupportsCoerce> nsICaptivePortalServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICaptivePortalService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICaptivePortalServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void recheckCaptivePortal (); */
    pub RecheckCaptivePortal: unsafe extern "system" fn (this: *const nsICaptivePortalService) -> ::nserror::nsresult,

    /* readonly attribute long state; */
    pub GetState: unsafe extern "system" fn (this: *const nsICaptivePortalService, aState: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long lastChecked; */
    pub GetLastChecked: unsafe extern "system" fn (this: *const nsICaptivePortalService, aLastChecked: *mut u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICaptivePortalService {

    pub const UNKNOWN: i64 = 0;


    pub const NOT_CAPTIVE: i64 = 1;


    pub const UNLOCKED_PORTAL: i64 = 2;


    pub const LOCKED_PORTAL: i64 = 3;

    /// ```text
    /// /**
    ///    * Called from XPCOM to trigger a captive portal recheck.
    ///    * A network request will only be performed if no other checks are currently
    ///    * ongoing.
    ///    * Will not do anything if called in the content process.
    ///    */
    /// ```
    ///

    /// `void recheckCaptivePortal ();`
    #[inline]
    pub unsafe fn RecheckCaptivePortal(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RecheckCaptivePortal)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns the state of the captive portal.
    ///    */
    /// ```
    ///

    /// `readonly attribute long state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * Returns the time difference between NOW and the last time a request was
    ///    * completed in milliseconds.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long lastChecked;`
    #[inline]
    pub unsafe fn GetLastChecked(&self, aLastChecked: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetLastChecked)(self, aLastChecked)
    }


}


