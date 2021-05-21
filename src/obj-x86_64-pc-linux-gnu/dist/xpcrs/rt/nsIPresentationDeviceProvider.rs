//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDeviceProvider.idl
//


/// `interface nsIPresentationDeviceListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDeviceListener {
    vtable: *const nsIPresentationDeviceListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDeviceListener.
unsafe impl XpCom for nsIPresentationDeviceListener {
    const IID: nsIID = nsID(0x46fd372b, 0x2e40, 0x4179,
        [0x9b, 0x36, 0x04, 0x78, 0xd1, 0x41, 0xe4, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDeviceListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDeviceListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDeviceListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDeviceListener`.
    fn coerce_from(v: &nsIPresentationDeviceListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDeviceListenerCoerce for nsIPresentationDeviceListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceListener) -> &Self {
        v
    }
}

impl nsIPresentationDeviceListener {
    /// Cast this `nsIPresentationDeviceListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDeviceListener {
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
impl<T: nsISupportsCoerce> nsIPresentationDeviceListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDeviceListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDeviceListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addDevice (in nsIPresentationDevice device); */
    pub AddDevice: unsafe extern "system" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice) -> ::nserror::nsresult,

    /* void removeDevice (in nsIPresentationDevice device); */
    pub RemoveDevice: unsafe extern "system" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice) -> ::nserror::nsresult,

    /* void updateDevice (in nsIPresentationDevice device); */
    pub UpdateDevice: unsafe extern "system" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice) -> ::nserror::nsresult,

    /* void onSessionRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel); */
    pub OnSessionRequest: unsafe extern "system" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice, url: *const ::nsstring::nsAString, presentationId: *const ::nsstring::nsAString, controlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult,

    /* void onTerminateRequest (in nsIPresentationDevice device, in AString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver); */
    pub OnTerminateRequest: unsafe extern "system" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice, presentationId: *const ::nsstring::nsAString, controlChannel: *const nsIPresentationControlChannel, aIsFromReceiver: bool) -> ::nserror::nsresult,

    /* void onReconnectRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel); */
    pub OnReconnectRequest: unsafe extern "system" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice, url: *const ::nsstring::nsAString, presentationId: *const ::nsstring::nsAString, controlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDeviceListener {


    /// `void addDevice (in nsIPresentationDevice device);`
    #[inline]
    pub unsafe fn AddDevice(&self, device: *const nsIPresentationDevice) -> ::nserror::nsresult {
        ((*self.vtable).AddDevice)(self, device)
    }



    /// `void removeDevice (in nsIPresentationDevice device);`
    #[inline]
    pub unsafe fn RemoveDevice(&self, device: *const nsIPresentationDevice) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDevice)(self, device)
    }



    /// `void updateDevice (in nsIPresentationDevice device);`
    #[inline]
    pub unsafe fn UpdateDevice(&self, device: *const nsIPresentationDevice) -> ::nserror::nsresult {
        ((*self.vtable).UpdateDevice)(self, device)
    }



    /// `void onSessionRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel);`
    #[inline]
    pub unsafe fn OnSessionRequest(&self, device: *const nsIPresentationDevice, url: *const ::nsstring::nsAString, presentationId: *const ::nsstring::nsAString, controlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).OnSessionRequest)(self, device, url, presentationId, controlChannel)
    }



    /// `void onTerminateRequest (in nsIPresentationDevice device, in AString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver);`
    #[inline]
    pub unsafe fn OnTerminateRequest(&self, device: *const nsIPresentationDevice, presentationId: *const ::nsstring::nsAString, controlChannel: *const nsIPresentationControlChannel, aIsFromReceiver: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnTerminateRequest)(self, device, presentationId, controlChannel, aIsFromReceiver)
    }



    /// `void onReconnectRequest (in nsIPresentationDevice device, in AString url, in AString presentationId, in nsIPresentationControlChannel controlChannel);`
    #[inline]
    pub unsafe fn OnReconnectRequest(&self, device: *const nsIPresentationDevice, url: *const ::nsstring::nsAString, presentationId: *const ::nsstring::nsAString, controlChannel: *const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).OnReconnectRequest)(self, device, url, presentationId, controlChannel)
    }


}


/// `interface nsIPresentationDeviceProvider : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDeviceProvider {
    vtable: *const nsIPresentationDeviceProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDeviceProvider.
unsafe impl XpCom for nsIPresentationDeviceProvider {
    const IID: nsIID = nsID(0x3db2578a, 0x0f50, 0x44ad,
        [0xb0, 0x1b, 0x28, 0x42, 0x7b, 0x71, 0xb7, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDeviceProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDeviceProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDeviceProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDeviceProvider`.
    fn coerce_from(v: &nsIPresentationDeviceProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDeviceProviderCoerce for nsIPresentationDeviceProvider {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceProvider) -> &Self {
        v
    }
}

impl nsIPresentationDeviceProvider {
    /// Cast this `nsIPresentationDeviceProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDeviceProvider {
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
impl<T: nsISupportsCoerce> nsIPresentationDeviceProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDeviceProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDeviceProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIPresentationDeviceListener listener; */
    pub GetListener: unsafe extern "system" fn (this: *const nsIPresentationDeviceProvider, aListener: *mut *const nsIPresentationDeviceListener) -> ::nserror::nsresult,

    /* attribute nsIPresentationDeviceListener listener; */
    pub SetListener: unsafe extern "system" fn (this: *const nsIPresentationDeviceProvider, aListener: *const nsIPresentationDeviceListener) -> ::nserror::nsresult,

    /* void forceDiscovery (); */
    pub ForceDiscovery: unsafe extern "system" fn (this: *const nsIPresentationDeviceProvider) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDeviceProvider {


    /// `attribute nsIPresentationDeviceListener listener;`
    #[inline]
    pub unsafe fn GetListener(&self, aListener: *mut *const nsIPresentationDeviceListener) -> ::nserror::nsresult {
        ((*self.vtable).GetListener)(self, aListener)
    }



    /// `attribute nsIPresentationDeviceListener listener;`
    #[inline]
    pub unsafe fn SetListener(&self, aListener: *const nsIPresentationDeviceListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, aListener)
    }



    /// `void forceDiscovery ();`
    #[inline]
    pub unsafe fn ForceDiscovery(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ForceDiscovery)(self, )
    }


}


