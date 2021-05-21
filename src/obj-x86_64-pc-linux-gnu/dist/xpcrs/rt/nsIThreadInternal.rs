//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThreadInternal.idl
//


/// `interface nsIThreadInternal : nsIThread`
///

/// ```text
/// /**
///  * The XPCOM thread object implements this interface, which allows a consumer
///  * to observe dispatch activity on the thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThreadInternal {
    vtable: *const nsIThreadInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThreadInternal.
unsafe impl XpCom for nsIThreadInternal {
    const IID: nsIID = nsID(0xa3a72e5f, 0x71d9, 0x4add,
        [0x8f, 0x30, 0x59, 0xa7, 0x8f, 0xb6, 0xd5, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThreadInternal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThreadInternal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadInternalCoerce {
    /// Cheaply cast a value of this type from a `nsIThreadInternal`.
    fn coerce_from(v: &nsIThreadInternal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadInternalCoerce for nsIThreadInternal {
    #[inline]
    fn coerce_from(v: &nsIThreadInternal) -> &Self {
        v
    }
}

impl nsIThreadInternal {
    /// Cast this `nsIThreadInternal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThreadInternal {
    type Target = nsIThread;
    #[inline]
    fn deref(&self) -> &nsIThread {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIThreadCoerce> nsIThreadInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadInternal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThreadInternal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadInternalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIThreadVTable,

    /* attribute nsIThreadObserver observer; */
    pub GetObserver: unsafe extern "system" fn (this: *const nsIThreadInternal, aObserver: *mut*const nsIThreadObserver) -> ::nserror::nsresult,

    /* attribute nsIThreadObserver observer; */
    pub SetObserver: unsafe extern "system" fn (this: *const nsIThreadInternal, aObserver: *const nsIThreadObserver) -> ::nserror::nsresult,

    /* void addObserver (in nsIThreadObserver observer); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsIThreadInternal, observer: *const nsIThreadObserver) -> ::nserror::nsresult,

    /* void removeObserver (in nsIThreadObserver observer); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsIThreadInternal, observer: *const nsIThreadObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThreadInternal {

    /// ```text
    /// /**
    ///    * Get/set the current thread observer (may be null).  This attribute may be
    ///    * read from any thread, but must only be set on the thread corresponding to
    ///    * this thread object.  The observer will be released on the thread
    ///    * corresponding to this thread object after all other events have been
    ///    * processed during a call to Shutdown.
    ///    */
    /// ```
    ///

    /// `attribute nsIThreadObserver observer;`
    #[inline]
    pub unsafe fn GetObserver(&self, aObserver: *mut*const nsIThreadObserver) -> ::nserror::nsresult {
        ((*self.vtable).GetObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Get/set the current thread observer (may be null).  This attribute may be
    ///    * read from any thread, but must only be set on the thread corresponding to
    ///    * this thread object.  The observer will be released on the thread
    ///    * corresponding to this thread object after all other events have been
    ///    * processed during a call to Shutdown.
    ///    */
    /// ```
    ///

    /// `attribute nsIThreadObserver observer;`
    #[inline]
    pub unsafe fn SetObserver(&self, aObserver: *const nsIThreadObserver) -> ::nserror::nsresult {
        ((*self.vtable).SetObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Add an observer that will *only* receive onProcessNextEvent,
    ///    * beforeProcessNextEvent. and afterProcessNextEvent callbacks. Always called
    ///    * on the target thread, and the implementation does not have to be
    ///    * threadsafe. Order of callbacks is not guaranteed (i.e.
        ///    * afterProcessNextEvent may be called first depending on whether or not the
        ///    * observer is added in a nested loop). Holds a strong ref.
    ///    */
    /// ```
    ///

    /// `void addObserver (in nsIThreadObserver observer);`
    #[inline]
    pub unsafe fn AddObserver(&self, observer: *const nsIThreadObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, observer)
    }


    /// ```text
    /// /**
    ///    * Remove an observer added via the addObserver call. Once removed the
    ///    * observer will never be called again by the thread.
    ///    */
    /// ```
    ///

    /// `void removeObserver (in nsIThreadObserver observer);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, observer: *const nsIThreadObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, observer)
    }


}


/// `interface nsIThreadObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThreadObserver {
    vtable: *const nsIThreadObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThreadObserver.
unsafe impl XpCom for nsIThreadObserver {
    const IID: nsIID = nsID(0xcc8da053, 0x1776, 0x44c2,
        [0x91, 0x99, 0xb5, 0xa6, 0x29, 0xd0, 0xa1, 0x9d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThreadObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThreadObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIThreadObserver`.
    fn coerce_from(v: &nsIThreadObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadObserverCoerce for nsIThreadObserver {
    #[inline]
    fn coerce_from(v: &nsIThreadObserver) -> &Self {
        v
    }
}

impl nsIThreadObserver {
    /// Cast this `nsIThreadObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThreadObserver {
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
impl<T: nsISupportsCoerce> nsIThreadObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThreadObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDispatchedEvent (); */
    pub OnDispatchedEvent: unsafe extern "system" fn (this: *const nsIThreadObserver) -> ::nserror::nsresult,

    /* void onProcessNextEvent (in nsIThreadInternal thread, in boolean mayWait); */
    pub OnProcessNextEvent: unsafe extern "system" fn (this: *const nsIThreadObserver, thread: *const nsIThreadInternal, mayWait: bool) -> ::nserror::nsresult,

    /* void afterProcessNextEvent (in nsIThreadInternal thread, in bool eventWasProcessed); */
    pub AfterProcessNextEvent: unsafe extern "system" fn (this: *const nsIThreadObserver, thread: *const nsIThreadInternal, eventWasProcessed: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThreadObserver {

    /// ```text
    /// /**
    ///  * This interface provides the observer with hooks to implement a layered
    ///  * event queue.  For example, it is possible to overlay processing events
    ///  * for a GUI toolkit on top of the events for a thread:
    ///  *
    ///  *   var NativeQueue;
    ///  *   Observer = {
        ///  *     onDispatchedEvent() {
            ///  *       NativeQueue.signal();
            ///  *     }
        ///  *     onProcessNextEvent(thread, mayWait) {
            ///  *       if (NativeQueue.hasNextEvent())
            ///  *         NativeQueue.processNextEvent();
            ///  *       while (mayWait && !thread.hasPendingEvent()) {
                ///  *         NativeQueue.wait();
                ///  *         NativeQueue.processNextEvent();
                ///  *       }
            ///  *     }
        ///  *   };
    ///  *
    ///  * NOTE: The implementation of this interface must be threadsafe.
    ///  *
    ///  * NOTE: It is valid to change the thread's observer during a call to an
    ///  *       observer method.
    ///  *
    ///  * NOTE: Will be split into two interfaces soon: one for onProcessNextEvent and
    ///  *       afterProcessNextEvent, then another that inherits the first and adds
    ///  *       onDispatchedEvent.
    ///  */
    /// /**
    ///    * This method is called after an event has been dispatched to the thread.
    ///    * This method may be called from any thread.
    ///    */
    /// ```
    ///

    /// `void onDispatchedEvent ();`
    #[inline]
    pub unsafe fn OnDispatchedEvent(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnDispatchedEvent)(self, )
    }


    /// ```text
    /// /**
    ///    * This method is called when nsIThread::ProcessNextEvent is called.  It does
    ///    * not guarantee that an event is actually going to be processed.  This method
    ///    * is only called on the target thread.
    ///    *
    ///    * @param thread
    ///    *   The thread being asked to process another event.
    ///    * @param mayWait
    ///    *   Indicates whether or not the method is allowed to block the calling
    ///    *   thread.  For example, this parameter is false during thread shutdown.
    ///    */
    /// ```
    ///

    /// `void onProcessNextEvent (in nsIThreadInternal thread, in boolean mayWait);`
    #[inline]
    pub unsafe fn OnProcessNextEvent(&self, thread: *const nsIThreadInternal, mayWait: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnProcessNextEvent)(self, thread, mayWait)
    }


    /// ```text
    /// /**
    ///    * This method is called (from nsIThread::ProcessNextEvent) after an event
    ///    * is processed.  It does not guarantee that an event was actually processed
    ///    * (depends on the value of |eventWasProcessed|.  This method is only called
        ///    * on the target thread.  DO NOT EVER RUN SCRIPT FROM THIS CALLBACK!!!
        ///    *
        ///    * @param thread
        ///    *   The thread that processed another event.
        ///    * @param eventWasProcessed
        ///    *   Indicates whether an event was actually processed. May be false if the
        ///    *   |mayWait| flag was false when calling nsIThread::ProcessNextEvent().
        ///    */
        /// ```
        ///

        /// `void afterProcessNextEvent (in nsIThreadInternal thread, in bool eventWasProcessed);`
        #[inline]
        pub unsafe fn AfterProcessNextEvent(&self, thread: *const nsIThreadInternal, eventWasProcessed: bool) -> ::nserror::nsresult {
            ((*self.vtable).AfterProcessNextEvent)(self, thread, eventWasProcessed)
        }


    }


