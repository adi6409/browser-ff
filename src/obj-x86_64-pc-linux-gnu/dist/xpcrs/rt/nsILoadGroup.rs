//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadGroup.idl
//


/// `interface nsILoadGroup : nsIRequest`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadGroup {
    vtable: *const nsILoadGroupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadGroup.
unsafe impl XpCom for nsILoadGroup {
    const IID: nsIID = nsID(0xf0c87725, 0x7a35, 0x463c,
        [0x9c, 0xeb, 0x2c, 0x07, 0xf2, 0x34, 0x06, 0xcc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadGroup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadGroup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadGroupCoerce {
    /// Cheaply cast a value of this type from a `nsILoadGroup`.
    fn coerce_from(v: &nsILoadGroup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadGroupCoerce for nsILoadGroup {
    #[inline]
    fn coerce_from(v: &nsILoadGroup) -> &Self {
        v
    }
}

impl nsILoadGroup {
    /// Cast this `nsILoadGroup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadGroupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadGroup {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestCoerce> nsILoadGroupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadGroup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadGroup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadGroupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* attribute nsIRequestObserver groupObserver; */
    pub GetGroupObserver: unsafe extern "system" fn (this: *const nsILoadGroup, aGroupObserver: *mut*const nsIRequestObserver) -> ::nserror::nsresult,

    /* attribute nsIRequestObserver groupObserver; */
    pub SetGroupObserver: unsafe extern "system" fn (this: *const nsILoadGroup, aGroupObserver: *const nsIRequestObserver) -> ::nserror::nsresult,

    /* attribute nsIRequest defaultLoadRequest; */
    pub GetDefaultLoadRequest: unsafe extern "system" fn (this: *const nsILoadGroup, aDefaultLoadRequest: *mut *const nsIRequest) -> ::nserror::nsresult,

    /* attribute nsIRequest defaultLoadRequest; */
    pub SetDefaultLoadRequest: unsafe extern "system" fn (this: *const nsILoadGroup, aDefaultLoadRequest: *const nsIRequest) -> ::nserror::nsresult,

    /* void addRequest (in nsIRequest aRequest, in nsISupports aContext); */
    pub AddRequest: unsafe extern "system" fn (this: *const nsILoadGroup, aRequest: *const nsIRequest, aContext: *const nsISupports) -> ::nserror::nsresult,

    /* void removeRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus); */
    pub RemoveRequest: unsafe extern "system" fn (this: *const nsILoadGroup, aRequest: *const nsIRequest, aContext: *const nsISupports, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,

    /* readonly attribute nsISimpleEnumerator requests; */
    pub GetRequests: unsafe extern "system" fn (this: *const nsILoadGroup, aRequests: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* readonly attribute unsigned long activeCount; */
    pub GetActiveCount: unsafe extern "system" fn (this: *const nsILoadGroup, aActiveCount: *mut u32) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub GetNotificationCallbacks: unsafe extern "system" fn (this: *const nsILoadGroup, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub SetNotificationCallbacks: unsafe extern "system" fn (this: *const nsILoadGroup, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long requestContextID; */
    pub GetRequestContextID: unsafe extern "system" fn (this: *const nsILoadGroup, aRequestContextID: *mut u64) -> ::nserror::nsresult,

    /* attribute nsLoadFlags defaultLoadFlags; */
    pub GetDefaultLoadFlags: unsafe extern "system" fn (this: *const nsILoadGroup, aDefaultLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult,

    /* attribute nsLoadFlags defaultLoadFlags; */
    pub SetDefaultLoadFlags: unsafe extern "system" fn (this: *const nsILoadGroup, aDefaultLoadFlags: nsLoadFlags) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isBrowsingContextDiscarded; */
    pub GetIsBrowsingContextDiscarded: unsafe extern "system" fn (this: *const nsILoadGroup, aIsBrowsingContextDiscarded: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadGroup {

    /// ```text
    /// /**
    ///  * A load group maintains a collection of nsIRequest objects.
    ///  * This is used in lots of places where groups of requests need to be tracked.
    ///  * For example, Document::mDocumentLoadGroup is used to track all requests
    ///  * made for subdocuments in order to track page load progress and allow all
    ///  * requests made on behalf of the document to be stopped, etc.
    ///  */
    /// /**
    ///      * The group observer is notified when requests are added to and removed
    ///      * from this load group.  The groupObserver is weak referenced.
    ///      */
    /// ```
    ///

    /// `attribute nsIRequestObserver groupObserver;`
    #[inline]
    pub unsafe fn GetGroupObserver(&self, aGroupObserver: *mut*const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).GetGroupObserver)(self, aGroupObserver)
    }


    /// ```text
    /// /**
    ///  * A load group maintains a collection of nsIRequest objects.
    ///  * This is used in lots of places where groups of requests need to be tracked.
    ///  * For example, Document::mDocumentLoadGroup is used to track all requests
    ///  * made for subdocuments in order to track page load progress and allow all
    ///  * requests made on behalf of the document to be stopped, etc.
    ///  */
    /// /**
    ///      * The group observer is notified when requests are added to and removed
    ///      * from this load group.  The groupObserver is weak referenced.
    ///      */
    /// ```
    ///

    /// `attribute nsIRequestObserver groupObserver;`
    #[inline]
    pub unsafe fn SetGroupObserver(&self, aGroupObserver: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).SetGroupObserver)(self, aGroupObserver)
    }


    /// ```text
    /// /**
    ///      * Accesses the default load request for the group.  Each time a number
    ///      * of requests are added to a group, the defaultLoadRequest may be set
    ///      * to indicate that all of the requests are related to a base request.
    ///      *
    ///      * The load group inherits its load flags from the default load request.
    ///      * If the default load request is NULL, then the group's load flags are
    ///      * not changed.
    ///      */
    /// ```
    ///

    /// `attribute nsIRequest defaultLoadRequest;`
    #[inline]
    pub unsafe fn GetDefaultLoadRequest(&self, aDefaultLoadRequest: *mut *const nsIRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultLoadRequest)(self, aDefaultLoadRequest)
    }


    /// ```text
    /// /**
    ///      * Accesses the default load request for the group.  Each time a number
    ///      * of requests are added to a group, the defaultLoadRequest may be set
    ///      * to indicate that all of the requests are related to a base request.
    ///      *
    ///      * The load group inherits its load flags from the default load request.
    ///      * If the default load request is NULL, then the group's load flags are
    ///      * not changed.
    ///      */
    /// ```
    ///

    /// `attribute nsIRequest defaultLoadRequest;`
    #[inline]
    pub unsafe fn SetDefaultLoadRequest(&self, aDefaultLoadRequest: *const nsIRequest) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultLoadRequest)(self, aDefaultLoadRequest)
    }


    /// ```text
    /// /**
    ///      * Adds a new request to the group.  This will cause the default load
    ///      * flags to be applied to the request.  If this is a foreground
    ///      * request then the groupObserver's onStartRequest will be called.
    ///      *
    ///      * If the request is the default load request or if the default load
    ///      * request is null, then the load group will inherit its load flags from
    ///      * the request.
    ///      */
    /// ```
    ///

    /// `void addRequest (in nsIRequest aRequest, in nsISupports aContext);`
    #[inline]
    pub unsafe fn AddRequest(&self, aRequest: *const nsIRequest, aContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).AddRequest)(self, aRequest, aContext)
    }


    /// ```text
    /// /**
    ///      * Removes a request from the group.  If this is a foreground request
    ///      * then the groupObserver's onStopRequest will be called.
    ///      *
    ///      * By the time this call ends, aRequest will have been removed from the
    ///      * loadgroup, even if this function throws an exception.
    ///      */
    /// ```
    ///

    /// `void removeRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus);`
    #[inline]
    pub unsafe fn RemoveRequest(&self, aRequest: *const nsIRequest, aContext: *const nsISupports, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).RemoveRequest)(self, aRequest, aContext, aStatus)
    }


    /// ```text
    /// /**
    ///      * Returns the requests contained directly in this group.
    ///      * Enumerator element type: nsIRequest.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsISimpleEnumerator requests;`
    #[inline]
    pub unsafe fn GetRequests(&self, aRequests: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetRequests)(self, aRequests)
    }


    /// ```text
    /// /**
    ///      * Returns the count of "active" requests (ie. requests without the
        ///      * LOAD_BACKGROUND bit set).
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long activeCount;`
    #[inline]
    pub unsafe fn GetActiveCount(&self, aActiveCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveCount)(self, aActiveCount)
    }


    /// ```text
    /// /**
    ///      * Notification callbacks for the load group.
    ///      */
    /// ```
    ///

    /// `attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn GetNotificationCallbacks(&self, aNotificationCallbacks: *mut*const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).GetNotificationCallbacks)(self, aNotificationCallbacks)
    }


    /// ```text
    /// /**
    ///      * Notification callbacks for the load group.
    ///      */
    /// ```
    ///

    /// `attribute nsIInterfaceRequestor notificationCallbacks;`
    #[inline]
    pub unsafe fn SetNotificationCallbacks(&self, aNotificationCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).SetNotificationCallbacks)(self, aNotificationCallbacks)
    }


    /// ```text
    /// /**
    ///      * Context for managing things like js/css connection blocking,
    ///      * and per-tab connection grouping.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long long requestContextID;`
    #[inline]
    pub unsafe fn GetRequestContextID(&self, aRequestContextID: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestContextID)(self, aRequestContextID)
    }


    /// ```text
    /// /**
    ///      * The set of load flags that will be added to all new requests added to
    ///      * this group. Any existing requests in the load group are not modified,
    ///      * so it is expected these flags will be added before requests are added
    ///      * to the group - typically via nsIDocShell::defaultLoadFlags on a new
    ///      * docShell.
    ///      * Note that these flags are *not* added to the default request for the
    ///      * load group; it is expected the default request will already have these
    ///      * flags (again, courtesy of setting nsIDocShell::defaultLoadFlags before
        ///      * the docShell has created the default request.)
    ///      */
    /// ```
    ///

    /// `attribute nsLoadFlags defaultLoadFlags;`
    #[inline]
    pub unsafe fn GetDefaultLoadFlags(&self, aDefaultLoadFlags: *mut nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultLoadFlags)(self, aDefaultLoadFlags)
    }


    /// ```text
    /// /**
    ///      * The set of load flags that will be added to all new requests added to
    ///      * this group. Any existing requests in the load group are not modified,
    ///      * so it is expected these flags will be added before requests are added
    ///      * to the group - typically via nsIDocShell::defaultLoadFlags on a new
    ///      * docShell.
    ///      * Note that these flags are *not* added to the default request for the
    ///      * load group; it is expected the default request will already have these
    ///      * flags (again, courtesy of setting nsIDocShell::defaultLoadFlags before
        ///      * the docShell has created the default request.)
    ///      */
    /// ```
    ///

    /// `attribute nsLoadFlags defaultLoadFlags;`
    #[inline]
    pub unsafe fn SetDefaultLoadFlags(&self, aDefaultLoadFlags: nsLoadFlags) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultLoadFlags)(self, aDefaultLoadFlags)
    }


    /// ```text
    /// /**
    ///      * Returns true if the loadGroup belongs to a discarded context, such as, a
    ///      * terminated private browsing session.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isBrowsingContextDiscarded;`
    #[inline]
    pub unsafe fn GetIsBrowsingContextDiscarded(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsBrowsingContextDiscarded)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


}


