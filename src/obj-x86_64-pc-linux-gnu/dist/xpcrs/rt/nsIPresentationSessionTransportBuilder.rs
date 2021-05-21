//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionTransportBuilder.idl
//


/// `interface nsIPresentationSessionTransportBuilderListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationSessionTransportBuilderListener {
    vtable: *const nsIPresentationSessionTransportBuilderListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationSessionTransportBuilderListener.
unsafe impl XpCom for nsIPresentationSessionTransportBuilderListener {
    const IID: nsIID = nsID(0x673f6de1, 0xe253, 0x41b8,
        [0x9b, 0xe8, 0xb7, 0xff, 0x16, 0x1f, 0xa8, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationSessionTransportBuilderListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationSessionTransportBuilderListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationSessionTransportBuilderListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationSessionTransportBuilderListener`.
    fn coerce_from(v: &nsIPresentationSessionTransportBuilderListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationSessionTransportBuilderListenerCoerce for nsIPresentationSessionTransportBuilderListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilderListener) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransportBuilderListener {
    /// Cast this `nsIPresentationSessionTransportBuilderListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportBuilderListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationSessionTransportBuilderListener {
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
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportBuilderListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilderListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationSessionTransportBuilderListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationSessionTransportBuilderListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onSessionTransport (in nsIPresentationSessionTransport transport); */
    pub OnSessionTransport: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportBuilderListener, transport: *const nsIPresentationSessionTransport) -> ::nserror::nsresult,

    /* void onError (in nsresult reason); */
    pub OnError: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportBuilderListener, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void sendOffer (in nsIPresentationChannelDescription offer); */
    pub SendOffer: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportBuilderListener, offer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
    pub SendAnswer: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportBuilderListener, answer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void sendIceCandidate (in AString candidate); */
    pub SendIceCandidate: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportBuilderListener, candidate: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void close (in nsresult reason); */
    pub Close: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportBuilderListener, reason: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationSessionTransportBuilderListener {


    /// `void onSessionTransport (in nsIPresentationSessionTransport transport);`
    #[inline]
    pub unsafe fn OnSessionTransport(&self, transport: *const nsIPresentationSessionTransport) -> ::nserror::nsresult {
        ((*self.vtable).OnSessionTransport)(self, transport)
    }



    /// `void onError (in nsresult reason);`
    #[inline]
    pub unsafe fn OnError(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnError)(self, reason)
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



    /// `void close (in nsresult reason);`
    #[inline]
    pub unsafe fn Close(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, reason)
    }


}


/// `interface nsIPresentationSessionTransportBuilder : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationSessionTransportBuilder {
    vtable: *const nsIPresentationSessionTransportBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationSessionTransportBuilder.
unsafe impl XpCom for nsIPresentationSessionTransportBuilder {
    const IID: nsIID = nsID(0x2fdbe67d, 0x80f9, 0x48dc,
        [0x82, 0x37, 0x5b, 0xef, 0x8f, 0xa1, 0x98, 0x01]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationSessionTransportBuilder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationSessionTransportBuilder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationSessionTransportBuilderCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationSessionTransportBuilder`.
    fn coerce_from(v: &nsIPresentationSessionTransportBuilder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationSessionTransportBuilderCoerce for nsIPresentationSessionTransportBuilder {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilder) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransportBuilder {
    /// Cast this `nsIPresentationSessionTransportBuilder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationSessionTransportBuilder {
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
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationSessionTransportBuilder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationSessionTransportBuilderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationSessionTransportBuilder {


}


/// `interface nsIPresentationTransportBuilderConstructor : nsISupports`
///

/// ```text
/// /**
///  * The constructor for creating a transport builder.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationTransportBuilderConstructor {
    vtable: *const nsIPresentationTransportBuilderConstructorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationTransportBuilderConstructor.
unsafe impl XpCom for nsIPresentationTransportBuilderConstructor {
    const IID: nsIID = nsID(0x706482b2, 0x1b51, 0x4bed,
        [0xa2, 0x1d, 0x78, 0x5a, 0x9c, 0xfc, 0xfa, 0xc7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationTransportBuilderConstructor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationTransportBuilderConstructor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationTransportBuilderConstructorCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationTransportBuilderConstructor`.
    fn coerce_from(v: &nsIPresentationTransportBuilderConstructor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationTransportBuilderConstructorCoerce for nsIPresentationTransportBuilderConstructor {
    #[inline]
    fn coerce_from(v: &nsIPresentationTransportBuilderConstructor) -> &Self {
        v
    }
}

impl nsIPresentationTransportBuilderConstructor {
    /// Cast this `nsIPresentationTransportBuilderConstructor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationTransportBuilderConstructorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationTransportBuilderConstructor {
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
impl<T: nsISupportsCoerce> nsIPresentationTransportBuilderConstructorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationTransportBuilderConstructor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationTransportBuilderConstructor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationTransportBuilderConstructorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIPresentationSessionTransportBuilder createTransportBuilder (in uint8_t type); */
    pub CreateTransportBuilder: unsafe extern "system" fn (this: *const nsIPresentationTransportBuilderConstructor, type_: uint8_t, _retval: *mut *const nsIPresentationSessionTransportBuilder) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationTransportBuilderConstructor {


    /// `nsIPresentationSessionTransportBuilder createTransportBuilder (in uint8_t type);`
    #[inline]
    pub unsafe fn CreateTransportBuilder(&self, type_: uint8_t, _retval: *mut *const nsIPresentationSessionTransportBuilder) -> ::nserror::nsresult {
        ((*self.vtable).CreateTransportBuilder)(self, type_, _retval)
    }


}


/// `interface nsIPresentationTCPSessionTransportBuilder : nsIPresentationSessionTransportBuilder`
///

/// ```text
/// /**
///  * Builder for TCP session transport
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationTCPSessionTransportBuilder {
    vtable: *const nsIPresentationTCPSessionTransportBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationTCPSessionTransportBuilder.
unsafe impl XpCom for nsIPresentationTCPSessionTransportBuilder {
    const IID: nsIID = nsID(0xcde36d6e, 0xf471, 0x4262,
        [0xa7, 0x0d, 0xf9, 0x32, 0xa2, 0x6b, 0x21, 0xd9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationTCPSessionTransportBuilder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationTCPSessionTransportBuilder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationTCPSessionTransportBuilderCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationTCPSessionTransportBuilder`.
    fn coerce_from(v: &nsIPresentationTCPSessionTransportBuilder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationTCPSessionTransportBuilderCoerce for nsIPresentationTCPSessionTransportBuilder {
    #[inline]
    fn coerce_from(v: &nsIPresentationTCPSessionTransportBuilder) -> &Self {
        v
    }
}

impl nsIPresentationTCPSessionTransportBuilder {
    /// Cast this `nsIPresentationTCPSessionTransportBuilder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationTCPSessionTransportBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationTCPSessionTransportBuilder {
    type Target = nsIPresentationSessionTransportBuilder;
    #[inline]
    fn deref(&self) -> &nsIPresentationSessionTransportBuilder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPresentationSessionTransportBuilderCoerce> nsIPresentationTCPSessionTransportBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationTCPSessionTransportBuilder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationTCPSessionTransportBuilder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationTCPSessionTransportBuilderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPresentationSessionTransportBuilderVTable,

    /* void buildTCPSenderTransport (in nsISocketTransport aTransport, in nsIPresentationSessionTransportBuilderListener aListener); */
    pub BuildTCPSenderTransport: unsafe extern "system" fn (this: *const nsIPresentationTCPSessionTransportBuilder, aTransport: *const nsISocketTransport, aListener: *const nsIPresentationSessionTransportBuilderListener) -> ::nserror::nsresult,

    /* void buildTCPReceiverTransport (in nsIPresentationChannelDescription aDescription, in nsIPresentationSessionTransportBuilderListener aListener); */
    pub BuildTCPReceiverTransport: unsafe extern "system" fn (this: *const nsIPresentationTCPSessionTransportBuilder, aDescription: *const nsIPresentationChannelDescription, aListener: *const nsIPresentationSessionTransportBuilderListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationTCPSessionTransportBuilder {

    /// ```text
    /// /**
    ///    * The following creation functions will trigger |listener.onSessionTransport|
    ///    * if the session transport is successfully built, |listener.onError| if some
    ///    * error occurs during building session transport.
    ///    */
    /// ```
    ///

    /// `void buildTCPSenderTransport (in nsISocketTransport aTransport, in nsIPresentationSessionTransportBuilderListener aListener);`
    #[inline]
    pub unsafe fn BuildTCPSenderTransport(&self, aTransport: *const nsISocketTransport, aListener: *const nsIPresentationSessionTransportBuilderListener) -> ::nserror::nsresult {
        ((*self.vtable).BuildTCPSenderTransport)(self, aTransport, aListener)
    }



    /// `void buildTCPReceiverTransport (in nsIPresentationChannelDescription aDescription, in nsIPresentationSessionTransportBuilderListener aListener);`
    #[inline]
    pub unsafe fn BuildTCPReceiverTransport(&self, aDescription: *const nsIPresentationChannelDescription, aListener: *const nsIPresentationSessionTransportBuilderListener) -> ::nserror::nsresult {
        ((*self.vtable).BuildTCPReceiverTransport)(self, aDescription, aListener)
    }


}


/// `interface nsIPresentationDataChannelSessionTransportBuilder : nsIPresentationSessionTransportBuilder`
///

/// ```text
/// /**
///  * Builder for WebRTC data channel session transport
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDataChannelSessionTransportBuilder {
    vtable: *const nsIPresentationDataChannelSessionTransportBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDataChannelSessionTransportBuilder.
unsafe impl XpCom for nsIPresentationDataChannelSessionTransportBuilder {
    const IID: nsIID = nsID(0x8131c4e0, 0x3a8c, 0x4bc1,
        [0xa9, 0x2a, 0x84, 0x31, 0x47, 0x3d, 0x2f, 0xe8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDataChannelSessionTransportBuilder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDataChannelSessionTransportBuilder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDataChannelSessionTransportBuilderCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDataChannelSessionTransportBuilder`.
    fn coerce_from(v: &nsIPresentationDataChannelSessionTransportBuilder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDataChannelSessionTransportBuilderCoerce for nsIPresentationDataChannelSessionTransportBuilder {
    #[inline]
    fn coerce_from(v: &nsIPresentationDataChannelSessionTransportBuilder) -> &Self {
        v
    }
}

impl nsIPresentationDataChannelSessionTransportBuilder {
    /// Cast this `nsIPresentationDataChannelSessionTransportBuilder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDataChannelSessionTransportBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDataChannelSessionTransportBuilder {
    type Target = nsIPresentationSessionTransportBuilder;
    #[inline]
    fn deref(&self) -> &nsIPresentationSessionTransportBuilder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPresentationSessionTransportBuilderCoerce> nsIPresentationDataChannelSessionTransportBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDataChannelSessionTransportBuilder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDataChannelSessionTransportBuilder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDataChannelSessionTransportBuilderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPresentationSessionTransportBuilderVTable,

    /* void buildDataChannelTransport (in uint8_t aRole, in mozIDOMWindow aWindow, in nsIPresentationSessionTransportBuilderListener aListener); */
    pub BuildDataChannelTransport: unsafe extern "system" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, aRole: uint8_t, aWindow: *const mozIDOMWindow, aListener: *const nsIPresentationSessionTransportBuilderListener) -> ::nserror::nsresult,

    /* void onOffer (in nsIPresentationChannelDescription offer); */
    pub OnOffer: unsafe extern "system" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, offer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void onAnswer (in nsIPresentationChannelDescription answer); */
    pub OnAnswer: unsafe extern "system" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, answer: *const nsIPresentationChannelDescription) -> ::nserror::nsresult,

    /* void onIceCandidate (in AString candidate); */
    pub OnIceCandidate: unsafe extern "system" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, candidate: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void notifyDisconnected (in nsresult reason); */
    pub NotifyDisconnected: unsafe extern "system" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, reason: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDataChannelSessionTransportBuilder {

    /// ```text
    /// /**
    ///    * The following creation function will trigger |listener.onSessionTransport|
    ///    * if the session transport is successfully built, |listener.onError| if some
    ///    * error occurs during creating session transport. The |notifyConnected| of
    ///    * |aControlChannel| should be called before calling
    ///    * |buildDataChannelTransport|.
    ///    */
    /// ```
    ///

    /// `void buildDataChannelTransport (in uint8_t aRole, in mozIDOMWindow aWindow, in nsIPresentationSessionTransportBuilderListener aListener);`
    #[inline]
    pub unsafe fn BuildDataChannelTransport(&self, aRole: uint8_t, aWindow: *const mozIDOMWindow, aListener: *const nsIPresentationSessionTransportBuilderListener) -> ::nserror::nsresult {
        ((*self.vtable).BuildDataChannelTransport)(self, aRole, aWindow, aListener)
    }



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



    /// `void notifyDisconnected (in nsresult reason);`
    #[inline]
    pub unsafe fn NotifyDisconnected(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).NotifyDisconnected)(self, reason)
    }


}


