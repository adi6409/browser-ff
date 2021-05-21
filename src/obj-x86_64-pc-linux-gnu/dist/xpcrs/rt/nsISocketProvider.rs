//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISocketProvider.idl
//


/// `interface nsISocketProvider : nsISupports`
///

/// ```text
/// /**
///  * nsISocketProvider
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISocketProvider {
    vtable: *const nsISocketProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISocketProvider.
unsafe impl XpCom for nsISocketProvider {
    const IID: nsIID = nsID(0x508d5469, 0x9e1e, 0x4a08,
        [0xb5, 0xb0, 0x7c, 0xfe, 0xbb, 0xa1, 0xe5, 0x1a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISocketProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISocketProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISocketProviderCoerce {
    /// Cheaply cast a value of this type from a `nsISocketProvider`.
    fn coerce_from(v: &nsISocketProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISocketProviderCoerce for nsISocketProvider {
    #[inline]
    fn coerce_from(v: &nsISocketProvider) -> &Self {
        v
    }
}

impl nsISocketProvider {
    /// Cast this `nsISocketProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISocketProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISocketProvider {
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
impl<T: nsISupportsCoerce> nsISocketProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISocketProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISocketProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void newSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in unsigned long aTlsFlags, out PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub NewSocket: *const ::libc::c_void,

    /* [noscript] void addToSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in unsigned long aTlsFlags, in PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub AddToSocket: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISocketProvider {
    /// ```text
    /// /**
    ///      * PROXY_RESOLVES_HOST
    ///      *
    ///      * This flag is set if the proxy is to perform hostname resolution instead
    ///      * of the client.  When set, the hostname parameter passed when in this
    ///      * interface will be used instead of the address structure passed for a
    ///      * later connect et al. request.
    ///      */
    /// ```
    ///

    pub const PROXY_RESOLVES_HOST: i64 = 1;

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
    ///      * If set, indicates that the connection was initiated from a source
    ///      * defined as being private in the sense of Private Browsing. Generally,
    ///      * there should be no state shared between connections that are private
    ///      * and those that are not; it is OK for multiple private connections
    ///      * to share state with each other, and it is OK for multiple non-private
    ///      * connections to share state with each other.
    ///      */
    /// ```
    ///

    pub const NO_PERMANENT_STORAGE: i64 = 4;

    /// ```text
    /// /**
    ///      * If set, do not use newer protocol features that might have interop problems
    ///      * on the Internet. Intended only for use with critical infra like the updater.
    ///      * default is false.
    ///      */
    /// ```
    ///

    pub const BE_CONSERVATIVE: i64 = 8;

    /// ```text
    /// /**
    ///      * This is used for a temporary workaround for a web-compat issue. The flag is
    ///      * only set on CORS preflight request to allowed sending client certificates
    ///      * on a connection for an anonymous request.
    ///      */
    /// ```
    ///

    pub const ANONYMOUS_CONNECT_ALLOW_CLIENT_CERT: i64 = 16;

    /// ```text
    /// /**
    ///      * newSocket
    ///      *
    ///      * @param aFamily
    ///      *        The address family for this socket (PR_AF_INET or PR_AF_INET6).
    ///      * @param aHost
    ///      *        The origin hostname for this connection.
    ///      * @param aPort
    ///      *        The origin port for this connection.
    ///      * @param aProxyHost
    ///      *        If non-null, the proxy hostname for this connection.
    ///      * @param aProxyPort
    ///      *        The proxy port for this connection.
    ///      * @param aFlags
    ///      *        Control flags that govern this connection (see below.)
    ///      * @param aTlsFlags
    ///      *        An opaque flags for non-standard behavior of the TLS system.
    ///      *        It is unlikely this will need to be set outside of telemetry
    ///      *        studies relating to the TLS implementation.
    ///      * @param aFileDesc
    ///      *        The resulting PRFileDesc.
    ///      * @param aSecurityInfo
    ///      *        Any security info that should be associated with aFileDesc.  This
    ///      *        object typically implements nsITransportSecurityInfo.
    ///      */
    /// ```
    ///

    /// `[noscript] void newSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in unsigned long aTlsFlags, out PRFileDescStar aFileDesc, out nsISupports aSecurityInfo);`
    const _NewSocket: () = ();

    /// ```text
    /// /**
    ///      * addToSocket
    ///      *
    ///      * This function is called to allow the socket provider to layer a
    ///      * PRFileDesc on top of another PRFileDesc.  For example, SSL via a SOCKS
    ///      * proxy.
    ///      *
    ///      * Parameters are the same as newSocket with the exception of aFileDesc,
    ///      * which is an in-param instead.
    ///      */
    /// ```
    ///

    /// `[noscript] void addToSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in unsigned long aTlsFlags, in PRFileDescStar aFileDesc, out nsISupports aSecurityInfo);`
    const _AddToSocket: () = ();

}


