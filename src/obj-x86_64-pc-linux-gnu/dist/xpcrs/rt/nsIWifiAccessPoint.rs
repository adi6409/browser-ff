//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiAccessPoint.idl
//


/// `interface nsIWifiAccessPoint : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWifiAccessPoint {
    vtable: *const nsIWifiAccessPointVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWifiAccessPoint.
unsafe impl XpCom for nsIWifiAccessPoint {
    const IID: nsIID = nsID(0xe28e614f, 0x8f86, 0x44ff,
        [0xbc, 0xf5, 0x5f, 0x18, 0x22, 0x58, 0x34, 0xa0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWifiAccessPoint {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWifiAccessPoint.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWifiAccessPointCoerce {
    /// Cheaply cast a value of this type from a `nsIWifiAccessPoint`.
    fn coerce_from(v: &nsIWifiAccessPoint) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWifiAccessPointCoerce for nsIWifiAccessPoint {
    #[inline]
    fn coerce_from(v: &nsIWifiAccessPoint) -> &Self {
        v
    }
}

impl nsIWifiAccessPoint {
    /// Cast this `nsIWifiAccessPoint` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWifiAccessPointCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWifiAccessPoint {
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
impl<T: nsISupportsCoerce> nsIWifiAccessPointCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWifiAccessPoint) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWifiAccessPoint
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWifiAccessPointVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString mac; */
    pub GetMac: unsafe extern "system" fn (this: *const nsIWifiAccessPoint, aMac: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AString ssid; */
    pub GetSsid: unsafe extern "system" fn (this: *const nsIWifiAccessPoint, aSsid: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute ACString rawSSID; */
    pub GetRawSSID: unsafe extern "system" fn (this: *const nsIWifiAccessPoint, aRawSSID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long signal; */
    pub GetSignal: unsafe extern "system" fn (this: *const nsIWifiAccessPoint, aSignal: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWifiAccessPoint {


    /// `readonly attribute ACString mac;`
    #[inline]
    pub unsafe fn GetMac(&self, aMac: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMac)(self, aMac)
    }



    /// `readonly attribute AString ssid;`
    #[inline]
    pub unsafe fn GetSsid(&self, aSsid: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSsid)(self, aSsid)
    }



    /// `readonly attribute ACString rawSSID;`
    #[inline]
    pub unsafe fn GetRawSSID(&self, aRawSSID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRawSSID)(self, aRawSSID)
    }



    /// `readonly attribute long signal;`
    #[inline]
    pub unsafe fn GetSignal(&self, aSignal: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSignal)(self, aSignal)
    }


}


