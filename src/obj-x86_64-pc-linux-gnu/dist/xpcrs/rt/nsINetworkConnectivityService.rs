//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkConnectivityService.idl
//


/// `interface nsINetworkConnectivityService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetworkConnectivityService {
    vtable: *const nsINetworkConnectivityServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetworkConnectivityService.
unsafe impl XpCom for nsINetworkConnectivityService {
    const IID: nsIID = nsID(0x2693457e, 0x3ba5, 0x4455,
        [0x99, 0x1f, 0x53, 0x50, 0x94, 0x6a, 0xdb, 0x12]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetworkConnectivityService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetworkConnectivityService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetworkConnectivityServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINetworkConnectivityService`.
    fn coerce_from(v: &nsINetworkConnectivityService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetworkConnectivityServiceCoerce for nsINetworkConnectivityService {
    #[inline]
    fn coerce_from(v: &nsINetworkConnectivityService) -> &Self {
        v
    }
}

impl nsINetworkConnectivityService {
    /// Cast this `nsINetworkConnectivityService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetworkConnectivityServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetworkConnectivityService {
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
impl<T: nsISupportsCoerce> nsINetworkConnectivityServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkConnectivityService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetworkConnectivityService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetworkConnectivityServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv4; */
    pub GetDNSv4: unsafe extern "system" fn (this: *const nsINetworkConnectivityService, aDNSv4: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv6; */
    pub GetDNSv6: unsafe extern "system" fn (this: *const nsINetworkConnectivityService, aDNSv6: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv4; */
    pub GetIPv4: unsafe extern "system" fn (this: *const nsINetworkConnectivityService, aIPv4: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv6; */
    pub GetIPv6: unsafe extern "system" fn (this: *const nsINetworkConnectivityService, aIPv6: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState NAT64; */
    pub GetNAT64: unsafe extern "system" fn (this: *const nsINetworkConnectivityService, aNAT64: *mut u32) -> ::nserror::nsresult,

    /* void recheckDNS (); */
    pub RecheckDNS: unsafe extern "system" fn (this: *const nsINetworkConnectivityService) -> ::nserror::nsresult,

    /* void recheckIPConnectivity (); */
    pub RecheckIPConnectivity: unsafe extern "system" fn (this: *const nsINetworkConnectivityService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetworkConnectivityService {


    /// `[infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv4;`
    #[inline]
    pub unsafe fn GetDNSv4(&self, aDNSv4: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDNSv4)(self, aDNSv4)
    }



    /// `[infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState DNSv6;`
    #[inline]
    pub unsafe fn GetDNSv6(&self, aDNSv6: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDNSv6)(self, aDNSv6)
    }



    /// `[infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv4;`
    #[inline]
    pub unsafe fn GetIPv4(&self, aIPv4: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetIPv4)(self, aIPv4)
    }



    /// `[infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState IPv6;`
    #[inline]
    pub unsafe fn GetIPv6(&self, aIPv6: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetIPv6)(self, aIPv6)
    }



    /// `[infallible] readonly attribute nsINetworkConnectivityService_ConnectivityState NAT64;`
    #[inline]
    pub unsafe fn GetNAT64(&self, aNAT64: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNAT64)(self, aNAT64)
    }



    /// `void recheckDNS ();`
    #[inline]
    pub unsafe fn RecheckDNS(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RecheckDNS)(self, )
    }



    /// `void recheckIPConnectivity ();`
    #[inline]
    pub unsafe fn RecheckIPConnectivity(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RecheckIPConnectivity)(self, )
    }


}


