//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxiedProtocolHandler.idl
//


/// `interface nsIProxiedProtocolHandler : nsIProtocolHandler`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProxiedProtocolHandler {
    vtable: *const nsIProxiedProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProxiedProtocolHandler.
unsafe impl XpCom for nsIProxiedProtocolHandler {
    const IID: nsIID = nsID(0x3756047a, 0xfa2b, 0x4b45,
        [0x99, 0x48, 0x3b, 0x5f, 0x8f, 0xc3, 0x75, 0xe7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProxiedProtocolHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProxiedProtocolHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProxiedProtocolHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIProxiedProtocolHandler`.
    fn coerce_from(v: &nsIProxiedProtocolHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProxiedProtocolHandlerCoerce for nsIProxiedProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIProxiedProtocolHandler) -> &Self {
        v
    }
}

impl nsIProxiedProtocolHandler {
    /// Cast this `nsIProxiedProtocolHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProxiedProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProxiedProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProtocolHandlerCoerce> nsIProxiedProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxiedProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProxiedProtocolHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProxiedProtocolHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProtocolHandlerVTable,

    /* nsIChannel newProxiedChannel (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI, in nsILoadInfo aLoadInfo); */
    pub NewProxiedChannel: unsafe extern "system" fn (this: *const nsIProxiedProtocolHandler, uri: *const nsIURI, proxyInfo: *const nsIProxyInfo, proxyResolveFlags: u32, proxyURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProxiedProtocolHandler {

    /// ```text
    /// /** Create a new channel with the given proxyInfo
    ///      *
    ///      * @param uri the channel uri
    ///      * @param proxyInfo any proxy information that has already been determined,
    ///      *        or null if channel should later determine the proxy on its own using
    ///      *        proxyResolveFlags/proxyURI
    ///      * @param proxyResolveFlags used if the proxy is later determined
    ///      *        from nsIProtocolProxyService::asyncResolve
    ///      * @param proxyURI used if the proxy is later determined from
    ///      *        nsIProtocolProxyService::asyncResolve with this as the proxyURI name.
    ///      *        Generally this is the same as uri (or null which has the same
        ///      *        effect), except in the case of websockets which wants to bootstrap
    ///      *        to an http:// channel but make its proxy determination based on
    ///      *        a ws:// uri.
    ///      * @param aLoadInfo used to evaluate who initated the resource request.
    ///      */
    /// ```
    ///

    /// `nsIChannel newProxiedChannel (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI, in nsILoadInfo aLoadInfo);`
    #[inline]
    pub unsafe fn NewProxiedChannel(&self, uri: *const nsIURI, proxyInfo: *const nsIProxyInfo, proxyResolveFlags: u32, proxyURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewProxiedChannel)(self, uri, proxyInfo, proxyResolveFlags, proxyURI, aLoadInfo, _retval)
    }


}


