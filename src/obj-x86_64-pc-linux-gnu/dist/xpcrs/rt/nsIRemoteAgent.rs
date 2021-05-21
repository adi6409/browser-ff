//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/remote/components/nsIRemoteAgent.idl
//


/// `interface nsIRemoteAgent : nsISupports`
///

/// ```text
/// /**
///  * The Gecko remote agent is an RPC subsystem that exposes
///  * browser-internal interfaces and services to the surrounding
///  * system.
///  *
///  * Consumers, whether remote or browser-local, can interface with
///  * the browser through an assorted set of services ranging from
///  * document introspection and script evaluation, to instrumentation,
///  * user interaction simulation, and event subscription.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRemoteAgent {
    vtable: *const nsIRemoteAgentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRemoteAgent.
unsafe impl XpCom for nsIRemoteAgent {
    const IID: nsIID = nsID(0x8f685a9d, 0x8181, 0x46d6,
        [0xa7, 0x1d, 0x86, 0x92, 0x89, 0x09, 0x9c, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRemoteAgent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRemoteAgent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRemoteAgentCoerce {
    /// Cheaply cast a value of this type from a `nsIRemoteAgent`.
    fn coerce_from(v: &nsIRemoteAgent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRemoteAgentCoerce for nsIRemoteAgent {
    #[inline]
    fn coerce_from(v: &nsIRemoteAgent) -> &Self {
        v
    }
}

impl nsIRemoteAgent {
    /// Cast this `nsIRemoteAgent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRemoteAgentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRemoteAgent {
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
impl<T: nsISupportsCoerce> nsIRemoteAgentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRemoteAgent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRemoteAgent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRemoteAgentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString debuggerAddress; */
    pub GetDebuggerAddress: unsafe extern "system" fn (this: *const nsIRemoteAgent, aDebuggerAddress: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean listening; */
    pub GetListening: unsafe extern "system" fn (this: *const nsIRemoteAgent, aListening: *mut bool) -> ::nserror::nsresult,

    /* void listen (in AString aURL); */
    pub Listen: unsafe extern "system" fn (this: *const nsIRemoteAgent, aURL: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIRemoteAgent) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRemoteAgent {

    /// ```text
    /// /**
    ///    * Address of the HTTP server under which the remote agent is reachable.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString debuggerAddress;`
    #[inline]
    pub unsafe fn GetDebuggerAddress(&self, aDebuggerAddress: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDebuggerAddress)(self, aDebuggerAddress)
    }


    /// ```text
    /// /**
    ///    * Whether the remote agent is currently listening for new,
    ///    * incoming connections.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean listening;`
    #[inline]
    pub unsafe fn GetListening(&self, aListening: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetListening)(self, aListening)
    }


    /// ```text
    /// /**
    ///    * Asynchronously starts the remote agent, and listens for new
    ///    * connections.
    ///    *
    ///    * The address must be a fully qualified URL that uses the "http://"
    ///    * scheme, and can optionally include a desired port.  If no port
    ///    * is chosen, the default port 9222 is used.
    ///    *
    ///    * If the requested port is 0, the system will atomically allocate
    ///    * a port.
    ///    *
    ///    * A "remote-listening" system observer notification with the URL
    ///    * of the main target's WebSocket will be emitted once listening.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *     When disabled by the remote.enabled preference.
    ///    * @throws NS_ERROR_LAUNCHED_CHILD_PROCESS
    ///    *     When called from a child process.
    ///    * @throws NS_ERROR_ILLEGAL_VALUE
    ///    *     If requested to bind to a non-loopback device
    ///    *     if remote.force-local is true.
    ///    */
    /// ```
    ///

    /// `void listen (in AString aURL);`
    #[inline]
    pub unsafe fn Listen(&self, aURL: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Listen)(self, aURL)
    }


    /// ```text
    /// /** Stops listening and drops existing connections. */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


}


