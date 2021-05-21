//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/asyncshutdown/nsIAsyncShutdown.idl
//


/// `interface nsIAsyncShutdownBlocker : nsISupports`
///

/// ```text
/// /**
///  * A blocker installed by a client to be informed during some stage of
///  * shutdown and block shutdown asynchronously until some condition is
///  * complete.
///  *
///  * If you wish to use AsyncShutdown, you will need to implement this
///  * interface (and only this interface).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncShutdownBlocker {
    vtable: *const nsIAsyncShutdownBlockerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncShutdownBlocker.
unsafe impl XpCom for nsIAsyncShutdownBlocker {
    const IID: nsIID = nsID(0x4ef43f29, 0x6715, 0x4b57,
        [0xa7, 0x50, 0x2f, 0xf8, 0x36, 0x95, 0xdd, 0xce]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncShutdownBlocker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncShutdownBlocker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncShutdownBlockerCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncShutdownBlocker`.
    fn coerce_from(v: &nsIAsyncShutdownBlocker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncShutdownBlockerCoerce for nsIAsyncShutdownBlocker {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBlocker) -> &Self {
        v
    }
}

impl nsIAsyncShutdownBlocker {
    /// Cast this `nsIAsyncShutdownBlocker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownBlockerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncShutdownBlocker {
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
impl<T: nsISupportsCoerce> nsIAsyncShutdownBlockerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBlocker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncShutdownBlocker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncShutdownBlockerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIAsyncShutdownBlocker, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void blockShutdown (in nsIAsyncShutdownClient aBarrierClient); */
    pub BlockShutdown: unsafe extern "system" fn (this: *const nsIAsyncShutdownBlocker, aBarrierClient: *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIPropertyBag state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIAsyncShutdownBlocker, aState: *mut *const nsIPropertyBag) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncShutdownBlocker {

    /// ```text
    /// /**
    ///    * The *unique* name of the blocker.
    ///    *
    ///    * By convention, it should respect the following format:
    ///    * "MyModuleName: Doing something while it's time"
    ///    * e.g.
    ///    * "OS.File: Flushing before profile-before-change"
    ///    *
    ///    * This attribute is uploaded as part of crash reports.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * Inform the blocker that the stage of shutdown has started.
    ///    * Shutdown will NOT proceed until `aBarrierClient.removeBlocker(this)`
    ///    * has been called.
    ///    */
    /// ```
    ///

    /// `void blockShutdown (in nsIAsyncShutdownClient aBarrierClient);`
    #[inline]
    pub unsafe fn BlockShutdown(&self, aBarrierClient: *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).BlockShutdown)(self, aBarrierClient)
    }


    /// ```text
    /// /**
    ///    * The current state of the blocker.
    ///    *
    ///    * In case of crash, this is converted to JSON and attached to
    ///    * the crash report.
    ///    *
    ///    * This field may be used to provide JSON-style data structures.
    ///    * For this purpose, use
    ///    * - nsIPropertyBag to represent objects;
    ///    * - nsIVariant to represent field values (which may hold nsIPropertyBag
        ///    * themselves).
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPropertyBag state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut *const nsIPropertyBag) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


}


/// `interface nsIAsyncShutdownClient : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncShutdownClient {
    vtable: *const nsIAsyncShutdownClientVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncShutdownClient.
unsafe impl XpCom for nsIAsyncShutdownClient {
    const IID: nsIID = nsID(0xd2031049, 0xb990, 0x43a2,
        [0x95, 0xbe, 0x59, 0xf8, 0xa3, 0xca, 0x59, 0x54]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncShutdownClient {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncShutdownClient.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncShutdownClientCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncShutdownClient`.
    fn coerce_from(v: &nsIAsyncShutdownClient) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncShutdownClientCoerce for nsIAsyncShutdownClient {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownClient) -> &Self {
        v
    }
}

impl nsIAsyncShutdownClient {
    /// Cast this `nsIAsyncShutdownClient` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownClientCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncShutdownClient {
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
impl<T: nsISupportsCoerce> nsIAsyncShutdownClientCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownClient) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncShutdownClient
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncShutdownClientVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIAsyncShutdownClient, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void addBlocker (in nsIAsyncShutdownBlocker aBlocker, in AString aFileName, in long aLineNumber, in AString aStack); */
    pub AddBlocker: unsafe extern "system" fn (this: *const nsIAsyncShutdownClient, aBlocker: *const nsIAsyncShutdownBlocker, aFileName: *const ::nsstring::nsAString, aLineNumber: i32, aStack: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeBlocker (in nsIAsyncShutdownBlocker aBlocker); */
    pub RemoveBlocker: unsafe extern "system" fn (this: *const nsIAsyncShutdownClient, aBlocker: *const nsIAsyncShutdownBlocker) -> ::nserror::nsresult,

    /* readonly attribute jsval jsclient; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetJsclient: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncShutdownClient {

    /// ```text
    /// /**
    ///  * A client for a nsIAsyncShutdownBarrier.
    ///  */
    /// /**
    ///    * The name of the barrier.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * Add a blocker.
    ///    *
    ///    * After a `blocker` has been added with `addBlocker`, if it is not
    ///    * removed with `removeBlocker`, this will, by design, eventually
    ///    * CAUSE A CRASH.
    ///    *
    ///    * Calling `addBlocker` once nsIAsyncShutdownBarrier::wait() has been
    ///    * called on the owning barrier returns an error.
    ///    *
    ///    * @param aBlocker The blocker to add. Once
    ///    * nsIAsyncShutdownBarrier::wait() has been called, it will not
    ///    * call its `aOnReady` callback until all blockers have been
    ///    * removed, each  by a call to `removeBlocker`.
    ///    * @param aFileName The filename of the callsite, as given by `__FILE__`.
    ///    * @param aLineNumber The linenumber of the callsite, as given by `__LINE__`.
    ///    * @param aStack Information on the stack that lead to this call. Generally
    ///    * empty when called from C++.
    ///    */
    /// ```
    ///

    /// `void addBlocker (in nsIAsyncShutdownBlocker aBlocker, in AString aFileName, in long aLineNumber, in AString aStack);`
    #[inline]
    pub unsafe fn AddBlocker(&self, aBlocker: *const nsIAsyncShutdownBlocker, aFileName: *const ::nsstring::nsAString, aLineNumber: i32, aStack: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddBlocker)(self, aBlocker, aFileName, aLineNumber, aStack)
    }


    /// ```text
    /// /**
    ///    * Remove a blocker.
    ///    *
    ///    * @param aBlocker A blocker previously added to this client through
    ///    * `addBlocker`. Noop if the blocker has never been added or has been
    ///    * removed already.
    ///    */
    /// ```
    ///

    /// `void removeBlocker (in nsIAsyncShutdownBlocker aBlocker);`
    #[inline]
    pub unsafe fn RemoveBlocker(&self, aBlocker: *const nsIAsyncShutdownBlocker) -> ::nserror::nsresult {
        ((*self.vtable).RemoveBlocker)(self, aBlocker)
    }


    /// ```text
    /// /**
    ///    * The JS implementation of the client.
    ///    *
    ///    * It is strongly recommended that JS clients of this API use
    ///    * `jsclient` instead of the `nsIAsyncShutdownClient`. See
    ///    * AsyncShutdown.jsm for more information on the JS version of
    ///    * this API.
    ///    */
    /// ```
    ///

    /// `readonly attribute jsval jsclient;`
    const _GetJsclient: () = ();

}


/// `interface nsIAsyncShutdownCompletionCallback : nsISupports`
///

/// ```text
/// /**
///  * Callback invoked once all blockers of a barrier have been removed.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncShutdownCompletionCallback {
    vtable: *const nsIAsyncShutdownCompletionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncShutdownCompletionCallback.
unsafe impl XpCom for nsIAsyncShutdownCompletionCallback {
    const IID: nsIID = nsID(0x910c9309, 0x1da0, 0x4dd0,
        [0x8b, 0xdb, 0xa3, 0x25, 0xa3, 0x8c, 0x60, 0x4e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncShutdownCompletionCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncShutdownCompletionCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncShutdownCompletionCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncShutdownCompletionCallback`.
    fn coerce_from(v: &nsIAsyncShutdownCompletionCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncShutdownCompletionCallbackCoerce for nsIAsyncShutdownCompletionCallback {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownCompletionCallback) -> &Self {
        v
    }
}

impl nsIAsyncShutdownCompletionCallback {
    /// Cast this `nsIAsyncShutdownCompletionCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownCompletionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncShutdownCompletionCallback {
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
impl<T: nsISupportsCoerce> nsIAsyncShutdownCompletionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownCompletionCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncShutdownCompletionCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncShutdownCompletionCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void done (); */
    pub Done: unsafe extern "system" fn (this: *const nsIAsyncShutdownCompletionCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncShutdownCompletionCallback {

    /// ```text
    /// /**
    ///    * The operation has been completed.
    ///    */
    /// ```
    ///

    /// `void done ();`
    #[inline]
    pub unsafe fn Done(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Done)(self, )
    }


}


/// `interface nsIAsyncShutdownBarrier : nsISupports`
///

/// ```text
/// /**
///  * A stage of shutdown that supports blocker registration.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncShutdownBarrier {
    vtable: *const nsIAsyncShutdownBarrierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncShutdownBarrier.
unsafe impl XpCom for nsIAsyncShutdownBarrier {
    const IID: nsIID = nsID(0x50fa8a86, 0x9c91, 0x4256,
        [0x83, 0x89, 0x17, 0xd3, 0x10, 0xad, 0xec, 0x90]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncShutdownBarrier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncShutdownBarrier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncShutdownBarrierCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncShutdownBarrier`.
    fn coerce_from(v: &nsIAsyncShutdownBarrier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncShutdownBarrierCoerce for nsIAsyncShutdownBarrier {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBarrier) -> &Self {
        v
    }
}

impl nsIAsyncShutdownBarrier {
    /// Cast this `nsIAsyncShutdownBarrier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownBarrierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncShutdownBarrier {
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
impl<T: nsISupportsCoerce> nsIAsyncShutdownBarrierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBarrier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncShutdownBarrier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncShutdownBarrierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAsyncShutdownClient client; */
    pub GetClient: unsafe extern "system" fn (this: *const nsIAsyncShutdownBarrier, aClient: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIPropertyBag state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIAsyncShutdownBarrier, aState: *mut *const nsIPropertyBag) -> ::nserror::nsresult,

    /* void wait (in nsIAsyncShutdownCompletionCallback aOnReady); */
    pub Wait: unsafe extern "system" fn (this: *const nsIAsyncShutdownBarrier, aOnReady: *const nsIAsyncShutdownCompletionCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncShutdownBarrier {

    /// ```text
    /// /**
    ///    * The blocker registration capability.  Most services may wish to
    ///    * publish this capability to let services that depend on it register
    ///    * blockers.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient client;`
    #[inline]
    pub unsafe fn GetClient(&self, aClient: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetClient)(self, aClient)
    }


    /// ```text
    /// /**
    ///    * The state of all the blockers of the barrier.
    ///    *
    ///    * See the documentation of `nsIAsyncShutdownBlocker` for the
    ///    * format.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPropertyBag state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut *const nsIPropertyBag) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * Wait for all blockers to complete.
    ///    *
    ///    * Method `aOnReady` will be called once all blockers have finished.
    ///    * The callback always receives NS_OK.
    ///    */
    /// ```
    ///

    /// `void wait (in nsIAsyncShutdownCompletionCallback aOnReady);`
    #[inline]
    pub unsafe fn Wait(&self, aOnReady: *const nsIAsyncShutdownCompletionCallback) -> ::nserror::nsresult {
        ((*self.vtable).Wait)(self, aOnReady)
    }


}


/// `interface nsIAsyncShutdownService : nsISupports`
///

/// ```text
/// /**
///  * A service that allows registering shutdown-time dependencies.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncShutdownService {
    vtable: *const nsIAsyncShutdownServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncShutdownService.
unsafe impl XpCom for nsIAsyncShutdownService {
    const IID: nsIID = nsID(0xdb365c78, 0xc860, 0x4e64,
        [0x9a, 0x63, 0x25, 0xb7, 0x3f, 0x89, 0xa0, 0x16]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncShutdownService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncShutdownService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncShutdownServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncShutdownService`.
    fn coerce_from(v: &nsIAsyncShutdownService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncShutdownServiceCoerce for nsIAsyncShutdownService {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownService) -> &Self {
        v
    }
}

impl nsIAsyncShutdownService {
    /// Cast this `nsIAsyncShutdownService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncShutdownService {
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
impl<T: nsISupportsCoerce> nsIAsyncShutdownServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncShutdownService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncShutdownServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIAsyncShutdownBarrier makeBarrier (in AString aName); */
    pub MakeBarrier: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aName: *const ::nsstring::nsAString, _retval: *mut *const nsIAsyncShutdownBarrier) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient profileBeforeChange; */
    pub GetProfileBeforeChange: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aProfileBeforeChange: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient profileChangeTeardown; */
    pub GetProfileChangeTeardown: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aProfileChangeTeardown: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient quitApplicationGranted; */
    pub GetQuitApplicationGranted: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aQuitApplicationGranted: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient sendTelemetry; */
    pub GetSendTelemetry: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aSendTelemetry: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient webWorkersShutdown; */
    pub GetWebWorkersShutdown: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aWebWorkersShutdown: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient xpcomWillShutdown; */
    pub GetXpcomWillShutdown: unsafe extern "system" fn (this: *const nsIAsyncShutdownService, aXpcomWillShutdown: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncShutdownService {

    /// ```text
    /// /**
    ///    * Create a new barrier.
    ///    *
    ///    * By convention, the name should respect the following format:
    ///    * "MyModuleName: Doing something while it's time"
    ///    * e.g.
    ///    * "OS.File: Waiting for clients to flush before shutting down"
    ///    *
    ///    * This attribute is uploaded as part of crash reports.
    ///    */
    /// ```
    ///

    /// `nsIAsyncShutdownBarrier makeBarrier (in AString aName);`
    #[inline]
    pub unsafe fn MakeBarrier(&self, aName: *const ::nsstring::nsAString, _retval: *mut *const nsIAsyncShutdownBarrier) -> ::nserror::nsresult {
        ((*self.vtable).MakeBarrier)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * Barrier for notification profile-before-change.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient profileBeforeChange;`
    #[inline]
    pub unsafe fn GetProfileBeforeChange(&self, aProfileBeforeChange: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetProfileBeforeChange)(self, aProfileBeforeChange)
    }


    /// ```text
    /// /**
    ///    * Barrier for notification profile-change-teardown.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient profileChangeTeardown;`
    #[inline]
    pub unsafe fn GetProfileChangeTeardown(&self, aProfileChangeTeardown: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetProfileChangeTeardown)(self, aProfileChangeTeardown)
    }


    /// ```text
    /// /**
    ///    * Barrier for notification quit-application-granted.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient quitApplicationGranted;`
    #[inline]
    pub unsafe fn GetQuitApplicationGranted(&self, aQuitApplicationGranted: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetQuitApplicationGranted)(self, aQuitApplicationGranted)
    }


    /// ```text
    /// /**
    ///    * Barrier for notification profile-before-change-telemetry.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient sendTelemetry;`
    #[inline]
    pub unsafe fn GetSendTelemetry(&self, aSendTelemetry: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetSendTelemetry)(self, aSendTelemetry)
    }


    /// ```text
    /// /**
    ///    * Barrier for notification web-workers-shutdown.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient webWorkersShutdown;`
    #[inline]
    pub unsafe fn GetWebWorkersShutdown(&self, aWebWorkersShutdown: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetWebWorkersShutdown)(self, aWebWorkersShutdown)
    }


    /// ```text
    /// /**
    ///    * Barrier for notification xpcom-will-shutdown.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAsyncShutdownClient xpcomWillShutdown;`
    #[inline]
    pub unsafe fn GetXpcomWillShutdown(&self, aXpcomWillShutdown: *mut *const nsIAsyncShutdownClient) -> ::nserror::nsresult {
        ((*self.vtable).GetXpcomWillShutdown)(self, aXpcomWillShutdown)
    }


}


