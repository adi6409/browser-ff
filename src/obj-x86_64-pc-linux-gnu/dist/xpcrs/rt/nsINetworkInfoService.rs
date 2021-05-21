//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkInfoService.idl
//


/// `interface nsIListNetworkAddressesListener : nsISupports`
///

/// ```text
/// /**
///  * Listener for getting list of addresses.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIListNetworkAddressesListener {
    vtable: *const nsIListNetworkAddressesListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIListNetworkAddressesListener.
unsafe impl XpCom for nsIListNetworkAddressesListener {
    const IID: nsIID = nsID(0xc4bdaac1, 0x3ab1, 0x4fdb,
        [0x9a, 0x16, 0x17, 0xcb, 0xed, 0x79, 0x46, 0x03]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIListNetworkAddressesListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIListNetworkAddressesListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIListNetworkAddressesListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIListNetworkAddressesListener`.
    fn coerce_from(v: &nsIListNetworkAddressesListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIListNetworkAddressesListenerCoerce for nsIListNetworkAddressesListener {
    #[inline]
    fn coerce_from(v: &nsIListNetworkAddressesListener) -> &Self {
        v
    }
}

impl nsIListNetworkAddressesListener {
    /// Cast this `nsIListNetworkAddressesListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIListNetworkAddressesListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIListNetworkAddressesListener {
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
impl<T: nsISupportsCoerce> nsIListNetworkAddressesListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIListNetworkAddressesListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIListNetworkAddressesListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIListNetworkAddressesListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onListedNetworkAddresses (in Array<ACString> aAddressArray); */
    pub OnListedNetworkAddresses: unsafe extern "system" fn (this: *const nsIListNetworkAddressesListener, aAddressArray: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void onListNetworkAddressesFailed (); */
    pub OnListNetworkAddressesFailed: unsafe extern "system" fn (this: *const nsIListNetworkAddressesListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIListNetworkAddressesListener {

    /// ```text
    /// /**
    ///    * Callback function that gets called by nsINetworkInfoService.listNetworkAddresses.
    ///    * Each address in the array is a string IP address in canonical form,
    ///    * e.g. 192.168.1.10, or an IPV6 address in string form.
    ///    */
    /// ```
    ///

    /// `void onListedNetworkAddresses (in Array<ACString> aAddressArray);`
    #[inline]
    pub unsafe fn OnListedNetworkAddresses(&self, aAddressArray: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).OnListedNetworkAddresses)(self, aAddressArray)
    }



    /// `void onListNetworkAddressesFailed ();`
    #[inline]
    pub unsafe fn OnListNetworkAddressesFailed(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnListNetworkAddressesFailed)(self, )
    }


}


/// `interface nsIGetHostnameListener : nsISupports`
///

/// ```text
/// /**
///  * Listener for getting hostname.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGetHostnameListener {
    vtable: *const nsIGetHostnameListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGetHostnameListener.
unsafe impl XpCom for nsIGetHostnameListener {
    const IID: nsIID = nsID(0x3ebdcb62, 0x2df4, 0x4042,
        [0x88, 0x64, 0x3f, 0xa8, 0x1a, 0xbd, 0x46, 0x93]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGetHostnameListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGetHostnameListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGetHostnameListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIGetHostnameListener`.
    fn coerce_from(v: &nsIGetHostnameListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGetHostnameListenerCoerce for nsIGetHostnameListener {
    #[inline]
    fn coerce_from(v: &nsIGetHostnameListener) -> &Self {
        v
    }
}

impl nsIGetHostnameListener {
    /// Cast this `nsIGetHostnameListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGetHostnameListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGetHostnameListener {
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
impl<T: nsISupportsCoerce> nsIGetHostnameListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGetHostnameListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGetHostnameListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGetHostnameListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onGotHostname (in AUTF8String aHostname); */
    pub OnGotHostname: unsafe extern "system" fn (this: *const nsIGetHostnameListener, aHostname: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void onGetHostnameFailed (); */
    pub OnGetHostnameFailed: unsafe extern "system" fn (this: *const nsIGetHostnameListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGetHostnameListener {


    /// `void onGotHostname (in AUTF8String aHostname);`
    #[inline]
    pub unsafe fn OnGotHostname(&self, aHostname: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnGotHostname)(self, aHostname)
    }



    /// `void onGetHostnameFailed ();`
    #[inline]
    pub unsafe fn OnGetHostnameFailed(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnGetHostnameFailed)(self, )
    }


}


/// `interface nsINetworkInfoService : nsISupports`
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
pub struct nsINetworkInfoService {
    vtable: *const nsINetworkInfoServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetworkInfoService.
unsafe impl XpCom for nsINetworkInfoService {
    const IID: nsIID = nsID(0x55fc8dae, 0x4a58, 0x4e0f,
        [0xa4, 0x9b, 0x90, 0x1c, 0xba, 0xba, 0xe8, 0x09]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetworkInfoService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetworkInfoService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetworkInfoServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINetworkInfoService`.
    fn coerce_from(v: &nsINetworkInfoService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetworkInfoServiceCoerce for nsINetworkInfoService {
    #[inline]
    fn coerce_from(v: &nsINetworkInfoService) -> &Self {
        v
    }
}

impl nsINetworkInfoService {
    /// Cast this `nsINetworkInfoService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetworkInfoServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetworkInfoService {
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
impl<T: nsISupportsCoerce> nsINetworkInfoServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkInfoService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetworkInfoService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetworkInfoServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void listNetworkAddresses (in nsIListNetworkAddressesListener aListener); */
    pub ListNetworkAddresses: unsafe extern "system" fn (this: *const nsINetworkInfoService, aListener: *const nsIListNetworkAddressesListener) -> ::nserror::nsresult,

    /* void getHostname (in nsIGetHostnameListener aListener); */
    pub GetHostname: unsafe extern "system" fn (this: *const nsINetworkInfoService, aListener: *const nsIGetHostnameListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetworkInfoService {

    /// ```text
    /// /**
    ///    * Obtain a list of local machine network addresses.  The listener object's
    ///    * onListedNetworkAddresses will be called with the obtained addresses.
    ///    * On failure, the listener object's onListNetworkAddressesFailed() will be called.
    ///    */
    /// ```
    ///

    /// `void listNetworkAddresses (in nsIListNetworkAddressesListener aListener);`
    #[inline]
    pub unsafe fn ListNetworkAddresses(&self, aListener: *const nsIListNetworkAddressesListener) -> ::nserror::nsresult {
        ((*self.vtable).ListNetworkAddresses)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * Obtain the hostname of the local machine.  The listener object's
    ///    * onGotHostname will be called with the obtained hostname.
    ///    * On failure, the listener object's onGetHostnameFailed() will be called.
    ///    */
    /// ```
    ///

    /// `void getHostname (in nsIGetHostnameListener aListener);`
    #[inline]
    pub unsafe fn GetHostname(&self, aListener: *const nsIGetHostnameListener) -> ::nserror::nsresult {
        ((*self.vtable).GetHostname)(self, aListener)
    }


}


