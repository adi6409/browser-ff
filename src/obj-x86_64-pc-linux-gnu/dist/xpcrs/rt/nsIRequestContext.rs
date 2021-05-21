//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestContext.idl
//


/// `interface nsIRequestTailUnblockCallback : nsISupports`
///

/// ```text
/// /**
///  * Requests capable of tail-blocking must implement this
///  * interfaces (typically channels).
///  * If the request is tail-blocked, it will be held in its request
///  * context queue until unblocked.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRequestTailUnblockCallback {
    vtable: *const nsIRequestTailUnblockCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRequestTailUnblockCallback.
unsafe impl XpCom for nsIRequestTailUnblockCallback {
    const IID: nsIID = nsID(0x7eb361d4, 0x37a5, 0x42c9,
        [0xaf, 0xae, 0xf6, 0xc8, 0x8f, 0xe7, 0xc3, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRequestTailUnblockCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRequestTailUnblockCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRequestTailUnblockCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIRequestTailUnblockCallback`.
    fn coerce_from(v: &nsIRequestTailUnblockCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRequestTailUnblockCallbackCoerce for nsIRequestTailUnblockCallback {
    #[inline]
    fn coerce_from(v: &nsIRequestTailUnblockCallback) -> &Self {
        v
    }
}

impl nsIRequestTailUnblockCallback {
    /// Cast this `nsIRequestTailUnblockCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRequestTailUnblockCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRequestTailUnblockCallback {
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
impl<T: nsISupportsCoerce> nsIRequestTailUnblockCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestTailUnblockCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRequestTailUnblockCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRequestTailUnblockCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onTailUnblock (in nsresult aResult); */
    pub OnTailUnblock: unsafe extern "system" fn (this: *const nsIRequestTailUnblockCallback, aResult: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRequestTailUnblockCallback {

    /// ```text
    /// /**
    ///    * Called when the requests is unblocked and proceed.
    ///    * @param result
    ///    *    NS_OK - the request is OK to go, unblocking is not
    ///    *            caused by cancelation of the request.
    ///    *    any error - the request must behave as it were canceled
    ///    *                with the result as status.
    ///    */
    /// ```
    ///

    /// `void onTailUnblock (in nsresult aResult);`
    #[inline]
    pub unsafe fn OnTailUnblock(&self, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnTailUnblock)(self, aResult)
    }


}


/// `interface nsIRequestContext : nsISupports`
///

/// ```text
/// /**
///  * The nsIRequestContext is used to maintain state about connections
///  * that are in some way associated with each other (often by being part
    ///  * of the same load group) and how they interact with blocking items like
///  * HEAD css/js loads.
///  *
///  * This used to be known as nsILoadGroupConnectionInfo and nsISchedulingContext.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRequestContext {
    vtable: *const nsIRequestContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRequestContext.
unsafe impl XpCom for nsIRequestContext {
    const IID: nsIID = nsID(0x658e3e6e, 0x8633, 0x4b1a,
        [0x8d, 0x66, 0xfa, 0x9f, 0x72, 0x29, 0x3e, 0x63]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRequestContext {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRequestContext.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRequestContextCoerce {
    /// Cheaply cast a value of this type from a `nsIRequestContext`.
    fn coerce_from(v: &nsIRequestContext) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRequestContextCoerce for nsIRequestContext {
    #[inline]
    fn coerce_from(v: &nsIRequestContext) -> &Self {
        v
    }
}

impl nsIRequestContext {
    /// Cast this `nsIRequestContext` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRequestContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRequestContext {
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
impl<T: nsISupportsCoerce> nsIRequestContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestContext) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRequestContext
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRequestContextVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [nostdcall,notxpcom] readonly attribute unsigned long long ID; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetID: *const ::libc::c_void,

    /* void beginLoad (); */
    pub BeginLoad: unsafe extern "system" fn (this: *const nsIRequestContext) -> ::nserror::nsresult,

    /* void DOMContentLoaded (); */
    pub DOMContentLoaded: unsafe extern "system" fn (this: *const nsIRequestContext) -> ::nserror::nsresult,

    /* readonly attribute unsigned long blockingTransactionCount; */
    pub GetBlockingTransactionCount: unsafe extern "system" fn (this: *const nsIRequestContext, aBlockingTransactionCount: *mut u32) -> ::nserror::nsresult,

    /* void addBlockingTransaction (); */
    pub AddBlockingTransaction: unsafe extern "system" fn (this: *const nsIRequestContext) -> ::nserror::nsresult,

    /* unsigned long removeBlockingTransaction (); */
    pub RemoveBlockingTransaction: unsafe extern "system" fn (this: *const nsIRequestContext, _retval: *mut u32) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] attribute SpdyPushCachePtr spdyPushCache; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetSpdyPushCache: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute SpdyPushCachePtr spdyPushCache; */
    /// Unable to generate binding because `native type mozilla::net::SpdyPushCache unsupported`
    pub SetSpdyPushCache: *const ::libc::c_void,

    /* void addNonTailRequest (); */
    pub AddNonTailRequest: unsafe extern "system" fn (this: *const nsIRequestContext) -> ::nserror::nsresult,

    /* void removeNonTailRequest (); */
    pub RemoveNonTailRequest: unsafe extern "system" fn (this: *const nsIRequestContext) -> ::nserror::nsresult,

    /* [must_use] boolean isContextTailBlocked (in nsIRequestTailUnblockCallback callback); */
    pub IsContextTailBlocked: unsafe extern "system" fn (this: *const nsIRequestContext, callback: *const nsIRequestTailUnblockCallback, _retval: *mut bool) -> ::nserror::nsresult,

    /* void cancelTailedRequest (in nsIRequestTailUnblockCallback request); */
    pub CancelTailedRequest: unsafe extern "system" fn (this: *const nsIRequestContext, request: *const nsIRequestTailUnblockCallback) -> ::nserror::nsresult,

    /* void cancelTailPendingRequests (in nsresult aResult); */
    pub CancelTailPendingRequests: unsafe extern "system" fn (this: *const nsIRequestContext, aResult: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRequestContext {

    /// ```text
    /// /**
    ///    * A unique identifier for this request context
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute unsigned long long ID;`
    const _GetID: () = ();

    /// ```text
    /// /**
    ///    * Called by the associated document when its load starts.  This resets
    ///    * context's internal states.
    ///    */
    /// ```
    ///

    /// `void beginLoad ();`
    #[inline]
    pub unsafe fn BeginLoad(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeginLoad)(self, )
    }


    /// ```text
    /// /**
    ///   * Called when the associated document notified the DOMContentLoaded event.
    ///   */
    /// ```
    ///

    /// `void DOMContentLoaded ();`
    #[inline]
    pub unsafe fn DOMContentLoaded(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DOMContentLoaded)(self, )
    }


    /// ```text
    /// /**
    ///    * Number of active blocking transactions associated with this context
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long blockingTransactionCount;`
    #[inline]
    pub unsafe fn GetBlockingTransactionCount(&self, aBlockingTransactionCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetBlockingTransactionCount)(self, aBlockingTransactionCount)
    }


    /// ```text
    /// /**
    ///    * Increase the number of active blocking transactions associated
    ///    * with this context by one.
    ///    */
    /// ```
    ///

    /// `void addBlockingTransaction ();`
    #[inline]
    pub unsafe fn AddBlockingTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AddBlockingTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Decrease the number of active blocking transactions associated
    ///    * with this context by one. The return value is the number of remaining
    ///    * blockers.
    ///    */
    /// ```
    ///

    /// `unsigned long removeBlockingTransaction ();`
    #[inline]
    pub unsafe fn RemoveBlockingTransaction(&self, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveBlockingTransaction)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * This gives out a weak pointer to the push cache.
    ///    * The nsIRequestContext implementation owns the cache
    ///    * and will destroy it when overwritten or when the context
    ///    * ends.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute SpdyPushCachePtr spdyPushCache;`
    const _GetSpdyPushCache: () = ();

    /// ```text
    /// /**
    ///    * This gives out a weak pointer to the push cache.
    ///    * The nsIRequestContext implementation owns the cache
    ///    * and will destroy it when overwritten or when the context
    ///    * ends.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute SpdyPushCachePtr spdyPushCache;`
    const _SetSpdyPushCache: () = ();

    /// ```text
    /// /**
    ///    * Increases/decrease the number of non-tailed requests in this context.
    ///    * If the count drops to zero, all tail-blocked callbacks are notified
    ///    * shortly after that to be unblocked.
    ///    */
    /// ```
    ///

    /// `void addNonTailRequest ();`
    #[inline]
    pub unsafe fn AddNonTailRequest(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AddNonTailRequest)(self, )
    }



    /// `void removeNonTailRequest ();`
    #[inline]
    pub unsafe fn RemoveNonTailRequest(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveNonTailRequest)(self, )
    }


    /// ```text
    /// /**
    ///    * If the request context is in tail-blocked state, the callback
    ///    * is queued and result is true.  The callback will be notified
    ///    * about tail-unblocking or when the request context is canceled.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean isContextTailBlocked (in nsIRequestTailUnblockCallback callback);`
    #[inline]
    pub unsafe fn IsContextTailBlocked(&self, callback: *const nsIRequestTailUnblockCallback, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsContextTailBlocked)(self, callback, _retval)
    }


    /// ```text
    /// /**
    ///    * Called when the request is sitting in the tail queue but has been
    ///    * canceled before untailing.  This just removes the request from the
    ///    * queue so that it is not notified on untail and not referenced.
    ///    */
    /// ```
    ///

    /// `void cancelTailedRequest (in nsIRequestTailUnblockCallback request);`
    #[inline]
    pub unsafe fn CancelTailedRequest(&self, request: *const nsIRequestTailUnblockCallback) -> ::nserror::nsresult {
        ((*self.vtable).CancelTailedRequest)(self, request)
    }


    /// ```text
    /// /**
    ///    * This notifies all queued tail-blocked requests, they will be notified
    ///    * aResult and released afterwards.  Called by the load group when
    ///    * it's canceled.
    ///    */
    /// ```
    ///

    /// `void cancelTailPendingRequests (in nsresult aResult);`
    #[inline]
    pub unsafe fn CancelTailPendingRequests(&self, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CancelTailPendingRequests)(self, aResult)
    }


}


/// `interface nsIRequestContextService : nsISupports`
///

/// ```text
/// /**
///  * The nsIRequestContextService is how anyone gets access to a request
///  * context when they haven't been explicitly given a strong reference to an
///  * existing one. It is responsible for creating and handing out strong
///  * references to nsIRequestContexts, but only keeps weak references itself.
///  * The shared request context will go away once no one else is keeping a
///  * reference to it. If you ask for a request context that has no one else
///  * holding a reference to it, you'll get a brand new request context. Anyone
///  * who asks for the same request context while you're holding a reference
///  * will get a reference to the same request context you have.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRequestContextService {
    vtable: *const nsIRequestContextServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRequestContextService.
unsafe impl XpCom for nsIRequestContextService {
    const IID: nsIID = nsID(0x7fcbf4da, 0xd828, 0x4acc,
        [0xb1, 0x44, 0xe5, 0x43, 0x51, 0x98, 0xf7, 0x27]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRequestContextService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRequestContextService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRequestContextServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIRequestContextService`.
    fn coerce_from(v: &nsIRequestContextService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRequestContextServiceCoerce for nsIRequestContextService {
    #[inline]
    fn coerce_from(v: &nsIRequestContextService) -> &Self {
        v
    }
}

impl nsIRequestContextService {
    /// Cast this `nsIRequestContextService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRequestContextServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRequestContextService {
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
impl<T: nsISupportsCoerce> nsIRequestContextServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestContextService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRequestContextService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRequestContextServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIRequestContext getRequestContext (in unsigned long long id); */
    pub GetRequestContext: unsafe extern "system" fn (this: *const nsIRequestContextService, id: u64, _retval: *mut *const nsIRequestContext) -> ::nserror::nsresult,

    /* nsIRequestContext getRequestContextFromLoadGroup (in nsILoadGroup lg); */
    pub GetRequestContextFromLoadGroup: unsafe extern "system" fn (this: *const nsIRequestContextService, lg: *const nsILoadGroup, _retval: *mut *const nsIRequestContext) -> ::nserror::nsresult,

    /* nsIRequestContext newRequestContext (); */
    pub NewRequestContext: unsafe extern "system" fn (this: *const nsIRequestContextService, _retval: *mut *const nsIRequestContext) -> ::nserror::nsresult,

    /* void removeRequestContext (in unsigned long long id); */
    pub RemoveRequestContext: unsafe extern "system" fn (this: *const nsIRequestContextService, id: u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRequestContextService {

    /// ```text
    /// /**
    ///    * Get an existing request context from its ID
    ///    */
    /// ```
    ///

    /// `nsIRequestContext getRequestContext (in unsigned long long id);`
    #[inline]
    pub unsafe fn GetRequestContext(&self, id: u64, _retval: *mut *const nsIRequestContext) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestContext)(self, id, _retval)
    }


    /// ```text
    /// /**
    ///    * Shorthand to get request context from a load group
    ///    */
    /// ```
    ///

    /// `nsIRequestContext getRequestContextFromLoadGroup (in nsILoadGroup lg);`
    #[inline]
    pub unsafe fn GetRequestContextFromLoadGroup(&self, lg: *const nsILoadGroup, _retval: *mut *const nsIRequestContext) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestContextFromLoadGroup)(self, lg, _retval)
    }


    /// ```text
    /// /**
    ///    * Create a new request context
    ///    */
    /// ```
    ///

    /// `nsIRequestContext newRequestContext ();`
    #[inline]
    pub unsafe fn NewRequestContext(&self, _retval: *mut *const nsIRequestContext) -> ::nserror::nsresult {
        ((*self.vtable).NewRequestContext)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Remove an existing request context from its ID
    ///    */
    /// ```
    ///

    /// `void removeRequestContext (in unsigned long long id);`
    #[inline]
    pub unsafe fn RemoveRequestContext(&self, id: u64) -> ::nserror::nsresult {
        ((*self.vtable).RemoveRequestContext)(self, id)
    }


}


