//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/bridge/IPeerConnection.idl
//


/// `interface IPeerConnectionObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct IPeerConnectionObserver {
    vtable: *const IPeerConnectionObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for IPeerConnectionObserver.
unsafe impl XpCom for IPeerConnectionObserver {
    const IID: nsIID = nsID(0xd7dfe148, 0x0416, 0x446b,
        [0xa1, 0x28, 0x66, 0xa7, 0xc7, 0x1a, 0xe8, 0xd3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for IPeerConnectionObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from IPeerConnectionObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait IPeerConnectionObserverCoerce {
    /// Cheaply cast a value of this type from a `IPeerConnectionObserver`.
    fn coerce_from(v: &IPeerConnectionObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl IPeerConnectionObserverCoerce for IPeerConnectionObserver {
    #[inline]
    fn coerce_from(v: &IPeerConnectionObserver) -> &Self {
        v
    }
}

impl IPeerConnectionObserver {
    /// Cast this `IPeerConnectionObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: IPeerConnectionObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for IPeerConnectionObserver {
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
impl<T: nsISupportsCoerce> IPeerConnectionObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &IPeerConnectionObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every IPeerConnectionObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct IPeerConnectionObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl IPeerConnectionObserver {


}


/// `interface IPeerConnection : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct IPeerConnection {
    vtable: *const IPeerConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for IPeerConnection.
unsafe impl XpCom for IPeerConnection {
    const IID: nsIID = nsID(0x14afc8e7, 0xe421, 0x4d0c,
        [0x99, 0xa5, 0x69, 0x30, 0x8d, 0x87, 0x14, 0x81]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for IPeerConnection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from IPeerConnection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait IPeerConnectionCoerce {
    /// Cheaply cast a value of this type from a `IPeerConnection`.
    fn coerce_from(v: &IPeerConnection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl IPeerConnectionCoerce for IPeerConnection {
    #[inline]
    fn coerce_from(v: &IPeerConnection) -> &Self {
        v
    }
}

impl IPeerConnection {
    /// Cast this `IPeerConnection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: IPeerConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for IPeerConnection {
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
impl<T: nsISupportsCoerce> IPeerConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &IPeerConnection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every IPeerConnection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct IPeerConnectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl IPeerConnection {

    pub const kHintAudio: i64 = 1;


    pub const kHintVideo: i64 = 2;


    pub const kActionNone: i64 = -1;


    pub const kActionOffer: i64 = 0;


    pub const kActionAnswer: i64 = 1;


    pub const kActionPRAnswer: i64 = 2;


    pub const kActionRollback: i64 = 3;


    pub const kIceGathering: i64 = 0;


    pub const kIceWaiting: i64 = 1;


    pub const kIceChecking: i64 = 2;


    pub const kIceConnected: i64 = 3;


    pub const kIceFailed: i64 = 4;


    pub const kNew: i64 = 0;


    pub const kNegotiating: i64 = 1;


    pub const kActive: i64 = 2;


    pub const kClosing: i64 = 3;


    pub const kClosed: i64 = 4;


    pub const kDataChannelReliable: i64 = 0;


    pub const kDataChannelPartialReliableRexmit: i64 = 1;


    pub const kDataChannelPartialReliableTimed: i64 = 2;


    pub const kNoError: i64 = 0;


    pub const kInvalidCandidate: i64 = 2;


    pub const kInvalidMediastreamTrack: i64 = 3;


    pub const kInvalidState: i64 = 4;


    pub const kInvalidSessionDescription: i64 = 5;


    pub const kIncompatibleSessionDescription: i64 = 6;


    pub const kIncompatibleMediaStreamTrack: i64 = 8;


    pub const kInternalError: i64 = 9;


    pub const kTypeError: i64 = 10;


    pub const kOperationError: i64 = 11;


    pub const kMaxErrorType: i64 = 11;


}


