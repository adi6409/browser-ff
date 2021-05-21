//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketChannel.idl
//


/// `interface nsIWebSocketChannel : nsISupports`
///

/// ```text
/// /**
///  * Low-level websocket API: handles network protocol.
///  *
///  * This is primarly intended for use by the higher-level nsIWebSocket.idl.
///  * We are also making it scriptable for now, but this may change once we have
///  * WebSockets for Workers.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebSocketChannel {
    vtable: *const nsIWebSocketChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebSocketChannel.
unsafe impl XpCom for nsIWebSocketChannel {
    const IID: nsIID = nsID(0xce71d028, 0x322a, 0x4105,
        [0xa9, 0x47, 0xa8, 0x94, 0x68, 0x9b, 0x52, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebSocketChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebSocketChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebSocketChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIWebSocketChannel`.
    fn coerce_from(v: &nsIWebSocketChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebSocketChannelCoerce for nsIWebSocketChannel {
    #[inline]
    fn coerce_from(v: &nsIWebSocketChannel) -> &Self {
        v
    }
}

impl nsIWebSocketChannel {
    /// Cast this `nsIWebSocketChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebSocketChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebSocketChannel {
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
impl<T: nsISupportsCoerce> nsIWebSocketChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebSocketChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebSocketChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute nsIURI originalURI; */
    pub GetOriginalURI: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aOriginalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
    pub GetNotificationCallbacks: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
    pub SetNotificationCallbacks: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsISupports securityInfo; */
    pub GetSecurityInfo: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [must_use] attribute nsILoadGroup loadGroup; */
    pub GetLoadGroup: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,

    /* [must_use] attribute nsILoadGroup loadGroup; */
    pub SetLoadGroup: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aLoadGroup: *const nsILoadGroup) -> ::nserror::nsresult,

    /* [must_use] attribute nsILoadInfo loadInfo; */
    pub GetLoadInfo: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aLoadInfo: *mut*const nsILoadInfo) -> ::nserror::nsresult,

    /* [must_use] attribute nsILoadInfo loadInfo; */
    pub SetLoadInfo: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aLoadInfo: *const nsILoadInfo) -> ::nserror::nsresult,

    /* [must_use] attribute ACString protocol; */
    pub GetProtocol: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aProtocol: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] attribute ACString protocol; */
    pub SetProtocol: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aProtocol: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString extensions; */
    pub GetExtensions: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aExtensions: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute uint64_t httpChannelId; */
    pub GetHttpChannelId: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aHttpChannelId: *mut uint64_t) -> ::nserror::nsresult,

    /* [notxpcom] nsresult initLoadInfoNative (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in nsICookieJarSettings aCookieJarSettings, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags); */
    pub InitLoadInfoNative: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aCookieJarSettings: *const nsICookieJarSettings, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, aSandboxFlags: u32) -> ::nserror::nsresult,

    /* [must_use] void initLoadInfo (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
    pub InitLoadInfo: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType) -> ::nserror::nsresult,

    /* [must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext); */
    pub AsyncOpen: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aURI: *const nsIURI, aOrigin: *const ::nsstring::nsACString, aInnerWindowID: u64, aListener: *const nsIWebSocketListener, aContext: *const nsISupports) -> ::nserror::nsresult,

    /* [must_use] void close (in unsigned short aCode, in AUTF8String aReason); */
    pub Close: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aCode: u16, aReason: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void sendMsg (in AUTF8String aMsg); */
    pub SendMsg: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void sendBinaryMsg (in ACString aMsg); */
    pub SendBinaryMsg: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length); */
    pub SendBinaryStream: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aStream: *const nsIInputStream, length: u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long pingInterval; */
    pub GetPingInterval: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aPingInterval: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long pingInterval; */
    pub SetPingInterval: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aPingInterval: u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long pingTimeout; */
    pub GetPingTimeout: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aPingTimeout: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long pingTimeout; */
    pub SetPingTimeout: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aPingTimeout: u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long serial; */
    pub GetSerial: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aSerial: *mut u32) -> ::nserror::nsresult,

    /* [must_use] attribute unsigned long serial; */
    pub SetSerial: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aSerial: u32) -> ::nserror::nsresult,

    /* [must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions); */
    pub SetServerParameters: unsafe extern "system" fn (this: *const nsIWebSocketChannel, aProvider: *const nsITransportProvider, aNegotiatedExtensions: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebSocketChannel {

    pub const CLOSE_NORMAL: i64 = 1000;


    pub const CLOSE_GOING_AWAY: i64 = 1001;


    pub const CLOSE_PROTOCOL_ERROR: i64 = 1002;


    pub const CLOSE_UNSUPPORTED_DATATYPE: i64 = 1003;


    pub const CLOSE_NO_STATUS: i64 = 1005;


    pub const CLOSE_ABNORMAL: i64 = 1006;


    pub const CLOSE_INVALID_PAYLOAD: i64 = 1007;


    pub const CLOSE_POLICY_VIOLATION: i64 = 1008;


    pub const CLOSE_TOO_LARGE: i64 = 1009;


    pub const CLOSE_EXTENSION_MISSING: i64 = 1010;


    pub const CLOSE_INTERNAL_ERROR: i64 = 1011;


    pub const CLOSE_TLS_FAILED: i64 = 1015;

    /// ```text
    /// /**
    ///      * The original URI used to construct the protocol connection. This is used
    ///      * in the case of a redirect or URI "resolution" (e.g. resolving a
        ///      * resource: URI to a file: URI) so that the original pre-redirect
    ///      * URI can still be obtained.  This is never null.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIURI originalURI;`
    #[inline]
    pub unsafe fn GetOriginalURI(&self, aOriginalURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalURI)(self, aOriginalURI)
    }


    /// ```text
    /// /**
    ///      * The readonly URI corresponding to the protocol connection after any
    ///      * redirections are completed.
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
    ///      * The notification callbacks for authorization, etc..
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn GetNotificationCallbacks(&self, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).GetNotificationCallbacks)(self, aNotificationCallbacks)
    }


    /// ```text
    /// /**
    ///      * The notification callbacks for authorization, etc..
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn SetNotificationCallbacks(&self, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).SetNotificationCallbacks)(self, aNotificationCallbacks)
    }


    /// ```text
    /// /**
    ///      * Transport-level security information (if any)
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsISupports securityInfo;`
    #[inline]
    pub unsafe fn GetSecurityInfo(&self, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetSecurityInfo)(self, aSecurityInfo)
    }


    /// ```text
    /// /**
    ///      * The load group of of the websocket
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsILoadGroup loadGroup;`
    #[inline]
    pub unsafe fn GetLoadGroup(&self, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadGroup)(self, aLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The load group of of the websocket
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsILoadGroup loadGroup;`
    #[inline]
    pub unsafe fn SetLoadGroup(&self, aLoadGroup: *const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadGroup)(self, aLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The load info of the websocket
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsILoadInfo loadInfo;`
    #[inline]
    pub unsafe fn GetLoadInfo(&self, aLoadInfo: *mut*const nsILoadInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadInfo)(self, aLoadInfo)
    }


    /// ```text
    /// /**
    ///      * The load info of the websocket
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsILoadInfo loadInfo;`
    #[inline]
    pub unsafe fn SetLoadInfo(&self, aLoadInfo: *const nsILoadInfo) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadInfo)(self, aLoadInfo)
    }


    /// ```text
    /// /**
    ///      * Sec-Websocket-Protocol value
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString protocol;`
    #[inline]
    pub unsafe fn GetProtocol(&self, aProtocol: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocol)(self, aProtocol)
    }


    /// ```text
    /// /**
    ///      * Sec-Websocket-Protocol value
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString protocol;`
    #[inline]
    pub unsafe fn SetProtocol(&self, aProtocol: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetProtocol)(self, aProtocol)
    }


    /// ```text
    /// /**
    ///      * Sec-Websocket-Extensions response header value
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute ACString extensions;`
    #[inline]
    pub unsafe fn GetExtensions(&self, aExtensions: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetExtensions)(self, aExtensions)
    }


    /// ```text
    /// /**
    ///      * The channelId of the underlying http channel.
    ///      * It's available only after nsIWebSocketListener::onStart
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute uint64_t httpChannelId;`
    #[inline]
    pub unsafe fn GetHttpChannelId(&self, aHttpChannelId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetHttpChannelId)(self, aHttpChannelId)
    }


    /// ```text
    /// /**
    ///      * Init the WebSocketChannel with LoadInfo arguments.
    ///      * @param aLoadingNode
    ///      * @param aLoadingPrincipal
    ///      * @param aTriggeringPrincipal
    ///      * @param aCookieJarSettings
    ///      * @param aSecurityFlags
    ///      * @param aContentPolicyType
    ///      *        These will be used as values for the nsILoadInfo object on the
    ///      *        created channel. For details, see nsILoadInfo in nsILoadInfo.idl
    ///      * @return reference to the new nsIChannel object
    ///      *
    ///      * Keep in mind that URIs coming from a webpage should *never* use the
    ///      * systemPrincipal as the loadingPrincipal.
    ///      *
    ///      * Please note, if you provide both a loadingNode and a loadingPrincipal,
    ///      * then loadingPrincipal must be equal to loadingNode->NodePrincipal().
    ///      * But less error prone is to just supply a loadingNode.
    ///      */
    /// ```
    ///

    /// `[notxpcom] nsresult initLoadInfoNative (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in nsICookieJarSettings aCookieJarSettings, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags);`
    #[inline]
    pub unsafe fn InitLoadInfoNative(&self, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aCookieJarSettings: *const nsICookieJarSettings, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, aSandboxFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).InitLoadInfoNative)(self, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aCookieJarSettings, aSecurityFlags, aContentPolicyType, aSandboxFlags)
    }


    /// ```text
    /// /**
    ///       * Similar to the previous one but without nsICookieJarSettings.
    ///       * This method is used by JS code where nsICookieJarSettings is not exposed.
    ///       */
    /// ```
    ///

    /// `[must_use] void initLoadInfo (in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn InitLoadInfo(&self, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType) -> ::nserror::nsresult {
        ((*self.vtable).InitLoadInfo)(self, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType)
    }


    /// ```text
    /// /**
    ///      * Asynchronously open the websocket connection.  Received messages are fed
    ///      * to the socket listener as they arrive.  The socket listener's methods
    ///      * are called on the thread that calls asyncOpen and are not called until
    ///      * after asyncOpen returns.  If asyncOpen returns successfully, the
    ///      * protocol implementation promises to call at least onStop on the listener.
    ///      *
    ///      * NOTE: Implementations should throw NS_ERROR_ALREADY_OPENED if the
    ///      * websocket connection is reopened.
    ///      *
    ///      * @param aURI the uri of the websocket protocol - may be redirected
    ///      * @param aOrigin the uri of the originating resource
    ///      * @param aInnerWindowID the inner window ID
    ///      * @param aListener the nsIWebSocketListener implementation
    ///      * @param aContext an opaque parameter forwarded to aListener's methods
    ///      */
    /// ```
    ///

    /// `[must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext);`
    #[inline]
    pub unsafe fn AsyncOpen(&self, aURI: *const nsIURI, aOrigin: *const ::nsstring::nsACString, aInnerWindowID: u64, aListener: *const nsIWebSocketListener, aContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).AsyncOpen)(self, aURI, aOrigin, aInnerWindowID, aListener, aContext)
    }



    /// `[must_use] void close (in unsigned short aCode, in AUTF8String aReason);`
    #[inline]
    pub unsafe fn Close(&self, aCode: u16, aReason: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, aCode, aReason)
    }


    /// ```text
    /// /**
    ///      * Use to send text message down the connection to WebSocket peer.
    ///      *
    ///      * @param aMsg the utf8 string to send
    ///      */
    /// ```
    ///

    /// `[must_use] void sendMsg (in AUTF8String aMsg);`
    #[inline]
    pub unsafe fn SendMsg(&self, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendMsg)(self, aMsg)
    }


    /// ```text
    /// /**
    ///      * Use to send binary message down the connection to WebSocket peer.
    ///      *
    ///      * @param aMsg the data to send
    ///      */
    /// ```
    ///

    /// `[must_use] void sendBinaryMsg (in ACString aMsg);`
    #[inline]
    pub unsafe fn SendBinaryMsg(&self, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendBinaryMsg)(self, aMsg)
    }


    /// ```text
    /// /**
    ///      * Use to send a binary stream (Blob) to Websocket peer.
    ///      *
    ///      * @param aStream The input stream to be sent.
    ///      */
    /// ```
    ///

    /// `[must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length);`
    #[inline]
    pub unsafe fn SendBinaryStream(&self, aStream: *const nsIInputStream, length: u32) -> ::nserror::nsresult {
        ((*self.vtable).SendBinaryStream)(self, aStream, length)
    }


    /// ```text
    /// /**
    ///      * This value determines how often (in seconds) websocket keepalive
    ///      * pings are sent.  If set to 0 (the default), no pings are ever sent.
    ///      *
    ///      * This value can currently only be set before asyncOpen is called, else
    ///      * NS_ERROR_IN_PROGRESS is thrown.
    ///      *
    ///      * Be careful using this setting: ping traffic can consume lots of power and
    ///      * bandwidth over time.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long pingInterval;`
    #[inline]
    pub unsafe fn GetPingInterval(&self, aPingInterval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPingInterval)(self, aPingInterval)
    }


    /// ```text
    /// /**
    ///      * This value determines how often (in seconds) websocket keepalive
    ///      * pings are sent.  If set to 0 (the default), no pings are ever sent.
    ///      *
    ///      * This value can currently only be set before asyncOpen is called, else
    ///      * NS_ERROR_IN_PROGRESS is thrown.
    ///      *
    ///      * Be careful using this setting: ping traffic can consume lots of power and
    ///      * bandwidth over time.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long pingInterval;`
    #[inline]
    pub unsafe fn SetPingInterval(&self, aPingInterval: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPingInterval)(self, aPingInterval)
    }


    /// ```text
    /// /**
    ///      * This value determines how long (in seconds) the websocket waits for
    ///      * the server to reply to a ping that has been sent before considering the
    ///      * connection broken.
    ///      *
    ///      * This value can currently only be set before asyncOpen is called, else
    ///      * NS_ERROR_IN_PROGRESS is thrown.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long pingTimeout;`
    #[inline]
    pub unsafe fn GetPingTimeout(&self, aPingTimeout: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPingTimeout)(self, aPingTimeout)
    }


    /// ```text
    /// /**
    ///      * This value determines how long (in seconds) the websocket waits for
    ///      * the server to reply to a ping that has been sent before considering the
    ///      * connection broken.
    ///      *
    ///      * This value can currently only be set before asyncOpen is called, else
    ///      * NS_ERROR_IN_PROGRESS is thrown.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long pingTimeout;`
    #[inline]
    pub unsafe fn SetPingTimeout(&self, aPingTimeout: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPingTimeout)(self, aPingTimeout)
    }


    /// ```text
    /// /**
    ///      * Unique ID for this channel. It's not readonly because when the channel is
    ///      * created via IPC, the serial number is received from the child process.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long serial;`
    #[inline]
    pub unsafe fn GetSerial(&self, aSerial: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSerial)(self, aSerial)
    }


    /// ```text
    /// /**
    ///      * Unique ID for this channel. It's not readonly because when the channel is
    ///      * created via IPC, the serial number is received from the child process.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute unsigned long serial;`
    #[inline]
    pub unsafe fn SetSerial(&self, aSerial: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetSerial)(self, aSerial)
    }


    /// ```text
    /// /**
    ///      * Set a nsITransportProvider and negotated extensions to be used by this
    ///      * channel. Calling this function also means that this channel will
    ///      * implement the server-side part of a websocket connection rather than the
    ///      * client-side part.
    ///      */
    /// ```
    ///

    /// `[must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions);`
    #[inline]
    pub unsafe fn SetServerParameters(&self, aProvider: *const nsITransportProvider, aNegotiatedExtensions: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetServerParameters)(self, aProvider, aNegotiatedExtensions)
    }


}


