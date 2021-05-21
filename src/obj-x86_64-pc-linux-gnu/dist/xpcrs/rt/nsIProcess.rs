//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIProcess.idl
//


/// `interface nsIProcess : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProcess {
    vtable: *const nsIProcessVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProcess.
unsafe impl XpCom for nsIProcess {
    const IID: nsIID = nsID(0x609610de, 0x9954, 0x4a63,
        [0x8a, 0x7c, 0x34, 0x63, 0x50, 0xa8, 0x64, 0x03]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProcess {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProcess.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProcessCoerce {
    /// Cheaply cast a value of this type from a `nsIProcess`.
    fn coerce_from(v: &nsIProcess) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProcessCoerce for nsIProcess {
    #[inline]
    fn coerce_from(v: &nsIProcess) -> &Self {
        v
    }
}

impl nsIProcess {
    /// Cast this `nsIProcess` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProcessCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProcess {
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
impl<T: nsISupportsCoerce> nsIProcessCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProcess) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProcess
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProcessVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in nsIFile executable); */
    pub Init: unsafe extern "system" fn (this: *const nsIProcess, executable: *const nsIFile) -> ::nserror::nsresult,

    /* void kill (); */
    pub Kill: unsafe extern "system" fn (this: *const nsIProcess) -> ::nserror::nsresult,

    /* void run (in boolean blocking, [array, size_is (count)] in string args, in unsigned long count); */
    pub Run: unsafe extern "system" fn (this: *const nsIProcess, blocking: bool, args: *mut *const libc::c_char, count: u32) -> ::nserror::nsresult,

    /* void runAsync ([array, size_is (count)] in string args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
    pub RunAsync: unsafe extern "system" fn (this: *const nsIProcess, args: *mut *const libc::c_char, count: u32, observer: *const nsIObserver, holdWeak: bool) -> ::nserror::nsresult,

    /* void runw (in boolean blocking, [array, size_is (count)] in wstring args, in unsigned long count); */
    pub Runw: unsafe extern "system" fn (this: *const nsIProcess, blocking: bool, args: *mut *const i16, count: u32) -> ::nserror::nsresult,

    /* void runwAsync ([array, size_is (count)] in wstring args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
    pub RunwAsync: unsafe extern "system" fn (this: *const nsIProcess, args: *mut *const i16, count: u32, observer: *const nsIObserver, holdWeak: bool) -> ::nserror::nsresult,

    /* attribute boolean startHidden; */
    pub GetStartHidden: unsafe extern "system" fn (this: *const nsIProcess, aStartHidden: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean startHidden; */
    pub SetStartHidden: unsafe extern "system" fn (this: *const nsIProcess, aStartHidden: bool) -> ::nserror::nsresult,

    /* attribute boolean noShell; */
    pub GetNoShell: unsafe extern "system" fn (this: *const nsIProcess, aNoShell: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean noShell; */
    pub SetNoShell: unsafe extern "system" fn (this: *const nsIProcess, aNoShell: bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long pid; */
    pub GetPid: unsafe extern "system" fn (this: *const nsIProcess, aPid: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute long exitValue; */
    pub GetExitValue: unsafe extern "system" fn (this: *const nsIProcess, aExitValue: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute boolean isRunning; */
    pub GetIsRunning: unsafe extern "system" fn (this: *const nsIProcess, aIsRunning: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProcess {

    /// ```text
    /// /**
    ///    * Initialises the process with an executable to be run. Call the run method
    ///    * to run the executable.
    ///    * @param executable The executable to run.
    ///    */
    /// ```
    ///

    /// `void init (in nsIFile executable);`
    #[inline]
    pub unsafe fn Init(&self, executable: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, executable)
    }


    /// ```text
    /// /**
    ///    * Kills the running process. After exiting the process will either have
    ///    * been killed or a failure will have been returned.
    ///    */
    /// ```
    ///

    /// `void kill ();`
    #[inline]
    pub unsafe fn Kill(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Kill)(self, )
    }


    /// ```text
    /// /**
    ///    * Executes the file this object was initialized with
    ///    * @param blocking   Whether to wait until the process terminates before
    ///                        returning or not.
    ///    * @param args       An array of arguments to pass to the process in the
    ///    *                   native character set.
    ///    * @param count      The length of the args array.
    ///    */
    /// ```
    ///

    /// `void run (in boolean blocking, [array, size_is (count)] in string args, in unsigned long count);`
    #[inline]
    pub unsafe fn Run(&self, blocking: bool, args: *mut *const libc::c_char, count: u32) -> ::nserror::nsresult {
        ((*self.vtable).Run)(self, blocking, args, count)
    }


    /// ```text
    /// /**
    ///    * Executes the file this object was initialized with optionally calling
    ///    * an observer after the process has finished running.
    ///    * @param args       An array of arguments to pass to the process in the
    ///    *                   native character set.
    ///    * @param count      The length of the args array.
    ///    * @param observer   An observer to notify when the process has completed. It
    ///    *                   will receive this process instance as the subject and
    ///    *                   "process-finished" or "process-failed" as the topic. The
    ///    *                   observer will be notified on the main thread.
    ///    * @param holdWeak   Whether to use a weak reference to hold the observer.
    ///    */
    /// ```
    ///

    /// `void runAsync ([array, size_is (count)] in string args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak);`
    #[inline]
    pub unsafe fn RunAsync(&self, args: *mut *const libc::c_char, count: u32, observer: *const nsIObserver, holdWeak: bool) -> ::nserror::nsresult {
        ((*self.vtable).RunAsync)(self, args, count, observer, holdWeak)
    }


    /// ```text
    /// /**
    ///    * Executes the file this object was initialized with
    ///    * @param blocking   Whether to wait until the process terminates before
    ///                        returning or not.
    ///    * @param args       An array of arguments to pass to the process in UTF-16
    ///    * @param count      The length of the args array.
    ///    */
    /// ```
    ///

    /// `void runw (in boolean blocking, [array, size_is (count)] in wstring args, in unsigned long count);`
    #[inline]
    pub unsafe fn Runw(&self, blocking: bool, args: *mut *const i16, count: u32) -> ::nserror::nsresult {
        ((*self.vtable).Runw)(self, blocking, args, count)
    }


    /// ```text
    /// /**
    ///    * Executes the file this object was initialized with optionally calling
    ///    * an observer after the process has finished running.
    ///    * @param args       An array of arguments to pass to the process in UTF-16
    ///    * @param count      The length of the args array.
    ///    * @param observer   An observer to notify when the process has completed. It
    ///    *                   will receive this process instance as the subject and
    ///    *                   "process-finished" or "process-failed" as the topic. The
    ///    *                   observer will be notified on the main thread.
    ///    * @param holdWeak   Whether to use a weak reference to hold the observer.
    ///    */
    /// ```
    ///

    /// `void runwAsync ([array, size_is (count)] in wstring args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak);`
    #[inline]
    pub unsafe fn RunwAsync(&self, args: *mut *const i16, count: u32, observer: *const nsIObserver, holdWeak: bool) -> ::nserror::nsresult {
        ((*self.vtable).RunwAsync)(self, args, count, observer, holdWeak)
    }


    /// ```text
    /// /**
    ///    * When set to true the process will not open a new window when started and
    ///    * will run hidden from the user. This currently affects only the Windows
    ///    * platform.
    ///    */
    /// ```
    ///

    /// `attribute boolean startHidden;`
    #[inline]
    pub unsafe fn GetStartHidden(&self, aStartHidden: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetStartHidden)(self, aStartHidden)
    }


    /// ```text
    /// /**
    ///    * When set to true the process will not open a new window when started and
    ///    * will run hidden from the user. This currently affects only the Windows
    ///    * platform.
    ///    */
    /// ```
    ///

    /// `attribute boolean startHidden;`
    #[inline]
    pub unsafe fn SetStartHidden(&self, aStartHidden: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetStartHidden)(self, aStartHidden)
    }


    /// ```text
    /// /**
    ///    * When set to true the process will be launched directly without using the
    ///    * shell. This currently affects only the Windows platform.
    ///    */
    /// ```
    ///

    /// `attribute boolean noShell;`
    #[inline]
    pub unsafe fn GetNoShell(&self, aNoShell: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNoShell)(self, aNoShell)
    }


    /// ```text
    /// /**
    ///    * When set to true the process will be launched directly without using the
    ///    * shell. This currently affects only the Windows platform.
    ///    */
    /// ```
    ///

    /// `attribute boolean noShell;`
    #[inline]
    pub unsafe fn SetNoShell(&self, aNoShell: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetNoShell)(self, aNoShell)
    }


    /// ```text
    /// /**
    ///    * The process identifier of the currently running process. This will only
    ///    * be available after the process has started and may not be available on
    ///    * some platforms.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long pid;`
    #[inline]
    pub unsafe fn GetPid(&self, aPid: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPid)(self, aPid)
    }


    /// ```text
    /// /**
    ///    * The exit value of the process. This is only valid after the process has
    ///    * exited.
    ///    */
    /// ```
    ///

    /// `readonly attribute long exitValue;`
    #[inline]
    pub unsafe fn GetExitValue(&self, aExitValue: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetExitValue)(self, aExitValue)
    }


    /// ```text
    /// /**
    ///    * Returns whether the process is currently running or not.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isRunning;`
    #[inline]
    pub unsafe fn GetIsRunning(&self, aIsRunning: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsRunning)(self, aIsRunning)
    }


}


