//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketTransportService.idl
//


/// `interface nsISTSShutdownObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISTSShutdownObserver {
    vtable: *const nsISTSShutdownObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISTSShutdownObserver.
unsafe impl XpCom for nsISTSShutdownObserver {
    const IID: nsIID = nsID(0x338947df, 0x2f3b, 0x4d24,
        [0x9c, 0xe4, 0xec, 0xf1, 0x61, 0xc1, 0xb7, 0xdf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISTSShutdownObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISTSShutdownObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISTSShutdownObserverCoerce {
    /// Cheaply cast a value of this type from a `nsISTSShutdownObserver`.
    fn coerce_from(v: &nsISTSShutdownObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISTSShutdownObserverCoerce for nsISTSShutdownObserver {
    #[inline]
    fn coerce_from(v: &nsISTSShutdownObserver) -> &Self {
        v
    }
}

impl nsISTSShutdownObserver {
    /// Cast this `nsISTSShutdownObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISTSShutdownObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISTSShutdownObserver {
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
impl<T: nsISupportsCoerce> nsISTSShutdownObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISTSShutdownObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISTSShutdownObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISTSShutdownObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void observe (); */
    pub Observe: unsafe extern "system" fn (this: *const nsISTSShutdownObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISTSShutdownObserver {

    /// ```text
    /// /**
    ///     * Observe will be called when the SocketTransportService is shutting down,
    ///     * before threads are stopped.
    ///     */
    /// ```
    ///

    /// `void observe ();`
    #[inline]
    pub unsafe fn Observe(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Observe)(self, )
    }


}


/// `interface nsISocketTransportService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISocketTransportService {
    vtable: *const nsISocketTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISocketTransportService.
unsafe impl XpCom for nsISocketTransportService {
    const IID: nsIID = nsID(0xad56b25f, 0xe6bb, 0x4db3,
        [0x9f, 0x7b, 0x5b, 0x7d, 0xb3, 0x3f, 0xd2, 0xb1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISocketTransportService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISocketTransportService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISocketTransportServiceCoerce {
    /// Cheaply cast a value of this type from a `nsISocketTransportService`.
    fn coerce_from(v: &nsISocketTransportService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISocketTransportServiceCoerce for nsISocketTransportService {
    #[inline]
    fn coerce_from(v: &nsISocketTransportService) -> &Self {
        v
    }
}

impl nsISocketTransportService {
    /// Cast this `nsISocketTransportService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISocketTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISocketTransportService {
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
impl<T: nsISupportsCoerce> nsISocketTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketTransportService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISocketTransportService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISocketTransportServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISocketTransport createTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in nsIProxyInfo aProxyInfo, in nsIDNSRecord dnsRecord); */
    pub CreateTransport: unsafe extern "system" fn (this: *const nsISocketTransportService, aSocketTypes: *const thin_vec::ThinVec<::nsstring::nsCString>, aHost: *const ::nsstring::nsACString, aPort: i32, aProxyInfo: *const nsIProxyInfo, dnsRecord: *const nsIDNSRecord, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult,

    /* nsISocketTransport createUnixDomainTransport (in nsIFile aPath); */
    pub CreateUnixDomainTransport: unsafe extern "system" fn (this: *const nsISocketTransportService, aPath: *const nsIFile, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult,

    /* nsISocketTransport createUnixDomainAbstractAddressTransport (in ACString aName); */
    pub CreateUnixDomainAbstractAddressTransport: unsafe extern "system" fn (this: *const nsISocketTransportService, aName: *const ::nsstring::nsACString, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult,

    /* [noscript] void attachSocket (in PRFileDescPtr aFd, in nsASocketHandlerPtr aHandler); */
    /// Unable to generate binding because `native type PRFileDesc unsupported`
    pub AttachSocket: *const ::libc::c_void,

    /* [noscript] void notifyWhenCanAttachSocket (in nsIRunnable aEvent); */
    pub NotifyWhenCanAttachSocket: unsafe extern "system" fn (this: *const nsISocketTransportService, aEvent: *const nsIRunnable) -> ::nserror::nsresult,

    /* [noscript] void addShutdownObserver (in nsISTSShutdownObserver aObserver); */
    pub AddShutdownObserver: unsafe extern "system" fn (this: *const nsISocketTransportService, aObserver: *const nsISTSShutdownObserver) -> ::nserror::nsresult,

    /* [noscript] void removeShutdownObserver (in nsISTSShutdownObserver aObserver); */
    pub RemoveShutdownObserver: unsafe extern "system" fn (this: *const nsISocketTransportService, aObserver: *const nsISTSShutdownObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISocketTransportService {

    /// ```text
    /// /**
    ///      * Creates a transport for a specified host and port.
    ///      *
    ///      * @param aSocketTypes
    ///      *        array of socket type strings.  Empty array if using default
    ///      *        socket type.
    ///      * @param aHost
    ///      *        specifies the target hostname or IP address literal of the peer
    ///      *        for this socket.
    ///      * @param aPort
    ///      *        specifies the target port of the peer for this socket.
    ///      * @param aProxyInfo
    ///      *        specifies the transport-layer proxy type to use.  null if no
    ///      *        proxy.  used for communicating information about proxies like
    ///      *        SOCKS (which are transparent to upper protocols).
    ///      * @param aDnsRecord
    ///      *        the dns record to be used for the connection
    ///      *
    ///      * @see nsIProxiedProtocolHandler
    ///      * @see nsIProtocolProxyService::GetProxyInfo
    ///      *
    ///      * NOTE: this function can be called from any thread
    ///      */
    /// ```
    ///

    /// `nsISocketTransport createTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in nsIProxyInfo aProxyInfo, in nsIDNSRecord dnsRecord);`
    #[inline]
    pub unsafe fn CreateTransport(&self, aSocketTypes: *const thin_vec::ThinVec<::nsstring::nsCString>, aHost: *const ::nsstring::nsACString, aPort: i32, aProxyInfo: *const nsIProxyInfo, dnsRecord: *const nsIDNSRecord, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult {
        ((*self.vtable).CreateTransport)(self, aSocketTypes, aHost, aPort, aProxyInfo, dnsRecord, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a transport built on a Unix domain socket, connecting to the
    ///      * given filename.
    ///      *
    ///      * Since Unix domain sockets are always local to the machine, they are
    ///      * not affected by the nsIIOService's 'offline' flag.
    ///      *
    ///      * On systems that don't support Unix domain sockets at all, this
    ///      * returns NS_ERROR_SOCKET_ADDRESS_NOT_SUPPORTED.
    ///      *
    ///      * The system-level socket API may impose restrictions on the length of
    ///      * the filename that are stricter than those of the underlying
    ///      * filesystem. If the file name is too long, this returns
    ///      * NS_ERROR_FILE_NAME_TOO_LONG.
    ///      *
    ///      * The |aPath| parameter must specify an existing directory entry.
    ///      * Otherwise, this returns NS_ERROR_FILE_NOT_FOUND.
    ///      *
    ///      * The program must have search permission on all components of the
    ///      * path prefix of |aPath|, and read and write permission on |aPath|
    ///      * itself. Without such permission, this returns
    ///      * NS_ERROR_CONNECTION_REFUSED.
    ///      *
    ///      * The |aPath| parameter must refer to a unix-domain socket. Otherwise,
    ///      * this returns NS_ERROR_CONNECTION_REFUSED. (POSIX specifies
        ///      * ECONNREFUSED when "the target address was not listening for
        ///      * connections", and this is what Linux returns.)
    ///      *
    ///      * @param aPath
    ///      *        The file name of the Unix domain socket to which we should
    ///      *        connect.
    ///      */
    /// ```
    ///

    /// `nsISocketTransport createUnixDomainTransport (in nsIFile aPath);`
    #[inline]
    pub unsafe fn CreateUnixDomainTransport(&self, aPath: *const nsIFile, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult {
        ((*self.vtable).CreateUnixDomainTransport)(self, aPath, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a transport built on a Unix domain socket that uses abstract
    ///      * address name.
    ///      *
    ///      * If abstract socket address isn't supported on System, this returns
    ///      * NS_ERROR_SOCKET_ADDRESS_NOT_SUPPORTED.
    ///      *
    ///      * @param aName
    ///      *        The name of abstract socket adress of the Unix domain socket to
    ///      *        which we should connect.
    ///      */
    /// ```
    ///

    /// `nsISocketTransport createUnixDomainAbstractAddressTransport (in ACString aName);`
    #[inline]
    pub unsafe fn CreateUnixDomainAbstractAddressTransport(&self, aName: *const ::nsstring::nsACString, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult {
        ((*self.vtable).CreateUnixDomainAbstractAddressTransport)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///      * Adds a new socket to the list of controlled sockets.
    ///      *
    ///      * This will fail with the error code NS_ERROR_NOT_AVAILABLE if the maximum
    ///      * number of sockets is already reached.
    ///      * In this case, the notifyWhenCanAttachSocket method should be used.
    ///      *
    ///      * @param aFd
    ///      *        Open file descriptor of the socket to control.
    ///      * @param aHandler
    ///      *        Socket handler that will receive notifications when the socket is
    ///      *        ready or detached.
    ///      *
    ///      * NOTE: this function may only be called from an event dispatch on the
    ///      *       socket thread.
    ///      */
    /// ```
    ///

    /// `[noscript] void attachSocket (in PRFileDescPtr aFd, in nsASocketHandlerPtr aHandler);`
    const _AttachSocket: () = ();

    /// ```text
    /// /**
    ///      * if the number of sockets reaches the limit, then consumers can be
    ///      * notified when the number of sockets becomes less than the limit.  the
    ///      * notification is asynchronous, delivered via the given nsIRunnable
    ///      * instance on the socket transport thread.
    ///      *
    ///      * @param aEvent
    ///      *        Event that will receive the notification when a new socket can
    ///      *        be attached
    ///      *
    ///      * NOTE: this function may only be called from an event dispatch on the
    ///      *       socket thread.
    ///      */
    /// ```
    ///

    /// `[noscript] void notifyWhenCanAttachSocket (in nsIRunnable aEvent);`
    #[inline]
    pub unsafe fn NotifyWhenCanAttachSocket(&self, aEvent: *const nsIRunnable) -> ::nserror::nsresult {
        ((*self.vtable).NotifyWhenCanAttachSocket)(self, aEvent)
    }



    /// `[noscript] void addShutdownObserver (in nsISTSShutdownObserver aObserver);`
    #[inline]
    pub unsafe fn AddShutdownObserver(&self, aObserver: *const nsISTSShutdownObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddShutdownObserver)(self, aObserver)
    }



    /// `[noscript] void removeShutdownObserver (in nsISTSShutdownObserver aObserver);`
    #[inline]
    pub unsafe fn RemoveShutdownObserver(&self, aObserver: *const nsISTSShutdownObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveShutdownObserver)(self, aObserver)
    }


}


/// `interface nsIRoutedSocketTransportService : nsISocketTransportService`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRoutedSocketTransportService {
    vtable: *const nsIRoutedSocketTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRoutedSocketTransportService.
unsafe impl XpCom for nsIRoutedSocketTransportService {
    const IID: nsIID = nsID(0xc5204623, 0x5b58, 0x4a16,
        [0x8b, 0x2e, 0x67, 0xc3, 0x4d, 0xd0, 0x2e, 0x3f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRoutedSocketTransportService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRoutedSocketTransportService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRoutedSocketTransportServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIRoutedSocketTransportService`.
    fn coerce_from(v: &nsIRoutedSocketTransportService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRoutedSocketTransportServiceCoerce for nsIRoutedSocketTransportService {
    #[inline]
    fn coerce_from(v: &nsIRoutedSocketTransportService) -> &Self {
        v
    }
}

impl nsIRoutedSocketTransportService {
    /// Cast this `nsIRoutedSocketTransportService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRoutedSocketTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRoutedSocketTransportService {
    type Target = nsISocketTransportService;
    #[inline]
    fn deref(&self) -> &nsISocketTransportService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISocketTransportServiceCoerce> nsIRoutedSocketTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRoutedSocketTransportService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRoutedSocketTransportService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRoutedSocketTransportServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISocketTransportServiceVTable,

    /* nsISocketTransport createRoutedTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in AUTF8String aHostRoute, in long aPortRoute, in nsIProxyInfo aProxyInfo, in nsIDNSRecord aDnsRecord); */
    pub CreateRoutedTransport: unsafe extern "system" fn (this: *const nsIRoutedSocketTransportService, aSocketTypes: *const thin_vec::ThinVec<::nsstring::nsCString>, aHost: *const ::nsstring::nsACString, aPort: i32, aHostRoute: *const ::nsstring::nsACString, aPortRoute: i32, aProxyInfo: *const nsIProxyInfo, aDnsRecord: *const nsIDNSRecord, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRoutedSocketTransportService {


    /// `nsISocketTransport createRoutedTransport (in Array<ACString> aSocketTypes, in AUTF8String aHost, in long aPort, in AUTF8String aHostRoute, in long aPortRoute, in nsIProxyInfo aProxyInfo, in nsIDNSRecord aDnsRecord);`
    #[inline]
    pub unsafe fn CreateRoutedTransport(&self, aSocketTypes: *const thin_vec::ThinVec<::nsstring::nsCString>, aHost: *const ::nsstring::nsACString, aPort: i32, aHostRoute: *const ::nsstring::nsACString, aPortRoute: i32, aProxyInfo: *const nsIProxyInfo, aDnsRecord: *const nsIDNSRecord, _retval: *mut*const nsISocketTransport) -> ::nserror::nsresult {
        ((*self.vtable).CreateRoutedTransport)(self, aSocketTypes, aHost, aPort, aHostRoute, aPortRoute, aProxyInfo, aDnsRecord, _retval)
    }


}


