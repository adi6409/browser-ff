//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketListener.idl
//


/// `interface nsIWebSocketListener : nsISupports`
///

/// ```text
/// /**
///  * nsIWebSocketListener: passed to nsIWebSocketChannel::AsyncOpen. Receives
///  * websocket traffic events as they arrive.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebSocketListener {
    vtable: *const nsIWebSocketListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebSocketListener.
unsafe impl XpCom for nsIWebSocketListener {
    const IID: nsIID = nsID(0xd74c96b2, 0x65b3, 0x4e39,
        [0x9e, 0x39, 0xc5, 0x77, 0xde, 0x5d, 0x7a, 0x73]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebSocketListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebSocketListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebSocketListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWebSocketListener`.
    fn coerce_from(v: &nsIWebSocketListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebSocketListenerCoerce for nsIWebSocketListener {
    #[inline]
    fn coerce_from(v: &nsIWebSocketListener) -> &Self {
        v
    }
}

impl nsIWebSocketListener {
    /// Cast this `nsIWebSocketListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebSocketListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebSocketListener {
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
impl<T: nsISupportsCoerce> nsIWebSocketListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebSocketListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebSocketListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void onStart (in nsISupports aContext); */
    pub OnStart: unsafe extern "system" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports) -> ::nserror::nsresult,

    /* [must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode); */
    pub OnStop: unsafe extern "system" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aStatusCode: ::nserror::nsresult) -> ::nserror::nsresult,

    /* [must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg); */
    pub OnMessageAvailable: unsafe extern "system" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg); */
    pub OnBinaryMessageAvailable: unsafe extern "system" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize); */
    pub OnAcknowledge: unsafe extern "system" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aSize: uint32_t) -> ::nserror::nsresult,

    /* [must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason); */
    pub OnServerClose: unsafe extern "system" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aCode: u16, aReason: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void OnError (); */
    pub OnError: unsafe extern "system" fn (this: *const nsIWebSocketListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebSocketListener {

    /// ```text
    /// /**
    ///      * Called to signify the establishment of the message stream.
    ///      *
    ///      * Unlike most other networking channels (which use nsIRequestObserver
        ///      * instead of this class), we do not guarantee that OnStart is always
    ///      * called: OnStop is called without calling this function if errors occur
    ///      * during connection setup.  If the websocket connection is successful,
    ///      * OnStart will be called before any other calls to this API.
    ///      *
    ///      * @param aContext user defined context
    ///      */
    /// ```
    ///

    /// `[must_use] void onStart (in nsISupports aContext);`
    #[inline]
    pub unsafe fn OnStart(&self, aContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).OnStart)(self, aContext)
    }


    /// ```text
    /// /**
    ///      * Called to signify the completion of the message stream.
    ///      * OnStop is the final notification the listener will receive and it
    ///      * completes the WebSocket connection: after it returns the
    ///      * nsIWebSocketChannel will release its reference to the listener.
    ///      *
    ///      * Note: this event can be received in error cases even if
    ///      * nsIWebSocketChannel::Close() has not been called.
    ///      *
    ///      * @param aContext user defined context
    ///      * @param aStatusCode reason for stopping (NS_OK if completed successfully)
    ///      */
    /// ```
    ///

    /// `[must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode);`
    #[inline]
    pub unsafe fn OnStop(&self, aContext: *const nsISupports, aStatusCode: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnStop)(self, aContext, aStatusCode)
    }


    /// ```text
    /// /**
    ///      * Called to deliver text message.
    ///      *
    ///      * @param aContext user defined context
    ///      * @param aMsg the message data
    ///      */
    /// ```
    ///

    /// `[must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg);`
    #[inline]
    pub unsafe fn OnMessageAvailable(&self, aContext: *const nsISupports, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnMessageAvailable)(self, aContext, aMsg)
    }


    /// ```text
    /// /**
    ///      * Called to deliver binary message.
    ///      *
    ///      * @param aContext user defined context
    ///      * @param aMsg the message data
    ///      */
    /// ```
    ///

    /// `[must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg);`
    #[inline]
    pub unsafe fn OnBinaryMessageAvailable(&self, aContext: *const nsISupports, aMsg: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnBinaryMessageAvailable)(self, aContext, aMsg)
    }


    /// ```text
    /// /**
    ///      * Called to acknowledge message sent via sendMsg() or sendBinaryMsg.
    ///      *
    ///      * @param aContext user defined context
    ///      * @param aSize number of bytes placed in OS send buffer
    ///      */
    /// ```
    ///

    /// `[must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize);`
    #[inline]
    pub unsafe fn OnAcknowledge(&self, aContext: *const nsISupports, aSize: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).OnAcknowledge)(self, aContext, aSize)
    }


    /// ```text
    /// /**
    ///      * Called to inform receipt of WebSocket Close message from server.
    ///      * In the case of errors onStop() can be called without ever
    ///      * receiving server close.
    ///      *
    ///      * No additional messages through onMessageAvailable(),
    ///      * onBinaryMessageAvailable() or onAcknowledge() will be delievered
    ///      * to the listener after onServerClose(), though outgoing messages can still
    ///      * be sent through the nsIWebSocketChannel connection.
    ///      *
    ///      * @param aContext user defined context
    ///      * @param aCode the websocket closing handshake close code.
    ///      * @param aReason the websocket closing handshake close reason
    ///      */
    /// ```
    ///

    /// `[must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason);`
    #[inline]
    pub unsafe fn OnServerClose(&self, aContext: *const nsISupports, aCode: u16, aReason: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnServerClose)(self, aContext, aCode, aReason)
    }


    /// ```text
    /// /**
    ///      * Called to inform an error is happened. The connection will be closed
    ///      * when this is called.
    ///      */
    /// ```
    ///

    /// `[must_use] void OnError ();`
    #[inline]
    pub unsafe fn OnError(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnError)(self, )
    }


}


