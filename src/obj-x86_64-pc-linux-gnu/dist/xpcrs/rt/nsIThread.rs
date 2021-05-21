//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThread.idl
//


/// `interface nsIThread : nsISerialEventTarget`
///

/// ```text
/// /**
///  * This interface provides a high-level abstraction for an operating system
///  * thread.
///  *
///  * Threads have a built-in event queue, and a thread is an event target that
///  * can receive nsIRunnable objects (events) to be processed on the thread.
///  *
///  * See nsIThreadManager for the API used to create and locate threads.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThread {
    vtable: *const nsIThreadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThread.
unsafe impl XpCom for nsIThread {
    const IID: nsIID = nsID(0x5801d193, 0x29d1, 0x4964,
        [0xa6, 0xb7, 0x70, 0xeb, 0x69, 0x7d, 0xdf, 0x2b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThread {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThread.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadCoerce {
    /// Cheaply cast a value of this type from a `nsIThread`.
    fn coerce_from(v: &nsIThread) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadCoerce for nsIThread {
    #[inline]
    fn coerce_from(v: &nsIThread) -> &Self {
        v
    }
}

impl nsIThread {
    /// Cast this `nsIThread` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThread {
    type Target = nsISerialEventTarget;
    #[inline]
    fn deref(&self) -> &nsISerialEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISerialEventTargetCoerce> nsIThreadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThread) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThread
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISerialEventTargetVTable,

    /* [noscript] readonly attribute PRThread PRThread; */
    /// Unable to generate binding because `native type PRThread unsupported`
    pub GetPRThread: *const ::libc::c_void,

    /* [noscript] attribute boolean CanInvokeJS; */
    pub GetCanInvokeJS: unsafe extern "system" fn (this: *const nsIThread, aCanInvokeJS: *mut bool) -> ::nserror::nsresult,

    /* [noscript] attribute boolean CanInvokeJS; */
    pub SetCanInvokeJS: unsafe extern "system" fn (this: *const nsIThread, aCanInvokeJS: bool) -> ::nserror::nsresult,

    /* void shutdown (); */
    pub Shutdown: unsafe extern "system" fn (this: *const nsIThread) -> ::nserror::nsresult,

    /* boolean hasPendingEvents (); */
    pub HasPendingEvents: unsafe extern "system" fn (this: *const nsIThread, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean hasPendingHighPriorityEvents (); */
    pub HasPendingHighPriorityEvents: unsafe extern "system" fn (this: *const nsIThread, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean processNextEvent (in boolean mayWait); */
    pub ProcessNextEvent: unsafe extern "system" fn (this: *const nsIThread, mayWait: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* void asyncShutdown (); */
    pub AsyncShutdown: unsafe extern "system" fn (this: *const nsIThread) -> ::nserror::nsresult,

    /* [noscript] void dispatchToQueue (in alreadyAddRefed_nsIRunnable event, in EventQueuePriority queue); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub DispatchToQueue: *const ::libc::c_void,

    /* readonly attribute nsIEventTarget eventTarget; */
    pub GetEventTarget: unsafe extern "system" fn (this: *const nsIThread, aEventTarget: *mut *const nsIEventTarget) -> ::nserror::nsresult,

    /* [noscript,notxpcom] nsIEventTargetPtr EventTarget (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub EventTarget: *const ::libc::c_void,

    /* [noscript,notxpcom] nsISerialEventTargetPtr SerialEventTarget (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SerialEventTarget: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp lastLongTaskEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLastLongTaskEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp lastLongNonIdleTaskEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLastLongNonIdleTaskEnd: *const ::libc::c_void,

    /* [noscript] void getRunningEventDelay (out TimeDuration delay, out TimeStamp start); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetRunningEventDelay: *const ::libc::c_void,

    /* [noscript] void setRunningEventDelay (in TimeDuration delay, in TimeStamp start); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetRunningEventDelay: *const ::libc::c_void,

    /* [noscript] void setNameForWakeupTelemetry (in ACString name); */
    pub SetNameForWakeupTelemetry: unsafe extern "system" fn (this: *const nsIThread, name: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThread {

    /// ```text
    /// /**
    ///    * @returns
    ///    *   The NSPR thread object corresponding to this nsIThread.
    ///    */
    /// ```
    ///

    /// `[noscript] readonly attribute PRThread PRThread;`
    const _GetPRThread: () = ();

    /// ```text
    /// /**
    ///    * @returns
    ///    *  Whether or not this thread may call into JS. Used in the profiler
    ///    *  to avoid some unnecessary locking.
    ///    */
    /// ```
    ///

    /// `[noscript] attribute boolean CanInvokeJS;`
    #[inline]
    pub unsafe fn GetCanInvokeJS(&self, aCanInvokeJS: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanInvokeJS)(self, aCanInvokeJS)
    }


    /// ```text
    /// /**
    ///    * @returns
    ///    *  Whether or not this thread may call into JS. Used in the profiler
    ///    *  to avoid some unnecessary locking.
    ///    */
    /// ```
    ///

    /// `[noscript] attribute boolean CanInvokeJS;`
    #[inline]
    pub unsafe fn SetCanInvokeJS(&self, aCanInvokeJS: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCanInvokeJS)(self, aCanInvokeJS)
    }


    /// ```text
    /// /**
    ///    * Shutdown the thread.  This method prevents further dispatch of events to
    ///    * the thread, and it causes any pending events to run to completion before
    ///    * the thread joins (see PR_JoinThread) with the current thread.  During this
    ///    * method call, events for the current thread may be processed.
    ///    *
    ///    * This method MAY NOT be executed from the thread itself.  Instead, it is
    ///    * meant to be executed from another thread (usually the thread that created
        ///    * this thread or the main application thread).  When this function returns,
    ///    * the thread will be shutdown, and it will no longer be possible to dispatch
    ///    * events to the thread.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that this method was erroneously called when this thread was
    ///    *   the current thread, that this thread was not created with a call to
    ///    *   nsIThreadManager::NewThread, or if this method was called more than once
    ///    *   on the thread object.
    ///    */
    /// ```
    ///

    /// `void shutdown ();`
    #[inline]
    pub unsafe fn Shutdown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Shutdown)(self, )
    }


    /// ```text
    /// /**
    ///    * This method may be called to determine if there are any events ready to be
    ///    * processed.  It may only be called when this thread is the current thread.
    ///    *
    ///    * Because events may be added to this thread by another thread, a "false"
    ///    * result does not mean that this thread has no pending events.  It only
    ///    * means that there were no pending events when this method was called.
    ///    *
    ///    * @returns
    ///    *   A boolean value that if "true" indicates that this thread has one or
    ///    *   more pending events.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that this method was erroneously called when this thread was
    ///    *   not the current thread.
    ///    */
    /// ```
    ///

    /// `boolean hasPendingEvents ();`
    #[inline]
    pub unsafe fn HasPendingEvents(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasPendingEvents)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Similar to above, but checks only possible high priority queue.
    ///    */
    /// ```
    ///

    /// `boolean hasPendingHighPriorityEvents ();`
    #[inline]
    pub unsafe fn HasPendingHighPriorityEvents(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasPendingHighPriorityEvents)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Process the next event.  If there are no pending events, then this method
    ///    * may wait -- depending on the value of the mayWait parameter -- until an
    ///    * event is dispatched to this thread.  This method is re-entrant but may
    ///    * only be called if this thread is the current thread.
    ///    *
    ///    * @param mayWait
    ///    *   A boolean parameter that if "true" indicates that the method may block
    ///    *   the calling thread to wait for a pending event.
    ///    *
    ///    * @returns
    ///    *   A boolean value that if "true" indicates that an event was processed.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that this method was erroneously called when this thread was
    ///    *   not the current thread.
    ///    */
    /// ```
    ///

    /// `boolean processNextEvent (in boolean mayWait);`
    #[inline]
    pub unsafe fn ProcessNextEvent(&self, mayWait: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ProcessNextEvent)(self, mayWait, _retval)
    }


    /// ```text
    /// /**
    ///    * Shutdown the thread asynchronously.  This method immediately prevents
    ///    * further dispatch of events to the thread, and it causes any pending events
    ///    * to run to completion before this thread joins with the current thread.
    ///    *
    ///    * UNLIKE shutdown() this does not process events on the current thread.
    ///    * Instead it merely ensures that the current thread continues running until
    ///    * this thread has shut down.
    ///    *
    ///    * This method MAY NOT be executed from the thread itself.  Instead, it is
    ///    * meant to be executed from another thread (usually the thread that created
        ///    * this thread or the main application thread).  When this function returns,
    ///    * the thread will continue running until it exhausts its event queue.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that this method was erroneously called when this thread was
    ///    *   the current thread, that this thread was not created with a call to
    ///    *   nsIThreadManager::NewThread, or if this method was called more than once
    ///    *   on the thread object.
    ///    */
    /// ```
    ///

    /// `void asyncShutdown ();`
    #[inline]
    pub unsafe fn AsyncShutdown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AsyncShutdown)(self, )
    }


    /// ```text
    /// /**
    ///    * Dispatch an event to a specified queue for the thread.  This function
    ///    * may be called from any thread, and it may be called re-entrantly.
    ///    * Most users should use the NS_Dispatch*() functions in nsThreadUtils instead
    ///    * of calling this directly.
    ///    *
    ///    * @param event
    ///    *   The alreadyAddRefed<> event to dispatch.
    ///    *   NOTE that the event will be leaked if it fails to dispatch.
    ///    * @param queue
    ///    *   Which event priority queue this should be added to
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   Indicates that event is null.
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   Indicates that the thread is shutting down and has finished processing
    ///    * events, so this event would never run and has not been dispatched.
    ///    */
    /// ```
    ///

    /// `[noscript] void dispatchToQueue (in alreadyAddRefed_nsIRunnable event, in EventQueuePriority queue);`
    const _DispatchToQueue: () = ();

    /// ```text
    /// /**
    ///    * Use this attribute to dispatch runnables to the thread. Eventually, the
    ///    * eventTarget attribute will be the only way to dispatch events to a
    ///    * thread--nsIThread will no longer inherit from nsIEventTarget.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIEventTarget eventTarget;`
    #[inline]
    pub unsafe fn GetEventTarget(&self, aEventTarget: *mut *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetEventTarget)(self, aEventTarget)
    }


    /// ```text
    /// /**
    ///    * A fast C++ getter for the eventTarget.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] nsIEventTargetPtr EventTarget ();`
    const _EventTarget: () = ();

    /// ```text
    /// /**
    ///    * A fast C++ getter for the eventTarget. It asserts that the thread's event
    ///    * target is an nsISerialEventTarget and then returns it.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] nsISerialEventTargetPtr SerialEventTarget ();`
    const _SerialEventTarget: () = ();

    /// ```text
    /// /**
    ///    * This is set to the end of the last 50+ms event that was executed on
    ///    * this thread (for MainThread only).  Otherwise returns a null TimeStamp.
    ///    */
    /// ```
    ///

    /// `[noscript] readonly attribute TimeStamp lastLongTaskEnd;`
    const _GetLastLongTaskEnd: () = ();


    /// `[noscript] readonly attribute TimeStamp lastLongNonIdleTaskEnd;`
    const _GetLastLongNonIdleTaskEnd: () = ();

    /// ```text
    /// /**
    ///    * Get information on the timing of the currently-running event.
    ///    *
    ///    * @param delay
    ///    *   The amount of time the current running event in the specified queue waited
    ///    *   to run. Will return TimeDuration() if the queue is empty or has not run any
    ///    *   new events since event delay monitoring started.  NOTE: delay will be
    ///    *   TimeDuration() if this thread uses a PrioritizedEventQueue (i.e. MainThread)
    ///    *   and the event priority is below Input.
    ///    * @param start
    ///    *   The time the currently running event began to run, or TimeStamp() if no
    ///    *   event is running.
    ///    */
    /// ```
    ///

    /// `[noscript] void getRunningEventDelay (out TimeDuration delay, out TimeStamp start);`
    const _GetRunningEventDelay: () = ();

    /// ```text
    /// /**
    ///    * Set information on the timing of the currently-running event.
    ///    * Overrides the values returned by getRunningEventDelay
    ///    *
    ///    * @param delay
    ///    *   Delay the running event spent in queues, or TimeDuration() if
    ///    *   there's no running event.
    ///    * @param start
    ///    *   The time the currently running event began to run, or TimeStamp() if no
    ///    *   event is running.
    ///    */
    /// ```
    ///

    /// `[noscript] void setRunningEventDelay (in TimeDuration delay, in TimeStamp start);`
    const _SetRunningEventDelay: () = ();


    /// `[noscript] void setNameForWakeupTelemetry (in ACString name);`
    #[inline]
    pub unsafe fn SetNameForWakeupTelemetry(&self, name: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetNameForWakeupTelemetry)(self, name)
    }


}


