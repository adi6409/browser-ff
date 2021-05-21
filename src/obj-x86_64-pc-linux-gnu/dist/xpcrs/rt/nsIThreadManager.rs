//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIThreadManager.idl
//


/// `interface nsINestedEventLoopCondition : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINestedEventLoopCondition {
    vtable: *const nsINestedEventLoopConditionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINestedEventLoopCondition.
unsafe impl XpCom for nsINestedEventLoopCondition {
    const IID: nsIID = nsID(0x039a227d, 0x0cb7, 0x44a5,
        [0xa8, 0xf9, 0xdb, 0xb7, 0x07, 0x19, 0x79, 0xf2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINestedEventLoopCondition {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINestedEventLoopCondition.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINestedEventLoopConditionCoerce {
    /// Cheaply cast a value of this type from a `nsINestedEventLoopCondition`.
    fn coerce_from(v: &nsINestedEventLoopCondition) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINestedEventLoopConditionCoerce for nsINestedEventLoopCondition {
    #[inline]
    fn coerce_from(v: &nsINestedEventLoopCondition) -> &Self {
        v
    }
}

impl nsINestedEventLoopCondition {
    /// Cast this `nsINestedEventLoopCondition` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINestedEventLoopConditionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINestedEventLoopCondition {
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
impl<T: nsISupportsCoerce> nsINestedEventLoopConditionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINestedEventLoopCondition) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINestedEventLoopCondition
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINestedEventLoopConditionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool isDone (); */
    pub IsDone: unsafe extern "system" fn (this: *const nsINestedEventLoopCondition, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINestedEventLoopCondition {

    /// ```text
    /// /**
    ///    * Returns true if the current nested event loop should stop spinning.
    ///    */
    /// ```
    ///

    /// `bool isDone ();`
    #[inline]
    pub unsafe fn IsDone(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDone)(self, _retval)
    }


}


/// `interface nsIThreadManager : nsISupports`
///

/// ```text
/// /**
///  * An interface for creating and locating nsIThread instances.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIThreadManager {
    vtable: *const nsIThreadManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIThreadManager.
unsafe impl XpCom for nsIThreadManager {
    const IID: nsIID = nsID(0x1be89eca, 0xe2f7, 0x453b,
        [0x8d, 0x38, 0xc1, 0x1b, 0xa2, 0x47, 0xf6, 0xf3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIThreadManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIThreadManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIThreadManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIThreadManager`.
    fn coerce_from(v: &nsIThreadManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIThreadManagerCoerce for nsIThreadManager {
    #[inline]
    fn coerce_from(v: &nsIThreadManager) -> &Self {
        v
    }
}

impl nsIThreadManager {
    /// Cast this `nsIThreadManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIThreadManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIThreadManager {
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
impl<T: nsISupportsCoerce> nsIThreadManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIThreadManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIThreadManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIThread newThread (in unsigned long creationFlags, [optional] in unsigned long stackSize); */
    pub NewThread: unsafe extern "system" fn (this: *const nsIThreadManager, creationFlags: u32, stackSize: u32, _retval: *mut*const nsIThread) -> ::nserror::nsresult,

    /* [noscript] nsIThread newNamedThread (in ACString name, [optional] in unsigned long stackSize); */
    pub NewNamedThread: unsafe extern "system" fn (this: *const nsIThreadManager, name: *const ::nsstring::nsACString, stackSize: u32, _retval: *mut*const nsIThread) -> ::nserror::nsresult,

    /* readonly attribute nsIThread mainThread; */
    pub GetMainThread: unsafe extern "system" fn (this: *const nsIThreadManager, aMainThread: *mut*const nsIThread) -> ::nserror::nsresult,

    /* readonly attribute nsIThread currentThread; */
    pub GetCurrentThread: unsafe extern "system" fn (this: *const nsIThreadManager, aCurrentThread: *mut*const nsIThread) -> ::nserror::nsresult,

    /* [optional_argc] void dispatchToMainThread (in nsIRunnable event, [optional] in uint32_t priority); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub DispatchToMainThread: *const ::libc::c_void,

    /* void idleDispatchToMainThread (in nsIRunnable event, [optional] in uint32_t timeout); */
    pub IdleDispatchToMainThread: unsafe extern "system" fn (this: *const nsIThreadManager, event: *const nsIRunnable, timeout: uint32_t) -> ::nserror::nsresult,

    /* void spinEventLoopUntil (in nsINestedEventLoopCondition condition); */
    pub SpinEventLoopUntil: unsafe extern "system" fn (this: *const nsIThreadManager, condition: *const nsINestedEventLoopCondition) -> ::nserror::nsresult,

    /* void spinEventLoopUntilOrShutdown (in nsINestedEventLoopCondition condition); */
    pub SpinEventLoopUntilOrShutdown: unsafe extern "system" fn (this: *const nsIThreadManager, condition: *const nsINestedEventLoopCondition) -> ::nserror::nsresult,

    /* void spinEventLoopUntilEmpty (); */
    pub SpinEventLoopUntilEmpty: unsafe extern "system" fn (this: *const nsIThreadManager) -> ::nserror::nsresult,

    /* readonly attribute nsIEventTarget mainThreadEventTarget; */
    pub GetMainThreadEventTarget: unsafe extern "system" fn (this: *const nsIThreadManager, aMainThreadEventTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIThreadManager {

    /// ```text
    /// /**
    ///    * Default number of bytes reserved for a thread's stack, if no stack size
    ///    * is specified in newThread().
    ///    *
    ///    * Defaults can be a little overzealous for many platforms.
    ///    *
    ///    * On Linux and OS X, for instance, the default thread stack size is whatever
    ///    * getrlimit(RLIMIT_STACK) returns, which is often set at 8MB. Or, on Linux,
    ///    * if the stack size is unlimited, we fall back to 2MB. This causes particular
    ///    * problems on Linux, which allocates 2MB huge VM pages, and will often
    ///    * immediately allocate them for any stacks which are 2MB or larger.
    ///    *
    ///    * The default on Windows is 1MB, which is a little more reasonable. But the
    ///    * vast majority of our threads don't need anywhere near that much space.
    ///    *
    ///    * ASan, TSan and non-opt builds, however, often need a bit more, so give
    ///    * them the platform default.
    ///    */
    /// /**
    ///    * Create a new thread (a global, user PRThread).
    ///    *
    ///    * @param creationFlags
    ///    *   Reserved for future use.  Pass 0.
    ///    * @param stackSize
    ///    *   Number of bytes to reserve for the thread's stack. 0 means use platform
    ///    *   default.
    ///    *
    ///    * @returns
    ///    *   The newly created nsIThread object.
    ///    */
    /// ```
    ///

    /// `nsIThread newThread (in unsigned long creationFlags, [optional] in unsigned long stackSize);`
    #[inline]
    pub unsafe fn NewThread(&self, creationFlags: u32, stackSize: u32, _retval: *mut*const nsIThread) -> ::nserror::nsresult {
        ((*self.vtable).NewThread)(self, creationFlags, stackSize, _retval)
    }


    /// ```text
    /// /**
    ///    * Create a new thread (a global, user PRThread) with the specified name.
    ///    *
    ///    * @param name
    ///    *   The name of the thread. Passing an empty name is equivalent to
    ///    *   calling newThread(0, stackSize), i.e. the thread will not be named.
    ///    * @param stackSize
    ///    *   Number of bytes to reserve for the thread's stack. 0 means use platform
    ///    *   default.
    ///    *
    ///    * @returns
    ///    *   The newly created nsIThread object.
    ///    */
    /// ```
    ///

    /// `[noscript] nsIThread newNamedThread (in ACString name, [optional] in unsigned long stackSize);`
    #[inline]
    pub unsafe fn NewNamedThread(&self, name: *const ::nsstring::nsACString, stackSize: u32, _retval: *mut*const nsIThread) -> ::nserror::nsresult {
        ((*self.vtable).NewNamedThread)(self, name, stackSize, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the main thread.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIThread mainThread;`
    #[inline]
    pub unsafe fn GetMainThread(&self, aMainThread: *mut*const nsIThread) -> ::nserror::nsresult {
        ((*self.vtable).GetMainThread)(self, aMainThread)
    }


    /// ```text
    /// /**
    ///    * Get the current thread.  If the calling thread does not already have a
    ///    * nsIThread associated with it, then a new nsIThread will be created and
    ///    * associated with the current PRThread.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIThread currentThread;`
    #[inline]
    pub unsafe fn GetCurrentThread(&self, aCurrentThread: *mut*const nsIThread) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentThread)(self, aCurrentThread)
    }


    /// ```text
    /// /**
    ///    * This queues a runnable to the main thread. It's a shortcut for JS callers
    ///    * to be used instead of
    ///    *   .mainThread.dispatch(runnable, Ci.nsIEventTarget.DISPATCH_NORMAL);
    ///    * or
    ///    *   .currentThread.dispatch(runnable, Ci.nsIEventTarget.DISPATCH_NORMAL);
    ///    * C++ callers should instead use NS_DispatchToMainThread.
    ///    */
    /// ```
    ///

    /// `[optional_argc] void dispatchToMainThread (in nsIRunnable event, [optional] in uint32_t priority);`
    const _DispatchToMainThread: () = ();

    /// ```text
    /// /**
    ///    * This queues a runnable to the main thread's idle queue.
    ///    *
    ///    * @param event
    ///    *   The event to dispatch.
    ///    * @param timeout
    ///    *   The time in milliseconds until this event should be moved from the idle
    ///    *   queue to the regular queue if it hasn't been executed by then.  If not
    ///    *   passed or a zero value is specified, the event will never be moved to
    ///    *   the regular queue.
    ///    */
    /// ```
    ///

    /// `void idleDispatchToMainThread (in nsIRunnable event, [optional] in uint32_t timeout);`
    #[inline]
    pub unsafe fn IdleDispatchToMainThread(&self, event: *const nsIRunnable, timeout: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).IdleDispatchToMainThread)(self, event, timeout)
    }


    /// ```text
    /// /**
    ///    * Enter a nested event loop on the current thread, waiting on, and
    ///    * processing events until condition.isDone() returns true.
    ///    *
    ///    * If condition.isDone() throws, this function will throw as well.
    ///    *
    ///    * C++ code should not use this function, instead preferring
    ///    * mozilla::SpinEventLoopUntil.
    ///    */
    /// ```
    ///

    /// `void spinEventLoopUntil (in nsINestedEventLoopCondition condition);`
    #[inline]
    pub unsafe fn SpinEventLoopUntil(&self, condition: *const nsINestedEventLoopCondition) -> ::nserror::nsresult {
        ((*self.vtable).SpinEventLoopUntil)(self, condition)
    }


    /// ```text
    /// /**
    ///    * Similar to the previous method, but the spinning of the event loop
    ///    * terminates when the shutting down starts.
    ///    *
    ///    * C++ code should not use this function, instead preferring
    ///    * mozilla::SpinEventLoopUntil.
    ///    */
    /// ```
    ///

    /// `void spinEventLoopUntilOrShutdown (in nsINestedEventLoopCondition condition);`
    #[inline]
    pub unsafe fn SpinEventLoopUntilOrShutdown(&self, condition: *const nsINestedEventLoopCondition) -> ::nserror::nsresult {
        ((*self.vtable).SpinEventLoopUntilOrShutdown)(self, condition)
    }


    /// ```text
    /// /**
    ///    * Spin the current thread's event loop until there are no more pending
    ///    * events.  This could be done with spinEventLoopUntil, but that would
    ///    * require access to the current thread from JavaScript, which we are
    ///    * moving away from.
    ///    */
    /// ```
    ///

    /// `void spinEventLoopUntilEmpty ();`
    #[inline]
    pub unsafe fn SpinEventLoopUntilEmpty(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SpinEventLoopUntilEmpty)(self, )
    }


    /// ```text
    /// /**
    ///    * Return the EventTarget for the main thread.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIEventTarget mainThreadEventTarget;`
    #[inline]
    pub unsafe fn GetMainThreadEventTarget(&self, aMainThreadEventTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetMainThreadEventTarget)(self, aMainThreadEventTarget)
    }


}


