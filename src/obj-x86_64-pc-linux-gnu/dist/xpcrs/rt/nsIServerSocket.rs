//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIServerSocket.idl
//


/// `typedef uint32_t  nsServerSocketFlag;`
///


pub type nsServerSocketFlag = u32;


/// `interface nsIServerSocket : nsISupports`
///

/// ```text
/// /**
///  * nsIServerSocket
///  *
///  * An interface to a server socket that can accept incoming connections.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServerSocket {
    vtable: *const nsIServerSocketVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServerSocket.
unsafe impl XpCom for nsIServerSocket {
    const IID: nsIID = nsID(0x7a9c39cb, 0xa13f, 0x4eef,
        [0x9b, 0xdf, 0xa7, 0x43, 0x01, 0x62, 0x87, 0x42]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServerSocket {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServerSocket.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServerSocketCoerce {
    /// Cheaply cast a value of this type from a `nsIServerSocket`.
    fn coerce_from(v: &nsIServerSocket) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServerSocketCoerce for nsIServerSocket {
    #[inline]
    fn coerce_from(v: &nsIServerSocket) -> &Self {
        v
    }
}

impl nsIServerSocket {
    /// Cast this `nsIServerSocket` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServerSocketCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServerSocket {
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
impl<T: nsISupportsCoerce> nsIServerSocketCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServerSocket) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServerSocket
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServerSocketVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in long aPort, in boolean aLoopbackOnly, in long aBackLog); */
    pub Init: unsafe extern "system" fn (this: *const nsIServerSocket, aPort: i32, aLoopbackOnly: bool, aBackLog: i32) -> ::nserror::nsresult,

    /* void initIPv6 (in long aPort, in boolean aLoopbackOnly, in long aBackLog); */
    pub InitIPv6: unsafe extern "system" fn (this: *const nsIServerSocket, aPort: i32, aLoopbackOnly: bool, aBackLog: i32) -> ::nserror::nsresult,

    /* void initSpecialConnection (in long aPort, in nsServerSocketFlag aFlags, in long aBackLog); */
    pub InitSpecialConnection: unsafe extern "system" fn (this: *const nsIServerSocket, aPort: i32, aFlags: nsServerSocketFlag, aBackLog: i32) -> ::nserror::nsresult,

    /* [noscript] void initWithAddress ([const] in PRNetAddrPtr aAddr, in long aBackLog); */
    /// Unable to generate binding because `native type union PRNetAddr unsupported`
    pub InitWithAddress: *const ::libc::c_void,

    /* void initWithFilename (in nsIFile aPath, in unsigned long aPermissions, in long aBacklog); */
    pub InitWithFilename: unsafe extern "system" fn (this: *const nsIServerSocket, aPath: *const nsIFile, aPermissions: u32, aBacklog: i32) -> ::nserror::nsresult,

    /* void initWithAbstractAddress (in AUTF8String aName, in long aBacklog); */
    pub InitWithAbstractAddress: unsafe extern "system" fn (this: *const nsIServerSocket, aName: *const ::nsstring::nsACString, aBacklog: i32) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIServerSocket) -> ::nserror::nsresult,

    /* void asyncListen (in nsIServerSocketListener aListener); */
    pub AsyncListen: unsafe extern "system" fn (this: *const nsIServerSocket, aListener: *const nsIServerSocketListener) -> ::nserror::nsresult,

    /* readonly attribute long port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIServerSocket, aPort: *mut i32) -> ::nserror::nsresult,

    /* [noscript] PRNetAddr getAddress (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetAddress: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServerSocket {
    /// ```text
    /// /**
    ///      * @name Server Socket Flags
    ///      * These flags define various socket options.
    ///      * @{
        ///      */
        /// ```
        ///

        pub const LoopbackOnly: i64 = 1;


        pub const KeepWhenOffline: i64 = 2;

        /// ```text
        /// /** @} */
    /// /**
    ///      * init
    ///      *
    ///      * This method initializes a server socket.
    ///      *
    ///      * @param aPort
    ///      *        The port of the server socket.  Pass -1 to indicate no preference,
    ///      *        and a port will be selected automatically.
    ///      * @param aLoopbackOnly
    ///      *        If true, the server socket will only respond to connections on the
    ///      *        local loopback interface.  Otherwise, it will accept connections
    ///      *        from any interface.  To specify a particular network interface,
    ///      *        use initWithAddress.
    ///      * @param aBackLog
    ///      *        The maximum length the queue of pending connections may grow to.
    ///      *        This parameter may be silently limited by the operating system.
    ///      *        Pass -1 to use the default value.
    ///      */
    /// ```
    ///

    /// `void init (in long aPort, in boolean aLoopbackOnly, in long aBackLog);`
    #[inline]
    pub unsafe fn Init(&self, aPort: i32, aLoopbackOnly: bool, aBackLog: i32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aPort, aLoopbackOnly, aBackLog)
    }


    /// ```text
    /// /**
    ///      * the same as init(), but initializes an IPv6 server socket
    ///      */
    /// ```
    ///

    /// `void initIPv6 (in long aPort, in boolean aLoopbackOnly, in long aBackLog);`
    #[inline]
    pub unsafe fn InitIPv6(&self, aPort: i32, aLoopbackOnly: bool, aBackLog: i32) -> ::nserror::nsresult {
        ((*self.vtable).InitIPv6)(self, aPort, aLoopbackOnly, aBackLog)
    }


    /// ```text
    /// /**
    ///      * initSpecialConnection
    ///      *
    ///      * This method initializes a server socket and offers the ability to have
    ///      * that socket not get terminated if Gecko is set offline.
    ///      *
    ///      * @param aPort
    ///      *        The port of the server socket.  Pass -1 to indicate no preference,
    ///      *        and a port will be selected automatically.
    ///      * @param aFlags
    ///      *        Flags for the socket.
    ///      * @param aBackLog
    ///      *        The maximum length the queue of pending connections may grow to.
    ///      *        This parameter may be silently limited by the operating system.
    ///      *        Pass -1 to use the default value.
    ///      */
    /// ```
    ///

    /// `void initSpecialConnection (in long aPort, in nsServerSocketFlag aFlags, in long aBackLog);`
    #[inline]
    pub unsafe fn InitSpecialConnection(&self, aPort: i32, aFlags: nsServerSocketFlag, aBackLog: i32) -> ::nserror::nsresult {
        ((*self.vtable).InitSpecialConnection)(self, aPort, aFlags, aBackLog)
    }


    /// ```text
    /// /**
    ///      * initWithAddress
    ///      *
    ///      * This method initializes a server socket, and binds it to a particular
    ///      * local address (and hence a particular local network interface).
    ///      *
    ///      * @param aAddr
    ///      *        The address to which this server socket should be bound.
    ///      * @param aBackLog
    ///      *        The maximum length the queue of pending connections may grow to.
    ///      *        This parameter may be silently limited by the operating system.
    ///      *        Pass -1 to use the default value.
    ///      */
    /// ```
    ///

    /// `[noscript] void initWithAddress ([const] in PRNetAddrPtr aAddr, in long aBackLog);`
    const _InitWithAddress: () = ();

    /// ```text
    /// /**
    ///      * initWithFilename
    ///      *
    ///      * This method initializes a Unix domain or "local" server socket. Such
    ///      * a socket has a name in the filesystem, like an ordinary file. To
    ///      * connect, a client supplies the socket's filename, and the usual
    ///      * permission checks on socket apply.
    ///      *
    ///      * This makes Unix domain sockets useful for communication between the
    ///      * programs being run by a specific user on a single machine: the
    ///      * operating system takes care of authentication, and the user's home
    ///      * directory or profile directory provide natural per-user rendezvous
    ///      * points.
    ///      *
    ///      * Since Unix domain sockets are always local to the machine, they are
    ///      * not affected by the nsIIOService's 'offline' flag.
    ///      *
    ///      * The system-level socket API may impose restrictions on the length of
    ///      * the filename that are stricter than those of the underlying
    ///      * filesystem. If the file name is too long, this returns
    ///      * NS_ERROR_FILE_NAME_TOO_LONG.
    ///      *
    ///      * All components of the path prefix of |aPath| must name directories;
    ///      * otherwise, this returns NS_ERROR_FILE_NOT_DIRECTORY.
    ///      *
    ///      * This call requires execute permission on all directories containing
    ///      * the one in which the socket is to be created, and write and execute
    ///      * permission on the directory itself. Otherwise, this returns
    ///      * NS_ERROR_CONNECTION_REFUSED.
    ///      *
    ///      * This call creates the socket's directory entry. There must not be
    ///      * any existing entry with the given name. If there is, this returns
    ///      * NS_ERROR_SOCKET_ADDRESS_IN_USE.
    ///      *
    ///      * On systems that don't support Unix domain sockets at all, this
    ///      * returns NS_ERROR_SOCKET_ADDRESS_NOT_SUPPORTED.
    ///      *
    ///      * @param aPath nsIFile
    ///      *        The file name at which the socket should be created.
    ///      *
    ///      * @param aPermissions unsigned long
    ///      *        Unix-style permission bits to be applied to the new socket.
    ///      *
    ///      * Note about permissions: Linux's unix(7) man page claims that some
    ///      * BSD-derived systems ignore permissions on UNIX-domain sockets;
    ///      * NetBSD's bind(2) man page agrees, but says it does check now (dated
        ///      * 2005). POSIX has required 'connect' to fail if write permission on
    ///      * the socket itself is not granted since 2003 (Issue 6). NetBSD says
    ///      * that the permissions on the containing directory (execute) have
    ///      * always applied, so creating sockets in appropriately protected
    ///      * directories should be secure on both old and new systems.
    ///      */
    /// ```
    ///

    /// `void initWithFilename (in nsIFile aPath, in unsigned long aPermissions, in long aBacklog);`
    #[inline]
    pub unsafe fn InitWithFilename(&self, aPath: *const nsIFile, aPermissions: u32, aBacklog: i32) -> ::nserror::nsresult {
        ((*self.vtable).InitWithFilename)(self, aPath, aPermissions, aBacklog)
    }


    /// ```text
    /// /**
    ///      * initWithAbstractAddress
    ///      *
    ///      * This mehtod is a flavor of initWithFilename method. This initializes
    ///      * a UNIX domain socket that uses abstract socket address.
    ///      * This socket type is only supported on Linux and Android.
    ///      *
    ///      * On systems that don't support this type's UNIX domain sockets at all,
    ///      * this returns NS_ERROR_SOCKET_ADDRESS_NOT_SUPPORTED.
    ///      *
    ///      * @param aName
    ///      *        The abstract socket address which the socket should be created.
    ///      * @param aBacklog
    ///      *        The maximum length the queue of pending connections may grow to.
    ///      */
    /// ```
    ///

    /// `void initWithAbstractAddress (in AUTF8String aName, in long aBacklog);`
    #[inline]
    pub unsafe fn InitWithAbstractAddress(&self, aName: *const ::nsstring::nsACString, aBacklog: i32) -> ::nserror::nsresult {
        ((*self.vtable).InitWithAbstractAddress)(self, aName, aBacklog)
    }


    /// ```text
    /// /**
    ///      * close
    ///      *
    ///      * This method closes a server socket.  This does not affect already
    ///      * connected client sockets (i.e., the nsISocketTransport instances
        ///      * created from this server socket).  This will cause the onStopListening
    ///      * event to asynchronously fire with a status of NS_BINDING_ABORTED.
    ///      */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///      * asyncListen
    ///      *
    ///      * This method puts the server socket in the listening state.  It will
    ///      * asynchronously listen for and accept client connections.  The listener
    ///      * will be notified once for each client connection that is accepted.  The
    ///      * listener's onSocketAccepted method will be called on the same thread
    ///      * that called asyncListen (the calling thread must have a nsIEventTarget).
    ///      *
    ///      * The listener will be passed a reference to an already connected socket
    ///      * transport (nsISocketTransport).  See below for more details.
    ///      *
    ///      * @param aListener
    ///      *        The listener to be notified when client connections are accepted.
    ///      */
    /// ```
    ///

    /// `void asyncListen (in nsIServerSocketListener aListener);`
    #[inline]
    pub unsafe fn AsyncListen(&self, aListener: *const nsIServerSocketListener) -> ::nserror::nsresult {
        ((*self.vtable).AsyncListen)(self, aListener)
    }


    /// ```text
    /// /**
    ///      * Returns the port of this server socket.
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
    ///      * Returns the address to which this server socket is bound.  Since a
    ///      * server socket may be bound to multiple network devices, this address
    ///      * may not necessarily be specific to a single network device.  In the
    ///      * case of an IP socket, the IP address field would be zerod out to
    ///      * indicate a server socket bound to all network devices.  Therefore,
    ///      * this method cannot be used to determine the IP address of the local
    ///      * system.  See nsIDNSService::myHostName if this is what you need.
    ///      */
    /// ```
    ///

    /// `[noscript] PRNetAddr getAddress ();`
    const _GetAddress: () = ();

}


/// `interface nsIServerSocketListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServerSocketListener {
    vtable: *const nsIServerSocketListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServerSocketListener.
unsafe impl XpCom for nsIServerSocketListener {
    const IID: nsIID = nsID(0x836d98ec, 0xfee2, 0x4bde,
        [0xb6, 0x09, 0xab, 0xd5, 0xe9, 0x66, 0xea, 0xbd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServerSocketListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServerSocketListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServerSocketListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIServerSocketListener`.
    fn coerce_from(v: &nsIServerSocketListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServerSocketListenerCoerce for nsIServerSocketListener {
    #[inline]
    fn coerce_from(v: &nsIServerSocketListener) -> &Self {
        v
    }
}

impl nsIServerSocketListener {
    /// Cast this `nsIServerSocketListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServerSocketListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServerSocketListener {
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
impl<T: nsISupportsCoerce> nsIServerSocketListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServerSocketListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServerSocketListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServerSocketListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport); */
    pub OnSocketAccepted: unsafe extern "system" fn (this: *const nsIServerSocketListener, aServ: *const nsIServerSocket, aTransport: *const nsISocketTransport) -> ::nserror::nsresult,

    /* void onStopListening (in nsIServerSocket aServ, in nsresult aStatus); */
    pub OnStopListening: unsafe extern "system" fn (this: *const nsIServerSocketListener, aServ: *const nsIServerSocket, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServerSocketListener {

    /// ```text
    /// /**
    ///  * nsIServerSocketListener
    ///  *
    ///  * This interface is notified whenever a server socket accepts a new connection.
    ///  * The transport is in the connected state, and read/write streams can be opened
    ///  * using the normal nsITransport API.  The address of the client can be found by
    ///  * calling the nsISocketTransport::GetAddress method or by inspecting
    ///  * nsISocketTransport::GetHost, which returns a string representation of the
    ///  * client's IP address (NOTE: this may be an IPv4 or IPv6 string literal).
    ///  */
    /// /**
    ///      * onSocketAccepted
    ///      *
    ///      * This method is called when a client connection is accepted.
    ///      *
    ///      * @param aServ
    ///      *        The server socket.
    ///      * @param aTransport
    ///      *        The connected socket transport.
    ///      */
    /// ```
    ///

    /// `void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport);`
    #[inline]
    pub unsafe fn OnSocketAccepted(&self, aServ: *const nsIServerSocket, aTransport: *const nsISocketTransport) -> ::nserror::nsresult {
        ((*self.vtable).OnSocketAccepted)(self, aServ, aTransport)
    }


    /// ```text
    /// /**
    ///      * onStopListening
    ///      *
    ///      * This method is called when the listening socket stops for some reason.
    ///      * The server socket is effectively dead after this notification.
    ///      *
    ///      * @param aServ
    ///      *        The server socket.
    ///      * @param aStatus
    ///      *        The reason why the server socket stopped listening.  If the
    ///      *        server socket was manually closed, then this value will be
    ///      *        NS_BINDING_ABORTED.
    ///      */
    /// ```
    ///

    /// `void onStopListening (in nsIServerSocket aServ, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnStopListening(&self, aServ: *const nsIServerSocket, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnStopListening)(self, aServ, aStatus)
    }


}


