//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIAppShell.idl
//


/// `interface nsIAppShell : nsISupports`
///

/// ```text
/// /**
///  * Interface for the native event system layer.  This interface is designed
///  * to be used on the main application thread only.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAppShell {
    vtable: *const nsIAppShellVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAppShell.
unsafe impl XpCom for nsIAppShell {
    const IID: nsIID = nsID(0x7cd5c71d, 0x223b, 0x4afe,
        [0x93, 0x1d, 0x5e, 0xed, 0xb1, 0xf2, 0xb0, 0x1f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAppShell {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAppShell.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAppShellCoerce {
    /// Cheaply cast a value of this type from a `nsIAppShell`.
    fn coerce_from(v: &nsIAppShell) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAppShellCoerce for nsIAppShell {
    #[inline]
    fn coerce_from(v: &nsIAppShell) -> &Self {
        v
    }
}

impl nsIAppShell {
    /// Cast this `nsIAppShell` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAppShellCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAppShell {
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
impl<T: nsISupportsCoerce> nsIAppShellCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppShell) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAppShell
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAppShellVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void run (); */
    pub Run: unsafe extern "system" fn (this: *const nsIAppShell) -> ::nserror::nsresult,

    /* void exit (); */
    pub Exit: unsafe extern "system" fn (this: *const nsIAppShell) -> ::nserror::nsresult,

    /* void favorPerformanceHint (in boolean favorPerfOverStarvation, in unsigned long starvationDelay); */
    pub FavorPerformanceHint: unsafe extern "system" fn (this: *const nsIAppShell, favorPerfOverStarvation: bool, starvationDelay: u32) -> ::nserror::nsresult,

    /* void suspendNative (); */
    pub SuspendNative: unsafe extern "system" fn (this: *const nsIAppShell) -> ::nserror::nsresult,

    /* void resumeNative (); */
    pub ResumeNative: unsafe extern "system" fn (this: *const nsIAppShell) -> ::nserror::nsresult,

    /* readonly attribute unsigned long eventloopNestingLevel; */
    pub GetEventloopNestingLevel: unsafe extern "system" fn (this: *const nsIAppShell, aEventloopNestingLevel: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAppShell {

    /// ```text
    /// /**
    ///    * Enter an event loop.  Don't leave until exit() is called.
    ///    */
    /// ```
    ///

    /// `void run ();`
    #[inline]
    pub unsafe fn Run(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Run)(self, )
    }


    /// ```text
    /// /**
    ///    * Exit the handle event loop
    ///    */
    /// ```
    ///

    /// `void exit ();`
    #[inline]
    pub unsafe fn Exit(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Exit)(self, )
    }


    /// ```text
    /// /**
    ///    * Give hint to native event queue notification mechanism. If the native
    ///    * platform needs to tradeoff performance vs. native event starvation this
    ///    * hint tells the native dispatch code which to favor.  The default is to
    ///    * prevent native event starvation.
    ///    *
    ///    * Calls to this function may be nested. When the number of calls that pass
    ///    * PR_TRUE is subtracted from the number of calls that pass PR_FALSE is
    ///    * greater than 0, performance is given precedence over preventing event
    ///    * starvation.
    ///    *
    ///    * The starvationDelay arg is only used when favorPerfOverStarvation is
    ///    * PR_FALSE. It is the amount of time in milliseconds to wait before the
    ///    * PR_FALSE actually takes effect.
    ///    */
    /// ```
    ///

    /// `void favorPerformanceHint (in boolean favorPerfOverStarvation, in unsigned long starvationDelay);`
    #[inline]
    pub unsafe fn FavorPerformanceHint(&self, favorPerfOverStarvation: bool, starvationDelay: u32) -> ::nserror::nsresult {
        ((*self.vtable).FavorPerformanceHint)(self, favorPerfOverStarvation, starvationDelay)
    }


    /// ```text
    /// /**
    ///    * Suspends the use of additional platform-specific methods (besides the
        ///    * nsIAppShell->run() event loop) to run Gecko events on the main
    ///    * application thread.  Under some circumstances these "additional methods"
    ///    * can cause Gecko event handlers to be re-entered, sometimes leading to
    ///    * hangs and crashes.  Calls to suspendNative() and resumeNative() may be
    ///    * nested.  On some platforms (those that don't use any "additional
        ///    * methods") this will be a no-op.  Does not (in itself) stop Gecko events
    ///    * from being processed on the main application thread.  But if the
    ///    * nsIAppShell->run() event loop is blocked when this call is made, Gecko
    ///    * events will stop being processed until resumeNative() is called (even
        ///    * if a plugin or library is temporarily processing events on a nested
        ///    * event loop).
    ///    */
    /// ```
    ///

    /// `void suspendNative ();`
    #[inline]
    pub unsafe fn SuspendNative(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SuspendNative)(self, )
    }


    /// ```text
    /// /**
    ///    * Resumes the use of additional platform-specific methods to run Gecko
    ///    * events on the main application thread.  Calls to suspendNative() and
    ///    * resumeNative() may be nested.  On some platforms this will be a no-op.
    ///    */
    /// ```
    ///

    /// `void resumeNative ();`
    #[inline]
    pub unsafe fn ResumeNative(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResumeNative)(self, )
    }


    /// ```text
    /// /**
    ///    * The current event loop nesting level.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long eventloopNestingLevel;`
    #[inline]
    pub unsafe fn GetEventloopNestingLevel(&self, aEventloopNestingLevel: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEventloopNestingLevel)(self, aEventloopNestingLevel)
    }


}


