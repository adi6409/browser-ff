//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketTransport.idl
//


/// `interface nsISocketTransport : nsITransport`
///

/// ```text
/// /**
///  * nsISocketTransport
///  *
///  * NOTE: Connection setup is triggered by opening an input or output stream,
///  * it does not start on its own. Completion of the connection setup is
///  * indicated by a STATUS_CONNECTED_TO notification to the event sink (if set).
///  *
///  * NOTE: This is a free-threaded interface, meaning that the methods on
///  * this interface may be called from any thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISocketTransport {
    vtable: *const nsISocketTransportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISocketTransport.
unsafe impl XpCom for nsISocketTransport {
    const IID: nsIID = nsID(0x79221831, 0x85e2, 0x43a8,
        [0x81, 0x52, 0x05, 0xd7, 0x7d, 0x6f, 0xde, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISocketTransport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISocketTransport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISocketTransportCoerce {
    /// Cheaply cast a value of this type from a `nsISocketTransport`.
    fn coerce_from(v: &nsISocketTransport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISocketTransportCoerce for nsISocketTransport {
    #[inline]
    fn coerce_from(v: &nsISocketTransport) -> &Self {
        v
    }
}

impl nsISocketTransport {
    /// Cast this `nsISocketTransport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISocketTransportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISocketTransport {
    type Target = nsITransport;
    #[inline]
    fn deref(&self) -> &nsITransport {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsITransportCoerce> nsISocketTransportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketTransport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISocketTransport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISocketTransportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsITransportVTable,

    /* readonly attribute AUTF8String host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsISocketTransport, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsISocketTransport, aPort: *mut i32) -> ::nserror::nsresult,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub SetOriginAttributes: *const ::libc::c_void,

    /* [noscript] NetAddr getPeerAddr (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetPeerAddr: *const ::libc::c_void,

    /* [noscript] NetAddr getSelfAddr (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetSelfAddr: *const ::libc::c_void,

    /* [noscript] void bind (in NetAddrPtr aLocalAddr); */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub Bind: *const ::libc::c_void,

    /* nsINetAddr getScriptablePeerAddr (); */
    pub GetScriptablePeerAddr: unsafe extern "system" fn (this: *const nsISocketTransport, _retval: *mut*const nsINetAddr) -> ::nserror::nsresult,

    /* nsINetAddr getScriptableSelfAddr (); */
    pub GetScriptableSelfAddr: unsafe extern "system" fn (this: *const nsISocketTransport, _retval: *mut*const nsINetAddr) -> ::nserror::nsresult,

    /* readonly attribute nsISupports securityInfo; */
    pub GetSecurityInfo: unsafe extern "system" fn (this: *const nsISocketTransport, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor securityCallbacks; */
    pub GetSecurityCallbacks: unsafe extern "system" fn (this: *const nsISocketTransport, aSecurityCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor securityCallbacks; */
    pub SetSecurityCallbacks: unsafe extern "system" fn (this: *const nsISocketTransport, aSecurityCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* boolean isAlive (); */
    pub IsAlive: unsafe extern "system" fn (this: *const nsISocketTransport, _retval: *mut bool) -> ::nserror::nsresult,

    /* unsigned long getTimeout (in unsigned long aType); */
    pub GetTimeout: unsafe extern "system" fn (this: *const nsISocketTransport, aType: u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* void setTimeout (in unsigned long aType, in unsigned long aValue); */
    pub SetTimeout: unsafe extern "system" fn (this: *const nsISocketTransport, aType: u32, aValue: u32) -> ::nserror::nsresult,

    /* void setLinger (in boolean aPolarity, in short aTimeout); */
    pub SetLinger: unsafe extern "system" fn (this: *const nsISocketTransport, aPolarity: bool, aTimeout: i16) -> ::nserror::nsresult,

    /* void setReuseAddrPort (in bool reuseAddrPort); */
    pub SetReuseAddrPort: unsafe extern "system" fn (this: *const nsISocketTransport, reuseAddrPort: bool) -> ::nserror::nsresult,

    /* attribute unsigned long connectionFlags; */
    pub GetConnectionFlags: unsafe extern "system" fn (this: *const nsISocketTransport, aConnectionFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long connectionFlags; */
    pub SetConnectionFlags: unsafe extern "system" fn (this: *const nsISocketTransport, aConnectionFlags: u32) -> ::nserror::nsresult,

    /* attribute unsigned long tlsFlags; */
    pub GetTlsFlags: unsafe extern "system" fn (this: *const nsISocketTransport, aTlsFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long tlsFlags; */
    pub SetTlsFlags: unsafe extern "system" fn (this: *const nsISocketTransport, aTlsFlags: u32) -> ::nserror::nsresult,

    /* attribute octet QoSBits; */
    pub GetQoSBits: unsafe extern "system" fn (this: *const nsISocketTransport, aQoSBits: *mut u8) -> ::nserror::nsresult,

    /* attribute octet QoSBits; */
    pub SetQoSBits: unsafe extern "system" fn (this: *const nsISocketTransport, aQoSBits: u8) -> ::nserror::nsresult,

    /* attribute unsigned long recvBufferSize; */
    pub GetRecvBufferSize: unsafe extern "system" fn (this: *const nsISocketTransport, aRecvBufferSize: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long recvBufferSize; */
    pub SetRecvBufferSize: unsafe extern "system" fn (this: *const nsISocketTransport, aRecvBufferSize: u32) -> ::nserror::nsresult,

    /* attribute unsigned long sendBufferSize; */
    pub GetSendBufferSize: unsafe extern "system" fn (this: *const nsISocketTransport, aSendBufferSize: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long sendBufferSize; */
    pub SetSendBufferSize: unsafe extern "system" fn (this: *const nsISocketTransport, aSendBufferSize: u32) -> ::nserror::nsresult,

    /* attribute boolean keepaliveEnabled; */
    pub GetKeepaliveEnabled: unsafe extern "system" fn (this: *const nsISocketTransport, aKeepaliveEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean keepaliveEnabled; */
    pub SetKeepaliveEnabled: unsafe extern "system" fn (this: *const nsISocketTransport, aKeepaliveEnabled: bool) -> ::nserror::nsresult,

    /* void setKeepaliveVals (in long keepaliveIdleTime, in long keepaliveRetryInterval); */
    pub SetKeepaliveVals: unsafe extern "system" fn (this: *const nsISocketTransport, keepaliveIdleTime: i32, keepaliveRetryInterval: i32) -> ::nserror::nsresult,

    /* readonly attribute boolean resetIPFamilyPreference; */
    pub GetResetIPFamilyPreference: unsafe extern "system" fn (this: *const nsISocketTransport, aResetIPFamilyPreference: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean echConfigUsed; */
    pub GetEchConfigUsed: unsafe extern "system" fn (this: *const nsISocketTransport, aEchConfigUsed: *mut bool) -> ::nserror::nsresult,

    /* void setEchConfig (in ACString echConfig); */
    pub SetEchConfig: unsafe extern "system" fn (this: *const nsISocketTransport, echConfig: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* bool resolvedByTRR (); */
    pub ResolvedByTRR: unsafe extern "system" fn (this: *const nsISocketTransport, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void setIsPrivate (in boolean isPrivate); */
    pub SetIsPrivate: unsafe extern "system" fn (this: *const nsISocketTransport, isPrivate: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean retryDnsIfPossible; */
    pub GetRetryDnsIfPossible: unsafe extern "system" fn (this: *const nsISocketTransport, aRetryDnsIfPossible: *mut bool) -> ::nserror::nsresult,

    /* [noscript] readonly attribute nsresult status; */
    pub GetStatus: unsafe extern "system" fn (this: *const nsISocketTransport, aStatus: *mut ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISocketTransport {
    /// ```text
    /// /**
    ///      * Values for the aType parameter passed to get/setTimeout.
    ///      */
    /// ```
    ///

    pub const TIMEOUT_CONNECT: i64 = 0;


    pub const TIMEOUT_READ_WRITE: i64 = 1;

    /// ```text
    /// /**
    ///      * nsITransportEventSink status codes.
    ///      *
    ///      * Although these look like XPCOM error codes and are passed in an nsresult
    ///      * variable, they are *not* error codes.  Note that while they *do* overlap
    ///      * with existing error codes in Necko, these status codes are confined
    ///      * within a very limited context where no error codes may appear, so there
    ///      * is no ambiguity.
    ///      *
    ///      * The values of these status codes must never change.
    ///      *
    ///      * The status codes appear in near-chronological order (not in numeric
        ///      * order).  STATUS_RESOLVING may be skipped if the host does not need to be
    ///      * resolved.  STATUS_WAITING_FOR is an optional status code, which the impl
    ///      * of this interface may choose not to generate.
    ///      *
    ///      * In C++, these constants have a type of uint32_t, so C++ callers must use
    ///      * the NS_NET_STATUS_* constants defined below, which have a type of
    ///      * nsresult.
    ///      */
    /// ```
    ///

    pub const STATUS_RESOLVING: i64 = 2152398851;


    pub const STATUS_RESOLVED: i64 = 2152398859;


    pub const STATUS_CONNECTING_TO: i64 = 2152398855;


    pub const STATUS_CONNECTED_TO: i64 = 2152398852;


    pub const STATUS_SENDING_TO: i64 = 2152398853;


    pub const STATUS_WAITING_FOR: i64 = 2152398858;


    pub const STATUS_RECEIVING_FROM: i64 = 2152398854;


    pub const STATUS_TLS_HANDSHAKE_STARTING: i64 = 2152398860;


    pub const STATUS_TLS_HANDSHAKE_ENDED: i64 = 2152398861;

    /// ```text
    /// /**
    ///      * Values for the connectionFlags
    ///      *
    ///      * When making a new connection BYPASS_CACHE will force the Necko DNS
    ///      * cache entry to be refreshed with a new call to NSPR if it is set before
    ///      * opening the new stream.
    ///      */
    /// ```
    ///

    pub const BYPASS_CACHE: i64 = 1;

    /// ```text
    /// /**
    ///      * When setting this flag, the socket will not apply any
    ///      * credentials when establishing a connection. For example,
    ///      * an SSL connection would not send any client-certificates
    ///      * if this flag is set.
    ///      */
    /// ```
    ///

    pub const ANONYMOUS_CONNECT: i64 = 2;

    /// ```text
    /// /**
    ///      * If set, we will skip all IPv6 addresses the host may have and only
    ///      * connect to IPv4 ones.
    ///      */
    /// ```
    ///

    pub const DISABLE_IPV6: i64 = 4;

    /// ```text
    /// /**
    ///      * If set, indicates that the connection was initiated from a source
    ///      * defined as being private in the sense of Private Browsing. Generally,
    ///      * there should be no state shared between connections that are private
    ///      * and those that are not; it is OK for multiple private connections
    ///      * to share state with each other, and it is OK for multiple non-private
    ///      * connections to share state with each other.
    ///      */
    /// ```
    ///

    pub const NO_PERMANENT_STORAGE: i64 = 8;

    /// ```text
    /// /**
    ///      * If set, we will skip all IPv4 addresses the host may have and only
    ///      * connect to IPv6 ones.
    ///      */
    /// ```
    ///

    pub const DISABLE_IPV4: i64 = 16;

    /// ```text
    /// /**
    ///      * If set, indicates that the socket should not connect if the hostname
    ///      * resolves to an RFC1918 address or IPv6 equivalent.
    ///      */
    /// ```
    ///

    pub const DISABLE_RFC1918: i64 = 32;

    /// ```text
    /// /**
    ///      * If set, do not use newer protocol features that might have interop problems
    ///      * on the Internet. Intended only for use with critical infra like the updater.
    ///      * default is false.
    ///      */
    /// ```
    ///

    pub const BE_CONSERVATIVE: i64 = 64;

    /// ```text
    /// /**
    ///      * If set, do not use TRR for resolving the host name. Intended only for
    ///      * retries or other scenarios when TRR is deemed likely to have returned a
    ///      * wrong adddress.
    ///      */
    /// ```
    ///

    pub const DISABLE_TRR: i64 = 128;

    /// ```text
    /// /**
    ///      * Values for the connectionFlags
    ///      *
    ///      * When using BYPASS_CACHE, setting this bit will invalidate the existing
    ///      * cached entry immediately while the new resolve is being done to avoid
    ///      * other users from using stale content in the mean time.
    ///      */
    /// ```
    ///

    pub const REFRESH_CACHE: i64 = 256;

    /// ```text
    /// /**
    ///      * If this flag is set then it means that if connecting the preferred ip
    ///      * family has failed, retry with the oppsite one once more.
    ///      */
    /// ```
    ///

    pub const RETRY_WITH_DIFFERENT_IP_FAMILY: i64 = 512;

    /// ```text
    /// /**
    ///      * If we know that a server speaks only tls <1.3 there is no need to try
    ///      * to use ech.
    ///      */
    /// ```
    ///

    pub const DONT_TRY_ECH: i64 = 1024;

    /// ```text
    /// /**
    ///      * These two bits encode the TRR mode of the request.
    ///      * Use the static helper methods convert between the TRR mode and flags.
    ///      */
    /// ```
    ///

    pub const TRR_MODE_FLAGS: i64 = 6144;

    /// ```text
    /// /**
    ///      * If set, we will use IP hint addresses to connect to the host.
    ///      */
    /// ```
    ///

    pub const USE_IP_HINT_ADDRESS: i64 = 8192;

    /// ```text
    /// /**
    ///      * This is used for a temporary workaround for a web-compat issue. The flag is
    ///      * only set on CORS preflight request to allowed sending client certificates
    ///      * on a connection for an anonymous request.
    ///      */
    /// ```
    ///

    pub const ANONYMOUS_CONNECT_ALLOW_CLIENT_CERT: i64 = 16384;

    /// ```text
    /// /**
    ///      * Get the peer's host for the underlying socket connection.
    ///      * For Unix domain sockets, this is a pathname, or the empty string for
    ///      * unnamed and abstract socket addresses.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String host;`
    #[inline]
    pub unsafe fn GetHost(&self, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///      * Get the port for the underlying socket connection.
    ///      * For Unix domain sockets, this is zero.
    ///      */
    /// ```
    ///

    /// `readonly attribute long port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///      * The origin attributes are used to create sockets.  The first party domain
    ///      * will eventually be used to isolate OCSP cache and is only non-empty when
    ///      * "privacy.firstparty.isolate" is enabled.  Setting this is the only way to
    ///      * carry origin attributes down to NSPR layers which are final consumers.
    ///      * It must be set before the socket transport is built.
    ///      */
    /// ```
    ///

    /// `[binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes;`
    const _GetScriptableOriginAttributes: () = ();

    /// ```text
    /// /**
    ///      * The origin attributes are used to create sockets.  The first party domain
    ///      * will eventually be used to isolate OCSP cache and is only non-empty when
    ///      * "privacy.firstparty.isolate" is enabled.  Setting this is the only way to
    ///      * carry origin attributes down to NSPR layers which are final consumers.
    ///      * It must be set before the socket transport is built.
    ///      */
    /// ```
    ///

    /// `[binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes;`
    const _SetScriptableOriginAttributes: () = ();


    /// `[binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes ();`
    const _GetOriginAttributes: () = ();


    /// `[binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs);`
    const _SetOriginAttributes: () = ();

    /// ```text
    /// /**
    ///      * Returns the IP address of the socket connection peer. This
    ///      * attribute is defined only once a connection has been established.
    ///      */
    /// ```
    ///

    /// `[noscript] NetAddr getPeerAddr ();`
    const _GetPeerAddr: () = ();

    /// ```text
    /// /**
    ///      * Returns the IP address of the initiating end. This attribute
    ///      * is defined only once a connection has been established.
    ///      */
    /// ```
    ///

    /// `[noscript] NetAddr getSelfAddr ();`
    const _GetSelfAddr: () = ();

    /// ```text
    /// /**
    ///      * Bind to a specific local address.
    ///      */
    /// ```
    ///

    /// `[noscript] void bind (in NetAddrPtr aLocalAddr);`
    const _Bind: () = ();

    /// ```text
    /// /**
    ///      * Returns a scriptable version of getPeerAddr. This attribute is defined
    ///      * only once a connection has been established.
    ///      */
    /// ```
    ///

    /// `nsINetAddr getScriptablePeerAddr ();`
    #[inline]
    pub unsafe fn GetScriptablePeerAddr(&self, _retval: *mut*const nsINetAddr) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptablePeerAddr)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns a scriptable version of getSelfAddr. This attribute is defined
    ///      * only once a connection has been established.
    ///      */
    /// ```
    ///

    /// `nsINetAddr getScriptableSelfAddr ();`
    #[inline]
    pub unsafe fn GetScriptableSelfAddr(&self, _retval: *mut*const nsINetAddr) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptableSelfAddr)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Security info object returned from the secure socket provider.  This
    ///      * object supports nsISSLSocketControl, nsITransportSecurityInfo, and
    ///      * possibly other interfaces.
    ///      *
    ///      * This attribute is only available once the socket is connected.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsISupports securityInfo;`
    #[inline]
    pub unsafe fn GetSecurityInfo(&self, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetSecurityInfo)(self, aSecurityInfo)
    }


    /// ```text
    /// /**
    ///      * Security notification callbacks passed to the secure socket provider
    ///      * via nsISSLSocketControl at socket creation time.
    ///      *
    ///      * NOTE: this attribute cannot be changed once a stream has been opened.
    ///      */
    /// ```
    ///

    /// `attribute nsIInterfaceRequestor securityCallbacks;`
    #[inline]
    pub unsafe fn GetSecurityCallbacks(&self, aSecurityCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).GetSecurityCallbacks)(self, aSecurityCallbacks)
    }


    /// ```text
    /// /**
    ///      * Security notification callbacks passed to the secure socket provider
    ///      * via nsISSLSocketControl at socket creation time.
    ///      *
    ///      * NOTE: this attribute cannot be changed once a stream has been opened.
    ///      */
    /// ```
    ///

    /// `attribute nsIInterfaceRequestor securityCallbacks;`
    #[inline]
    pub unsafe fn SetSecurityCallbacks(&self, aSecurityCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).SetSecurityCallbacks)(self, aSecurityCallbacks)
    }


    /// ```text
    /// /**
    ///      * Test if this socket transport is (still) connected.
    ///      */
    /// ```
    ///

    /// `boolean isAlive ();`
    #[inline]
    pub unsafe fn IsAlive(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsAlive)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Socket timeouts in seconds.  To specify no timeout, pass UINT32_MAX
    ///      * as aValue to setTimeout.  The implementation may truncate timeout values
    ///      * to a smaller range of values (e.g., 0 to 0xFFFF).
    ///      */
    /// ```
    ///

    /// `unsigned long getTimeout (in unsigned long aType);`
    #[inline]
    pub unsafe fn GetTimeout(&self, aType: u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeout)(self, aType, _retval)
    }



    /// `void setTimeout (in unsigned long aType, in unsigned long aValue);`
    #[inline]
    pub unsafe fn SetTimeout(&self, aType: u32, aValue: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetTimeout)(self, aType, aValue)
    }


    /// ```text
    /// /**
    ///      * Sets the SO_LINGER option with the specified values for the l_onoff and
    ///      * l_linger parameters. This applies PR_SockOpt_Linger before PR_Close and
    ///      * can be used with a timeout of zero to send an RST packet when closing.
    ///      */
    /// ```
    ///

    /// `void setLinger (in boolean aPolarity, in short aTimeout);`
    #[inline]
    pub unsafe fn SetLinger(&self, aPolarity: bool, aTimeout: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetLinger)(self, aPolarity, aTimeout)
    }


    /// ```text
    /// /**
    ///      * True to set addr and port reuse socket options.
    ///      */
    /// ```
    ///

    /// `void setReuseAddrPort (in bool reuseAddrPort);`
    #[inline]
    pub unsafe fn SetReuseAddrPort(&self, reuseAddrPort: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetReuseAddrPort)(self, reuseAddrPort)
    }


    /// ```text
    /// /**
    ///      * connectionFlags is a bitmask that can be used to modify underlying
    ///      * behavior of the socket connection. See the flags below.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long connectionFlags;`
    #[inline]
    pub unsafe fn GetConnectionFlags(&self, aConnectionFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectionFlags)(self, aConnectionFlags)
    }


    /// ```text
    /// /**
    ///      * connectionFlags is a bitmask that can be used to modify underlying
    ///      * behavior of the socket connection. See the flags below.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long connectionFlags;`
    #[inline]
    pub unsafe fn SetConnectionFlags(&self, aConnectionFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetConnectionFlags)(self, aConnectionFlags)
    }


    /// ```text
    /// /**
    ///      * An opaque flags for non-standard behavior of the TLS system.
    ///      * It is unlikely this will need to be set outside of telemetry studies
    ///      * relating to the TLS implementation.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long tlsFlags;`
    #[inline]
    pub unsafe fn GetTlsFlags(&self, aTlsFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTlsFlags)(self, aTlsFlags)
    }


    /// ```text
    /// /**
    ///      * An opaque flags for non-standard behavior of the TLS system.
    ///      * It is unlikely this will need to be set outside of telemetry studies
    ///      * relating to the TLS implementation.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long tlsFlags;`
    #[inline]
    pub unsafe fn SetTlsFlags(&self, aTlsFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetTlsFlags)(self, aTlsFlags)
    }


    /// ```text
    /// /**
    ///      * Socket QoS/ToS markings. Valid values are IPTOS_DSCP_AFxx or
    ///      * IPTOS_CLASS_CSx (or IPTOS_DSCP_EF, but currently no supported
        ///      * services require expedited-forwarding).
    ///      * Not setting this value will leave the socket with the default
    ///      * ToS value, which on most systems if IPTOS_CLASS_CS0 (formerly
        ///      * IPTOS_PREC_ROUTINE).
    ///      */
    /// ```
    ///

    /// `attribute octet QoSBits;`
    #[inline]
    pub unsafe fn GetQoSBits(&self, aQoSBits: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetQoSBits)(self, aQoSBits)
    }


    /// ```text
    /// /**
    ///      * Socket QoS/ToS markings. Valid values are IPTOS_DSCP_AFxx or
    ///      * IPTOS_CLASS_CSx (or IPTOS_DSCP_EF, but currently no supported
        ///      * services require expedited-forwarding).
    ///      * Not setting this value will leave the socket with the default
    ///      * ToS value, which on most systems if IPTOS_CLASS_CS0 (formerly
        ///      * IPTOS_PREC_ROUTINE).
    ///      */
    /// ```
    ///

    /// `attribute octet QoSBits;`
    #[inline]
    pub unsafe fn SetQoSBits(&self, aQoSBits: u8) -> ::nserror::nsresult {
        ((*self.vtable).SetQoSBits)(self, aQoSBits)
    }


    /// ```text
    /// /**
    ///      * TCP send and receive buffer sizes. A value of 0 means OS level
    ///      * auto-tuning is in effect.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long recvBufferSize;`
    #[inline]
    pub unsafe fn GetRecvBufferSize(&self, aRecvBufferSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRecvBufferSize)(self, aRecvBufferSize)
    }


    /// ```text
    /// /**
    ///      * TCP send and receive buffer sizes. A value of 0 means OS level
    ///      * auto-tuning is in effect.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long recvBufferSize;`
    #[inline]
    pub unsafe fn SetRecvBufferSize(&self, aRecvBufferSize: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetRecvBufferSize)(self, aRecvBufferSize)
    }



    /// `attribute unsigned long sendBufferSize;`
    #[inline]
    pub unsafe fn GetSendBufferSize(&self, aSendBufferSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSendBufferSize)(self, aSendBufferSize)
    }



    /// `attribute unsigned long sendBufferSize;`
    #[inline]
    pub unsafe fn SetSendBufferSize(&self, aSendBufferSize: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetSendBufferSize)(self, aSendBufferSize)
    }


    /// ```text
    /// /**
    ///      * TCP keepalive configuration (support varies by platform).
    ///      * Note that the attribute as well as the setter can only accessed
    ///      * in the socket thread.
    ///      */
    /// ```
    ///

    /// `attribute boolean keepaliveEnabled;`
    #[inline]
    pub unsafe fn GetKeepaliveEnabled(&self, aKeepaliveEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetKeepaliveEnabled)(self, aKeepaliveEnabled)
    }


    /// ```text
    /// /**
    ///      * TCP keepalive configuration (support varies by platform).
    ///      * Note that the attribute as well as the setter can only accessed
    ///      * in the socket thread.
    ///      */
    /// ```
    ///

    /// `attribute boolean keepaliveEnabled;`
    #[inline]
    pub unsafe fn SetKeepaliveEnabled(&self, aKeepaliveEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetKeepaliveEnabled)(self, aKeepaliveEnabled)
    }



    /// `void setKeepaliveVals (in long keepaliveIdleTime, in long keepaliveRetryInterval);`
    #[inline]
    pub unsafe fn SetKeepaliveVals(&self, keepaliveIdleTime: i32, keepaliveRetryInterval: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetKeepaliveVals)(self, keepaliveIdleTime, keepaliveRetryInterval)
    }


    /// ```text
    /// /**
    ///      * If true, this socket transport has found out the prefered family
    ///      * according it's connection flags could not be used to establish
    ///      * connections any more.  Hence, the preference should be reset.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean resetIPFamilyPreference;`
    #[inline]
    pub unsafe fn GetResetIPFamilyPreference(&self, aResetIPFamilyPreference: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetResetIPFamilyPreference)(self, aResetIPFamilyPreference)
    }


    /// ```text
    /// /**
    ///      * This attribute holds information whether echConfig has been used.
    ///      * The value is set after PR_Connect is called.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean echConfigUsed;`
    #[inline]
    pub unsafe fn GetEchConfigUsed(&self, aEchConfigUsed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEchConfigUsed)(self, aEchConfigUsed)
    }


    /// ```text
    /// /**
    ///      * Called to set the echConfig to the securityInfo object.
    ///      */
    /// ```
    ///

    /// `void setEchConfig (in ACString echConfig);`
    #[inline]
    pub unsafe fn SetEchConfig(&self, echConfig: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetEchConfig)(self, echConfig)
    }


    /// ```text
    /// /**
    ///      * IP address resolved using TRR.
    ///      */
    /// ```
    ///

    /// `bool resolvedByTRR ();`
    #[inline]
    pub unsafe fn ResolvedByTRR(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ResolvedByTRR)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Indicate whether this socket is created from a private window. If yes,
    ///      * this socket will be closed when the last private window is closed.
    ///      */
    /// ```
    ///

    /// `[noscript] void setIsPrivate (in boolean isPrivate);`
    #[inline]
    pub unsafe fn SetIsPrivate(&self, isPrivate: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsPrivate)(self, isPrivate)
    }


    /// ```text
    /// /**
    ///      * If DNS is performed externally, this flag informs the caller that it may
    ///      * retry connecting with a different DNS configuration (e.g. different IP
        ///      * family preference). The flag is set only if a network error is encounder,
    ///      * e.g. NS_ERROR_CONNECTION_REFUSED, NS_ERROR_RESET, etc.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean retryDnsIfPossible;`
    #[inline]
    pub unsafe fn GetRetryDnsIfPossible(&self, aRetryDnsIfPossible: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRetryDnsIfPossible)(self, aRetryDnsIfPossible)
    }


    /// ```text
    /// /**
    ///      * Return the current status of the socket.
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute nsresult status;`
    #[inline]
    pub unsafe fn GetStatus(&self, aStatus: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetStatus)(self, aStatus)
    }


}


