//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgress.idl
//


/// `interface nsIWebProgress : nsISupports`
///

/// ```text
/// /**
///  * The nsIWebProgress interface is used to add or remove nsIWebProgressListener
///  * instances to observe the loading of asynchronous requests (usually in the
    ///  * context of a DOM window).
///  *
///  * nsIWebProgress instances may be arranged in a parent-child configuration,
///  * corresponding to the parent-child configuration of their respective DOM
///  * windows.  However, in some cases a nsIWebProgress instance may not have an
///  * associated DOM window.  The parent-child relationship of nsIWebProgress
///  * instances is not made explicit by this interface, but the relationship may
///  * exist in some implementations.
///  *
///  * A nsIWebProgressListener instance receives notifications for the
///  * nsIWebProgress instance to which it added itself, and it may also receive
///  * notifications from any nsIWebProgress instances that are children of that
///  * nsIWebProgress instance.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebProgress {
    vtable: *const nsIWebProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebProgress.
unsafe impl XpCom for nsIWebProgress {
    const IID: nsIID = nsID(0xc4d64640, 0xb332, 0x4db6,
        [0xa2, 0xa5, 0xe0, 0x85, 0x66, 0x00, 0x0d, 0xc9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebProgress {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebProgress.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebProgressCoerce {
    /// Cheaply cast a value of this type from a `nsIWebProgress`.
    fn coerce_from(v: &nsIWebProgress) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebProgressCoerce for nsIWebProgress {
    #[inline]
    fn coerce_from(v: &nsIWebProgress) -> &Self {
        v
    }
}

impl nsIWebProgress {
    /// Cast this `nsIWebProgress` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebProgress {
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
impl<T: nsISupportsCoerce> nsIWebProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebProgress) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebProgress
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebProgressVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask); */
    pub AddProgressListener: unsafe extern "system" fn (this: *const nsIWebProgress, aListener: *const nsIWebProgressListener, aNotifyMask: u32) -> ::nserror::nsresult,

    /* void removeProgressListener (in nsIWebProgressListener aListener); */
    pub RemoveProgressListener: unsafe extern "system" fn (this: *const nsIWebProgress, aListener: *const nsIWebProgressListener) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy DOMWindow; */
    pub GetDOMWindow: unsafe extern "system" fn (this: *const nsIWebProgress, aDOMWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute boolean isTopLevel; */
    pub GetIsTopLevel: unsafe extern "system" fn (this: *const nsIWebProgress, aIsTopLevel: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isLoadingDocument; */
    pub GetIsLoadingDocument: unsafe extern "system" fn (this: *const nsIWebProgress, aIsLoadingDocument: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long loadType; */
    pub GetLoadType: unsafe extern "system" fn (this: *const nsIWebProgress, aLoadType: *mut u32) -> ::nserror::nsresult,

    /* attribute nsIEventTarget target; */
    pub GetTarget: unsafe extern "system" fn (this: *const nsIWebProgress, aTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,

    /* attribute nsIEventTarget target; */
    pub SetTarget: unsafe extern "system" fn (this: *const nsIWebProgress, aTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebProgress {
    /// ```text
    /// /**
    ///    * The following flags may be combined to form the aNotifyMask parameter for
    ///    * the addProgressListener method.  They limit the set of events that are
    ///    * delivered to an nsIWebProgressListener instance.
    ///    */
    /// /**
    ///    * These flags indicate the state transistions to observe, corresponding to
    ///    * nsIWebProgressListener::onStateChange.
    ///    *
    ///    * NOTIFY_STATE_REQUEST
    ///    *   Only receive the onStateChange event if the aStateFlags parameter
    ///    *   includes nsIWebProgressListener::STATE_IS_REQUEST.
    ///    *
    ///    * NOTIFY_STATE_DOCUMENT
    ///    *   Only receive the onStateChange event if the aStateFlags parameter
    ///    *   includes nsIWebProgressListener::STATE_IS_DOCUMENT.
    ///    *
    ///    * NOTIFY_STATE_NETWORK
    ///    *   Only receive the onStateChange event if the aStateFlags parameter
    ///    *   includes nsIWebProgressListener::STATE_IS_NETWORK.
    ///    *
    ///    * NOTIFY_STATE_WINDOW
    ///    *   Only receive the onStateChange event if the aStateFlags parameter
    ///    *   includes nsIWebProgressListener::STATE_IS_WINDOW.
    ///    *
    ///    * NOTIFY_STATE_ALL
    ///    *   Receive all onStateChange events.
    ///    */
    /// ```
    ///

    pub const NOTIFY_STATE_REQUEST: i64 = 1;


    pub const NOTIFY_STATE_DOCUMENT: i64 = 2;


    pub const NOTIFY_STATE_NETWORK: i64 = 4;


    pub const NOTIFY_STATE_WINDOW: i64 = 8;


    pub const NOTIFY_STATE_ALL: i64 = 15;

    /// ```text
    /// /**
    ///    * These flags indicate the other events to observe, corresponding to the
    ///    * other four methods defined on nsIWebProgressListener.
    ///    *
    ///    * NOTIFY_PROGRESS
    ///    *   Receive onProgressChange events.
    ///    *
    ///    * NOTIFY_STATUS
    ///    *   Receive onStatusChange events.
    ///    *
    ///    * NOTIFY_SECURITY
    ///    *   Receive onSecurityChange events.
    ///    *
    ///    * NOTIFY_LOCATION
    ///    *   Receive onLocationChange events.
    ///    *
    ///    * NOTIFY_CONTENT_BLOCKING
    ///    *   Receive onContentBlockingEvent events.
    ///    *
    ///    * NOTIFY_REFRESH
    ///    *   Receive onRefreshAttempted events.
    ///    *   This is defined on nsIWebProgressListener2.
    ///    */
    /// ```
    ///

    pub const NOTIFY_PROGRESS: i64 = 16;


    pub const NOTIFY_STATUS: i64 = 32;


    pub const NOTIFY_SECURITY: i64 = 64;


    pub const NOTIFY_LOCATION: i64 = 128;


    pub const NOTIFY_REFRESH: i64 = 256;


    pub const NOTIFY_CONTENT_BLOCKING: i64 = 512;

    /// ```text
    /// /**
    ///    * This flag enables all notifications.
    ///    */
    /// ```
    ///

    pub const NOTIFY_ALL: i64 = 1023;

    /// ```text
    /// /**
    ///    * Registers a listener to receive web progress events.
    ///    *
    ///    * @param aListener
    ///    *        The listener interface to be called when a progress event occurs.
    ///    *        This object must also implement nsISupportsWeakReference.
    ///    * @param aNotifyMask
    ///    *        The types of notifications to receive.
    ///    *
    ///    * @throw NS_ERROR_INVALID_ARG
    ///    *        Indicates that aListener was either null or that it does not
    ///    *        support weak references.
    ///    * @throw NS_ERROR_FAILURE
    ///    *        Indicates that aListener was already registered.
    ///    */
    /// ```
    ///

    /// `void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask);`
    #[inline]
    pub unsafe fn AddProgressListener(&self, aListener: *const nsIWebProgressListener, aNotifyMask: u32) -> ::nserror::nsresult {
        ((*self.vtable).AddProgressListener)(self, aListener, aNotifyMask)
    }


    /// ```text
    /// /**
    ///    * Removes a previously registered listener of progress events.
    ///    *
    ///    * @param aListener
    ///    *        The listener interface previously registered with a call to
    ///    *        addProgressListener.
    ///    *
    ///    * @throw NS_ERROR_FAILURE
    ///    *        Indicates that aListener was not registered.
    ///    */
    /// ```
    ///

    /// `void removeProgressListener (in nsIWebProgressListener aListener);`
    #[inline]
    pub unsafe fn RemoveProgressListener(&self, aListener: *const nsIWebProgressListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveProgressListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * The DOM window associated with this nsIWebProgress instance.
    ///    *
    ///    * @throw NS_ERROR_FAILURE
    ///    *        Indicates that there is no associated DOM window.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy DOMWindow;`
    #[inline]
    pub unsafe fn GetDOMWindow(&self, aDOMWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetDOMWindow)(self, aDOMWindow)
    }


    /// ```text
    /// /**
    ///    * Indicates whether DOMWindow.top == DOMWindow.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isTopLevel;`
    #[inline]
    pub unsafe fn GetIsTopLevel(&self, aIsTopLevel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsTopLevel)(self, aIsTopLevel)
    }


    /// ```text
    /// /**
    ///    * Indicates whether or not a document is currently being loaded
    ///    * in the context of this nsIWebProgress instance.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isLoadingDocument;`
    #[inline]
    pub unsafe fn GetIsLoadingDocument(&self, aIsLoadingDocument: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsLoadingDocument)(self, aIsLoadingDocument)
    }


    /// ```text
    /// /**
    ///    * Contains a load type as specified by the load* constants in
    ///    * nsIDocShellLoadInfo.idl.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long loadType;`
    #[inline]
    pub unsafe fn GetLoadType(&self, aLoadType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadType)(self, aLoadType)
    }


    /// ```text
    /// /**
    ///    * Main thread event target to which progress updates should be
    ///    * dispatched. This typically will be a SchedulerEventTarget
    ///    * corresponding to the tab requesting updates.
    ///    */
    /// ```
    ///

    /// `attribute nsIEventTarget target;`
    #[inline]
    pub unsafe fn GetTarget(&self, aTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetTarget)(self, aTarget)
    }


    /// ```text
    /// /**
    ///    * Main thread event target to which progress updates should be
    ///    * dispatched. This typically will be a SchedulerEventTarget
    ///    * corresponding to the tab requesting updates.
    ///    */
    /// ```
    ///

    /// `attribute nsIEventTarget target;`
    #[inline]
    pub unsafe fn SetTarget(&self, aTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).SetTarget)(self, aTarget)
    }


}


