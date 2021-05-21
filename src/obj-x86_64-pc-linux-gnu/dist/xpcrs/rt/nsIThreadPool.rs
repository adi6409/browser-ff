//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThreadPool.idl
//


/// `interface nsIThreadPoolListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThreadPoolListener {
    vtable: *const nsIThreadPoolListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThreadPoolListener.
unsafe impl XpCom for nsIThreadPoolListener {
    const IID: nsIID = nsID(0xef194cab, 0x3f86, 0x4b61,
        [0xb1, 0x32, 0xe5, 0xe9, 0x6a, 0x79, 0xe5, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThreadPoolListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThreadPoolListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadPoolListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIThreadPoolListener`.
    fn coerce_from(v: &nsIThreadPoolListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadPoolListenerCoerce for nsIThreadPoolListener {
    #[inline]
    fn coerce_from(v: &nsIThreadPoolListener) -> &Self {
        v
    }
}

impl nsIThreadPoolListener {
    /// Cast this `nsIThreadPoolListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadPoolListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThreadPoolListener {
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
impl<T: nsISupportsCoerce> nsIThreadPoolListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadPoolListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThreadPoolListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadPoolListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onThreadCreated (); */
    pub OnThreadCreated: unsafe extern "system" fn (this: *const nsIThreadPoolListener) -> ::nserror::nsresult,

    /* void onThreadShuttingDown (); */
    pub OnThreadShuttingDown: unsafe extern "system" fn (this: *const nsIThreadPoolListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThreadPoolListener {

    /// ```text
    /// /**
    ///    * Called when a new thread is created by the thread pool. The notification
    ///    * happens on the newly-created thread.
    ///    */
    /// ```
    ///

    /// `void onThreadCreated ();`
    #[inline]
    pub unsafe fn OnThreadCreated(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnThreadCreated)(self, )
    }


    /// ```text
    /// /**
    ///    * Called when a thread is about to be destroyed by the thread pool. The
    ///    * notification happens on the thread that is about to be destroyed.
    ///    */
    /// ```
    ///

    /// `void onThreadShuttingDown ();`
    #[inline]
    pub unsafe fn OnThreadShuttingDown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnThreadShuttingDown)(self, )
    }


}


/// `interface nsIThreadPool : nsIEventTarget`
///

/// ```text
/// /**
///  * An interface to a thread pool.  A thread pool creates a limited number of
///  * anonymous (unnamed) worker threads.  An event dispatched to the thread pool
///  * will be run on the next available worker thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThreadPool {
    vtable: *const nsIThreadPoolVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThreadPool.
unsafe impl XpCom for nsIThreadPool {
    const IID: nsIID = nsID(0x76ce99c9, 0x8e43, 0x489a,
        [0x97, 0x89, 0xf2, 0x7c, 0xc4, 0x42, 0x49, 0x65]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThreadPool {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThreadPool.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadPoolCoerce {
    /// Cheaply cast a value of this type from a `nsIThreadPool`.
    fn coerce_from(v: &nsIThreadPool) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadPoolCoerce for nsIThreadPool {
    #[inline]
    fn coerce_from(v: &nsIThreadPool) -> &Self {
        v
    }
}

impl nsIThreadPool {
    /// Cast this `nsIThreadPool` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadPoolCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThreadPool {
    type Target = nsIEventTarget;
    #[inline]
    fn deref(&self) -> &nsIEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIEventTargetCoerce> nsIThreadPoolCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadPool) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThreadPool
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadPoolVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIEventTargetVTable,

    /* void shutdown (); */
    pub Shutdown: unsafe extern "system" fn (this: *const nsIThreadPool) -> ::nserror::nsresult,

    /* [noscript] void shutdownWithTimeout (in long aTimeoutMs); */
    pub ShutdownWithTimeout: unsafe extern "system" fn (this: *const nsIThreadPool, aTimeoutMs: i32) -> ::nserror::nsresult,

    /* attribute unsigned long threadLimit; */
    pub GetThreadLimit: unsafe extern "system" fn (this: *const nsIThreadPool, aThreadLimit: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long threadLimit; */
    pub SetThreadLimit: unsafe extern "system" fn (this: *const nsIThreadPool, aThreadLimit: u32) -> ::nserror::nsresult,

    /* attribute unsigned long idleThreadLimit; */
    pub GetIdleThreadLimit: unsafe extern "system" fn (this: *const nsIThreadPool, aIdleThreadLimit: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long idleThreadLimit; */
    pub SetIdleThreadLimit: unsafe extern "system" fn (this: *const nsIThreadPool, aIdleThreadLimit: u32) -> ::nserror::nsresult,

    /* attribute unsigned long idleThreadTimeout; */
    pub GetIdleThreadTimeout: unsafe extern "system" fn (this: *const nsIThreadPool, aIdleThreadTimeout: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long idleThreadTimeout; */
    pub SetIdleThreadTimeout: unsafe extern "system" fn (this: *const nsIThreadPool, aIdleThreadTimeout: u32) -> ::nserror::nsresult,

    /* attribute boolean idleThreadTimeoutRegressive; */
    pub GetIdleThreadTimeoutRegressive: unsafe extern "system" fn (this: *const nsIThreadPool, aIdleThreadTimeoutRegressive: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean idleThreadTimeoutRegressive; */
    pub SetIdleThreadTimeoutRegressive: unsafe extern "system" fn (this: *const nsIThreadPool, aIdleThreadTimeoutRegressive: bool) -> ::nserror::nsresult,

    /* attribute unsigned long threadStackSize; */
    pub GetThreadStackSize: unsafe extern "system" fn (this: *const nsIThreadPool, aThreadStackSize: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long threadStackSize; */
    pub SetThreadStackSize: unsafe extern "system" fn (this: *const nsIThreadPool, aThreadStackSize: u32) -> ::nserror::nsresult,

    /* attribute nsIThreadPoolListener listener; */
    pub GetListener: unsafe extern "system" fn (this: *const nsIThreadPool, aListener: *mut *const nsIThreadPoolListener) -> ::nserror::nsresult,

    /* attribute nsIThreadPoolListener listener; */
    pub SetListener: unsafe extern "system" fn (this: *const nsIThreadPool, aListener: *const nsIThreadPoolListener) -> ::nserror::nsresult,

    /* void setName (in ACString aName); */
    pub SetName: unsafe extern "system" fn (this: *const nsIThreadPool, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThreadPool {

    /// ```text
    /// /**
    ///    * Shutdown the thread pool.  This method may not be executed from any thread
    ///    * in the thread pool.  Instead, it is meant to be executed from another
    ///    * thread (usually the thread that created this thread pool).  When this
    ///    * function returns, the thread pool and all of its threads will be shutdown,
    ///    * and it will no longer be possible to dispatch tasks to the thread pool.
    ///    *
    ///    * As a side effect, events on the current thread will be processed.
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
    ///    * Shutdown the thread pool, but only wait for aTimeoutMs. After the timeout
    ///    * expires, any threads that have not shutdown yet are leaked and will not
    ///    * block shutdown.
    ///    *
    ///    * This method should only be used at during shutdown to cleanup threads that
    ///    * made blocking calls to code outside our control, and can't be safely
    ///    * terminated. We choose to leak them intentionally to avoid a shutdown hang.
    ///    */
    /// ```
    ///

    /// `[noscript] void shutdownWithTimeout (in long aTimeoutMs);`
    #[inline]
    pub unsafe fn ShutdownWithTimeout(&self, aTimeoutMs: i32) -> ::nserror::nsresult {
        ((*self.vtable).ShutdownWithTimeout)(self, aTimeoutMs)
    }


    /// ```text
    /// /**
    ///    * Get/set the maximum number of threads allowed at one time in this pool.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long threadLimit;`
    #[inline]
    pub unsafe fn GetThreadLimit(&self, aThreadLimit: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetThreadLimit)(self, aThreadLimit)
    }


    /// ```text
    /// /**
    ///    * Get/set the maximum number of threads allowed at one time in this pool.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long threadLimit;`
    #[inline]
    pub unsafe fn SetThreadLimit(&self, aThreadLimit: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetThreadLimit)(self, aThreadLimit)
    }


    /// ```text
    /// /**
    ///    * Get/set the maximum number of idle threads kept alive.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long idleThreadLimit;`
    #[inline]
    pub unsafe fn GetIdleThreadLimit(&self, aIdleThreadLimit: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetIdleThreadLimit)(self, aIdleThreadLimit)
    }


    /// ```text
    /// /**
    ///    * Get/set the maximum number of idle threads kept alive.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long idleThreadLimit;`
    #[inline]
    pub unsafe fn SetIdleThreadLimit(&self, aIdleThreadLimit: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetIdleThreadLimit)(self, aIdleThreadLimit)
    }


    /// ```text
    /// /**
    ///    * Get/set the amount of time in milliseconds before an idle thread is
    ///    * destroyed.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long idleThreadTimeout;`
    #[inline]
    pub unsafe fn GetIdleThreadTimeout(&self, aIdleThreadTimeout: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetIdleThreadTimeout)(self, aIdleThreadTimeout)
    }


    /// ```text
    /// /**
    ///    * Get/set the amount of time in milliseconds before an idle thread is
    ///    * destroyed.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long idleThreadTimeout;`
    #[inline]
    pub unsafe fn SetIdleThreadTimeout(&self, aIdleThreadTimeout: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetIdleThreadTimeout)(self, aIdleThreadTimeout)
    }


    /// ```text
    /// /**
    ///    * If set to true the idle timeout will be calculated as idleThreadTimeout
    ///    * divideded by the number of idle threads at the moment.  This may help
    ///    * save memory allocations but still keep reasonable amount of idle threads.
    ///    * Default is false, use |idleThreadTimeout| for all threads.
    ///    */
    /// ```
    ///

    /// `attribute boolean idleThreadTimeoutRegressive;`
    #[inline]
    pub unsafe fn GetIdleThreadTimeoutRegressive(&self, aIdleThreadTimeoutRegressive: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIdleThreadTimeoutRegressive)(self, aIdleThreadTimeoutRegressive)
    }


    /// ```text
    /// /**
    ///    * If set to true the idle timeout will be calculated as idleThreadTimeout
    ///    * divideded by the number of idle threads at the moment.  This may help
    ///    * save memory allocations but still keep reasonable amount of idle threads.
    ///    * Default is false, use |idleThreadTimeout| for all threads.
    ///    */
    /// ```
    ///

    /// `attribute boolean idleThreadTimeoutRegressive;`
    #[inline]
    pub unsafe fn SetIdleThreadTimeoutRegressive(&self, aIdleThreadTimeoutRegressive: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIdleThreadTimeoutRegressive)(self, aIdleThreadTimeoutRegressive)
    }


    /// ```text
    /// /**
    ///    * Get/set the number of bytes reserved for the stack of all threads in
    ///    * the pool. By default this is nsIThreadManager::DEFAULT_STACK_SIZE.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long threadStackSize;`
    #[inline]
    pub unsafe fn GetThreadStackSize(&self, aThreadStackSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetThreadStackSize)(self, aThreadStackSize)
    }


    /// ```text
    /// /**
    ///    * Get/set the number of bytes reserved for the stack of all threads in
    ///    * the pool. By default this is nsIThreadManager::DEFAULT_STACK_SIZE.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long threadStackSize;`
    #[inline]
    pub unsafe fn SetThreadStackSize(&self, aThreadStackSize: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetThreadStackSize)(self, aThreadStackSize)
    }


    /// ```text
    /// /**
    ///    * An optional listener that will be notified when a thread is created or
    ///    * destroyed in the course of the thread pool's operation.
    ///    *
    ///    * A listener will only receive notifications about threads created after the
    ///    * listener is set so it is recommended that the consumer set the listener
    ///    * before dispatching the first event. A listener that receives an
    ///    * onThreadCreated() notification is guaranteed to always receive the
    ///    * corresponding onThreadShuttingDown() notification.
    ///    *
    ///    * The thread pool takes ownership of the listener and releases it when the
    ///    * shutdown() method is called. Threads created after the listener is set will
    ///    * also take ownership of the listener so that the listener will be kept alive
    ///    * long enough to receive the guaranteed onThreadShuttingDown() notification.
    ///    */
    /// ```
    ///

    /// `attribute nsIThreadPoolListener listener;`
    #[inline]
    pub unsafe fn GetListener(&self, aListener: *mut *const nsIThreadPoolListener) -> ::nserror::nsresult {
        ((*self.vtable).GetListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * An optional listener that will be notified when a thread is created or
    ///    * destroyed in the course of the thread pool's operation.
    ///    *
    ///    * A listener will only receive notifications about threads created after the
    ///    * listener is set so it is recommended that the consumer set the listener
    ///    * before dispatching the first event. A listener that receives an
    ///    * onThreadCreated() notification is guaranteed to always receive the
    ///    * corresponding onThreadShuttingDown() notification.
    ///    *
    ///    * The thread pool takes ownership of the listener and releases it when the
    ///    * shutdown() method is called. Threads created after the listener is set will
    ///    * also take ownership of the listener so that the listener will be kept alive
    ///    * long enough to receive the guaranteed onThreadShuttingDown() notification.
    ///    */
    /// ```
    ///

    /// `attribute nsIThreadPoolListener listener;`
    #[inline]
    pub unsafe fn SetListener(&self, aListener: *const nsIThreadPoolListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * Set the label for threads in the pool. All threads will be named
    ///    * "<aName> #<n>", where <n> is a serial number.
    ///    */
    /// ```
    ///

    /// `void setName (in ACString aName);`
    #[inline]
    pub unsafe fn SetName(&self, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetName)(self, aName)
    }


}


