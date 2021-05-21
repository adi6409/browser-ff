//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDashboard.idl
//


/// `interface nsINetDashboardCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetDashboardCallback {
    vtable: *const nsINetDashboardCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetDashboardCallback.
unsafe impl XpCom for nsINetDashboardCallback {
    const IID: nsIID = nsID(0x19d7f24f, 0xa95a, 0x4fd9,
        [0x87, 0xe2, 0xd9, 0x6e, 0x9e, 0x4b, 0x1f, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetDashboardCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetDashboardCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetDashboardCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINetDashboardCallback`.
    fn coerce_from(v: &nsINetDashboardCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetDashboardCallbackCoerce for nsINetDashboardCallback {
    #[inline]
    fn coerce_from(v: &nsINetDashboardCallback) -> &Self {
        v
    }
}

impl nsINetDashboardCallback {
    /// Cast this `nsINetDashboardCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetDashboardCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetDashboardCallback {
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
impl<T: nsISupportsCoerce> nsINetDashboardCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetDashboardCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetDashboardCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetDashboardCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDashboardDataAvailable (in jsval data); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub OnDashboardDataAvailable: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetDashboardCallback {


    /// `void onDashboardDataAvailable (in jsval data);`
    const _OnDashboardDataAvailable: () = ();

}


/// `interface nsIDashboard : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDashboard {
    vtable: *const nsIDashboardVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDashboard.
unsafe impl XpCom for nsIDashboard {
    const IID: nsIID = nsID(0xc79eb3c6, 0x091a, 0x45a6,
        [0x85, 0x44, 0x5a, 0x8d, 0x1a, 0xb7, 0x95, 0x37]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDashboard {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDashboard.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDashboardCoerce {
    /// Cheaply cast a value of this type from a `nsIDashboard`.
    fn coerce_from(v: &nsIDashboard) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDashboardCoerce for nsIDashboard {
    #[inline]
    fn coerce_from(v: &nsIDashboard) -> &Self {
        v
    }
}

impl nsIDashboard {
    /// Cast this `nsIDashboard` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDashboardCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDashboard {
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
impl<T: nsISupportsCoerce> nsIDashboardCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDashboard) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDashboard
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDashboardVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void requestSockets (in nsINetDashboardCallback cb); */
    pub RequestSockets: unsafe extern "system" fn (this: *const nsIDashboard, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* void requestHttpConnections (in nsINetDashboardCallback cb); */
    pub RequestHttpConnections: unsafe extern "system" fn (this: *const nsIDashboard, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* void requestWebsocketConnections (in nsINetDashboardCallback cb); */
    pub RequestWebsocketConnections: unsafe extern "system" fn (this: *const nsIDashboard, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* void requestDNSInfo (in nsINetDashboardCallback cb); */
    pub RequestDNSInfo: unsafe extern "system" fn (this: *const nsIDashboard, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in nsINetDashboardCallback cb); */
    pub RequestConnection: unsafe extern "system" fn (this: *const nsIDashboard, aHost: *const ::nsstring::nsACString, aPort: u32, aProtocol: *const libc::c_char, aTimeout: u32, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* attribute boolean enableLogging; */
    pub GetEnableLogging: unsafe extern "system" fn (this: *const nsIDashboard, aEnableLogging: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean enableLogging; */
    pub SetEnableLogging: unsafe extern "system" fn (this: *const nsIDashboard, aEnableLogging: bool) -> ::nserror::nsresult,

    /* void requestDNSLookup (in ACString aHost, in nsINetDashboardCallback cb); */
    pub RequestDNSLookup: unsafe extern "system" fn (this: *const nsIDashboard, aHost: *const ::nsstring::nsACString, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* void requestDNSHTTPSRRLookup (in ACString aHost, in nsINetDashboardCallback cb); */
    pub RequestDNSHTTPSRRLookup: unsafe extern "system" fn (this: *const nsIDashboard, aHost: *const ::nsstring::nsACString, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* void requestRcwnStats (in nsINetDashboardCallback cb); */
    pub RequestRcwnStats: unsafe extern "system" fn (this: *const nsIDashboard, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult,

    /* AUTF8String getLogPath (); */
    pub GetLogPath: unsafe extern "system" fn (this: *const nsIDashboard, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDashboard {


    /// `void requestSockets (in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestSockets(&self, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestSockets)(self, cb)
    }



    /// `void requestHttpConnections (in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestHttpConnections(&self, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestHttpConnections)(self, cb)
    }



    /// `void requestWebsocketConnections (in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestWebsocketConnections(&self, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestWebsocketConnections)(self, cb)
    }



    /// `void requestDNSInfo (in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestDNSInfo(&self, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestDNSInfo)(self, cb)
    }



    /// `void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestConnection(&self, aHost: *const ::nsstring::nsACString, aPort: u32, aProtocol: *const libc::c_char, aTimeout: u32, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestConnection)(self, aHost, aPort, aProtocol, aTimeout, cb)
    }



    /// `attribute boolean enableLogging;`
    #[inline]
    pub unsafe fn GetEnableLogging(&self, aEnableLogging: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEnableLogging)(self, aEnableLogging)
    }



    /// `attribute boolean enableLogging;`
    #[inline]
    pub unsafe fn SetEnableLogging(&self, aEnableLogging: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEnableLogging)(self, aEnableLogging)
    }



    /// `void requestDNSLookup (in ACString aHost, in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestDNSLookup(&self, aHost: *const ::nsstring::nsACString, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestDNSLookup)(self, aHost, cb)
    }



    /// `void requestDNSHTTPSRRLookup (in ACString aHost, in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestDNSHTTPSRRLookup(&self, aHost: *const ::nsstring::nsACString, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestDNSHTTPSRRLookup)(self, aHost, cb)
    }


    /// ```text
    /// /**
    ///      * Asyncly returns stats regarding the "Race Cache With Network" feature.
    ///      */
    /// ```
    ///

    /// `void requestRcwnStats (in nsINetDashboardCallback cb);`
    #[inline]
    pub unsafe fn RequestRcwnStats(&self, cb: *const nsINetDashboardCallback) -> ::nserror::nsresult {
        ((*self.vtable).RequestRcwnStats)(self, cb)
    }



    /// `AUTF8String getLogPath ();`
    #[inline]
    pub unsafe fn GetLogPath(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLogPath)(self, _retval)
    }


}


