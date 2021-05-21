//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkLinkService.idl
//


/// `interface nsINetworkLinkService : nsISupports`
///

/// ```text
/// /**
///  * Network link status monitoring service.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetworkLinkService {
    vtable: *const nsINetworkLinkServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetworkLinkService.
unsafe impl XpCom for nsINetworkLinkService {
    const IID: nsIID = nsID(0x103e5293, 0x77b3, 0x4b70,
        [0xaf, 0x59, 0x6e, 0x9e, 0x4a, 0x1f, 0x99, 0x4a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetworkLinkService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetworkLinkService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetworkLinkServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINetworkLinkService`.
    fn coerce_from(v: &nsINetworkLinkService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetworkLinkServiceCoerce for nsINetworkLinkService {
    #[inline]
    fn coerce_from(v: &nsINetworkLinkService) -> &Self {
        v
    }
}

impl nsINetworkLinkService {
    /// Cast this `nsINetworkLinkService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetworkLinkServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetworkLinkService {
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
impl<T: nsISupportsCoerce> nsINetworkLinkServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkLinkService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetworkLinkService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetworkLinkServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isLinkUp; */
    pub GetIsLinkUp: unsafe extern "system" fn (this: *const nsINetworkLinkService, aIsLinkUp: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean linkStatusKnown; */
    pub GetLinkStatusKnown: unsafe extern "system" fn (this: *const nsINetworkLinkService, aLinkStatusKnown: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long linkType; */
    pub GetLinkType: unsafe extern "system" fn (this: *const nsINetworkLinkService, aLinkType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute ACString networkID; */
    pub GetNetworkID: unsafe extern "system" fn (this: *const nsINetworkLinkService, aNetworkID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> dnsSuffixList; */
    pub GetDnsSuffixList: unsafe extern "system" fn (this: *const nsINetworkLinkService, aDnsSuffixList: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute unsigned long platformDNSIndications; */
    pub GetPlatformDNSIndications: unsafe extern "system" fn (this: *const nsINetworkLinkService, aPlatformDNSIndications: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetworkLinkService {

    pub const LINK_TYPE_UNKNOWN: i64 = 0;


    pub const LINK_TYPE_ETHERNET: i64 = 1;


    pub const LINK_TYPE_USB: i64 = 2;


    pub const LINK_TYPE_WIFI: i64 = 3;


    pub const LINK_TYPE_WIMAX: i64 = 4;


    pub const LINK_TYPE_2G: i64 = 5;


    pub const LINK_TYPE_3G: i64 = 6;


    pub const LINK_TYPE_4G: i64 = 7;


    pub const LINK_TYPE_5G: i64 = 8;


    pub const NONE_DETECTED: i64 = 0;


    pub const VPN_DETECTED: i64 = 1;


    pub const PROXY_DETECTED: i64 = 2;


    pub const NRPT_DETECTED: i64 = 4;

    /// ```text
    /// /**
    ///    * This is set to true when the system is believed to have a usable
    ///    * network connection.
    ///    *
    ///    * The link is only up when network connections can be established. For
    ///    * example, the link is down during DHCP configuration (unless there
        ///    * is another usable interface already configured).
    ///    *
    ///    * If the link status is not currently known, we generally assume that
    ///    * it is up.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isLinkUp;`
    #[inline]
    pub unsafe fn GetIsLinkUp(&self, aIsLinkUp: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsLinkUp)(self, aIsLinkUp)
    }


    /// ```text
    /// /**
    ///    * This is set to true when we believe that isLinkUp is accurate.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean linkStatusKnown;`
    #[inline]
    pub unsafe fn GetLinkStatusKnown(&self, aLinkStatusKnown: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLinkStatusKnown)(self, aLinkStatusKnown)
    }


    /// ```text
    /// /**
    ///    * The type of network connection.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long linkType;`
    #[inline]
    pub unsafe fn GetLinkType(&self, aLinkType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetLinkType)(self, aLinkType)
    }


    /// ```text
    /// /**
    ///    * A string uniquely identifying the current active network interfaces.
    ///    * Empty when there are no active network interfaces.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString networkID;`
    #[inline]
    pub unsafe fn GetNetworkID(&self, aNetworkID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNetworkID)(self, aNetworkID)
    }


    /// ```text
    /// /**
    ///    * The list of DNS suffixes for the currently active network interfaces.
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> dnsSuffixList;`
    #[inline]
    pub unsafe fn GetDnsSuffixList(&self, aDnsSuffixList: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetDnsSuffixList)(self, aDnsSuffixList)
    }


    /// ```text
    /// /**
    ///    * A bitfield that encodes the platform attributes we detected which
    ///    * indicate that we should only use DNS, not TRR.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long platformDNSIndications;`
    #[inline]
    pub unsafe fn GetPlatformDNSIndications(&self, aPlatformDNSIndications: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPlatformDNSIndications)(self, aPlatformDNSIndications)
    }


}


