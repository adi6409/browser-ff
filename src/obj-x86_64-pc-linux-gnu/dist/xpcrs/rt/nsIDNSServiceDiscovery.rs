//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/mdns/nsIDNSServiceDiscovery.idl
//


/// `interface nsIDNSServiceInfo : nsISupports`
///

/// ```text
/// /**
///  * Service information
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSServiceInfo {
    vtable: *const nsIDNSServiceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSServiceInfo.
unsafe impl XpCom for nsIDNSServiceInfo {
    const IID: nsIID = nsID(0x670ed0f9, 0x2fa5, 0x4544,
        [0xbf, 0x1e, 0xea, 0x58, 0xac, 0x17, 0x93, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSServiceInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSServiceInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSServiceInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSServiceInfo`.
    fn coerce_from(v: &nsIDNSServiceInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSServiceInfoCoerce for nsIDNSServiceInfo {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceInfo) -> &Self {
        v
    }
}

impl nsIDNSServiceInfo {
    /// Cast this `nsIDNSServiceInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSServiceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSServiceInfo {
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
impl<T: nsISupportsCoerce> nsIDNSServiceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSServiceInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSServiceInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String host; */
    pub SetHost: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aHost: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String address; */
    pub GetAddress: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String address; */
    pub SetAddress: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aAddress: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute unsigned short port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aPort: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short port; */
    pub SetPort: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aPort: u16) -> ::nserror::nsresult,

    /* attribute AUTF8String serviceName; */
    pub GetServiceName: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aServiceName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String serviceName; */
    pub SetServiceName: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aServiceName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String serviceType; */
    pub GetServiceType: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aServiceType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String serviceType; */
    pub SetServiceType: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aServiceType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String domainName; */
    pub GetDomainName: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aDomainName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String domainName; */
    pub SetDomainName: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aDomainName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute nsIPropertyBag2 attributes; */
    pub GetAttributes: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aAttributes: *mut*const nsIPropertyBag2) -> ::nserror::nsresult,

    /* attribute nsIPropertyBag2 attributes; */
    pub SetAttributes: unsafe extern "system" fn (this: *const nsIDNSServiceInfo, aAttributes: *const nsIPropertyBag2) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSServiceInfo {

    /// ```text
    /// /**
    ///    * The host name of the service. (E.g. "Android.local.")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String host;`
    #[inline]
    pub unsafe fn GetHost(&self, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///    * The host name of the service. (E.g. "Android.local.")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String host;`
    #[inline]
    pub unsafe fn SetHost(&self, aHost: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///    * The IP address of the service.
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String address;`
    #[inline]
    pub unsafe fn GetAddress(&self, aAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAddress)(self, aAddress)
    }


    /// ```text
    /// /**
    ///    * The IP address of the service.
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String address;`
    #[inline]
    pub unsafe fn SetAddress(&self, aAddress: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAddress)(self, aAddress)
    }


    /// ```text
    /// /**
    ///    * The port number of the service. (E.g. 80)
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///    * The port number of the service. (E.g. 80)
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short port;`
    #[inline]
    pub unsafe fn SetPort(&self, aPort: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///    * The service name of the service for display. (E.g. "My TV")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String serviceName;`
    #[inline]
    pub unsafe fn GetServiceName(&self, aServiceName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetServiceName)(self, aServiceName)
    }


    /// ```text
    /// /**
    ///    * The service name of the service for display. (E.g. "My TV")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String serviceName;`
    #[inline]
    pub unsafe fn SetServiceName(&self, aServiceName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetServiceName)(self, aServiceName)
    }


    /// ```text
    /// /**
    ///    * The type of the service. (E.g. "_http._tcp")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String serviceType;`
    #[inline]
    pub unsafe fn GetServiceType(&self, aServiceType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetServiceType)(self, aServiceType)
    }


    /// ```text
    /// /**
    ///    * The type of the service. (E.g. "_http._tcp")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String serviceType;`
    #[inline]
    pub unsafe fn SetServiceType(&self, aServiceType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetServiceType)(self, aServiceType)
    }


    /// ```text
    /// /**
    ///    * The domain name of the service. (E.g. "local.")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String domainName;`
    #[inline]
    pub unsafe fn GetDomainName(&self, aDomainName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDomainName)(self, aDomainName)
    }


    /// ```text
    /// /**
    ///    * The domain name of the service. (E.g. "local.")
    ///    * @throws NS_ERROR_NOT_INITIALIZED when getting unset value.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String domainName;`
    #[inline]
    pub unsafe fn SetDomainName(&self, aDomainName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDomainName)(self, aDomainName)
    }


    /// ```text
    /// /**
    ///    * The attributes of the service.
    ///    */
    /// ```
    ///

    /// `attribute nsIPropertyBag2 attributes;`
    #[inline]
    pub unsafe fn GetAttributes(&self, aAttributes: *mut*const nsIPropertyBag2) -> ::nserror::nsresult {
        ((*self.vtable).GetAttributes)(self, aAttributes)
    }


    /// ```text
    /// /**
    ///    * The attributes of the service.
    ///    */
    /// ```
    ///

    /// `attribute nsIPropertyBag2 attributes;`
    #[inline]
    pub unsafe fn SetAttributes(&self, aAttributes: *const nsIPropertyBag2) -> ::nserror::nsresult {
        ((*self.vtable).SetAttributes)(self, aAttributes)
    }


}


/// `interface nsIDNSServiceDiscoveryListener : nsISupports`
///

/// ```text
/// /**
///  * The callback interface for service discovery
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSServiceDiscoveryListener {
    vtable: *const nsIDNSServiceDiscoveryListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSServiceDiscoveryListener.
unsafe impl XpCom for nsIDNSServiceDiscoveryListener {
    const IID: nsIID = nsID(0x3025b7f2, 0x97bb, 0x435b,
        [0xb4, 0x3d, 0x26, 0x73, 0x1b, 0x3f, 0x5f, 0xc4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSServiceDiscoveryListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSServiceDiscoveryListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSServiceDiscoveryListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSServiceDiscoveryListener`.
    fn coerce_from(v: &nsIDNSServiceDiscoveryListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSServiceDiscoveryListenerCoerce for nsIDNSServiceDiscoveryListener {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscoveryListener) -> &Self {
        v
    }
}

impl nsIDNSServiceDiscoveryListener {
    /// Cast this `nsIDNSServiceDiscoveryListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSServiceDiscoveryListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSServiceDiscoveryListener {
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
impl<T: nsISupportsCoerce> nsIDNSServiceDiscoveryListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscoveryListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSServiceDiscoveryListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSServiceDiscoveryListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDiscoveryStarted (in AUTF8String aServiceType); */
    pub OnDiscoveryStarted: unsafe extern "system" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void onDiscoveryStopped (in AUTF8String aServiceType); */
    pub OnDiscoveryStopped: unsafe extern "system" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void onServiceFound (in nsIDNSServiceInfo aServiceInfo); */
    pub OnServiceFound: unsafe extern "system" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult,

    /* void onServiceLost (in nsIDNSServiceInfo aServiceInfo); */
    pub OnServiceLost: unsafe extern "system" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult,

    /* void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
    pub OnStartDiscoveryFailed: unsafe extern "system" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const ::nsstring::nsACString, aErrorCode: i32) -> ::nserror::nsresult,

    /* void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
    pub OnStopDiscoveryFailed: unsafe extern "system" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const ::nsstring::nsACString, aErrorCode: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSServiceDiscoveryListener {

    /// ```text
    /// /**
    ///    * Callback when the discovery begins.
    ///    * @param   aServiceType
    ///    *          the service type of |startDiscovery|.
    ///    */
    /// ```
    ///

    /// `void onDiscoveryStarted (in AUTF8String aServiceType);`
    #[inline]
    pub unsafe fn OnDiscoveryStarted(&self, aServiceType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnDiscoveryStarted)(self, aServiceType)
    }


    /// ```text
    /// /**
    ///    * Callback when the discovery ends.
    ///    * @param   aServiceType
    ///    *          the service type of |startDiscovery|.
    ///    */
    /// ```
    ///

    /// `void onDiscoveryStopped (in AUTF8String aServiceType);`
    #[inline]
    pub unsafe fn OnDiscoveryStopped(&self, aServiceType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnDiscoveryStopped)(self, aServiceType)
    }


    /// ```text
    /// /**
    ///    * Callback when the a service is found.
    ///    * @param   aServiceInfo
    ///    *          the info about the found service, where |serviceName|, |aServiceType|, and |domainName| are set.
    ///    */
    /// ```
    ///

    /// `void onServiceFound (in nsIDNSServiceInfo aServiceInfo);`
    #[inline]
    pub unsafe fn OnServiceFound(&self, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnServiceFound)(self, aServiceInfo)
    }


    /// ```text
    /// /**
    ///    * Callback when the a service is lost.
    ///    * @param   aServiceInfo
    ///    *          the info about the lost service, where |serviceName|, |aServiceType|, and |domainName| are set.
    ///    */
    /// ```
    ///

    /// `void onServiceLost (in nsIDNSServiceInfo aServiceInfo);`
    #[inline]
    pub unsafe fn OnServiceLost(&self, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnServiceLost)(self, aServiceInfo)
    }


    /// ```text
    /// /**
    ///    * Callback when the discovery cannot start.
    ///    * @param   aServiceType
    ///    *          the service type of |startDiscovery|.
    ///    * @param   aErrorCode
    ///    *          the error code.
    ///    */
    /// ```
    ///

    /// `void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode);`
    #[inline]
    pub unsafe fn OnStartDiscoveryFailed(&self, aServiceType: *const ::nsstring::nsACString, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).OnStartDiscoveryFailed)(self, aServiceType, aErrorCode)
    }


    /// ```text
    /// /**
    ///    * Callback when the discovery cannot stop.
    ///    * @param   aServiceType
    ///    *          the service type of |startDiscovery|.
    ///    * @param   aErrorCode
    ///    *          the error code.
    ///    */
    /// ```
    ///

    /// `void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode);`
    #[inline]
    pub unsafe fn OnStopDiscoveryFailed(&self, aServiceType: *const ::nsstring::nsACString, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).OnStopDiscoveryFailed)(self, aServiceType, aErrorCode)
    }


}


/// `interface nsIDNSRegistrationListener : nsISupports`
///

/// ```text
/// /**
///  * The callback interface for service registration
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSRegistrationListener {
    vtable: *const nsIDNSRegistrationListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSRegistrationListener.
unsafe impl XpCom for nsIDNSRegistrationListener {
    const IID: nsIID = nsID(0xe165e4be, 0xabf4, 0x4963,
        [0xa6, 0x6d, 0xed, 0x3c, 0xa1, 0x16, 0xe5, 0xe4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSRegistrationListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSRegistrationListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSRegistrationListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSRegistrationListener`.
    fn coerce_from(v: &nsIDNSRegistrationListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSRegistrationListenerCoerce for nsIDNSRegistrationListener {
    #[inline]
    fn coerce_from(v: &nsIDNSRegistrationListener) -> &Self {
        v
    }
}

impl nsIDNSRegistrationListener {
    /// Cast this `nsIDNSRegistrationListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSRegistrationListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSRegistrationListener {
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
impl<T: nsISupportsCoerce> nsIDNSRegistrationListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSRegistrationListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSRegistrationListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSRegistrationListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo); */
    pub OnServiceRegistered: unsafe extern "system" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult,

    /* void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo); */
    pub OnServiceUnregistered: unsafe extern "system" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult,

    /* void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    pub OnRegistrationFailed: unsafe extern "system" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: i32) -> ::nserror::nsresult,

    /* void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    pub OnUnregistrationFailed: unsafe extern "system" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSRegistrationListener {

    pub const ERROR_SERVICE_NOT_RUNNING: i64 = -65563;

    /// ```text
    /// /**
    ///    * Callback when the service is registered successfully.
    ///    * @param   aServiceInfo
    ///    *          the info about the registered service,
    ///    *          where |serviceName|, |aServiceType|, and |domainName| are set.
    ///    */
    /// ```
    ///

    /// `void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo);`
    #[inline]
    pub unsafe fn OnServiceRegistered(&self, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnServiceRegistered)(self, aServiceInfo)
    }


    /// ```text
    /// /**
    ///    * Callback when the service is unregistered successfully.
    ///    * @param   aServiceInfo
    ///    *          the info about the unregistered service.
    ///    */
    /// ```
    ///

    /// `void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo);`
    #[inline]
    pub unsafe fn OnServiceUnregistered(&self, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnServiceUnregistered)(self, aServiceInfo)
    }


    /// ```text
    /// /**
    ///    * Callback when the service cannot be registered.
    ///    * @param   aServiceInfo
    ///    *          the info about the service to be registered.
    ///    * @param   aErrorCode
    ///    *          the error code.
    ///    */
    /// ```
    ///

    /// `void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode);`
    #[inline]
    pub unsafe fn OnRegistrationFailed(&self, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).OnRegistrationFailed)(self, aServiceInfo, aErrorCode)
    }


    /// ```text
    /// /**
    ///    * Callback when the service cannot be unregistered.
    ///    * @param   aServiceInfo
    ///    *          the info about the registered service.
    ///    * @param   aErrorCode
    ///    *          the error code.
    ///    */
    /// ```
    ///

    /// `void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode);`
    #[inline]
    pub unsafe fn OnUnregistrationFailed(&self, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).OnUnregistrationFailed)(self, aServiceInfo, aErrorCode)
    }


}


/// `interface nsIDNSServiceResolveListener : nsISupports`
///

/// ```text
/// /**
///  * The callback interface for service resolve
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSServiceResolveListener {
    vtable: *const nsIDNSServiceResolveListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSServiceResolveListener.
unsafe impl XpCom for nsIDNSServiceResolveListener {
    const IID: nsIID = nsID(0x24ee6408, 0x648e, 0x421d,
        [0xac, 0xcf, 0xc6, 0xe5, 0xad, 0xec, 0xcf, 0x97]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSServiceResolveListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSServiceResolveListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSServiceResolveListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSServiceResolveListener`.
    fn coerce_from(v: &nsIDNSServiceResolveListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSServiceResolveListenerCoerce for nsIDNSServiceResolveListener {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceResolveListener) -> &Self {
        v
    }
}

impl nsIDNSServiceResolveListener {
    /// Cast this `nsIDNSServiceResolveListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSServiceResolveListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSServiceResolveListener {
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
impl<T: nsISupportsCoerce> nsIDNSServiceResolveListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceResolveListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSServiceResolveListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSServiceResolveListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onServiceResolved (in nsIDNSServiceInfo aServiceInfo); */
    pub OnServiceResolved: unsafe extern "system" fn (this: *const nsIDNSServiceResolveListener, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult,

    /* void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    pub OnResolveFailed: unsafe extern "system" fn (this: *const nsIDNSServiceResolveListener, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSServiceResolveListener {

    /// ```text
    /// /**
    ///    * Callback when the service is resolved successfully.
    ///    * @param   aServiceInfo
    ///    *          the info about the resolved service, where |host| and |port| are set.
    ///    */
    /// ```
    ///

    /// `void onServiceResolved (in nsIDNSServiceInfo aServiceInfo);`
    #[inline]
    pub unsafe fn OnServiceResolved(&self, aServiceInfo: *const nsIDNSServiceInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnServiceResolved)(self, aServiceInfo)
    }


    /// ```text
    /// /**
    ///    * Callback when the service cannot be resolved.
    ///    * @param   aServiceInfo
    ///    *          the info about the service to be resolved.
    ///    * @param   aErrorCode
    ///    *          the error code.
    ///    */
    /// ```
    ///

    /// `void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode);`
    #[inline]
    pub unsafe fn OnResolveFailed(&self, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).OnResolveFailed)(self, aServiceInfo, aErrorCode)
    }


}


/// `interface nsIDNSServiceDiscovery : nsISupports`
///

/// ```text
/// /**
///  * The interface for DNS service discovery/registration/resolve
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSServiceDiscovery {
    vtable: *const nsIDNSServiceDiscoveryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSServiceDiscovery.
unsafe impl XpCom for nsIDNSServiceDiscovery {
    const IID: nsIID = nsID(0x6487899b, 0xbeb1, 0x455a,
        [0xba, 0x65, 0xe4, 0xfd, 0x46, 0x50, 0x66, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSServiceDiscovery {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSServiceDiscovery.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSServiceDiscoveryCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSServiceDiscovery`.
    fn coerce_from(v: &nsIDNSServiceDiscovery) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSServiceDiscoveryCoerce for nsIDNSServiceDiscovery {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscovery) -> &Self {
        v
    }
}

impl nsIDNSServiceDiscovery {
    /// Cast this `nsIDNSServiceDiscovery` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSServiceDiscoveryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSServiceDiscovery {
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
impl<T: nsISupportsCoerce> nsIDNSServiceDiscoveryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscovery) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSServiceDiscovery
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSServiceDiscoveryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener); */
    pub StartDiscovery: unsafe extern "system" fn (this: *const nsIDNSServiceDiscovery, aServiceType: *const ::nsstring::nsACString, aListener: *const nsIDNSServiceDiscoveryListener, _retval: *mut*const nsICancelable) -> ::nserror::nsresult,

    /* nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener); */
    pub RegisterService: unsafe extern "system" fn (this: *const nsIDNSServiceDiscovery, aServiceInfo: *const nsIDNSServiceInfo, aListener: *const nsIDNSRegistrationListener, _retval: *mut*const nsICancelable) -> ::nserror::nsresult,

    /* void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener); */
    pub ResolveService: unsafe extern "system" fn (this: *const nsIDNSServiceDiscovery, aServiceInfo: *const nsIDNSServiceInfo, aListener: *const nsIDNSServiceResolveListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSServiceDiscovery {

    /// ```text
    /// /**
    ///    * Browse for instances of a service.
    ///    * @param   aServiceType
    ///    *          the service type to be discovered, E.g. "_http._tcp".
    ///    * @param   aListener
    ///    *          callback interface for discovery notifications.
    ///    * @return  An object that can be used to cancel the service discovery.
    ///    */
    /// ```
    ///

    /// `nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener);`
    #[inline]
    pub unsafe fn StartDiscovery(&self, aServiceType: *const ::nsstring::nsACString, aListener: *const nsIDNSServiceDiscoveryListener, _retval: *mut*const nsICancelable) -> ::nserror::nsresult {
        ((*self.vtable).StartDiscovery)(self, aServiceType, aListener, _retval)
    }


    /// ```text
    /// /**
    ///    * Register a service that is discovered via |startDiscovery| and |resolveService| calls.
    ///    * @param   aServiceInfo
    ///    *          the service information to be registered.
    ///    *          |port| and |aServiceType| are required attributes.
    ///    * @param   aListener
    ///    *          callback interface for registration notifications.
    ///    * @return  An object that can be used to cancel the service registration.
    ///    */
    /// ```
    ///

    /// `nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener);`
    #[inline]
    pub unsafe fn RegisterService(&self, aServiceInfo: *const nsIDNSServiceInfo, aListener: *const nsIDNSRegistrationListener, _retval: *mut*const nsICancelable) -> ::nserror::nsresult {
        ((*self.vtable).RegisterService)(self, aServiceInfo, aListener, _retval)
    }


    /// ```text
    /// /**
    ///    * Resolve a service name discovered via |startDiscovery| to a target host name, port number.
    ///    * @param   aServiceInfo
    ///    *          the service information to be registered.
    ///    *          |serviceName|, |aServiceType|, and |domainName| are required attributes as reported to the |onServiceFound| callback.
    ///    * @param   aListener
    ///    *          callback interface for registration notifications.
    ///    */
    /// ```
    ///

    /// `void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener);`
    #[inline]
    pub unsafe fn ResolveService(&self, aServiceInfo: *const nsIDNSServiceInfo, aListener: *const nsIDNSServiceResolveListener) -> ::nserror::nsresult {
        ((*self.vtable).ResolveService)(self, aServiceInfo, aListener)
    }


}


