//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxiedChannel.idl
//


/// `interface nsIProxiedChannel : nsISupports`
///

/// ```text
/// /**
///  * An interface for accessing the proxy info that a channel was
///  * constructed with.
///  *
///  * @see nsIProxiedProtocolHandler
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProxiedChannel {
    vtable: *const nsIProxiedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProxiedChannel.
unsafe impl XpCom for nsIProxiedChannel {
    const IID: nsIID = nsID(0x6238f134, 0x8c3f, 0x4354,
        [0x95, 0x8f, 0xdf, 0xd9, 0xd5, 0x4a, 0x44, 0x46]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProxiedChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProxiedChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProxiedChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIProxiedChannel`.
    fn coerce_from(v: &nsIProxiedChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProxiedChannelCoerce for nsIProxiedChannel {
    #[inline]
    fn coerce_from(v: &nsIProxiedChannel) -> &Self {
        v
    }
}

impl nsIProxiedChannel {
    /// Cast this `nsIProxiedChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProxiedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProxiedChannel {
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
impl<T: nsISupportsCoerce> nsIProxiedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxiedChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProxiedChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProxiedChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIProxyInfo proxyInfo; */
    pub GetProxyInfo: unsafe extern "system" fn (this: *const nsIProxiedChannel, aProxyInfo: *mut*const nsIProxyInfo) -> ::nserror::nsresult,

    /* readonly attribute int32_t httpProxyConnectResponseCode; */
    pub GetHttpProxyConnectResponseCode: unsafe extern "system" fn (this: *const nsIProxiedChannel, aHttpProxyConnectResponseCode: *mut int32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProxiedChannel {

    /// ```text
    /// /**
    ///    * Gets the proxy info the channel was constructed with. null or a
    ///    * proxyInfo with type "direct" mean no proxy.
    ///    *
    ///    * The returned proxy info must not be modified.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIProxyInfo proxyInfo;`
    #[inline]
    pub unsafe fn GetProxyInfo(&self, aProxyInfo: *mut*const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyInfo)(self, aProxyInfo)
    }


    /// ```text
    /// /**
    ///    * The HTTP response code returned from the proxy to the CONNECT method.
    ///    * The response code is only available when we get the response from
    ///    * the proxy server, so this value is known in and after OnStartRequest.
    ///    *
    ///    * If CONNECT method is not used, httpProxyConnectResponseCode is always -1.
    ///    * After OnStartRequest, httpProxyConnectResponseCode is the real HTTP
    ///    * response code or 0 if we can't reach to the proxy.
    ///    */
    /// ```
    ///

    /// `readonly attribute int32_t httpProxyConnectResponseCode;`
    #[inline]
    pub unsafe fn GetHttpProxyConnectResponseCode(&self, aHttpProxyConnectResponseCode: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetHttpProxyConnectResponseCode)(self, aHttpProxyConnectResponseCode)
    }


}


