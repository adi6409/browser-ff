//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketEventService.idl
//


/// `interface nsIWebSocketFrame : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebSocketFrame {
    vtable: *const nsIWebSocketFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebSocketFrame.
unsafe impl XpCom for nsIWebSocketFrame {
    const IID: nsIID = nsID(0x6714a6be, 0x2265, 0x4f73,
        [0xa9, 0x88, 0xd7, 0x8a, 0x12, 0x41, 0x60, 0x37]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebSocketFrame {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebSocketFrame.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebSocketFrameCoerce {
    /// Cheaply cast a value of this type from a `nsIWebSocketFrame`.
    fn coerce_from(v: &nsIWebSocketFrame) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebSocketFrameCoerce for nsIWebSocketFrame {
    #[inline]
    fn coerce_from(v: &nsIWebSocketFrame) -> &Self {
        v
    }
}

impl nsIWebSocketFrame {
    /// Cast this `nsIWebSocketFrame` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebSocketFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebSocketFrame {
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
impl<T: nsISupportsCoerce> nsIWebSocketFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketFrame) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebSocketFrame
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebSocketFrameVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute DOMHighResTimeStamp timeStamp; */
    pub GetTimeStamp: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aTimeStamp: *mut DOMHighResTimeStamp) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean finBit; */
    pub GetFinBit: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aFinBit: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean rsvBit1; */
    pub GetRsvBit1: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aRsvBit1: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean rsvBit2; */
    pub GetRsvBit2: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aRsvBit2: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean rsvBit3; */
    pub GetRsvBit3: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aRsvBit3: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned short opCode; */
    pub GetOpCode: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aOpCode: *mut u16) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean maskBit; */
    pub GetMaskBit: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aMaskBit: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned long mask; */
    pub GetMask: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aMask: *mut u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString payload; */
    pub GetPayload: unsafe extern "system" fn (this: *const nsIWebSocketFrame, aPayload: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebSocketFrame {

    pub const OPCODE_CONTINUATION: i64 = 0;


    pub const OPCODE_TEXT: i64 = 1;


    pub const OPCODE_BINARY: i64 = 2;


    pub const OPCODE_CLOSE: i64 = 8;


    pub const OPCODE_PING: i64 = 9;


    pub const OPCODE_PONG: i64 = 10;


    /// `[must_use] readonly attribute DOMHighResTimeStamp timeStamp;`
    #[inline]
    pub unsafe fn GetTimeStamp(&self, aTimeStamp: *mut DOMHighResTimeStamp) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeStamp)(self, aTimeStamp)
    }



    /// `[must_use] readonly attribute boolean finBit;`
    #[inline]
    pub unsafe fn GetFinBit(&self, aFinBit: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFinBit)(self, aFinBit)
    }



    /// `[must_use] readonly attribute boolean rsvBit1;`
    #[inline]
    pub unsafe fn GetRsvBit1(&self, aRsvBit1: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRsvBit1)(self, aRsvBit1)
    }



    /// `[must_use] readonly attribute boolean rsvBit2;`
    #[inline]
    pub unsafe fn GetRsvBit2(&self, aRsvBit2: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRsvBit2)(self, aRsvBit2)
    }



    /// `[must_use] readonly attribute boolean rsvBit3;`
    #[inline]
    pub unsafe fn GetRsvBit3(&self, aRsvBit3: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRsvBit3)(self, aRsvBit3)
    }



    /// `[must_use] readonly attribute unsigned short opCode;`
    #[inline]
    pub unsafe fn GetOpCode(&self, aOpCode: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetOpCode)(self, aOpCode)
    }



    /// `[must_use] readonly attribute boolean maskBit;`
    #[inline]
    pub unsafe fn GetMaskBit(&self, aMaskBit: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMaskBit)(self, aMaskBit)
    }



    /// `[must_use] readonly attribute unsigned long mask;`
    #[inline]
    pub unsafe fn GetMask(&self, aMask: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMask)(self, aMask)
    }



    /// `[must_use] readonly attribute ACString payload;`
    #[inline]
    pub unsafe fn GetPayload(&self, aPayload: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPayload)(self, aPayload)
    }


}


/// `interface nsIWebSocketEventListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebSocketEventListener {
    vtable: *const nsIWebSocketEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebSocketEventListener.
unsafe impl XpCom for nsIWebSocketEventListener {
    const IID: nsIID = nsID(0xe7c005ab, 0xe694, 0x489b,
        [0xb7, 0x41, 0x96, 0xdb, 0x43, 0xff, 0xb1, 0x6f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebSocketEventListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebSocketEventListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebSocketEventListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWebSocketEventListener`.
    fn coerce_from(v: &nsIWebSocketEventListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebSocketEventListenerCoerce for nsIWebSocketEventListener {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventListener) -> &Self {
        v
    }
}

impl nsIWebSocketEventListener {
    /// Cast this `nsIWebSocketEventListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebSocketEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebSocketEventListener {
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
impl<T: nsISupportsCoerce> nsIWebSocketEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebSocketEventListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebSocketEventListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols); */
    pub WebSocketCreated: unsafe extern "system" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: u32, aURI: *const ::nsstring::nsAString, aProtocols: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions, in uint64_t aHttpChannelId); */
    pub WebSocketOpened: unsafe extern "system" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: u32, aEffectiveURI: *const ::nsstring::nsAString, aProtocols: *const ::nsstring::nsACString, aExtensions: *const ::nsstring::nsACString, aHttpChannelId: uint64_t) -> ::nserror::nsresult,

    /* [must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType); */
    pub WebSocketMessageAvailable: unsafe extern "system" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: u32, aMessage: *const ::nsstring::nsACString, aType: u16) -> ::nserror::nsresult,

    /* [must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason); */
    pub WebSocketClosed: unsafe extern "system" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: u32, aWasClean: bool, aCode: u16, aReason: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
    pub FrameReceived: unsafe extern "system" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: u32, aFrame: *const nsIWebSocketFrame) -> ::nserror::nsresult,

    /* [must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
    pub FrameSent: unsafe extern "system" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: u32, aFrame: *const nsIWebSocketFrame) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebSocketEventListener {

    pub const TYPE_STRING: i64 = 0;


    pub const TYPE_BLOB: i64 = 1;


    pub const TYPE_ARRAYBUFFER: i64 = 2;


    /// `[must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols);`
    #[inline]
    pub unsafe fn WebSocketCreated(&self, aWebSocketSerialID: u32, aURI: *const ::nsstring::nsAString, aProtocols: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).WebSocketCreated)(self, aWebSocketSerialID, aURI, aProtocols)
    }



    /// `[must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions, in uint64_t aHttpChannelId);`
    #[inline]
    pub unsafe fn WebSocketOpened(&self, aWebSocketSerialID: u32, aEffectiveURI: *const ::nsstring::nsAString, aProtocols: *const ::nsstring::nsACString, aExtensions: *const ::nsstring::nsACString, aHttpChannelId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).WebSocketOpened)(self, aWebSocketSerialID, aEffectiveURI, aProtocols, aExtensions, aHttpChannelId)
    }



    /// `[must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType);`
    #[inline]
    pub unsafe fn WebSocketMessageAvailable(&self, aWebSocketSerialID: u32, aMessage: *const ::nsstring::nsACString, aType: u16) -> ::nserror::nsresult {
        ((*self.vtable).WebSocketMessageAvailable)(self, aWebSocketSerialID, aMessage, aType)
    }



    /// `[must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason);`
    #[inline]
    pub unsafe fn WebSocketClosed(&self, aWebSocketSerialID: u32, aWasClean: bool, aCode: u16, aReason: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).WebSocketClosed)(self, aWebSocketSerialID, aWasClean, aCode, aReason)
    }



    /// `[must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame);`
    #[inline]
    pub unsafe fn FrameReceived(&self, aWebSocketSerialID: u32, aFrame: *const nsIWebSocketFrame) -> ::nserror::nsresult {
        ((*self.vtable).FrameReceived)(self, aWebSocketSerialID, aFrame)
    }



    /// `[must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame);`
    #[inline]
    pub unsafe fn FrameSent(&self, aWebSocketSerialID: u32, aFrame: *const nsIWebSocketFrame) -> ::nserror::nsresult {
        ((*self.vtable).FrameSent)(self, aWebSocketSerialID, aFrame)
    }


}


/// `interface nsIWebSocketEventService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebSocketEventService {
    vtable: *const nsIWebSocketEventServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebSocketEventService.
unsafe impl XpCom for nsIWebSocketEventService {
    const IID: nsIID = nsID(0xb89d1b90, 0x2cf3, 0x4d8f,
        [0xac, 0x21, 0x5a, 0xed, 0xfb, 0x25, 0xc7, 0x60]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebSocketEventService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebSocketEventService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebSocketEventServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIWebSocketEventService`.
    fn coerce_from(v: &nsIWebSocketEventService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebSocketEventServiceCoerce for nsIWebSocketEventService {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventService) -> &Self {
        v
    }
}

impl nsIWebSocketEventService {
    /// Cast this `nsIWebSocketEventService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebSocketEventServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebSocketEventService {
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
impl<T: nsISupportsCoerce> nsIWebSocketEventServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebSocketEventService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebSocketEventServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void sendMessage (in unsigned long aWebSocketSerialID, in AString aMessage); */
    pub SendMessage: unsafe extern "system" fn (this: *const nsIWebSocketEventService, aWebSocketSerialID: u32, aMessage: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIWebSocketEventService, aInnerWindowID: u64, aListener: *const nsIWebSocketEventListener) -> ::nserror::nsresult,

    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIWebSocketEventService, aInnerWindowID: u64, aListener: *const nsIWebSocketEventListener) -> ::nserror::nsresult,

    /* [must_use] bool hasListenerFor (in unsigned long long aInnerWindowID); */
    pub HasListenerFor: unsafe extern "system" fn (this: *const nsIWebSocketEventService, aInnerWindowID: u64, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebSocketEventService {


    /// `[must_use] void sendMessage (in unsigned long aWebSocketSerialID, in AString aMessage);`
    #[inline]
    pub unsafe fn SendMessage(&self, aWebSocketSerialID: u32, aMessage: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SendMessage)(self, aWebSocketSerialID, aMessage)
    }



    /// `[must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener);`
    #[inline]
    pub unsafe fn AddListener(&self, aInnerWindowID: u64, aListener: *const nsIWebSocketEventListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, aInnerWindowID, aListener)
    }



    /// `[must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, aInnerWindowID: u64, aListener: *const nsIWebSocketEventListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, aInnerWindowID, aListener)
    }



    /// `[must_use] bool hasListenerFor (in unsigned long long aInnerWindowID);`
    #[inline]
    pub unsafe fn HasListenerFor(&self, aInnerWindowID: u64, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasListenerFor)(self, aInnerWindowID, _retval)
    }


}


