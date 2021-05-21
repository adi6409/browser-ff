//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIEventTarget.idl
//


/// `interface nsIEventTarget : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEventTarget {
    vtable: *const nsIEventTargetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEventTarget.
unsafe impl XpCom for nsIEventTarget {
    const IID: nsIID = nsID(0xa03b8b63, 0xaf8b, 0x4164,
        [0xb0, 0xe5, 0xc4, 0x1e, 0x8b, 0x2b, 0x7c, 0xfa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEventTarget {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEventTarget.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEventTargetCoerce {
    /// Cheaply cast a value of this type from a `nsIEventTarget`.
    fn coerce_from(v: &nsIEventTarget) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEventTargetCoerce for nsIEventTarget {
    #[inline]
    fn coerce_from(v: &nsIEventTarget) -> &Self {
        v
    }
}

impl nsIEventTarget {
    /// Cast this `nsIEventTarget` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEventTargetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEventTarget {
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
impl<T: nsISupportsCoerce> nsIEventTargetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventTarget) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEventTarget
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEventTargetVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript,notxpcom] boolean isOnCurrentThreadInfallible (); */
    pub IsOnCurrentThreadInfallible: unsafe extern "system" fn (this: *const nsIEventTarget) -> bool,

    /* boolean isOnCurrentThread (); */
    pub IsOnCurrentThread: unsafe extern "system" fn (this: *const nsIEventTarget, _retval: *mut bool) -> ::nserror::nsresult,

    /* [binaryname(Dispatch),noscript] void dispatchFromC (in alreadyAddRefed_nsIRunnable event, [default (DISPATCH_NORMAL)] in unsigned long flags); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub Dispatch: *const ::libc::c_void,

    /* [binaryname(DispatchFromScript)] void dispatch (in nsIRunnable event, in unsigned long flags); */
    pub DispatchFromScript: unsafe extern "system" fn (this: *const nsIEventTarget, event: *const nsIRunnable, flags: u32) -> ::nserror::nsresult,

    /* [noscript] void delayedDispatch (in alreadyAddRefed_nsIRunnable event, in unsigned long delay); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub DelayedDispatch: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEventTarget {
    /// ```text
    /// /**
    ///    * This flag specifies the default mode of event dispatch, whereby the event
    ///    * is simply queued for later processing.  When this flag is specified,
    ///    * dispatch returns immediately after the event is queued.
    ///    */
    /// ```
    ///

    pub const DISPATCH_NORMAL: i64 = 0;

    /// ```text
    /// /**
    ///    * This flag specifies the synchronous mode of event dispatch, in which the
    ///    * dispatch method does not return until the event has been processed.
    ///    *
    ///    * NOTE: passing this flag to dispatch may have the side-effect of causing
    ///    * other events on the current thread to be processed while waiting for the
    ///    * given event to be processed.
    ///    */
    /// ```
    ///

    pub const DISPATCH_SYNC: i64 = 1;

    /// ```text
    /// /**
    ///    * This flag specifies that the dispatch is occurring from a running event
    ///    * that was dispatched to the same event target, and that event is about to
    ///    * finish.
    ///    *
    ///    * A thread pool can use this as an optimization hint to not spin up
    ///    * another thread, since the current thread is about to become idle.
    ///    *
    ///    * These events are always async.
    ///    */
    /// ```
    ///

    pub const DISPATCH_AT_END: i64 = 2;

    /// ```text
    /// /**
    ///    * This flag specifies that the dispatched event may block the thread on
    ///    * which it executes, usually by doing some sort of I/O.  This information
    ///    * may be used by the event target to execute the job on a thread
    ///    * specifically dedicated to doing I/O, leaving other threads available for
    ///    * CPU-intensive work.
    ///    */
    /// ```
    ///

    pub const DISPATCH_EVENT_MAY_BLOCK: i64 = 4;

    /// ```text
    /// /**
    ///    * IsOnCurrentThread() should return true if events dispatched to this target
    ///    * can possibly run on the current thread, and false otherwise. In the case
    ///    * of an nsIEventTarget for a thread pool, it should return true on all
    ///    * threads in the pool. In the case of a non-thread nsIEventTarget such as
    ///    * ThrottledEventQueue, it should return true on the thread where events are
    ///    * expected to be processed, even if no events from the queue are actually
    ///    * being processed right now.
    ///    *
    ///    * When called on an nsISerialEventTarget, IsOnCurrentThread can be used to
    ///    * ensure that no other thread has "ownership" of the event target. As such,
    ///    * it's useful for asserting that an object is only used on a particular
    ///    * thread. IsOnCurrentThread can't guarantee that the current event has been
    ///    * dispatched through a particular event target.
    ///    *
    ///    * The infallible version of IsOnCurrentThread() is optimized to avoid a
    ///    * virtual call for non-thread event targets. Thread targets should set
    ///    * mThread to their virtual PRThread. Non-thread targets should leave
    ///    * mThread null and implement IsOnCurrentThreadInfallible() to
    ///    * return the correct answer.
    ///    *
    ///    * The fallible version of IsOnCurrentThread may return errors, such as during
    ///    * shutdown. If it does not return an error, it should return the same result
    ///    * as the infallible version. The infallible method should return the correct
    ///    * result regardless of whether the fallible method returns an error.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] boolean isOnCurrentThreadInfallible ();`
    #[inline]
    pub unsafe fn IsOnCurrentThreadInfallible(&self, ) -> bool {
        ((*self.vtable).IsOnCurrentThreadInfallible)(self, )
    }



    /// `boolean isOnCurrentThread ();`
    #[inline]
    pub unsafe fn IsOnCurrentThread(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsOnCurrentThread)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Dispatch an event to this event target.  This function may be called from
    ///    * any thread, and it may be called re-entrantly.
    ///    *
    ///    * @param event
    ///    *   The alreadyAddRefed<> event to dispatch.
    ///    *   NOTE that the event will be leaked if it fails to dispatch.
    ///    * @param flags
    ///    *   The flags modifying event dispatch.  The flags are described in detail
    ///    *   below.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   Indicates that event is null.
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that the thread is shutting down and has finished processing
    ///    * events, so this event would never run and has not been dispatched.
    ///    */
    /// ```
    ///

    /// `[binaryname(Dispatch),noscript] void dispatchFromC (in alreadyAddRefed_nsIRunnable event, [default (DISPATCH_NORMAL)] in unsigned long flags);`
    const _Dispatch: () = ();

    /// ```text
    /// /**
    ///    * Version of Dispatch to expose to JS, which doesn't require an alreadyAddRefed<>
    ///    * (it will be converted to that internally)
    ///    *
    ///    * @param event
    ///    *   The (raw) event to dispatch.
    ///    * @param flags
    ///    *   The flags modifying event dispatch.  The flags are described in detail
    ///    *   below.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   Indicates that event is null.
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that the thread is shutting down and has finished processing
    ///    * events, so this event would never run and has not been dispatched.
    ///    */
    /// ```
    ///

    /// `[binaryname(DispatchFromScript)] void dispatch (in nsIRunnable event, in unsigned long flags);`
    #[inline]
    pub unsafe fn DispatchFromScript(&self, event: *const nsIRunnable, flags: u32) -> ::nserror::nsresult {
        ((*self.vtable).DispatchFromScript)(self, event, flags)
    }


    /// ```text
    /// /**
    ///    * Dispatch an event to this event target, but do not run it before delay
    ///    * milliseconds have passed.  This function may be called from any thread.
    ///    *
    ///    * @param event
    ///    *   The alreadyAddrefed<> event to dispatch.
    ///    * @param delay
    ///    *   The delay (in ms) before running the event.  If event does not rise to
    ///    *   the top of the event queue before the delay has passed, it will be set
    ///    *   aside to execute once the delay has passed.  Otherwise, it will be
    ///    *   executed immediately.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   Indicates that event is null.
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that the thread is shutting down and has finished processing
    ///    * events, so this event would never run and has not been dispatched, or
    ///    * that delay is zero.
    ///    */
    /// ```
    ///

    /// `[noscript] void delayedDispatch (in alreadyAddRefed_nsIRunnable event, in unsigned long delay);`
    const _DelayedDispatch: () = ();

}


