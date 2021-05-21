//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsPISocketTransportService.idl
//


/// `interface nsPISocketTransportService : nsIRoutedSocketTransportService`
///

/// ```text
/// /**
///  * This is a private interface used by the internals of the networking library.
///  * It will never be frozen.  Do not use it in external code.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsPISocketTransportService {
    vtable: *const nsPISocketTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsPISocketTransportService.
unsafe impl XpCom for nsPISocketTransportService {
    const IID: nsIID = nsID(0x18f73bf1, 0xb35b, 0x4b7b,
        [0xaa, 0x9a, 0x11, 0xbc, 0xbd, 0xbc, 0x38, 0x9c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsPISocketTransportService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsPISocketTransportService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsPISocketTransportServiceCoerce {
    /// Cheaply cast a value of this type from a `nsPISocketTransportService`.
    fn coerce_from(v: &nsPISocketTransportService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsPISocketTransportServiceCoerce for nsPISocketTransportService {
    #[inline]
    fn coerce_from(v: &nsPISocketTransportService) -> &Self {
        v
    }
}

impl nsPISocketTransportService {
    /// Cast this `nsPISocketTransportService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsPISocketTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsPISocketTransportService {
    type Target = nsIRoutedSocketTransportService;
    #[inline]
    fn deref(&self) -> &nsIRoutedSocketTransportService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRoutedSocketTransportServiceCoerce> nsPISocketTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPISocketTransportService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsPISocketTransportService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsPISocketTransportServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRoutedSocketTransportServiceVTable,

    /* void init (); */
    pub Init: unsafe extern "system" fn (this: *const nsPISocketTransportService) -> ::nserror::nsresult,

    /* void shutdown (in bool aXpcomShutdown); */
    pub Shutdown: unsafe extern "system" fn (this: *const nsPISocketTransportService, aXpcomShutdown: bool) -> ::nserror::nsresult,

    /* readonly attribute long sendBufferSize; */
    pub GetSendBufferSize: unsafe extern "system" fn (this: *const nsPISocketTransportService, aSendBufferSize: *mut i32) -> ::nserror::nsresult,

    /* attribute boolean offline; */
    pub GetOffline: unsafe extern "system" fn (this: *const nsPISocketTransportService, aOffline: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean offline; */
    pub SetOffline: unsafe extern "system" fn (this: *const nsPISocketTransportService, aOffline: bool) -> ::nserror::nsresult,

    /* readonly attribute long keepaliveIdleTime; */
    pub GetKeepaliveIdleTime: unsafe extern "system" fn (this: *const nsPISocketTransportService, aKeepaliveIdleTime: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long keepaliveRetryInterval; */
    pub GetKeepaliveRetryInterval: unsafe extern "system" fn (this: *const nsPISocketTransportService, aKeepaliveRetryInterval: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long keepaliveProbeCount; */
    pub GetKeepaliveProbeCount: unsafe extern "system" fn (this: *const nsPISocketTransportService, aKeepaliveProbeCount: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsPISocketTransportService {

    /// ```text
    /// /**
    ///    * init/shutdown routines.
    ///    */
    /// ```
    ///

    /// `void init ();`
    #[inline]
    pub unsafe fn Init(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, )
    }



    /// `void shutdown (in bool aXpcomShutdown);`
    #[inline]
    pub unsafe fn Shutdown(&self, aXpcomShutdown: bool) -> ::nserror::nsresult {
        ((*self.vtable).Shutdown)(self, aXpcomShutdown)
    }


    /// ```text
    /// /**
    ///    * controls the TCP sender window clamp
    ///    */
    /// ```
    ///

    /// `readonly attribute long sendBufferSize;`
    #[inline]
    pub unsafe fn GetSendBufferSize(&self, aSendBufferSize: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSendBufferSize)(self, aSendBufferSize)
    }


    /// ```text
    /// /**
    ///    * Controls whether the socket transport service is offline.
    ///    * Setting it offline will cause non-local socket detachment.
    ///    */
    /// ```
    ///

    /// `attribute boolean offline;`
    #[inline]
    pub unsafe fn GetOffline(&self, aOffline: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOffline)(self, aOffline)
    }


    /// ```text
    /// /**
    ///    * Controls whether the socket transport service is offline.
    ///    * Setting it offline will cause non-local socket detachment.
    ///    */
    /// ```
    ///

    /// `attribute boolean offline;`
    #[inline]
    pub unsafe fn SetOffline(&self, aOffline: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetOffline)(self, aOffline)
    }


    /// ```text
    /// /**
    ///    * Controls the default timeout (in seconds) for sending keepalive probes.
    ///    */
    /// ```
    ///

    /// `readonly attribute long keepaliveIdleTime;`
    #[inline]
    pub unsafe fn GetKeepaliveIdleTime(&self, aKeepaliveIdleTime: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetKeepaliveIdleTime)(self, aKeepaliveIdleTime)
    }


    /// ```text
    /// /**
    ///    * Controls the default interval (in seconds) between retrying keepalive probes.
    ///    */
    /// ```
    ///

    /// `readonly attribute long keepaliveRetryInterval;`
    #[inline]
    pub unsafe fn GetKeepaliveRetryInterval(&self, aKeepaliveRetryInterval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetKeepaliveRetryInterval)(self, aKeepaliveRetryInterval)
    }


    /// ```text
    /// /**
    ///    * Controls the default retransmission count for keepalive probes.
    ///    */
    /// ```
    ///

    /// `readonly attribute long keepaliveProbeCount;`
    #[inline]
    pub unsafe fn GetKeepaliveProbeCount(&self, aKeepaliveProbeCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetKeepaliveProbeCount)(self, aKeepaliveProbeCount)
    }


}


