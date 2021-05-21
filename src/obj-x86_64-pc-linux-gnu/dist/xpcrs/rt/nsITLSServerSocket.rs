//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITLSServerSocket.idl
//


/// `interface nsITLSServerSocket : nsIServerSocket`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITLSServerSocket {
    vtable: *const nsITLSServerSocketVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITLSServerSocket.
unsafe impl XpCom for nsITLSServerSocket {
    const IID: nsIID = nsID(0xcc2c30f9, 0xcfaa, 0x4b8a,
        [0xbd, 0x44, 0xc2, 0x48, 0x81, 0x98, 0x1b, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITLSServerSocket {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITLSServerSocket.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITLSServerSocketCoerce {
    /// Cheaply cast a value of this type from a `nsITLSServerSocket`.
    fn coerce_from(v: &nsITLSServerSocket) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITLSServerSocketCoerce for nsITLSServerSocket {
    #[inline]
    fn coerce_from(v: &nsITLSServerSocket) -> &Self {
        v
    }
}

impl nsITLSServerSocket {
    /// Cast this `nsITLSServerSocket` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITLSServerSocketCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITLSServerSocket {
    type Target = nsIServerSocket;
    #[inline]
    fn deref(&self) -> &nsIServerSocket {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIServerSocketCoerce> nsITLSServerSocketCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSServerSocket) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITLSServerSocket
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITLSServerSocketVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIServerSocketVTable,

    /* attribute nsIX509Cert serverCert; */
    pub GetServerCert: unsafe extern "system" fn (this: *const nsITLSServerSocket, aServerCert: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* attribute nsIX509Cert serverCert; */
    pub SetServerCert: unsafe extern "system" fn (this: *const nsITLSServerSocket, aServerCert: *const nsIX509Cert) -> ::nserror::nsresult,

    /* void setSessionTickets (in boolean aSessionTickets); */
    pub SetSessionTickets: unsafe extern "system" fn (this: *const nsITLSServerSocket, aSessionTickets: bool) -> ::nserror::nsresult,

    /* void setRequestClientCertificate (in unsigned long aRequestClientCert); */
    pub SetRequestClientCertificate: unsafe extern "system" fn (this: *const nsITLSServerSocket, aRequestClientCert: u32) -> ::nserror::nsresult,

    /* void setVersionRange (in unsigned short aMinVersion, in unsigned short aMaxVersion); */
    pub SetVersionRange: unsafe extern "system" fn (this: *const nsITLSServerSocket, aMinVersion: u16, aMaxVersion: u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITLSServerSocket {
    /// ```text
    /// /**
    ///    * Values for setRequestClientCertificate
    ///    */
    /// ```
    ///

    pub const REQUEST_NEVER: i64 = 0;


    pub const REQUEST_FIRST_HANDSHAKE: i64 = 1;


    pub const REQUEST_ALWAYS: i64 = 2;


    pub const REQUIRE_FIRST_HANDSHAKE: i64 = 3;


    pub const REQUIRE_ALWAYS: i64 = 4;

    /// ```text
    /// /**
    ///    * serverCert
    ///    *
    ///    * The server's certificate that is presented to the client during the TLS
    ///    * handshake.  This is required to be set before calling |asyncListen|.
    ///    */
    /// ```
    ///

    /// `attribute nsIX509Cert serverCert;`
    #[inline]
    pub unsafe fn GetServerCert(&self, aServerCert: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).GetServerCert)(self, aServerCert)
    }


    /// ```text
    /// /**
    ///    * serverCert
    ///    *
    ///    * The server's certificate that is presented to the client during the TLS
    ///    * handshake.  This is required to be set before calling |asyncListen|.
    ///    */
    /// ```
    ///

    /// `attribute nsIX509Cert serverCert;`
    #[inline]
    pub unsafe fn SetServerCert(&self, aServerCert: *const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).SetServerCert)(self, aServerCert)
    }


    /// ```text
    /// /**
    ///    * setSessionTickets
    ///    *
    ///    * Whether the server should support session tickets.  Defaults to true.  This
    ///    * should be set before calling |asyncListen| if you wish to change the
    ///    * default.
    ///    */
    /// ```
    ///

    /// `void setSessionTickets (in boolean aSessionTickets);`
    #[inline]
    pub unsafe fn SetSessionTickets(&self, aSessionTickets: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSessionTickets)(self, aSessionTickets)
    }


    /// ```text
    /// /**
    ///    * setRequestClientCertificate
    ///    *
    ///    * Whether the server should request and/or require a client auth certificate
    ///    * from the client.  Defaults to REQUEST_NEVER.  See the possible options
    ///    * above.  This should be set before calling |asyncListen| if you wish to
    ///    * change the default.
    ///    */
    /// ```
    ///

    /// `void setRequestClientCertificate (in unsigned long aRequestClientCert);`
    #[inline]
    pub unsafe fn SetRequestClientCertificate(&self, aRequestClientCert: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestClientCertificate)(self, aRequestClientCert)
    }


    /// ```text
    /// /**
    ///    * setVersionRange
    ///    *
    ///    * The server's TLS versions that is used by the TLS handshake.
    ///    * This is required to be set before calling |asyncListen|.
    ///    *
    ///    * aMinVersion and aMaxVersion is a TLS version value from
    ///    * |nsITLSClientStatus| constants.
    ///    */
    /// ```
    ///

    /// `void setVersionRange (in unsigned short aMinVersion, in unsigned short aMaxVersion);`
    #[inline]
    pub unsafe fn SetVersionRange(&self, aMinVersion: u16, aMaxVersion: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetVersionRange)(self, aMinVersion, aMaxVersion)
    }


}


/// `interface nsITLSClientStatus : nsISupports`
///

/// ```text
/// /**
///  * Security summary for a given TLS client connection being handled by a
///  * |nsITLSServerSocket| server.
///  *
///  * This is accessible through the security info object on the transport, which
///  * will be an instance of |nsITLSServerConnectionInfo| (see below).
///  *
///  * The values of these attributes are available once the |onHandshakeDone|
///  * method of the security observer has been called (see
    ///  * |nsITLSServerSecurityObserver| below).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITLSClientStatus {
    vtable: *const nsITLSClientStatusVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITLSClientStatus.
unsafe impl XpCom for nsITLSClientStatus {
    const IID: nsIID = nsID(0x19668ea4, 0xe5ad, 0x4182,
        [0x96, 0x98, 0x7e, 0x89, 0x0d, 0x48, 0xf3, 0x27]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITLSClientStatus {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITLSClientStatus.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITLSClientStatusCoerce {
    /// Cheaply cast a value of this type from a `nsITLSClientStatus`.
    fn coerce_from(v: &nsITLSClientStatus) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITLSClientStatusCoerce for nsITLSClientStatus {
    #[inline]
    fn coerce_from(v: &nsITLSClientStatus) -> &Self {
        v
    }
}

impl nsITLSClientStatus {
    /// Cast this `nsITLSClientStatus` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITLSClientStatusCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITLSClientStatus {
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
impl<T: nsISupportsCoerce> nsITLSClientStatusCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSClientStatus) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITLSClientStatus
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITLSClientStatusVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIX509Cert peerCert; */
    pub GetPeerCert: unsafe extern "system" fn (this: *const nsITLSClientStatus, aPeerCert: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* readonly attribute short tlsVersionUsed; */
    pub GetTlsVersionUsed: unsafe extern "system" fn (this: *const nsITLSClientStatus, aTlsVersionUsed: *mut i16) -> ::nserror::nsresult,

    /* readonly attribute ACString cipherName; */
    pub GetCipherName: unsafe extern "system" fn (this: *const nsITLSClientStatus, aCipherName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long keyLength; */
    pub GetKeyLength: unsafe extern "system" fn (this: *const nsITLSClientStatus, aKeyLength: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long macLength; */
    pub GetMacLength: unsafe extern "system" fn (this: *const nsITLSClientStatus, aMacLength: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITLSClientStatus {
    /// ```text
    /// /**
    ///    * Values for tlsVersionUsed, as defined by TLS
    ///    */
    /// ```
    ///

    pub const SSL_VERSION_3: i64 = 768;


    pub const TLS_VERSION_1: i64 = 769;


    pub const TLS_VERSION_1_1: i64 = 770;


    pub const TLS_VERSION_1_2: i64 = 771;


    pub const TLS_VERSION_1_3: i64 = 772;


    pub const TLS_VERSION_UNKNOWN: i64 = -1;

    /// ```text
    /// /**
    ///    * peerCert
    ///    *
    ///    * The client's certificate, if one was requested via |requestCertificate|
    ///    * above and supplied by the client.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIX509Cert peerCert;`
    #[inline]
    pub unsafe fn GetPeerCert(&self, aPeerCert: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).GetPeerCert)(self, aPeerCert)
    }


    /// ```text
    /// /**
    ///    * tlsVersionUsed
    ///    *
    ///    * The version of TLS used by the connection.  See values above.
    ///    */
    /// ```
    ///

    /// `readonly attribute short tlsVersionUsed;`
    #[inline]
    pub unsafe fn GetTlsVersionUsed(&self, aTlsVersionUsed: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetTlsVersionUsed)(self, aTlsVersionUsed)
    }


    /// ```text
    /// /**
    ///    * cipherName
    ///    *
    ///    * Name of the cipher suite used, such as
    ///    * "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256".
    ///    * See security/nss/lib/ssl/sslinfo.c for the possible values.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString cipherName;`
    #[inline]
    pub unsafe fn GetCipherName(&self, aCipherName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCipherName)(self, aCipherName)
    }


    /// ```text
    /// /**
    ///    * keyLength
    ///    *
    ///    * The "effective" key size of the symmetric key in bits.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long keyLength;`
    #[inline]
    pub unsafe fn GetKeyLength(&self, aKeyLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetKeyLength)(self, aKeyLength)
    }


    /// ```text
    /// /**
    ///    * macLength
    ///    *
    ///    * The size of the MAC in bits.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long macLength;`
    #[inline]
    pub unsafe fn GetMacLength(&self, aMacLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMacLength)(self, aMacLength)
    }


}


/// `interface nsITLSServerConnectionInfo : nsISupports`
///

/// ```text
/// /**
///  * Connection info for a given TLS client connection being handled by a
///  * |nsITLSServerSocket| server.  This object is thread-safe.
///  *
///  * This is exposed as the security info object on the transport, so it can be
///  * accessed via |transport.securityInfo|.
///  *
///  * This interface is available by the time the |onSocketAttached| is called,
///  * which is the first time the TLS server consumer is notified of a new client.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITLSServerConnectionInfo {
    vtable: *const nsITLSServerConnectionInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITLSServerConnectionInfo.
unsafe impl XpCom for nsITLSServerConnectionInfo {
    const IID: nsIID = nsID(0x8a93f5d5, 0xeddd, 0x4c62,
        [0xa4, 0xbd, 0xbf, 0xd2, 0x97, 0x65, 0x31, 0x84]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITLSServerConnectionInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITLSServerConnectionInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITLSServerConnectionInfoCoerce {
    /// Cheaply cast a value of this type from a `nsITLSServerConnectionInfo`.
    fn coerce_from(v: &nsITLSServerConnectionInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITLSServerConnectionInfoCoerce for nsITLSServerConnectionInfo {
    #[inline]
    fn coerce_from(v: &nsITLSServerConnectionInfo) -> &Self {
        v
    }
}

impl nsITLSServerConnectionInfo {
    /// Cast this `nsITLSServerConnectionInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITLSServerConnectionInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITLSServerConnectionInfo {
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
impl<T: nsISupportsCoerce> nsITLSServerConnectionInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSServerConnectionInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITLSServerConnectionInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITLSServerConnectionInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setSecurityObserver (in nsITLSServerSecurityObserver observer); */
    pub SetSecurityObserver: unsafe extern "system" fn (this: *const nsITLSServerConnectionInfo, observer: *const nsITLSServerSecurityObserver) -> ::nserror::nsresult,

    /* readonly attribute nsITLSServerSocket serverSocket; */
    pub GetServerSocket: unsafe extern "system" fn (this: *const nsITLSServerConnectionInfo, aServerSocket: *mut *const nsITLSServerSocket) -> ::nserror::nsresult,

    /* readonly attribute nsITLSClientStatus status; */
    pub GetStatus: unsafe extern "system" fn (this: *const nsITLSServerConnectionInfo, aStatus: *mut *const nsITLSClientStatus) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITLSServerConnectionInfo {

    /// ```text
    /// /**
    ///    * setSecurityObserver
    ///    *
    ///    * Set the security observer to be notified when the TLS handshake has
    ///    * completed.
    ///    */
    /// ```
    ///

    /// `void setSecurityObserver (in nsITLSServerSecurityObserver observer);`
    #[inline]
    pub unsafe fn SetSecurityObserver(&self, observer: *const nsITLSServerSecurityObserver) -> ::nserror::nsresult {
        ((*self.vtable).SetSecurityObserver)(self, observer)
    }


    /// ```text
    /// /**
    ///    * serverSocket
    ///    *
    ///    * The nsITLSServerSocket instance that accepted this client connection.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsITLSServerSocket serverSocket;`
    #[inline]
    pub unsafe fn GetServerSocket(&self, aServerSocket: *mut *const nsITLSServerSocket) -> ::nserror::nsresult {
        ((*self.vtable).GetServerSocket)(self, aServerSocket)
    }


    /// ```text
    /// /**
    ///    * status
    ///    *
    ///    * Security summary for this TLS client connection.  Note that the values of
    ///    * this interface are not available until the TLS handshake has completed.
    ///    * See |nsITLSClientStatus| above for more details.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsITLSClientStatus status;`
    #[inline]
    pub unsafe fn GetStatus(&self, aStatus: *mut *const nsITLSClientStatus) -> ::nserror::nsresult {
        ((*self.vtable).GetStatus)(self, aStatus)
    }


}


/// `interface nsITLSServerSecurityObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITLSServerSecurityObserver {
    vtable: *const nsITLSServerSecurityObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITLSServerSecurityObserver.
unsafe impl XpCom for nsITLSServerSecurityObserver {
    const IID: nsIID = nsID(0x1f62e1ae, 0xe546, 0x4a38,
        [0x89, 0x17, 0xd4, 0x28, 0x47, 0x2e, 0xd7, 0x36]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITLSServerSecurityObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITLSServerSecurityObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITLSServerSecurityObserverCoerce {
    /// Cheaply cast a value of this type from a `nsITLSServerSecurityObserver`.
    fn coerce_from(v: &nsITLSServerSecurityObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITLSServerSecurityObserverCoerce for nsITLSServerSecurityObserver {
    #[inline]
    fn coerce_from(v: &nsITLSServerSecurityObserver) -> &Self {
        v
    }
}

impl nsITLSServerSecurityObserver {
    /// Cast this `nsITLSServerSecurityObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITLSServerSecurityObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITLSServerSecurityObserver {
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
impl<T: nsISupportsCoerce> nsITLSServerSecurityObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSServerSecurityObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITLSServerSecurityObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITLSServerSecurityObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus); */
    pub OnHandshakeDone: unsafe extern "system" fn (this: *const nsITLSServerSecurityObserver, aServer: *const nsITLSServerSocket, aStatus: *const nsITLSClientStatus) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITLSServerSecurityObserver {

    /// ```text
    /// /**
    ///    * onHandsakeDone
    ///    *
    ///    * This method is called once the TLS handshake is completed.  This takes
    ///    * place after |onSocketAccepted| has been called, which typically opens the
    ///    * streams to keep things moving along. It's important to be aware that the
    ///    * handshake has not completed at the point that |onSocketAccepted| is called,
    ///    * so no security verification can be done until this method is called.
    ///    */
    /// ```
    ///

    /// `void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus);`
    #[inline]
    pub unsafe fn OnHandshakeDone(&self, aServer: *const nsITLSServerSocket, aStatus: *const nsITLSClientStatus) -> ::nserror::nsresult {
        ((*self.vtable).OnHandshakeDone)(self, aServer, aStatus)
    }


}


