//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISSLSocketControl.idl
//


/// `interface nsISSLSocketControl : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISSLSocketControl {
    vtable: *const nsISSLSocketControlVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISSLSocketControl.
unsafe impl XpCom for nsISSLSocketControl {
    const IID: nsIID = nsID(0x418265c8, 0x654e, 0x4fbb,
        [0xba, 0x62, 0x4e, 0xed, 0x27, 0xde, 0x1f, 0x03]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISSLSocketControl {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISSLSocketControl.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISSLSocketControlCoerce {
    /// Cheaply cast a value of this type from a `nsISSLSocketControl`.
    fn coerce_from(v: &nsISSLSocketControl) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISSLSocketControlCoerce for nsISSLSocketControl {
    #[inline]
    fn coerce_from(v: &nsISSLSocketControl) -> &Self {
        v
    }
}

impl nsISSLSocketControl {
    /// Cast this `nsISSLSocketControl` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISSLSocketControlCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISSLSocketControl {
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
impl<T: nsISupportsCoerce> nsISSLSocketControlCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISSLSocketControl) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISSLSocketControl
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISSLSocketControlVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub GetNotificationCallbacks: unsafe extern "system" fn (this: *const nsISSLSocketControl, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub SetNotificationCallbacks: unsafe extern "system" fn (this: *const nsISSLSocketControl, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* void proxyStartSSL (); */
    pub ProxyStartSSL: unsafe extern "system" fn (this: *const nsISSLSocketControl) -> ::nserror::nsresult,

    /* void StartTLS (); */
    pub StartTLS: unsafe extern "system" fn (this: *const nsISSLSocketControl) -> ::nserror::nsresult,

    /* [noscript] void setNPNList (in nsCStringTArrayRef aNPNList); */
    /// Unable to generate binding because `native type nsTArray<nsCString> unsupported`
    pub SetNPNList: *const ::libc::c_void,

    /* ACString getAlpnEarlySelection (); */
    pub GetAlpnEarlySelection: unsafe extern "system" fn (this: *const nsISSLSocketControl, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute bool earlyDataAccepted; */
    pub GetEarlyDataAccepted: unsafe extern "system" fn (this: *const nsISSLSocketControl, aEarlyDataAccepted: *mut bool) -> ::nserror::nsresult,

    /* void driveHandshake (); */
    pub DriveHandshake: unsafe extern "system" fn (this: *const nsISSLSocketControl) -> ::nserror::nsresult,

    /* boolean joinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
    pub JoinConnection: unsafe extern "system" fn (this: *const nsISSLSocketControl, npnProtocol: *const ::nsstring::nsACString, hostname: *const ::nsstring::nsACString, port: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean testJoinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
    pub TestJoinConnection: unsafe extern "system" fn (this: *const nsISSLSocketControl, npnProtocol: *const ::nsstring::nsACString, hostname: *const ::nsstring::nsACString, port: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isAcceptableForHost (in ACString hostname); */
    pub IsAcceptableForHost: unsafe extern "system" fn (this: *const nsISSLSocketControl, hostname: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute short KEAUsed; */
    pub GetKEAUsed: unsafe extern "system" fn (this: *const nsISSLSocketControl, aKEAUsed: *mut i16) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long KEAKeyBits; */
    pub GetKEAKeyBits: unsafe extern "system" fn (this: *const nsISSLSocketControl, aKEAKeyBits: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute uint32_t providerFlags; */
    pub GetProviderFlags: unsafe extern "system" fn (this: *const nsISSLSocketControl, aProviderFlags: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t providerTlsFlags; */
    pub GetProviderTlsFlags: unsafe extern "system" fn (this: *const nsISSLSocketControl, aProviderTlsFlags: *mut uint32_t) -> ::nserror::nsresult,

    /* [infallible] readonly attribute short SSLVersionUsed; */
    pub GetSSLVersionUsed: unsafe extern "system" fn (this: *const nsISSLSocketControl, aSSLVersionUsed: *mut i16) -> ::nserror::nsresult,

    /* [infallible] readonly attribute short SSLVersionOffered; */
    pub GetSSLVersionOffered: unsafe extern "system" fn (this: *const nsISSLSocketControl, aSSLVersionOffered: *mut i16) -> ::nserror::nsresult,

    /* [infallible] readonly attribute short MACAlgorithmUsed; */
    pub GetMACAlgorithmUsed: unsafe extern "system" fn (this: *const nsISSLSocketControl, aMACAlgorithmUsed: *mut i16) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] attribute boolean denyClientCert; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetDenyClientCert: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute boolean denyClientCert; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetDenyClientCert: *const ::libc::c_void,

    /* attribute nsIX509Cert clientCert; */
    pub GetClientCert: unsafe extern "system" fn (this: *const nsISSLSocketControl, aClientCert: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* attribute nsIX509Cert clientCert; */
    pub SetClientCert: unsafe extern "system" fn (this: *const nsISSLSocketControl, aClientCert: *const nsIX509Cert) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean clientCertSent; */
    pub GetClientCertSent: unsafe extern "system" fn (this: *const nsISSLSocketControl, aClientCertSent: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean failedVerification; */
    pub GetFailedVerification: unsafe extern "system" fn (this: *const nsISSLSocketControl, aFailedVerification: *mut bool) -> ::nserror::nsresult,

    /* attribute ACString esniTxt; */
    pub GetEsniTxt: unsafe extern "system" fn (this: *const nsISSLSocketControl, aEsniTxt: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString esniTxt; */
    pub SetEsniTxt: unsafe extern "system" fn (this: *const nsISSLSocketControl, aEsniTxt: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString echConfig; */
    pub GetEchConfig: unsafe extern "system" fn (this: *const nsISSLSocketControl, aEchConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString echConfig; */
    pub SetEchConfig: unsafe extern "system" fn (this: *const nsISSLSocketControl, aEchConfig: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString peerId; */
    pub GetPeerId: unsafe extern "system" fn (this: *const nsISSLSocketControl, aPeerId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString retryEchConfig; */
    pub GetRetryEchConfig: unsafe extern "system" fn (this: *const nsISSLSocketControl, aRetryEchConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISSLSocketControl {

    pub const KEY_EXCHANGE_UNKNOWN: i64 = -1;


    pub const SSL_VERSION_3: i64 = 768;


    pub const TLS_VERSION_1: i64 = 769;


    pub const TLS_VERSION_1_1: i64 = 770;


    pub const TLS_VERSION_1_2: i64 = 771;


    pub const TLS_VERSION_1_3: i64 = 772;


    pub const SSL_VERSION_UNKNOWN: i64 = -1;


    pub const SSL_MAC_UNKNOWN: i64 = -1;


    pub const SSL_MAC_NULL: i64 = 0;


    pub const SSL_MAC_MD5: i64 = 1;


    pub const SSL_MAC_SHA: i64 = 2;


    pub const SSL_HMAC_MD5: i64 = 3;


    pub const SSL_HMAC_SHA: i64 = 4;


    pub const SSL_HMAC_SHA256: i64 = 5;


    pub const SSL_MAC_AEAD: i64 = 6;


    /// `attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn GetNotificationCallbacks(&self, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).GetNotificationCallbacks)(self, aNotificationCallbacks)
    }



    /// `attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn SetNotificationCallbacks(&self, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).SetNotificationCallbacks)(self, aNotificationCallbacks)
    }



    /// `void proxyStartSSL ();`
    #[inline]
    pub unsafe fn ProxyStartSSL(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ProxyStartSSL)(self, )
    }



    /// `void StartTLS ();`
    #[inline]
    pub unsafe fn StartTLS(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StartTLS)(self, )
    }



    /// `[noscript] void setNPNList (in nsCStringTArrayRef aNPNList);`
    const _SetNPNList: () = ();


    /// `ACString getAlpnEarlySelection ();`
    #[inline]
    pub unsafe fn GetAlpnEarlySelection(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAlpnEarlySelection)(self, _retval)
    }



    /// `readonly attribute bool earlyDataAccepted;`
    #[inline]
    pub unsafe fn GetEarlyDataAccepted(&self, aEarlyDataAccepted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEarlyDataAccepted)(self, aEarlyDataAccepted)
    }



    /// `void driveHandshake ();`
    #[inline]
    pub unsafe fn DriveHandshake(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DriveHandshake)(self, )
    }



    /// `boolean joinConnection (in ACString npnProtocol, in ACString hostname, in long port);`
    #[inline]
    pub unsafe fn JoinConnection(&self, npnProtocol: *const ::nsstring::nsACString, hostname: *const ::nsstring::nsACString, port: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).JoinConnection)(self, npnProtocol, hostname, port, _retval)
    }



    /// `boolean testJoinConnection (in ACString npnProtocol, in ACString hostname, in long port);`
    #[inline]
    pub unsafe fn TestJoinConnection(&self, npnProtocol: *const ::nsstring::nsACString, hostname: *const ::nsstring::nsACString, port: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).TestJoinConnection)(self, npnProtocol, hostname, port, _retval)
    }



    /// `boolean isAcceptableForHost (in ACString hostname);`
    #[inline]
    pub unsafe fn IsAcceptableForHost(&self, hostname: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsAcceptableForHost)(self, hostname, _retval)
    }



    /// `[infallible] readonly attribute short KEAUsed;`
    #[inline]
    pub unsafe fn GetKEAUsed(&self) -> i16 {
        let mut result = <i16 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetKEAUsed)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute unsigned long KEAKeyBits;`
    #[inline]
    pub unsafe fn GetKEAKeyBits(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetKEAKeyBits)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `readonly attribute uint32_t providerFlags;`
    #[inline]
    pub unsafe fn GetProviderFlags(&self, aProviderFlags: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetProviderFlags)(self, aProviderFlags)
    }



    /// `readonly attribute uint32_t providerTlsFlags;`
    #[inline]
    pub unsafe fn GetProviderTlsFlags(&self, aProviderTlsFlags: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetProviderTlsFlags)(self, aProviderTlsFlags)
    }



    /// `[infallible] readonly attribute short SSLVersionUsed;`
    #[inline]
    pub unsafe fn GetSSLVersionUsed(&self) -> i16 {
        let mut result = <i16 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetSSLVersionUsed)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute short SSLVersionOffered;`
    #[inline]
    pub unsafe fn GetSSLVersionOffered(&self) -> i16 {
        let mut result = <i16 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetSSLVersionOffered)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute short MACAlgorithmUsed;`
    #[inline]
    pub unsafe fn GetMACAlgorithmUsed(&self) -> i16 {
        let mut result = <i16 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetMACAlgorithmUsed)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * If set to true before the server requests a client cert
    ///      * no cert will be sent.
    ///      */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute boolean denyClientCert;`
    const _GetDenyClientCert: () = ();

    /// ```text
    /// /**
    ///      * If set to true before the server requests a client cert
    ///      * no cert will be sent.
    ///      */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute boolean denyClientCert;`
    const _SetDenyClientCert: () = ();

    /// ```text
    /// /**
    ///      * If set before the server requests a client cert (assuming it does so at
        ///      * all), then this cert will be presented to the server, instead of asking
    ///      * the user or searching the set of rememebered user cert decisions.
    ///      */
    /// ```
    ///

    /// `attribute nsIX509Cert clientCert;`
    #[inline]
    pub unsafe fn GetClientCert(&self, aClientCert: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).GetClientCert)(self, aClientCert)
    }


    /// ```text
    /// /**
    ///      * If set before the server requests a client cert (assuming it does so at
        ///      * all), then this cert will be presented to the server, instead of asking
    ///      * the user or searching the set of rememebered user cert decisions.
    ///      */
    /// ```
    ///

    /// `attribute nsIX509Cert clientCert;`
    #[inline]
    pub unsafe fn SetClientCert(&self, aClientCert: *const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).SetClientCert)(self, aClientCert)
    }


    /// ```text
    /// /**
    ///      * True iff a client cert has been sent to the server - i.e. this
    ///      * socket has been client-cert authenticated.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean clientCertSent;`
    #[inline]
    pub unsafe fn GetClientCertSent(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetClientCertSent)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean failedVerification;`
    #[inline]
    pub unsafe fn GetFailedVerification(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetFailedVerification)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `attribute ACString esniTxt;`
    #[inline]
    pub unsafe fn GetEsniTxt(&self, aEsniTxt: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEsniTxt)(self, aEsniTxt)
    }



    /// `attribute ACString esniTxt;`
    #[inline]
    pub unsafe fn SetEsniTxt(&self, aEsniTxt: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetEsniTxt)(self, aEsniTxt)
    }



    /// `attribute ACString echConfig;`
    #[inline]
    pub unsafe fn GetEchConfig(&self, aEchConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEchConfig)(self, aEchConfig)
    }



    /// `attribute ACString echConfig;`
    #[inline]
    pub unsafe fn SetEchConfig(&self, aEchConfig: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetEchConfig)(self, aEchConfig)
    }


    /// ```text
    /// /**
    ///      * The id used to uniquely identify the connection to the peer.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString peerId;`
    #[inline]
    pub unsafe fn GetPeerId(&self, aPeerId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPeerId)(self, aPeerId)
    }


    /// ```text
    /// /**
    ///      * The echConfig that should be used to retry for the connection setup.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString retryEchConfig;`
    #[inline]
    pub unsafe fn GetRetryEchConfig(&self, aRetryEchConfig: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRetryEchConfig)(self, aRetryEchConfig)
    }


}


