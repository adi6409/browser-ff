//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthenticableChannel.idl
//


/// `interface nsIHttpAuthenticableChannel : nsIProxiedChannel`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpAuthenticableChannel {
    vtable: *const nsIHttpAuthenticableChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpAuthenticableChannel.
unsafe impl XpCom for nsIHttpAuthenticableChannel {
    const IID: nsIID = nsID(0x701093ac, 0x5c7f, 0x429c,
        [0x99, 0xe3, 0x42, 0x3b, 0x04, 0x1f, 0xcc, 0xb4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpAuthenticableChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpAuthenticableChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpAuthenticableChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpAuthenticableChannel`.
    fn coerce_from(v: &nsIHttpAuthenticableChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpAuthenticableChannelCoerce for nsIHttpAuthenticableChannel {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticableChannel) -> &Self {
        v
    }
}

impl nsIHttpAuthenticableChannel {
    /// Cast this `nsIHttpAuthenticableChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpAuthenticableChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpAuthenticableChannel {
    type Target = nsIProxiedChannel;
    #[inline]
    fn deref(&self) -> &nsIProxiedChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProxiedChannelCoerce> nsIHttpAuthenticableChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticableChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpAuthenticableChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpAuthenticableChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProxiedChannelVTable,

    /* [must_use] readonly attribute boolean isSSL; */
    pub GetIsSSL: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aIsSSL: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean proxyMethodIsConnect; */
    pub GetProxyMethodIsConnect: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aProxyMethodIsConnect: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void cancel (in nsresult aStatus); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsLoadFlags loadFlags; */
    pub GetLoadFlags: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsILoadGroup loadGroup; */
    pub GetLoadGroup: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks; */
    pub GetNotificationCallbacks: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString requestMethod; */
    pub GetRequestMethod: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aRequestMethod: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString serverResponseHeader; */
    pub GetServerResponseHeader: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aServerResponseHeader: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString proxyChallenges; */
    pub GetProxyChallenges: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aProxyChallenges: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString WWWChallenges; */
    pub GetWWWChallenges: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, aWWWChallenges: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setProxyCredentials (in ACString credentials); */
    pub SetProxyCredentials: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, credentials: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setWWWCredentials (in ACString credentials); */
    pub SetWWWCredentials: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, credentials: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void onAuthAvailable (); */
    pub OnAuthAvailable: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel) -> ::nserror::nsresult,

    /* [must_use] void onAuthCancelled (in boolean userCancel); */
    pub OnAuthCancelled: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, userCancel: bool) -> ::nserror::nsresult,

    /* [must_use] void closeStickyConnection (); */
    pub CloseStickyConnection: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel) -> ::nserror::nsresult,

    /* void connectionRestartable (in boolean restartable); */
    pub ConnectionRestartable: unsafe extern "system" fn (this: *const nsIHttpAuthenticableChannel, restartable: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpAuthenticableChannel {

    /// ```text
    /// /**
    ///      * If the channel being authenticated is using SSL.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean isSSL;`
    #[inline]
    pub unsafe fn GetIsSSL(&self, aIsSSL: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSSL)(self, aIsSSL)
    }


    /// ```text
    /// /**
    ///      * Returns if the proxy HTTP method used is CONNECT. If no proxy is being
    ///      * used it must return PR_FALSE.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean proxyMethodIsConnect;`
    #[inline]
    pub unsafe fn GetProxyMethodIsConnect(&self, aProxyMethodIsConnect: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyMethodIsConnect)(self, aProxyMethodIsConnect)
    }


    /// ```text
    /// /**
    ///      * Cancels the current request. See nsIRequest.
    ///      */
    /// ```
    ///

    /// `[must_use] void cancel (in nsresult aStatus);`
    #[inline]
    pub unsafe fn Cancel(&self, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, aStatus)
    }


    /// ```text
    /// /**
    ///      * The load flags of this request. See nsIRequest.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsLoadFlags loadFlags;`
    #[inline]
    pub unsafe fn GetLoadFlags(&self, aLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadFlags)(self, aLoadFlags)
    }


    /// ```text
    /// /**
    ///      * The URI corresponding to the channel. See nsIChannel.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIURI URI;`
    #[inline]
    pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///      * The load group of this request. It is here for querying its
    ///      * notificationCallbacks. See nsIRequest.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsILoadGroup loadGroup;`
    #[inline]
    pub unsafe fn GetLoadGroup(&self, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadGroup)(self, aLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The notification callbacks for the channel. See nsIChannel.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn GetNotificationCallbacks(&self, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).GetNotificationCallbacks)(self, aNotificationCallbacks)
    }


    /// ```text
    /// /**
    ///      * The HTTP request method. See nsIHttpChannel.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString requestMethod;`
    #[inline]
    pub unsafe fn GetRequestMethod(&self, aRequestMethod: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestMethod)(self, aRequestMethod)
    }


    /// ```text
    /// /**
    ///      * The "Server" response header.
    ///      * Return NS_ERROR_NOT_AVAILABLE if not available.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString serverResponseHeader;`
    #[inline]
    pub unsafe fn GetServerResponseHeader(&self, aServerResponseHeader: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetServerResponseHeader)(self, aServerResponseHeader)
    }


    /// ```text
    /// /**
    ///      * The Proxy-Authenticate response header.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString proxyChallenges;`
    #[inline]
    pub unsafe fn GetProxyChallenges(&self, aProxyChallenges: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProxyChallenges)(self, aProxyChallenges)
    }


    /// ```text
    /// /**
    ///      * The WWW-Authenticate response header.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString WWWChallenges;`
    #[inline]
    pub unsafe fn GetWWWChallenges(&self, aWWWChallenges: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetWWWChallenges)(self, aWWWChallenges)
    }


    /// ```text
    /// /**
    ///      * Sets the Proxy-Authorization request header. An empty string
    ///      * will clear it.
    ///      */
    /// ```
    ///

    /// `[must_use] void setProxyCredentials (in ACString credentials);`
    #[inline]
    pub unsafe fn SetProxyCredentials(&self, credentials: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetProxyCredentials)(self, credentials)
    }


    /// ```text
    /// /**
    ///      * Sets the Authorization request header. An empty string
    ///      * will clear it.
    ///      */
    /// ```
    ///

    /// `[must_use] void setWWWCredentials (in ACString credentials);`
    #[inline]
    pub unsafe fn SetWWWCredentials(&self, credentials: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetWWWCredentials)(self, credentials)
    }


    /// ```text
    /// /**
    ///      * Called when authentication information is ready and has been set on this
    ///      * object using setWWWCredentials/setProxyCredentials. Implementations can
    ///      * continue with the request and send the given information to the server.
    ///      *
    ///      * It is called asynchronously from
    ///      * nsIHttpChannelAuthProvider::processAuthentication if that method returns
    ///      * NS_ERROR_IN_PROGRESS.
    ///      *
    ///      * @note  Any exceptions thrown from this method should be ignored.
    ///      */
    /// ```
    ///

    /// `[must_use] void onAuthAvailable ();`
    #[inline]
    pub unsafe fn OnAuthAvailable(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnAuthAvailable)(self, )
    }


    /// ```text
    /// /**
    ///      * Notifies that the prompt was cancelled. It is called asynchronously
    ///      * from nsIHttpChannelAuthProvider::processAuthentication if that method
    ///      * returns NS_ERROR_IN_PROGRESS.
    ///      *
    ///      * @param userCancel
    ///      *        If the user was cancelled has cancelled the authentication prompt.
    ///      */
    /// ```
    ///

    /// `[must_use] void onAuthCancelled (in boolean userCancel);`
    #[inline]
    pub unsafe fn OnAuthCancelled(&self, userCancel: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnAuthCancelled)(self, userCancel)
    }


    /// ```text
    /// /**
    ///      * Tells the channel to drop and close any sticky connection, since this
    ///      * connection oriented schema cannot be negotiated second time on
    ///      * the same connection.
    ///      */
    /// ```
    ///

    /// `[must_use] void closeStickyConnection ();`
    #[inline]
    pub unsafe fn CloseStickyConnection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CloseStickyConnection)(self, )
    }


    /// ```text
    /// /**
    ///      * Tells the channel to mark the connection as allowed to restart on
    ///      * authentication retry.  This is allowed when the request is a start
    ///      * of a new authentication round.
    ///      */
    /// ```
    ///

    /// `void connectionRestartable (in boolean restartable);`
    #[inline]
    pub unsafe fn ConnectionRestartable(&self, restartable: bool) -> ::nserror::nsresult {
        ((*self.vtable).ConnectionRestartable)(self, restartable)
    }


}


