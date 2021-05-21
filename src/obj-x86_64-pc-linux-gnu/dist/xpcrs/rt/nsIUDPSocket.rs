//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUDPSocket.idl
//


/// `interface nsIUDPSocket : nsISupports`
///

/// ```text
/// /**
///  * nsIUDPSocket
///  *
///  * An interface to a UDP socket that can accept incoming connections.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUDPSocket {
    vtable: *const nsIUDPSocketVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUDPSocket.
unsafe impl XpCom for nsIUDPSocket {
    const IID: nsIID = nsID(0xd423bf4e, 0x4499, 0x40cf,
        [0xbc, 0x03, 0x15, 0x3e, 0x2b, 0xf2, 0x06, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUDPSocket {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUDPSocket.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUDPSocketCoerce {
    /// Cheaply cast a value of this type from a `nsIUDPSocket`.
    fn coerce_from(v: &nsIUDPSocket) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUDPSocketCoerce for nsIUDPSocket {
    #[inline]
    fn coerce_from(v: &nsIUDPSocket) -> &Self {
        v
    }
}

impl nsIUDPSocket {
    /// Cast this `nsIUDPSocket` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUDPSocketCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUDPSocket {
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
impl<T: nsISupportsCoerce> nsIUDPSocketCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocket) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUDPSocket
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUDPSocketVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [optional_argc] void init (in long aPort, in boolean aLoopbackOnly, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub Init: *const ::libc::c_void,

    /* [optional_argc] void init2 (in AUTF8String aAddr, in long aPort, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub Init2: *const ::libc::c_void,

    /* [noscript,optional_argc] void initWithAddress ([const] in NetAddrPtr aAddr, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub InitWithAddress: *const ::libc::c_void,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIUDPSocket) -> ::nserror::nsresult,

    /* void asyncListen (in nsIUDPSocketListener aListener); */
    pub AsyncListen: unsafe extern "system" fn (this: *const nsIUDPSocket, aListener: *const nsIUDPSocketListener) -> ::nserror::nsresult,

    /* void connect ([const] in NetAddrPtr aAddr); */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub Connect: *const ::libc::c_void,

    /* readonly attribute nsINetAddr localAddr; */
    pub GetLocalAddr: unsafe extern "system" fn (this: *const nsIUDPSocket, aLocalAddr: *mut*const nsINetAddr) -> ::nserror::nsresult,

    /* readonly attribute long port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIUDPSocket, aPort: *mut i32) -> ::nserror::nsresult,

    /* [noscript] NetAddr getAddress (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetAddress: *const ::libc::c_void,

    /* unsigned long send (in AUTF8String host, in unsigned short port, in Array<uint8_t> data); */
    pub Send: unsafe extern "system" fn (this: *const nsIUDPSocket, host: *const ::nsstring::nsACString, port: u16, data: *const thin_vec::ThinVec<uint8_t>, _retval: *mut u32) -> ::nserror::nsresult,

    /* unsigned long sendWithAddr (in nsINetAddr addr, in Array<uint8_t> data); */
    pub SendWithAddr: unsafe extern "system" fn (this: *const nsIUDPSocket, addr: *const nsINetAddr, data: *const thin_vec::ThinVec<uint8_t>, _retval: *mut u32) -> ::nserror::nsresult,

    /* [noscript] unsigned long sendWithAddress ([const] in NetAddrPtr addr, in Array<uint8_t> data); */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub SendWithAddress: *const ::libc::c_void,

    /* void sendBinaryStream (in AUTF8String host, in unsigned short port, in nsIInputStream stream); */
    pub SendBinaryStream: unsafe extern "system" fn (this: *const nsIUDPSocket, host: *const ::nsstring::nsACString, port: u16, stream: *const nsIInputStream) -> ::nserror::nsresult,

    /* void sendBinaryStreamWithAddress ([const] in NetAddrPtr addr, in nsIInputStream stream); */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub SendBinaryStreamWithAddress: *const ::libc::c_void,

    /* void joinMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
    pub JoinMulticast: unsafe extern "system" fn (this: *const nsIUDPSocket, addr: *const ::nsstring::nsACString, iface: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void joinMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub JoinMulticastAddr: *const ::libc::c_void,

    /* void leaveMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
    pub LeaveMulticast: unsafe extern "system" fn (this: *const nsIUDPSocket, addr: *const ::nsstring::nsACString, iface: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void leaveMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub LeaveMulticastAddr: *const ::libc::c_void,

    /* attribute boolean multicastLoopback; */
    pub GetMulticastLoopback: unsafe extern "system" fn (this: *const nsIUDPSocket, aMulticastLoopback: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean multicastLoopback; */
    pub SetMulticastLoopback: unsafe extern "system" fn (this: *const nsIUDPSocket, aMulticastLoopback: bool) -> ::nserror::nsresult,

    /* attribute AUTF8String multicastInterface; */
    pub GetMulticastInterface: unsafe extern "system" fn (this: *const nsIUDPSocket, aMulticastInterface: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String multicastInterface; */
    pub SetMulticastInterface: unsafe extern "system" fn (this: *const nsIUDPSocket, aMulticastInterface: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] attribute NetAddr multicastInterfaceAddr; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetMulticastInterfaceAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddr multicastInterfaceAddr; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetMulticastInterfaceAddr: *const ::libc::c_void,

    /* [noscript] attribute long recvBufferSize; */
    pub GetRecvBufferSize: unsafe extern "system" fn (this: *const nsIUDPSocket, aRecvBufferSize: *mut i32) -> ::nserror::nsresult,

    /* [noscript] attribute long recvBufferSize; */
    pub SetRecvBufferSize: unsafe extern "system" fn (this: *const nsIUDPSocket, aRecvBufferSize: i32) -> ::nserror::nsresult,

    /* [noscript] attribute long sendBufferSize; */
    pub GetSendBufferSize: unsafe extern "system" fn (this: *const nsIUDPSocket, aSendBufferSize: *mut i32) -> ::nserror::nsresult,

    /* [noscript] attribute long sendBufferSize; */
    pub SetSendBufferSize: unsafe extern "system" fn (this: *const nsIUDPSocket, aSendBufferSize: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUDPSocket {

    /// ```text
    /// /**
    ///      * init
    ///      *
    ///      * This method initializes a UDP socket.
    ///      *
    ///      * @param aPort
    ///      *        The port of the UDP socket.  Pass -1 to indicate no preference,
    ///      *        and a port will be selected automatically.
    ///      * @param aLoopbackOnly
    ///      *        If true, the UDP socket will only respond to connections on the
    ///      *        local loopback interface.  Otherwise, it will accept connections
    ///      *        from any interface.  To specify a particular network interface,
    ///      *        use initWithAddress.
    ///      * @param aPrincipal
    ///      *        The principal connected to this socket.
    ///      * @param aAddressReuse
    ///      *        If true, the socket is allowed to be bound to an address that is
    ///      *        already in use. Default is true.
    ///      */
    /// ```
    ///

    /// `[optional_argc] void init (in long aPort, in boolean aLoopbackOnly, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse);`
    const _Init: () = ();


    /// `[optional_argc] void init2 (in AUTF8String aAddr, in long aPort, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse);`
    const _Init2: () = ();

    /// ```text
    /// /**
    ///      * initWithAddress
    ///      *
    ///      * This method initializes a UDP socket, and binds it to a particular
    ///      * local address (and hence a particular local network interface).
    ///      *
    ///      * @param aAddr
    ///      *        The address to which this UDP socket should be bound.
    ///      * @param aPrincipal
    ///      *        The principal connected to this socket.
    ///      * @param aAddressReuse
    ///      *        If true, the socket is allowed to be bound to an address that is
    ///      *        already in use. Default is true.
    ///      */
    /// ```
    ///

    /// `[noscript,optional_argc] void initWithAddress ([const] in NetAddrPtr aAddr, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse);`
    const _InitWithAddress: () = ();

    /// ```text
    /// /**
    ///      * close
    ///      *
    ///      * This method closes a UDP socket.  This does not affect already
    ///      * connected client sockets (i.e., the nsISocketTransport instances
        ///      * created from this UDP socket).  This will cause the onStopListening
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
    ///      * This method puts the UDP socket in the listening state.  It will
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

    /// `void asyncListen (in nsIUDPSocketListener aListener);`
    #[inline]
    pub unsafe fn AsyncListen(&self, aListener: *const nsIUDPSocketListener) -> ::nserror::nsresult {
        ((*self.vtable).AsyncListen)(self, aListener)
    }


    /// ```text
    /// /**
    ///      * connect
    ///      *
    ///      * This method connects the UDP socket to a remote UDP address.
    ///      *
    ///      * @param aRemoteAddr
    ///      *        The remote address to connect to
    ///      */
    /// ```
    ///

    /// `void connect ([const] in NetAddrPtr aAddr);`
    const _Connect: () = ();

    /// ```text
    /// /**
    ///      * Returns the local address of this UDP socket
    ///      */
    /// ```
    ///

    /// `readonly attribute nsINetAddr localAddr;`
    #[inline]
    pub unsafe fn GetLocalAddr(&self, aLocalAddr: *mut*const nsINetAddr) -> ::nserror::nsresult {
        ((*self.vtable).GetLocalAddr)(self, aLocalAddr)
    }


    /// ```text
    /// /**
    ///      * Returns the port of this UDP socket.
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
    ///      * Returns the address to which this UDP socket is bound.  Since a
    ///      * UDP socket may be bound to multiple network devices, this address
    ///      * may not necessarily be specific to a single network device.  In the
    ///      * case of an IP socket, the IP address field would be zerod out to
    ///      * indicate a UDP socket bound to all network devices.  Therefore,
    ///      * this method cannot be used to determine the IP address of the local
    ///      * system.  See nsIDNSService::myHostName if this is what you need.
    ///      */
    /// ```
    ///

    /// `[noscript] NetAddr getAddress ();`
    const _GetAddress: () = ();

    /// ```text
    /// /**
    ///      * send
    ///      *
    ///      * Send out the datagram to specified remote host and port.
    ///      * DNS lookup will be triggered.
    ///      *
    ///      * @param host The remote host name.
    ///      * @param port The remote port.
    ///      * @param data The buffer containing the data to be written.
    ///      * @return number of bytes written. (0 or length of data)
    ///      */
    /// ```
    ///

    /// `unsigned long send (in AUTF8String host, in unsigned short port, in Array<uint8_t> data);`
    #[inline]
    pub unsafe fn Send(&self, host: *const ::nsstring::nsACString, port: u16, data: *const thin_vec::ThinVec<uint8_t>, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Send)(self, host, port, data, _retval)
    }


    /// ```text
    /// /**
    ///      * sendWithAddr
    ///      *
    ///      * Send out the datagram to specified remote host and port.
    ///      *
    ///      * @param addr The remote host address.
    ///      * @param data The buffer containing the data to be written.
    ///      * @return number of bytes written. (0 or length of data)
    ///      */
    /// ```
    ///

    /// `unsigned long sendWithAddr (in nsINetAddr addr, in Array<uint8_t> data);`
    #[inline]
    pub unsafe fn SendWithAddr(&self, addr: *const nsINetAddr, data: *const thin_vec::ThinVec<uint8_t>, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).SendWithAddr)(self, addr, data, _retval)
    }


    /// ```text
    /// /**
    ///      * sendWithAddress
    ///      *
    ///      * Send out the datagram to specified remote address and port.
    ///      *
    ///      * @param addr The remote host address.
    ///      * @param data The buffer containing the data to be written.
    ///      * @return number of bytes written. (0 or length of data)
    ///      */
    /// ```
    ///

    /// `[noscript] unsigned long sendWithAddress ([const] in NetAddrPtr addr, in Array<uint8_t> data);`
    const _SendWithAddress: () = ();

    /// ```text
    /// /**
    ///      * sendBinaryStream
    ///      *
    ///      * Send out the datagram to specified remote address and port.
    ///      *
    ///      * @param host The remote host name.
    ///      * @param port The remote port.
    ///      * @param stream The input stream to be sent. This must be a buffered stream implementation.
    ///      */
    /// ```
    ///

    /// `void sendBinaryStream (in AUTF8String host, in unsigned short port, in nsIInputStream stream);`
    #[inline]
    pub unsafe fn SendBinaryStream(&self, host: *const ::nsstring::nsACString, port: u16, stream: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SendBinaryStream)(self, host, port, stream)
    }


    /// ```text
    /// /**
    ///      * sendBinaryStreamWithAddress
    ///      *
    ///      * Send out the datagram to specified remote address and port.
    ///      *
    ///      * @param addr The remote host address.
    ///      * @param stream The input stream to be sent. This must be a buffered stream implementation.
    ///      */
    /// ```
    ///

    /// `void sendBinaryStreamWithAddress ([const] in NetAddrPtr addr, in nsIInputStream stream);`
    const _SendBinaryStreamWithAddress: () = ();

    /// ```text
    /// /**
    ///      * joinMulticast
    ///      *
    ///      * Join the multicast group specified by |addr|.  You are then able to
    ///      * receive future datagrams addressed to the group.
    ///      *
    ///      * @param addr
    ///      *        The multicast group address.
    ///      * @param iface
    ///      *        The local address of the interface on which to join the group.  If
    ///      *        this is not specified, the OS may join the group on all interfaces
    ///      *        or only the primary interface.
    ///      */
    /// ```
    ///

    /// `void joinMulticast (in AUTF8String addr, [optional] in AUTF8String iface);`
    #[inline]
    pub unsafe fn JoinMulticast(&self, addr: *const ::nsstring::nsACString, iface: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).JoinMulticast)(self, addr, iface)
    }



    /// `[noscript] void joinMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface);`
    const _JoinMulticastAddr: () = ();

    /// ```text
    /// /**
    ///      * leaveMulticast
    ///      *
    ///      * Leave the multicast group specified by |addr|.  You will no longer
    ///      * receive future datagrams addressed to the group.
    ///      *
    ///      * @param addr
    ///      *        The multicast group address.
    ///      * @param iface
    ///      *        The local address of the interface on which to leave the group.
    ///      *        If this is not specified, the OS may leave the group on all
    ///      *        interfaces or only the primary interface.
    ///      */
    /// ```
    ///

    /// `void leaveMulticast (in AUTF8String addr, [optional] in AUTF8String iface);`
    #[inline]
    pub unsafe fn LeaveMulticast(&self, addr: *const ::nsstring::nsACString, iface: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).LeaveMulticast)(self, addr, iface)
    }



    /// `[noscript] void leaveMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface);`
    const _LeaveMulticastAddr: () = ();

    /// ```text
    /// /**
    ///      * multicastLoopback
    ///      *
    ///      * Whether multicast datagrams sent via this socket should be looped back to
    ///      * this host (assuming this host has joined the relevant group).  Defaults
    ///      * to true.
    ///      * Note: This is currently write-only.
    ///      */
    /// ```
    ///

    /// `attribute boolean multicastLoopback;`
    #[inline]
    pub unsafe fn GetMulticastLoopback(&self, aMulticastLoopback: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMulticastLoopback)(self, aMulticastLoopback)
    }


    /// ```text
    /// /**
    ///      * multicastLoopback
    ///      *
    ///      * Whether multicast datagrams sent via this socket should be looped back to
    ///      * this host (assuming this host has joined the relevant group).  Defaults
    ///      * to true.
    ///      * Note: This is currently write-only.
    ///      */
    /// ```
    ///

    /// `attribute boolean multicastLoopback;`
    #[inline]
    pub unsafe fn SetMulticastLoopback(&self, aMulticastLoopback: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMulticastLoopback)(self, aMulticastLoopback)
    }


    /// ```text
    /// /**
    ///      * multicastInterface
    ///      *
    ///      * The interface that should be used for sending future multicast datagrams.
    ///      * Note: This is currently write-only.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String multicastInterface;`
    #[inline]
    pub unsafe fn GetMulticastInterface(&self, aMulticastInterface: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMulticastInterface)(self, aMulticastInterface)
    }


    /// ```text
    /// /**
    ///      * multicastInterface
    ///      *
    ///      * The interface that should be used for sending future multicast datagrams.
    ///      * Note: This is currently write-only.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String multicastInterface;`
    #[inline]
    pub unsafe fn SetMulticastInterface(&self, aMulticastInterface: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetMulticastInterface)(self, aMulticastInterface)
    }


    /// ```text
    /// /**
    ///      * multicastInterfaceAddr
    ///      *
    ///      * The interface that should be used for sending future multicast datagrams.
    ///      * Note: This is currently write-only.
    ///      */
    /// ```
    ///

    /// `[noscript] attribute NetAddr multicastInterfaceAddr;`
    const _GetMulticastInterfaceAddr: () = ();

    /// ```text
    /// /**
    ///      * multicastInterfaceAddr
    ///      *
    ///      * The interface that should be used for sending future multicast datagrams.
    ///      * Note: This is currently write-only.
    ///      */
    /// ```
    ///

    /// `[noscript] attribute NetAddr multicastInterfaceAddr;`
    const _SetMulticastInterfaceAddr: () = ();

    /// ```text
    /// /**
    ///      * recvBufferSize
    ///      *
    ///      * The size of the receive buffer. Default depends on the OS.
    ///      */
    /// ```
    ///

    /// `[noscript] attribute long recvBufferSize;`
    #[inline]
    pub unsafe fn GetRecvBufferSize(&self, aRecvBufferSize: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRecvBufferSize)(self, aRecvBufferSize)
    }


    /// ```text
    /// /**
    ///      * recvBufferSize
    ///      *
    ///      * The size of the receive buffer. Default depends on the OS.
    ///      */
    /// ```
    ///

    /// `[noscript] attribute long recvBufferSize;`
    #[inline]
    pub unsafe fn SetRecvBufferSize(&self, aRecvBufferSize: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetRecvBufferSize)(self, aRecvBufferSize)
    }


    /// ```text
    /// /**
    ///      * sendBufferSize
    ///      *
    ///      * The size of the send buffer. Default depends on the OS.
    ///      */
    /// ```
    ///

    /// `[noscript] attribute long sendBufferSize;`
    #[inline]
    pub unsafe fn GetSendBufferSize(&self, aSendBufferSize: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSendBufferSize)(self, aSendBufferSize)
    }


    /// ```text
    /// /**
    ///      * sendBufferSize
    ///      *
    ///      * The size of the send buffer. Default depends on the OS.
    ///      */
    /// ```
    ///

    /// `[noscript] attribute long sendBufferSize;`
    #[inline]
    pub unsafe fn SetSendBufferSize(&self, aSendBufferSize: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetSendBufferSize)(self, aSendBufferSize)
    }


}


/// `interface nsIUDPSocketListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUDPSocketListener {
    vtable: *const nsIUDPSocketListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUDPSocketListener.
unsafe impl XpCom for nsIUDPSocketListener {
    const IID: nsIID = nsID(0x2e4b5dd3, 0x7358, 0x4281,
        [0xb8, 0x1f, 0x10, 0xc6, 0x2e, 0xf3, 0x9c, 0xb5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUDPSocketListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUDPSocketListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUDPSocketListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIUDPSocketListener`.
    fn coerce_from(v: &nsIUDPSocketListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUDPSocketListenerCoerce for nsIUDPSocketListener {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketListener) -> &Self {
        v
    }
}

impl nsIUDPSocketListener {
    /// Cast this `nsIUDPSocketListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUDPSocketListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUDPSocketListener {
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
impl<T: nsISupportsCoerce> nsIUDPSocketListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUDPSocketListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUDPSocketListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage); */
    pub OnPacketReceived: unsafe extern "system" fn (this: *const nsIUDPSocketListener, aSocket: *const nsIUDPSocket, aMessage: *const nsIUDPMessage) -> ::nserror::nsresult,

    /* void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus); */
    pub OnStopListening: unsafe extern "system" fn (this: *const nsIUDPSocketListener, aSocket: *const nsIUDPSocket, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUDPSocketListener {

    /// ```text
    /// /**
    ///  * nsIUDPSocketListener
    ///  *
    ///  * This interface is notified whenever a UDP socket accepts a new connection.
    ///  * The transport is in the connected state, and read/write streams can be opened
    ///  * using the normal nsITransport API.  The address of the client can be found by
    ///  * calling the nsISocketTransport::GetAddress method or by inspecting
    ///  * nsISocketTransport::GetHost, which returns a string representation of the
    ///  * client's IP address (NOTE: this may be an IPv4 or IPv6 string literal).
    ///  */
    /// /**
    ///      * onPacketReceived
    ///      *
    ///      * This method is called when a client sends an UDP packet.
    ///      *
    ///      * @param aSocket
    ///      *        The UDP socket.
    ///      * @param aMessage
    ///      *        The message.
    ///      */
    /// ```
    ///

    /// `void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage);`
    #[inline]
    pub unsafe fn OnPacketReceived(&self, aSocket: *const nsIUDPSocket, aMessage: *const nsIUDPMessage) -> ::nserror::nsresult {
        ((*self.vtable).OnPacketReceived)(self, aSocket, aMessage)
    }


    /// ```text
    /// /**
    ///      * onStopListening
    ///      *
    ///      * This method is called when the listening socket stops for some reason.
    ///      * The UDP socket is effectively dead after this notification.
    ///      *
    ///      * @param aSocket
    ///      *        The UDP socket.
    ///      * @param aStatus
    ///      *        The reason why the UDP socket stopped listening.  If the
    ///      *        UDP socket was manually closed, then this value will be
    ///      *        NS_BINDING_ABORTED.
    ///      */
    /// ```
    ///

    /// `void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnStopListening(&self, aSocket: *const nsIUDPSocket, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnStopListening)(self, aSocket, aStatus)
    }


}


/// `interface nsIUDPMessage : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUDPMessage {
    vtable: *const nsIUDPMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUDPMessage.
unsafe impl XpCom for nsIUDPMessage {
    const IID: nsIID = nsID(0xafdc743f, 0x9cc0, 0x40d8,
        [0xb4, 0x42, 0x69, 0x5d, 0xc5, 0x4b, 0xbb, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUDPMessage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUDPMessage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUDPMessageCoerce {
    /// Cheaply cast a value of this type from a `nsIUDPMessage`.
    fn coerce_from(v: &nsIUDPMessage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUDPMessageCoerce for nsIUDPMessage {
    #[inline]
    fn coerce_from(v: &nsIUDPMessage) -> &Self {
        v
    }
}

impl nsIUDPMessage {
    /// Cast this `nsIUDPMessage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUDPMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUDPMessage {
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
impl<T: nsISupportsCoerce> nsIUDPMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPMessage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUDPMessage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUDPMessageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsINetAddr fromAddr; */
    pub GetFromAddr: unsafe extern "system" fn (this: *const nsIUDPMessage, aFromAddr: *mut*const nsINetAddr) -> ::nserror::nsresult,

    /* readonly attribute ACString data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIUDPMessage, aData: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIOutputStream outputStream; */
    pub GetOutputStream: unsafe extern "system" fn (this: *const nsIUDPMessage, aOutputStream: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval rawData; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetRawData: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] Uint8TArrayRef getDataAsTArray (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetDataAsTArray: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUDPMessage {

    /// ```text
    /// /**
    ///  * nsIUDPMessage
    ///  *
    ///  * This interface is used to encapsulate an incomming UDP message
    ///  */
    /// /**
    ///      * Address of the source of the message
    ///      */
    /// ```
    ///

    /// `readonly attribute nsINetAddr fromAddr;`
    #[inline]
    pub unsafe fn GetFromAddr(&self, aFromAddr: *mut*const nsINetAddr) -> ::nserror::nsresult {
        ((*self.vtable).GetFromAddr)(self, aFromAddr)
    }


    /// ```text
    /// /**
    ///      * Data of the message
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


    /// ```text
    /// /**
    ///      * Stream to send a response
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIOutputStream outputStream;`
    #[inline]
    pub unsafe fn GetOutputStream(&self, aOutputStream: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetOutputStream)(self, aOutputStream)
    }


    /// ```text
    /// /**
    ///      * Raw Data of the message
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval rawData;`
    const _GetRawData: () = ();


    /// `[noscript,nostdcall,notxpcom] Uint8TArrayRef getDataAsTArray ();`
    const _GetDataAsTArray: () = ();

}


