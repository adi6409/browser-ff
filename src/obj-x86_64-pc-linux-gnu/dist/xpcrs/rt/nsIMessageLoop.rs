//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMessageLoop.idl
//


/// `interface nsIMessageLoop : nsISupports`
///

/// ```text
/// /**
///  * This service allows access to the current thread's Chromium MessageLoop
///  * instance, with some extra sugar added.  If you're calling from C++, it may
///  * or may not make sense for you to use this interface.  If you're calling from
///  * JS, you don't have a choice!
///  *
///  * Right now, you can only call PostIdleTask(), and the wrath of khuey is
///  * stopping you from adding other methods.
///  *
///  * nsIMessageLoop's contractid is "@mozilla.org/message-loop;1".
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMessageLoop {
    vtable: *const nsIMessageLoopVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMessageLoop.
unsafe impl XpCom for nsIMessageLoop {
    const IID: nsIID = nsID(0x3e8c58e8, 0xe52c, 0x43e0,
        [0x8e, 0x66, 0x66, 0x9c, 0xa7, 0x88, 0xff, 0x5f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMessageLoop {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMessageLoop.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMessageLoopCoerce {
    /// Cheaply cast a value of this type from a `nsIMessageLoop`.
    fn coerce_from(v: &nsIMessageLoop) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMessageLoopCoerce for nsIMessageLoop {
    #[inline]
    fn coerce_from(v: &nsIMessageLoop) -> &Self {
        v
    }
}

impl nsIMessageLoop {
    /// Cast this `nsIMessageLoop` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMessageLoopCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMessageLoop {
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
impl<T: nsISupportsCoerce> nsIMessageLoopCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageLoop) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMessageLoop
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMessageLoopVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS); */
    pub PostIdleTask: unsafe extern "system" fn (this: *const nsIMessageLoop, task: *const nsIRunnable, ensureRunsAfterMS: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMessageLoop {

    /// ```text
    /// /**
    ///    * Posts a task to be run when this thread's message loop is idle, or after
    ///    * ensureRunsAfterMS milliseconds have elapsed.  (That is, the task is
        ///    * guaranteed to run /eventually/.)
    ///    *
    ///    * Note that if the event loop is busy, we will hold a reference to the task
    ///    * until ensureRunsAfterMS milliseconds have elapsed.  Be careful when
    ///    * specifying long timeouts and tasks which hold references to windows or
    ///    * other large objects, because you can leak memory in a difficult-to-detect
    ///    * way!
    ///    */
    /// ```
    ///

    /// `void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS);`
    #[inline]
    pub unsafe fn PostIdleTask(&self, task: *const nsIRunnable, ensureRunsAfterMS: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).PostIdleTask)(self, task, ensureRunsAfterMS)
    }


}


