//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelAuthProvider.idl
//


/// `interface nsIHttpChannelAuthProvider : nsICancelable`
///

/// ```text
/// /**
///  * nsIHttpChannelAuthProvider
///  *
///  * This interface is intended for providing authentication for http-style
///  * channels, like nsIHttpChannel and nsIWebSocket, which implement the
///  * nsIHttpAuthenticableChannel interface.
///  *
///  * When requesting pages AddAuthorizationHeaders MUST be called
///  * in order to get the http cached headers credentials. When the request is
///  * unsuccessful because of receiving either a 401 or 407 http response code
///  * ProcessAuthentication MUST be called and the page MUST be requested again
///  * with the new credentials that the user has provided. After a successful
///  * request, checkForSuperfluousAuth MAY be called, and disconnect MUST be
///  * called.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpChannelAuthProvider {
    vtable: *const nsIHttpChannelAuthProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpChannelAuthProvider.
unsafe impl XpCom for nsIHttpChannelAuthProvider {
    const IID: nsIID = nsID(0x788f331b, 0x2e1f, 0x436c,
        [0xb4, 0x05, 0x4f, 0x88, 0xa3, 0x1a, 0x10, 0x5b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpChannelAuthProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpChannelAuthProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpChannelAuthProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpChannelAuthProvider`.
    fn coerce_from(v: &nsIHttpChannelAuthProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpChannelAuthProviderCoerce for nsIHttpChannelAuthProvider {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelAuthProvider) -> &Self {
        v
    }
}

impl nsIHttpChannelAuthProvider {
    /// Cast this `nsIHttpChannelAuthProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpChannelAuthProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpChannelAuthProvider {
    type Target = nsICancelable;
    #[inline]
    fn deref(&self) -> &nsICancelable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICancelableCoerce> nsIHttpChannelAuthProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelAuthProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpChannelAuthProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpChannelAuthProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICancelableVTable,

    /* [must_use] void init (in nsIHttpAuthenticableChannel channel); */
    pub Init: unsafe extern "system" fn (this: *const nsIHttpChannelAuthProvider, channel: *const nsIHttpAuthenticableChannel) -> ::nserror::nsresult,

    /* [must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed); */
    pub ProcessAuthentication: unsafe extern "system" fn (this: *const nsIHttpChannelAuthProvider, httpStatus: u32, sslConnectFailed: bool) -> ::nserror::nsresult,

    /* [must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds); */
    pub AddAuthorizationHeaders: unsafe extern "system" fn (this: *const nsIHttpChannelAuthProvider, dontUseCachedWWWCreds: bool) -> ::nserror::nsresult,

    /* [must_use] void checkForSuperfluousAuth (); */
    pub CheckForSuperfluousAuth: unsafe extern "system" fn (this: *const nsIHttpChannelAuthProvider) -> ::nserror::nsresult,

    /* [must_use] void disconnect (in nsresult status); */
    pub Disconnect: unsafe extern "system" fn (this: *const nsIHttpChannelAuthProvider, status: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void clearProxyIdent (); */
    pub ClearProxyIdent: unsafe extern "system" fn (this: *const nsIHttpChannelAuthProvider) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpChannelAuthProvider {

    /// ```text
    /// /**
    ///    * Initializes the http authentication support for the channel.
    ///    * Implementations must hold a weak reference of the channel.
    ///    */
    /// ```
    ///

    /// `[must_use] void init (in nsIHttpAuthenticableChannel channel);`
    #[inline]
    pub unsafe fn Init(&self, channel: *const nsIHttpAuthenticableChannel) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, channel)
    }


    /// ```text
    /// /**
    ///    * Upon receipt of a server challenge, this function is called to determine
    ///    * the credentials to send.
    ///    *
    ///    * @param httpStatus
    ///    *        the http status received.
    ///    * @param sslConnectFailed
    ///    *        if the last ssl tunnel connection attempt was or not successful.
    ///    * @param callback
    ///    *        the callback to be called when it returns NS_ERROR_IN_PROGRESS.
    ///    *        The implementation must hold a weak reference.
    ///    *
    ///    * @returns NS_OK if the credentials were got and set successfully.
    ///    *          NS_ERROR_IN_PROGRESS if the credentials are going to be asked to
    ///    *                               the user. The channel reference must be
    ///    *                               alive until the feedback from
    ///    *                               nsIHttpAuthenticableChannel's methods or
    ///    *                               until disconnect be called.
    ///    */
    /// ```
    ///

    /// `[must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed);`
    #[inline]
    pub unsafe fn ProcessAuthentication(&self, httpStatus: u32, sslConnectFailed: bool) -> ::nserror::nsresult {
        ((*self.vtable).ProcessAuthentication)(self, httpStatus, sslConnectFailed)
    }


    /// ```text
    /// /**
    ///    * Add credentials from the http auth cache.
    ///    *
    ///    * @param dontUseCachedWWWCreds
    ///    *    When true, the method will not add any Authorization headers from
    ///    *    the auth cache.
    ///    */
    /// ```
    ///

    /// `[must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds);`
    #[inline]
    pub unsafe fn AddAuthorizationHeaders(&self, dontUseCachedWWWCreds: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddAuthorizationHeaders)(self, dontUseCachedWWWCreds)
    }


    /// ```text
    /// /**
    ///    * Check if an unnecessary(and maybe malicious) url authentication has been
    ///    * provided.
    ///    */
    /// ```
    ///

    /// `[must_use] void checkForSuperfluousAuth ();`
    #[inline]
    pub unsafe fn CheckForSuperfluousAuth(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CheckForSuperfluousAuth)(self, )
    }


    /// ```text
    /// /**
    ///    * Cancel pending user auth prompts and release the callback and channel
    ///    * weak references.
    ///    */
    /// ```
    ///

    /// `[must_use] void disconnect (in nsresult status);`
    #[inline]
    pub unsafe fn Disconnect(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Disconnect)(self, status)
    }


    /// ```text
    /// /**
    ///    * Clear the proxy ident to not consider it invalid on re-athentication.
    ///    * Called when the channel finds out its transaction has been internally
    ///    * restarted.
    ///    */
    /// ```
    ///

    /// `void clearProxyIdent ();`
    #[inline]
    pub unsafe fn ClearProxyIdent(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearProxyIdent)(self, )
    }


}


