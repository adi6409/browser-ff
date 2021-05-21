//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/devtools/platform/nsIJSInspector.idl
//


/// `interface nsIJSInspector : nsISupports`
///

/// ```text
/// /**
///  * Utilities for running nested event loops, asking them to return, and
///  * keeping track of which ones are still running.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJSInspector {
    vtable: *const nsIJSInspectorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJSInspector.
unsafe impl XpCom for nsIJSInspector {
    const IID: nsIID = nsID(0x6758d0d7, 0xe96a, 0x4c5c,
        [0xbc, 0xa8, 0x3b, 0xcb, 0xe5, 0xa1, 0x59, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJSInspector {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJSInspector.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJSInspectorCoerce {
    /// Cheaply cast a value of this type from a `nsIJSInspector`.
    fn coerce_from(v: &nsIJSInspector) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJSInspectorCoerce for nsIJSInspector {
    #[inline]
    fn coerce_from(v: &nsIJSInspector) -> &Self {
        v
    }
}

impl nsIJSInspector {
    /// Cast this `nsIJSInspector` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJSInspectorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJSInspector {
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
impl<T: nsISupportsCoerce> nsIJSInspectorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSInspector) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJSInspector
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJSInspectorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* unsigned long enterNestedEventLoop (in jsval requestor); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub EnterNestedEventLoop: *const ::libc::c_void,

    /* unsigned long exitNestedEventLoop (); */
    pub ExitNestedEventLoop: unsafe extern "system" fn (this: *const nsIJSInspector, _retval: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long eventLoopNestLevel; */
    pub GetEventLoopNestLevel: unsafe extern "system" fn (this: *const nsIJSInspector, aEventLoopNestLevel: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute jsval lastNestRequestor; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetLastNestRequestor: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJSInspector {

    /// ```text
    /// /**
    ///    * Process the current thread's event queue, calling event handlers until
    ///    * a call to exitNestedEventLoop, below, asks us to return.
    ///    *
    ///    * The name 'enterNestedEventLoop' may be misleading if read too literally.
    ///    * This method loops calling event handlers until one asks it to stop, and
    ///    * then returns. So by that point, the nested event loop has been not only
    ///    * entered, but also run and exited.
    ///    *
    ///    * When enterNestedEventLoop calls an event handler, that handler may itself
    ///    * call enterNestedEventLoop, and so on, so that there may be arbitrarily
    ///    * many such calls on the stack at the same time.
    ///    *
    ///    * We say an enterNestedEventLoop call is "running" if it has not yet been
    ///    * asked to return, or "stopped" if it has been asked to return once it has
    ///    * finished processing the current event.
    ///    *
    ///    * @param requestor   A token of the caller's choice to identify this event
    ///    *                    loop.
    ///    *
    ///    * @return depth      The number of running enterNestedEventLoop calls
    ///    *                    remaining, now that this one has returned.
    ///    *
    ///    *                    (Note that not all calls still on the stack are
        ///    *                    necessary running; exitNestedEventLoop can ask any
        ///    *                    number of enterNestedEventLoop calls to return.)
    ///    */
    /// ```
    ///

    /// `unsigned long enterNestedEventLoop (in jsval requestor);`
    const _EnterNestedEventLoop: () = ();

    /// ```text
    /// /**
    ///    * Stop the youngest running enterNestedEventLoop call, asking it to return
    ///    * once it has finished processing the current event.
    ///    *
    ///    * The name 'exitNestedEventLoop' may be misleading if read too literally.
    ///    * The affected event loop does not return immediately when this method is
    ///    * called. Rather, this method simply returns to its caller; the affected
    ///    * loop's current event handler is allowed to run to completion; and then
    ///    * that loop returns without processing any more events.
    ///    *
    ///    * This method ignores loops that have already been stopped, and operates on
    ///    * the youngest loop that is still running. Each call to this method stops
    ///    * another running loop.
    ///    *
    ///    * @return depth      The number of running enterNestedEventLoop calls
    ///    *                    remaining, now that one has been stopped.
    ///    *
    ///    * @throws NS_ERROR_FAILURE if there are no running enterNestedEventLoop calls.
    ///    */
    /// ```
    ///

    /// `unsigned long exitNestedEventLoop ();`
    #[inline]
    pub unsafe fn ExitNestedEventLoop(&self, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).ExitNestedEventLoop)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * The number of running enterNestedEventLoop calls on the stack.
    ///     * This count does not include stopped enterNestedEventLoop calls.
    ///     */
    /// ```
    ///

    /// `readonly attribute unsigned long eventLoopNestLevel;`
    #[inline]
    pub unsafe fn GetEventLoopNestLevel(&self, aEventLoopNestLevel: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEventLoopNestLevel)(self, aEventLoopNestLevel)
    }


    /// ```text
    /// /**
    ///    * The |requestor| value that was passed to the youngest running
    ///    * enterNestedEventLoop call.
    ///    */
    /// ```
    ///

    /// `readonly attribute jsval lastNestRequestor;`
    const _GetLastNestRequestor: () = ();

}


