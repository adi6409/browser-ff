//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationControlChannel.idl
//


/// `interface nsIPresentationChannelDescription : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationChannelDescription {
    vtable: *const nsIPresentationChannelDescriptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationChannelDescription.
unsafe impl XpCom for nsIPresentationChannelDescription {
    const IID: nsIID = nsID(0xae318e05, 0x2a4e, 0x4f85,
        [0x95, 0xc0, 0xe8, 0xb1, 0x91, 0xad, 0x81, 0x2c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationChannelDescription {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationChannelDescription.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationChannelDescriptionCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationChannelDescription`.
    fn coerce_from(v: &nsIPresentationChannelDescription) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationChannelDescriptionCoerce for nsIPresentationChannelDescription {
    #[inline]
    fn coerce_from(v: &nsIPresentationChannelDescription) -> &Self {
        v
    }
}

impl nsIPresentationChannelDescription {
    /// Cast this `nsIPresentationChannelDescription` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationChannelDescriptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationChannelDescription {
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
impl<T: nsISupportsCoerce> nsIPresentationChannelDescriptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationChannelDescription) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationChannelDescription
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationChannelDescriptionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint8_t type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIPresentationChannelDescription, aType: *mut uint8_t) -> ::nserror::nsresult,

    /* readonly attribute nsIArray tcpAddress; */
    pub GetTcpAddress: unsafe extern "system" fn (this: *const nsIPresentationChannelDescription, aTcpAddress: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute uint16_t tcpPort; */
    pub GetTcpPort: unsafe extern "system" fn (this: *const nsIPresentationChannelDescription, aTcpPort: *mut uint16_t) -> ::nserror::nsresult,

    /* readonly attribute AString dataChannelSDP; */
    pub GetDataChannelSDP: unsafe extern "system" fn (this: *const nsIPresentationChannelDescription, aDataChannelSDP: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationChannelDescription {

    pub const TYPE_TCP: i64 = 1;


    pub const TYPE_DATACHANNEL: i64 = 2;


    /// `readonly attribute uint8_t type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `readonly attribute nsIArray tcpAddress;`
    #[inline]
    pub unsafe fn GetTcpAddress(&self, aTcpAddress: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetTcpAddress)(self, aTcpAddress)
    }



    /// `readonly attribute uint16_t tcpPort;`
    #[inline]
    pub unsafe fn GetTcpPort(&self, aTcpPort: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTcpPort)(self, aTcpPort)
    }



    /// `readonly attribute AString dataChannelSDP;`
    #[inline]
    pub unsafe fn GetDataChannelSDP(&self, aDataChannelSDP: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDataChannelSDP)(self, aDataChannelSDP)
    }


}


/// `interface nsIPresentationControlChannelListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationControlChannelListener {
    vtable: *const nsIPresentationControlChannelListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationControlChannelListener.
unsafe impl XpCom for nsIPresentationControlChannelListener {
    const IID: nsIID = nsID(0x96dd548f, 0x7d0f, 0x43c1,
        [0xb1, 0xad, 0x28, 0xe6, 0x66, 0xcf, 0x1e, 0x82]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationControlChannelListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationControlChannelListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationControlChannelListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationControlChannelListener`.
    fn coerce_from(v: &nsIPresentationControlChannelListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationControlChannelListenerCoerce for nsIPresentationControlChannelListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannelListener) -> &Self {
        v
    }
}

impl nsIPresentationControlChannelListener {
    /// Cast this `nsIPresentationControlChannelListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationControlChannelListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationControlChannelListener {
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
impl<T: nsISupportsCoerce> nsIPresentationControlChannelListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannelListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationControlChannelListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationControlChannelListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onOffer (in nsIPresentationChannelDescription offer); */
    pub OnOffer: unsafe extern "system" fn (this: *const nsIPresentationControlChannelListener, offer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void onAnswer (in nsIPresentationChannelDescription answer); */
    pub OnAnswer: unsafe extern "system" fn (this: *const nsIPresentationControlChannelListener, answer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void onIceCandidate (in AString candidate); */
    pub OnIceCandidate: unsafe extern "system" fn (this: *const nsIPresentationControlChannelListener, candidate: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void notifyConnected (); */
    pub NotifyConnected: unsafe extern "system" fn (this: *const nsIPresentationControlChannelListener) -> ::nserror::nsresult,

    /* void notifyDisconnected (in nsresult reason); */
    pub NotifyDisconnected: unsafe extern "system" fn (this: *const nsIPresentationControlChannelListener, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void notifyReconnected (); */
    pub NotifyReconnected: unsafe extern "system" fn (this: *const nsIPresentationControlChannelListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationControlChannelListener {


    /// `void onOffer (in nsIPresentationChannelDescription offer);`
    #[inline]
    pub unsafe fn OnOffer(&self, offer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult {
        ((*self.vtable).OnOffer)(self, offer)
    }



    /// `void onAnswer (in nsIPresentationChannelDescription answer);`
    #[inline]
    pub unsafe fn OnAnswer(&self, answer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult {
        ((*self.vtable).OnAnswer)(self, answer)
    }



    /// `void onIceCandidate (in AString candidate);`
    #[inline]
    pub unsafe fn OnIceCandidate(&self, candidate: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnIceCandidate)(self, candidate)
    }



    /// `void notifyConnected ();`
    #[inline]
    pub unsafe fn NotifyConnected(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyConnected)(self, )
    }



    /// `void notifyDisconnected (in nsresult reason);`
    #[inline]
    pub unsafe fn NotifyDisconnected(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).NotifyDisconnected)(self, reason)
    }



    /// `void notifyReconnected ();`
    #[inline]
    pub unsafe fn NotifyReconnected(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyReconnected)(self, )
    }


}


/// `interface nsIPresentationControlChannel : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationControlChannel {
    vtable: *const nsIPresentationControlChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationControlChannel.
unsafe impl XpCom for nsIPresentationControlChannel {
    const IID: nsIID = nsID(0xe60e208c, 0xa9f5, 0x4bc6,
        [0x9a, 0x3e, 0x47, 0xf3, 0xe4, 0xae, 0x9c, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationControlChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationControlChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationControlChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationControlChannel`.
    fn coerce_from(v: &nsIPresentationControlChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationControlChannelCoerce for nsIPresentationControlChannel {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannel) -> &Self {
        v
    }
}

impl nsIPresentationControlChannel {
    /// Cast this `nsIPresentationControlChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationControlChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationControlChannel {
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
impl<T: nsISupportsCoerce> nsIPresentationControlChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationControlChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationControlChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIPresentationControlChannelListener listener; */
    pub GetListener: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, aListener: *mut *const nsIPresentationControlChannelListener) -> ::nserror::nsresult,

    /* attribute nsIPresentationControlChannelListener listener; */
    pub SetListener: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, aListener: *const nsIPresentationControlChannelListener) -> ::nserror::nsresult,

    /* void sendOffer (in nsIPresentationChannelDescription offer); */
    pub SendOffer: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, offer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
    pub SendAnswer: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, answer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void sendIceCandidate (in AString candidate); */
    pub SendIceCandidate: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, candidate: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void launch (in AString presentationId, in AString url); */
    pub Launch: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, presentationId: *const ::nsstring::nsAString, url: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void terminate (in AString presentationId); */
    pub Terminate: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, presentationId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void disconnect (in nsresult reason); */
    pub Disconnect: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void reconnect (in AString presentationId, in AString url); */
    pub Reconnect: unsafe extern "system" fn (this: *const nsIPresentationControlChannel, presentationId: *const ::nsstring::nsAString, url: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationControlChannel {


    /// `attribute nsIPresentationControlChannelListener listener;`
    #[inline]
    pub unsafe fn GetListener(&self, aListener: *mut *const nsIPresentationControlChannelListener) -> ::nserror::nsresult {
        ((*self.vtable).GetListener)(self, aListener)
    }



    /// `attribute nsIPresentationControlChannelListener listener;`
    #[inline]
    pub unsafe fn SetListener(&self, aListener: *const nsIPresentationControlChannelListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, aListener)
    }



    /// `void sendOffer (in nsIPresentationChannelDescription offer);`
    #[inline]
    pub unsafe fn SendOffer(&self, offer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult {
        ((*self.vtable).SendOffer)(self, offer)
    }



    /// `void sendAnswer (in nsIPresentationChannelDescription answer);`
    #[inline]
    pub unsafe fn SendAnswer(&self, answer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult {
        ((*self.vtable).SendAnswer)(self, answer)
    }



    /// `void sendIceCandidate (in AString candidate);`
    #[inline]
    pub unsafe fn SendIceCandidate(&self, candidate: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SendIceCandidate)(self, candidate)
    }



    /// `void launch (in AString presentationId, in AString url);`
    #[inline]
    pub unsafe fn Launch(&self, presentationId: *const ::nsstring::nsAString, url: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Launch)(self, presentationId, url)
    }



    /// `void terminate (in AString presentationId);`
    #[inline]
    pub unsafe fn Terminate(&self, presentationId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Terminate)(self, presentationId)
    }



    /// `void disconnect (in nsresult reason);`
    #[inline]
    pub unsafe fn Disconnect(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Disconnect)(self, reason)
    }



    /// `void reconnect (in AString presentationId, in AString url);`
    #[inline]
    pub unsafe fn Reconnect(&self, presentationId: *const ::nsstring::nsAString, url: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Reconnect)(self, presentationId, url)
    }


}


