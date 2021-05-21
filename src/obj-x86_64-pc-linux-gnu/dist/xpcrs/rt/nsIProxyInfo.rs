//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxyInfo.idl
//


/// `interface nsIProxyInfo : nsISupports`
///

/// ```text
/// /**
///  * This interface identifies a proxy server.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProxyInfo {
    vtable: *const nsIProxyInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProxyInfo.
unsafe impl XpCom for nsIProxyInfo {
    const IID: nsIID = nsID(0x63fff172, 0x2564, 0x4138,
        [0x96, 0xc6, 0x3a, 0xe7, 0xd2, 0x45, 0xfb, 0xed]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProxyInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProxyInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProxyInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIProxyInfo`.
    fn coerce_from(v: &nsIProxyInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProxyInfoCoerce for nsIProxyInfo {
    #[inline]
    fn coerce_from(v: &nsIProxyInfo) -> &Self {
        v
    }
}

impl nsIProxyInfo {
    /// Cast this `nsIProxyInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProxyInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProxyInfo {
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
impl<T: nsISupportsCoerce> nsIProxyInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxyInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProxyInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProxyInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsIProxyInfo, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIProxyInfo, aPort: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute ACString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIProxyInfo, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long flags; */
    pub GetFlags: unsafe extern "system" fn (this: *const nsIProxyInfo, aFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long resolveFlags; */
    pub GetResolveFlags: unsafe extern "system" fn (this: *const nsIProxyInfo, aResolveFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long resolveFlags; */
    pub SetResolveFlags: unsafe extern "system" fn (this: *const nsIProxyInfo, aResolveFlags: u32) -> ::nserror::nsresult,

    /* readonly attribute ACString username; */
    pub GetUsername: unsafe extern "system" fn (this: *const nsIProxyInfo, aUsername: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString password; */
    pub GetPassword: unsafe extern "system" fn (this: *const nsIProxyInfo, aPassword: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long failoverTimeout; */
    pub GetFailoverTimeout: unsafe extern "system" fn (this: *const nsIProxyInfo, aFailoverTimeout: *mut u32) -> ::nserror::nsresult,

    /* attribute nsIProxyInfo failoverProxy; */
    pub GetFailoverProxy: unsafe extern "system" fn (this: *const nsIProxyInfo, aFailoverProxy: *mut *const nsIProxyInfo) -> ::nserror::nsresult,

    /* attribute nsIProxyInfo failoverProxy; */
    pub SetFailoverProxy: unsafe extern "system" fn (this: *const nsIProxyInfo, aFailoverProxy: *const nsIProxyInfo) -> ::nserror::nsresult,

    /* readonly attribute ACString proxyAuthorizationHeader; */
    pub GetProxyAuthorizationHeader: unsafe extern "system" fn (this: *const nsIProxyInfo, aProxyAuthorizationHeader: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString connectionIsolationKey; */
    pub GetConnectionIsolationKey: unsafe extern "system" fn (this: *const nsIProxyInfo, aConnectionIsolationKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProxyInfo {
    /// ```text
    /// /****************************************************************************
    ///    * The following "Proxy Flags" may be bit-wise combined to construct the
    ///    * flags attribute defined on this interface.  All unspecified bits are
    ///    * reserved for future use.
    ///    */
    /// /**
    ///    * This flag is set if the proxy is to perform name resolution itself.  If
    ///    * this is the case, the hostname is used in some fashion, and we shouldn't
    ///    * do any form of DNS lookup ourselves.
    ///    */
    /// ```
    ///

    pub const TRANSPARENT_PROXY_RESOLVES_HOST: i64 = 1;

    /// ```text
    /// /**
    ///    * This attribute specifies the hostname of the proxy server.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String host;`
    #[inline]
    pub unsafe fn GetHost(&self, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies the port number of the proxy server.
    ///    */
    /// ```
    ///

    /// `readonly attribute long port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies the type of the proxy server as an ASCII string.
    ///    *
    ///    * Some special values for this attribute include (but are not limited to)
    ///    * the following:
    ///    *   "http"     HTTP proxy (or SSL CONNECT for HTTPS)
    ///    *   "https"    HTTP proxying over TLS connection to proxy
    ///    *   "socks"    SOCKS v5 proxy
    ///    *   "socks4"   SOCKS v4 proxy
    ///    *   "direct"   no proxy
    ///    *   "unknown"  unknown proxy (see nsIProtocolProxyService::resolve)
    ///    *
    ///    * A future version of this interface may define additional types.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies flags that modify the proxy type.  The value of
    ///    * this attribute is the bit-wise combination of the Proxy Flags defined
    ///    * below.  Any undefined bits are reserved for future use.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long flags;`
    #[inline]
    pub unsafe fn GetFlags(&self, aFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFlags)(self, aFlags)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies flags that were used by nsIProxyProtocolService when
    ///    * creating this ProxyInfo element.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long resolveFlags;`
    #[inline]
    pub unsafe fn GetResolveFlags(&self, aResolveFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetResolveFlags)(self, aResolveFlags)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies flags that were used by nsIProxyProtocolService when
    ///    * creating this ProxyInfo element.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long resolveFlags;`
    #[inline]
    pub unsafe fn SetResolveFlags(&self, aResolveFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetResolveFlags)(self, aResolveFlags)
    }


    /// ```text
    /// /**
    ///    * Specifies a proxy username.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString username;`
    #[inline]
    pub unsafe fn GetUsername(&self, aUsername: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUsername)(self, aUsername)
    }


    /// ```text
    /// /**
    ///    * Specifies a proxy password.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString password;`
    #[inline]
    pub unsafe fn GetPassword(&self, aPassword: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPassword)(self, aPassword)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies the failover timeout in seconds for this proxy.
    ///    * If a nsIProxyInfo is reported as failed via nsIProtocolProxyService::
    ///    * getFailoverForProxy, then the failed proxy will not be used again for this
    ///    * many seconds.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long failoverTimeout;`
    #[inline]
    pub unsafe fn GetFailoverTimeout(&self, aFailoverTimeout: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFailoverTimeout)(self, aFailoverTimeout)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies the proxy to failover to when this proxy fails.
    ///    */
    /// ```
    ///

    /// `attribute nsIProxyInfo failoverProxy;`
    #[inline]
    pub unsafe fn GetFailoverProxy(&self, aFailoverProxy: *mut *const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetFailoverProxy)(self, aFailoverProxy)
    }


    /// ```text
    /// /**
    ///    * This attribute specifies the proxy to failover to when this proxy fails.
    ///    */
    /// ```
    ///

    /// `attribute nsIProxyInfo failoverProxy;`
    #[inline]
    pub unsafe fn SetFailoverProxy(&self, aFailoverProxy: *const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).SetFailoverProxy)(self, aFailoverProxy)
    }


    /// ```text
    /// /**
    ///    * Any non-empty value will be passed directly as Proxy-Authorization header
    ///    * value for the CONNECT request attempt.  However, this header set on the
    ///    * resource request itself takes precedence.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString proxyAuthorizationHeader;`
    #[inline]
    pub unsafe fn GetProxyAuthorizationHeader(&self, aProxyAuthorizationHeader: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyAuthorizationHeader)(self, aProxyAuthorizationHeader)
    }


    /// ```text
    /// /**
    ///    * An optional key used for additional isolation of this proxy connection.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString connectionIsolationKey;`
    #[inline]
    pub unsafe fn GetConnectionIsolationKey(&self, aConnectionIsolationKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectionIsolationKey)(self, aConnectionIsolationKey)
    }


}


