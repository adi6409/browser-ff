//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationControlService.idl
//


/// `interface nsITCPDeviceInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITCPDeviceInfo {
    vtable: *const nsITCPDeviceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITCPDeviceInfo.
unsafe impl XpCom for nsITCPDeviceInfo {
    const IID: nsIID = nsID(0x296fd171, 0xe4d0, 0x4de0,
        [0x99, 0xff, 0xad, 0x8e, 0xd5, 0x2d, 0xde, 0xf3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITCPDeviceInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITCPDeviceInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITCPDeviceInfoCoerce {
    /// Cheaply cast a value of this type from a `nsITCPDeviceInfo`.
    fn coerce_from(v: &nsITCPDeviceInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITCPDeviceInfoCoerce for nsITCPDeviceInfo {
    #[inline]
    fn coerce_from(v: &nsITCPDeviceInfo) -> &Self {
        v
    }
}

impl nsITCPDeviceInfo {
    /// Cast this `nsITCPDeviceInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITCPDeviceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITCPDeviceInfo {
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
impl<T: nsISupportsCoerce> nsITCPDeviceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITCPDeviceInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITCPDeviceInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITCPDeviceInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String id; */
    pub GetId: unsafe extern "system" fn (this: *const nsITCPDeviceInfo, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String address; */
    pub GetAddress: unsafe extern "system" fn (this: *const nsITCPDeviceInfo, aAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute uint16_t port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsITCPDeviceInfo, aPort: *mut uint16_t) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String certFingerprint; */
    pub GetCertFingerprint: unsafe extern "system" fn (this: *const nsITCPDeviceInfo, aCertFingerprint: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITCPDeviceInfo {


    /// `readonly attribute AUTF8String id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute AUTF8String address;`
    #[inline]
    pub unsafe fn GetAddress(&self, aAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAddress)(self, aAddress)
    }



    /// `readonly attribute uint16_t port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }



    /// `readonly attribute AUTF8String certFingerprint;`
    #[inline]
    pub unsafe fn GetCertFingerprint(&self, aCertFingerprint: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCertFingerprint)(self, aCertFingerprint)
    }


}


/// `interface nsIPresentationControlServerListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationControlServerListener {
    vtable: *const nsIPresentationControlServerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationControlServerListener.
unsafe impl XpCom for nsIPresentationControlServerListener {
    const IID: nsIID = nsID(0x09bddfaf, 0xfcc2, 0x4dc9,
        [0xb3, 0x3e, 0xa5, 0x09, 0xa1, 0xc2, 0xfb, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationControlServerListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationControlServerListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationControlServerListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationControlServerListener`.
    fn coerce_from(v: &nsIPresentationControlServerListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationControlServerListenerCoerce for nsIPresentationControlServerListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlServerListener) -> &Self {
        v
    }
}

impl nsIPresentationControlServerListener {
    /// Cast this `nsIPresentationControlServerListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationControlServerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationControlServerListener {
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
impl<T: nsISupportsCoerce> nsIPresentationControlServerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlServerListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationControlServerListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationControlServerListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint); */
    pub OnServerReady: unsafe extern "system" fn (this: *const nsIPresentationControlServerListener, aPort: uint16_t, aCertFingerprint: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void onServerStopped (in nsresult aResult); */
    pub OnServerStopped: unsafe extern "system" fn (this: *const nsIPresentationControlServerListener, aResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aUrl, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
    pub OnSessionRequest: unsafe extern "system" fn (this: *const nsIPresentationControlServerListener, aDeviceInfo: *const nsITCPDeviceInfo, aUrl: *const ::nsstring::nsAString, aPresentationId: *const ::nsstring::nsAString, aControlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult,

    /* void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver); */
    pub OnTerminateRequest: unsafe extern "system" fn (this: *const nsIPresentationControlServerListener, aDeviceInfo: *const nsITCPDeviceInfo, aPresentationId: *const ::nsstring::nsAString, aControlChannel: *const nsIPresentationControlChannel, aIsFromReceiver: bool) -> ::nserror::nsresult,

    /* void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in AString url, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
    pub OnReconnectRequest: unsafe extern "system" fn (this: *const nsIPresentationControlServerListener, aDeviceInfo: *const nsITCPDeviceInfo, url: *const ::nsstring::nsAString, aPresentationId: *const ::nsstring::nsAString, aControlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationControlServerListener {

    /// ```text
    /// /**
    ///    * Callback while the server is ready or restarted.
    ///    * @param   aPort
    ///    *          The port of the server socket.
    ///    * @param   aCertFingerprint
    ///    *          The SHA-256 fingerprint of TLS server certificate.
    ///    *          Empty string represents server started without encryption.
    ///    */
    /// ```
    ///

    /// `void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint);`
    #[inline]
    pub unsafe fn OnServerReady(&self, aPort: uint16_t, aCertFingerprint: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnServerReady)(self, aPort, aCertFingerprint)
    }


    /// ```text
    /// /**
    ///    * Callback while the server is stopped or fails to start.
    ///    * @param   aResult
    ///    *          The error cause of server stopped or the reason of
    ///    *          start failure.
    ///    *          NS_OK means the server is stopped by close.
    ///    */
    /// ```
    ///

    /// `void onServerStopped (in nsresult aResult);`
    #[inline]
    pub unsafe fn OnServerStopped(&self, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnServerStopped)(self, aResult)
    }


    /// ```text
    /// /**
    ///    * Callback while the remote host is requesting to start a presentation session.
    ///    * @param aDeviceInfo The device information related to the remote host.
    ///    * @param aUrl The URL requested to open by remote device.
    ///    * @param aPresentationId The Id for representing this session.
    ///    * @param aControlChannel The control channel for this session.
    ///    */
    /// ```
    ///

    /// `void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aUrl, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel);`
    #[inline]
    pub unsafe fn OnSessionRequest(&self, aDeviceInfo: *const nsITCPDeviceInfo, aUrl: *const ::nsstring::nsAString, aPresentationId: *const ::nsstring::nsAString, aControlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).OnSessionRequest)(self, aDeviceInfo, aUrl, aPresentationId, aControlChannel)
    }


    /// ```text
    /// /**
    ///    * Callback while the remote host is requesting to terminate a presentation session.
    ///    * @param aDeviceInfo The device information related to the remote host.
    ///    * @param aPresentationId The Id for representing this session.
    ///    * @param aControlChannel The control channel for this session.
    ///    * @param aIsFromReceiver true if termination is initiated by receiver.
    ///    */
    /// ```
    ///

    /// `void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver);`
    #[inline]
    pub unsafe fn OnTerminateRequest(&self, aDeviceInfo: *const nsITCPDeviceInfo, aPresentationId: *const ::nsstring::nsAString, aControlChannel: *const nsIPresentationControlChannel, aIsFromReceiver: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnTerminateRequest)(self, aDeviceInfo, aPresentationId, aControlChannel, aIsFromReceiver)
    }


    /// ```text
    /// /**
    ///    * Callback while the remote host is requesting to reconnect a presentation session.
    ///    * @param aDeviceInfo The device information related to the remote host.
    ///    * @param aUrl The URL requested to open by remote device.
    ///    * @param aPresentationId The Id for representing this session.
    ///    * @param aControlChannel The control channel for this session.
    ///    */
    /// ```
    ///

    /// `void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in AString url, in AString aPresentationId, in nsIPresentationControlChannel aControlChannel);`
    #[inline]
    pub unsafe fn OnReconnectRequest(&self, aDeviceInfo: *const nsITCPDeviceInfo, url: *const ::nsstring::nsAString, aPresentationId: *const ::nsstring::nsAString, aControlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).OnReconnectRequest)(self, aDeviceInfo, url, aPresentationId, aControlChannel)
    }


}


/// `interface nsIPresentationControlService : nsISupports`
///

/// ```text
/// /**
///  * Presentation control service which can be used for both presentation
///  * control client and server.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationControlService {
    vtable: *const nsIPresentationControlServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationControlService.
unsafe impl XpCom for nsIPresentationControlService {
    const IID: nsIID = nsID(0x55d6b605, 0x2389, 0x4aae,
        [0xa8, 0xfe, 0x60, 0xd4, 0x44, 0x05, 0x40, 0xea]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationControlService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationControlService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationControlServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationControlService`.
    fn coerce_from(v: &nsIPresentationControlService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationControlServiceCoerce for nsIPresentationControlService {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlService) -> &Self {
        v
    }
}

impl nsIPresentationControlService {
    /// Cast this `nsIPresentationControlService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationControlServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationControlService {
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
impl<T: nsISupportsCoerce> nsIPresentationControlServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationControlService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationControlServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void startServer (in boolean aEncrypted, [optional] in uint16_t aPort); */
    pub StartServer: unsafe extern "system" fn (this: *const nsIPresentationControlService, aEncrypted: bool, aPort: uint16_t) -> ::nserror::nsresult,

    /* nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo); */
    pub Connect: unsafe extern "system" fn (this: *const nsIPresentationControlService, aDeviceInfo: *const nsITCPDeviceInfo, _retval: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult,

    /* boolean isCompatibleServer (in uint32_t aVersion); */
    pub IsCompatibleServer: unsafe extern "system" fn (this: *const nsIPresentationControlService, aVersion: uint32_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIPresentationControlService) -> ::nserror::nsresult,

    /* readonly attribute uint16_t port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIPresentationControlService, aPort: *mut uint16_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t version; */
    pub GetVersion: unsafe extern "system" fn (this: *const nsIPresentationControlService, aVersion: *mut uint32_t) -> ::nserror::nsresult,

    /* attribute AUTF8String id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIPresentationControlService, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String id; */
    pub SetId: unsafe extern "system" fn (this: *const nsIPresentationControlService, aId: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String certFingerprint; */
    pub GetCertFingerprint: unsafe extern "system" fn (this: *const nsIPresentationControlService, aCertFingerprint: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String certFingerprint; */
    pub SetCertFingerprint: unsafe extern "system" fn (this: *const nsIPresentationControlService, aCertFingerprint: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute nsIPresentationControlServerListener listener; */
    pub GetListener: unsafe extern "system" fn (this: *const nsIPresentationControlService, aListener: *mut *const nsIPresentationControlServerListener) -> ::nserror::nsresult,

    /* attribute nsIPresentationControlServerListener listener; */
    pub SetListener: unsafe extern "system" fn (this: *const nsIPresentationControlService, aListener: *const nsIPresentationControlServerListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationControlService {

    /// ```text
    /// /**
    ///    * This method initializes server socket. Caller should set listener and
    ///    * monitor onServerReady event to get the correct server info.
    ///    * @param   aEncrypted
    ///    *          True for using TLS control channel.
    ///    * @param   aPort
    ///    *          The port of the server socket.  Pass 0 or opt-out to indicate no
    ///    *          preference, and a port will be selected automatically.
    ///    * @throws  NS_ERROR_FAILURE if the server socket has been inited or the
    ///    *          server socket can not be inited.
    ///    */
    /// ```
    ///

    /// `void startServer (in boolean aEncrypted, [optional] in uint16_t aPort);`
    #[inline]
    pub unsafe fn StartServer(&self, aEncrypted: bool, aPort: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).StartServer)(self, aEncrypted, aPort)
    }


    /// ```text
    /// /**
    ///    * Request connection to designated remote presentation control receiver.
    ///    * @param   aDeviceInfo
    ///    *          The remtoe device info for establish connection.
    ///    * @returns The control channel for this session.
    ///    * @throws  NS_ERROR_FAILURE if the Id hasn't been inited.
    ///    */
    /// ```
    ///

    /// `nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo);`
    #[inline]
    pub unsafe fn Connect(&self, aDeviceInfo: *const nsITCPDeviceInfo, _retval: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).Connect)(self, aDeviceInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Check the compatibility to remote presentation control server.
    ///    * @param  aVersion
    ///    *         The version of remote server.
    ///    */
    /// ```
    ///

    /// `boolean isCompatibleServer (in uint32_t aVersion);`
    #[inline]
    pub unsafe fn IsCompatibleServer(&self, aVersion: uint32_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCompatibleServer)(self, aVersion, _retval)
    }


    /// ```text
    /// /**
    ///    * Close server socket and call |listener.onClose(NS_OK)|
    ///    */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///    * Get the listen port of the TCP socket, valid after the server is ready.
    ///    * 0 indicates the server socket is not ready or is closed.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint16_t port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///    * The protocol version implemented by this server.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t version;`
    #[inline]
    pub unsafe fn GetVersion(&self, aVersion: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetVersion)(self, aVersion)
    }


    /// ```text
    /// /**
    ///    * The id of the TCP presentation server. |requestSession| won't
    ///    * work until the |id| is set.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }


    /// ```text
    /// /**
    ///    * The id of the TCP presentation server. |requestSession| won't
    ///    * work until the |id| is set.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String id;`
    #[inline]
    pub unsafe fn SetId(&self, aId: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetId)(self, aId)
    }


    /// ```text
    /// /**
    ///    * The fingerprint of the TLS server certificate.
    ///    * Empty string indicates the server is not ready or not encrypted.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String certFingerprint;`
    #[inline]
    pub unsafe fn GetCertFingerprint(&self, aCertFingerprint: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCertFingerprint)(self, aCertFingerprint)
    }


    /// ```text
    /// /**
    ///    * The fingerprint of the TLS server certificate.
    ///    * Empty string indicates the server is not ready or not encrypted.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String certFingerprint;`
    #[inline]
    pub unsafe fn SetCertFingerprint(&self, aCertFingerprint: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCertFingerprint)(self, aCertFingerprint)
    }


    /// ```text
    /// /**
    ///    * The listener for handling events of this presentation control server.
    ///    * Listener must be provided before invoke |startServer| and |close|.
    ///    */
    /// ```
    ///

    /// `attribute nsIPresentationControlServerListener listener;`
    #[inline]
    pub unsafe fn GetListener(&self, aListener: *mut *const nsIPresentationControlServerListener) -> ::nserror::nsresult {
        ((*self.vtable).GetListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * The listener for handling events of this presentation control server.
    ///    * Listener must be provided before invoke |startServer| and |close|.
    ///    */
    /// ```
    ///

    /// `attribute nsIPresentationControlServerListener listener;`
    #[inline]
    pub unsafe fn SetListener(&self, aListener: *const nsIPresentationControlServerListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, aListener)
    }


}


