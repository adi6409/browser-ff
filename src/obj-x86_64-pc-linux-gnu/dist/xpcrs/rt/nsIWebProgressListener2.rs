//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgressListener2.idl
//


/// `interface nsIWebProgressListener2 : nsIWebProgressListener`
///

/// ```text
/// /**
///  * An extended version of nsIWebProgressListener.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebProgressListener2 {
    vtable: *const nsIWebProgressListener2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebProgressListener2.
unsafe impl XpCom for nsIWebProgressListener2 {
    const IID: nsIID = nsID(0xdde39de0, 0xe4e0, 0x11da,
        [0x8a, 0xd9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebProgressListener2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebProgressListener2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebProgressListener2Coerce {
    /// Cheaply cast a value of this type from a `nsIWebProgressListener2`.
    fn coerce_from(v: &nsIWebProgressListener2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebProgressListener2Coerce for nsIWebProgressListener2 {
    #[inline]
    fn coerce_from(v: &nsIWebProgressListener2) -> &Self {
        v
    }
}

impl nsIWebProgressListener2 {
    /// Cast this `nsIWebProgressListener2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebProgressListener2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebProgressListener2 {
    type Target = nsIWebProgressListener;
    #[inline]
    fn deref(&self) -> &nsIWebProgressListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIWebProgressListenerCoerce> nsIWebProgressListener2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebProgressListener2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebProgressListener2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebProgressListener2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIWebProgressListenerVTable,

    /* void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress); */
    pub OnProgressChange64: unsafe extern "system" fn (this: *const nsIWebProgressListener2, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aCurSelfProgress: i64, aMaxSelfProgress: i64, aCurTotalProgress: i64, aMaxTotalProgress: i64) -> ::nserror::nsresult,

    /* boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI); */
    pub OnRefreshAttempted: unsafe extern "system" fn (this: *const nsIWebProgressListener2, aWebProgress: *const nsIWebProgress, aRefreshURI: *const nsIURI, aMillis: i32, aSameURI: bool, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebProgressListener2 {

    /// ```text
    /// /**
    ///    * Notification that the progress has changed for one of the requests
    ///    * associated with aWebProgress.  Progress totals are reset to zero when all
    ///    * requests in aWebProgress complete (corresponding to onStateChange being
        ///    * called with aStateFlags including the STATE_STOP and STATE_IS_WINDOW
        ///    * flags).
    ///    *
    ///    * This function is identical to nsIWebProgressListener::onProgressChange,
    ///    * except that this function supports 64-bit values.
    ///    *
    ///    * @param aWebProgress
    ///    *        The nsIWebProgress instance that fired the notification.
    ///    * @param aRequest
    ///    *        The nsIRequest that has new progress.
    ///    * @param aCurSelfProgress
    ///    *        The current progress for aRequest.
    ///    * @param aMaxSelfProgress
    ///    *        The maximum progress for aRequest.
    ///    * @param aCurTotalProgress
    ///    *        The current progress for all requests associated with aWebProgress.
    ///    * @param aMaxTotalProgress
    ///    *        The total progress for all requests associated with aWebProgress.
    ///    *
    ///    * NOTE: If any progress value is unknown, then its value is replaced with -1.
    ///    *
    ///    * @see nsIWebProgressListener2::onProgressChange64
    ///    */
    /// ```
    ///

    /// `void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress);`
    #[inline]
    pub unsafe fn OnProgressChange64(&self, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aCurSelfProgress: i64, aMaxSelfProgress: i64, aCurTotalProgress: i64, aMaxTotalProgress: i64) -> ::nserror::nsresult {
        ((*self.vtable).OnProgressChange64)(self, aWebProgress, aRequest, aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress)
    }


    /// ```text
    /// /**
    ///    * Notification that a refresh or redirect has been requested in aWebProgress
    ///    * For example, via a <meta http-equiv="refresh"> or an HTTP Refresh: header
    ///    *
    ///    * @param aWebProgress
    ///    *        The nsIWebProgress instance that fired the notification.
    ///    * @param aRefreshURI
    ///    *        The new URI that aWebProgress has requested redirecting to.
    ///    * @param aMillis
    ///    *        The delay (in milliseconds) before refresh.
    ///    * @param aSameURI
    ///    *        True if aWebProgress is requesting a refresh of the
    ///    *        current URI.
    ///    *        False if aWebProgress is requesting a redirection to
    ///    *        a different URI.
    ///    *
    ///    * @return True if the refresh may proceed.
    ///    *         False if the refresh should be aborted.
    ///    */
    /// ```
    ///

    /// `boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI);`
    #[inline]
    pub unsafe fn OnRefreshAttempted(&self, aWebProgress: *const nsIWebProgress, aRefreshURI: *const nsIURI, aMillis: i32, aSameURI: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnRefreshAttempted)(self, aWebProgress, aRefreshURI, aMillis, aSameURI, _retval)
    }


}


