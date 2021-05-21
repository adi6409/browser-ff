//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIDirectTaskDispatcher.idl
//


/// `interface nsIDirectTaskDispatcher : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirectTaskDispatcher {
    vtable: *const nsIDirectTaskDispatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirectTaskDispatcher.
unsafe impl XpCom for nsIDirectTaskDispatcher {
    const IID: nsIID = nsID(0xe05bf0fe, 0x94b7, 0x4e28,
        [0x84, 0x62, 0xa8, 0x36, 0x8d, 0xa9, 0xc1, 0x36]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirectTaskDispatcher {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirectTaskDispatcher.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirectTaskDispatcherCoerce {
    /// Cheaply cast a value of this type from a `nsIDirectTaskDispatcher`.
    fn coerce_from(v: &nsIDirectTaskDispatcher) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirectTaskDispatcherCoerce for nsIDirectTaskDispatcher {
    #[inline]
    fn coerce_from(v: &nsIDirectTaskDispatcher) -> &Self {
        v
    }
}

impl nsIDirectTaskDispatcher {
    /// Cast this `nsIDirectTaskDispatcher` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirectTaskDispatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirectTaskDispatcher {
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
impl<T: nsISupportsCoerce> nsIDirectTaskDispatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectTaskDispatcher) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirectTaskDispatcher
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirectTaskDispatcherVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void dispatchDirectTask (in alreadyAddRefed_nsIRunnable event); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub DispatchDirectTask: *const ::libc::c_void,

    /* [noscript] void drainDirectTasks (); */
    pub DrainDirectTasks: unsafe extern "system" fn (this: *const nsIDirectTaskDispatcher) -> ::nserror::nsresult,

    /* [noscript] bool haveDirectTasks (); */
    pub HaveDirectTasks: unsafe extern "system" fn (this: *const nsIDirectTaskDispatcher, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirectTaskDispatcher {

    /// ```text
    /// /**
    ///    * Dispatch an event for the nsISerialEventTarget, using the direct task
    ///    * queue.
    ///    *
    ///    * This function must be called from the same nsISerialEventTarget
    ///    * implementing direct task dispatching.
    ///    *
    ///    * @param event
    ///    *   The alreadyAddRefed<> event to dispatch.
    ///    *
    ///    */
    /// ```
    ///

    /// `[noscript] void dispatchDirectTask (in alreadyAddRefed_nsIRunnable event);`
    const _DispatchDirectTask: () = ();

    /// ```text
    /// /**
    ///    * Synchronously run any pending direct tasks queued.
    ///    */
    /// ```
    ///

    /// `[noscript] void drainDirectTasks ();`
    #[inline]
    pub unsafe fn DrainDirectTasks(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DrainDirectTasks)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns true if any direct tasks are pending.
    ///    */
    /// ```
    ///

    /// `[noscript] bool haveDirectTasks ();`
    #[inline]
    pub unsafe fn HaveDirectTasks(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HaveDirectTasks)(self, _retval)
    }


}


