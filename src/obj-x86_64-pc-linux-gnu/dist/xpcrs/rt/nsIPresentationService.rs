//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationService.idl
//


/// `interface nsIPresentationServiceCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationServiceCallback {
    vtable: *const nsIPresentationServiceCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationServiceCallback.
unsafe impl XpCom for nsIPresentationServiceCallback {
    const IID: nsIID = nsID(0x12073206, 0x0065, 0x4b10,
        [0x94, 0x88, 0xa6, 0xeb, 0x9b, 0x23, 0xe6, 0x5b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationServiceCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationServiceCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationServiceCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationServiceCallback`.
    fn coerce_from(v: &nsIPresentationServiceCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationServiceCallbackCoerce for nsIPresentationServiceCallback {
    #[inline]
    fn coerce_from(v: &nsIPresentationServiceCallback) -> &Self {
        v
    }
}

impl nsIPresentationServiceCallback {
    /// Cast this `nsIPresentationServiceCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationServiceCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationServiceCallback {
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
impl<T: nsISupportsCoerce> nsIPresentationServiceCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationServiceCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationServiceCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationServiceCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notifySuccess (in AString url); */
    pub NotifySuccess: unsafe extern "system" fn (this: *const nsIPresentationServiceCallback, url: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void notifyError (in nsresult error); */
    pub NotifyError: unsafe extern "system" fn (this: *const nsIPresentationServiceCallback, error: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationServiceCallback {


    /// `void notifySuccess (in AString url);`
    #[inline]
    pub unsafe fn NotifySuccess(&self, url: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).NotifySuccess)(self, url)
    }



    /// `void notifyError (in nsresult error);`
    #[inline]
    pub unsafe fn NotifyError(&self, error: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).NotifyError)(self, error)
    }


}


/// `interface nsIPresentationService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationService {
    vtable: *const nsIPresentationServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationService.
unsafe impl XpCom for nsIPresentationService {
    const IID: nsIID = nsID(0xde42b741, 0x5619, 0x4650,
        [0xb9, 0x61, 0xc2, 0xce, 0xbb, 0x57, 0x2c, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationService`.
    fn coerce_from(v: &nsIPresentationService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationServiceCoerce for nsIPresentationService {
    #[inline]
    fn coerce_from(v: &nsIPresentationService) -> &Self {
        v
    }
}

impl nsIPresentationService {
    /// Cast this `nsIPresentationService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationService {
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
impl<T: nsISupportsCoerce> nsIPresentationServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void startSession (in URLArrayRef urls, in AString sessionId, in AString origin, in AString deviceId, in unsigned long long windowId, in EventTarget eventTarget, in nsIPrincipal principal, in nsIPresentationServiceCallback callback, in nsIPresentationTransportBuilderConstructor constructor); */
    /// Unable to generate binding because `native type const nsTArray<nsString> unsupported`
    pub StartSession: *const ::libc::c_void,

    /* void sendSessionMessage (in AString sessionId, in uint8_t role, in AString data); */
    pub SendSessionMessage: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, data: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void sendSessionBinaryMsg (in AString sessionId, in uint8_t role, in ACString data); */
    pub SendSessionBinaryMsg: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void sendSessionBlob (in AString sessionId, in uint8_t role, in Blob blob); */
    pub SendSessionBlob: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, blob: *const libc::c_void) -> ::nserror::nsresult,

    /* void closeSession (in AString sessionId, in uint8_t role, in uint8_t closedReason); */
    pub CloseSession: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, closedReason: uint8_t) -> ::nserror::nsresult,

    /* void terminateSession (in AString sessionId, in uint8_t role); */
    pub TerminateSession: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult,

    /* [noscript] void reconnectSession (in URLArrayRef urls, in AString sessionId, in uint8_t role, in nsIPresentationServiceCallback callback); */
    /// Unable to generate binding because `native type const nsTArray<nsString> unsupported`
    pub ReconnectSession: *const ::libc::c_void,

    /* [noscript] void registerAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */
    /// Unable to generate binding because `native type const nsTArray<nsString> unsupported`
    pub RegisterAvailabilityListener: *const ::libc::c_void,

    /* [noscript] void unregisterAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */
    /// Unable to generate binding because `native type const nsTArray<nsString> unsupported`
    pub UnregisterAvailabilityListener: *const ::libc::c_void,

    /* void registerSessionListener (in AString sessionId, in uint8_t role, in nsIPresentationSessionListener listener); */
    pub RegisterSessionListener: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, listener: *const nsIPresentationSessionListener) -> ::nserror::nsresult,

    /* void unregisterSessionListener (in AString sessionId, in uint8_t role); */
    pub UnregisterSessionListener: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult,

    /* void registerRespondingListener (in unsigned long long windowId, in nsIPresentationRespondingListener listener); */
    pub RegisterRespondingListener: unsafe extern "system" fn (this: *const nsIPresentationService, windowId: u64, listener: *const nsIPresentationRespondingListener) -> ::nserror::nsresult,

    /* void unregisterRespondingListener (in unsigned long long windowId); */
    pub UnregisterRespondingListener: unsafe extern "system" fn (this: *const nsIPresentationService, windowId: u64) -> ::nserror::nsresult,

    /* void notifyReceiverReady (in AString sessionId, in unsigned long long windowId, in boolean isLoading, in nsIPresentationTransportBuilderConstructor constructor); */
    pub NotifyReceiverReady: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, windowId: u64, isLoading: bool, constructor: *const nsIPresentationTransportBuilderConstructor) -> ::nserror::nsresult,

    /* void NotifyTransportClosed (in AString sessionId, in uint8_t role, in nsresult reason); */
    pub NotifyTransportClosed: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void untrackSessionInfo (in AString sessionId, in uint8_t role); */
    pub UntrackSessionInfo: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult,

    /* unsigned long long getWindowIdBySessionId (in AString sessionId, in uint8_t role); */
    pub GetWindowIdBySessionId: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, _retval: *mut u64) -> ::nserror::nsresult,

    /* void updateWindowIdBySessionId (in AString sessionId, in uint8_t role, in unsigned long long windowId); */
    pub UpdateWindowIdBySessionId: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t, windowId: u64) -> ::nserror::nsresult,

    /* void buildTransport (in AString sessionId, in uint8_t role); */
    pub BuildTransport: unsafe extern "system" fn (this: *const nsIPresentationService, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationService {

    pub const ROLE_CONTROLLER: i64 = 1;


    pub const ROLE_RECEIVER: i64 = 2;


    pub const CLOSED_REASON_ERROR: i64 = 1;


    pub const CLOSED_REASON_CLOSED: i64 = 2;


    pub const CLOSED_REASON_WENTAWAY: i64 = 3;


    /// `[noscript] void startSession (in URLArrayRef urls, in AString sessionId, in AString origin, in AString deviceId, in unsigned long long windowId, in EventTarget eventTarget, in nsIPrincipal principal, in nsIPresentationServiceCallback callback, in nsIPresentationTransportBuilderConstructor constructor);`
    const _StartSession: () = ();


    /// `void sendSessionMessage (in AString sessionId, in uint8_t role, in AString data);`
    #[inline]
    pub unsafe fn SendSessionMessage(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, data: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SendSessionMessage)(self, sessionId, role, data)
    }



    /// `void sendSessionBinaryMsg (in AString sessionId, in uint8_t role, in ACString data);`
    #[inline]
    pub unsafe fn SendSessionBinaryMsg(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendSessionBinaryMsg)(self, sessionId, role, data)
    }



    /// `void sendSessionBlob (in AString sessionId, in uint8_t role, in Blob blob);`
    #[inline]
    pub unsafe fn SendSessionBlob(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, blob: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SendSessionBlob)(self, sessionId, role, blob)
    }



    /// `void closeSession (in AString sessionId, in uint8_t role, in uint8_t closedReason);`
    #[inline]
    pub unsafe fn CloseSession(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, closedReason: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).CloseSession)(self, sessionId, role, closedReason)
    }



    /// `void terminateSession (in AString sessionId, in uint8_t role);`
    #[inline]
    pub unsafe fn TerminateSession(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).TerminateSession)(self, sessionId, role)
    }



    /// `[noscript] void reconnectSession (in URLArrayRef urls, in AString sessionId, in uint8_t role, in nsIPresentationServiceCallback callback);`
    const _ReconnectSession: () = ();


    /// `[noscript] void registerAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener);`
    const _RegisterAvailabilityListener: () = ();


    /// `[noscript] void unregisterAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener);`
    const _UnregisterAvailabilityListener: () = ();


    /// `void registerSessionListener (in AString sessionId, in uint8_t role, in nsIPresentationSessionListener listener);`
    #[inline]
    pub unsafe fn RegisterSessionListener(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, listener: *const nsIPresentationSessionListener) -> ::nserror::nsresult {
        ((*self.vtable).RegisterSessionListener)(self, sessionId, role, listener)
    }



    /// `void unregisterSessionListener (in AString sessionId, in uint8_t role);`
    #[inline]
    pub unsafe fn UnregisterSessionListener(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterSessionListener)(self, sessionId, role)
    }



    /// `void registerRespondingListener (in unsigned long long windowId, in nsIPresentationRespondingListener listener);`
    #[inline]
    pub unsafe fn RegisterRespondingListener(&self, windowId: u64, listener: *const nsIPresentationRespondingListener) -> ::nserror::nsresult {
        ((*self.vtable).RegisterRespondingListener)(self, windowId, listener)
    }



    /// `void unregisterRespondingListener (in unsigned long long windowId);`
    #[inline]
    pub unsafe fn UnregisterRespondingListener(&self, windowId: u64) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterRespondingListener)(self, windowId)
    }



    /// `void notifyReceiverReady (in AString sessionId, in unsigned long long windowId, in boolean isLoading, in nsIPresentationTransportBuilderConstructor constructor);`
    #[inline]
    pub unsafe fn NotifyReceiverReady(&self, sessionId: *const ::nsstring::nsAString, windowId: u64, isLoading: bool, constructor: *const nsIPresentationTransportBuilderConstructor) -> ::nserror::nsresult {
        ((*self.vtable).NotifyReceiverReady)(self, sessionId, windowId, isLoading, constructor)
    }



    /// `void NotifyTransportClosed (in AString sessionId, in uint8_t role, in nsresult reason);`
    #[inline]
    pub unsafe fn NotifyTransportClosed(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).NotifyTransportClosed)(self, sessionId, role, reason)
    }



    /// `void untrackSessionInfo (in AString sessionId, in uint8_t role);`
    #[inline]
    pub unsafe fn UntrackSessionInfo(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).UntrackSessionInfo)(self, sessionId, role)
    }



    /// `unsigned long long getWindowIdBySessionId (in AString sessionId, in uint8_t role);`
    #[inline]
    pub unsafe fn GetWindowIdBySessionId(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, _retval: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowIdBySessionId)(self, sessionId, role, _retval)
    }



    /// `void updateWindowIdBySessionId (in AString sessionId, in uint8_t role, in unsigned long long windowId);`
    #[inline]
    pub unsafe fn UpdateWindowIdBySessionId(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t, windowId: u64) -> ::nserror::nsresult {
        ((*self.vtable).UpdateWindowIdBySessionId)(self, sessionId, role, windowId)
    }



    /// `void buildTransport (in AString sessionId, in uint8_t role);`
    #[inline]
    pub unsafe fn BuildTransport(&self, sessionId: *const ::nsstring::nsAString, role: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).BuildTransport)(self, sessionId, role)
    }


}


