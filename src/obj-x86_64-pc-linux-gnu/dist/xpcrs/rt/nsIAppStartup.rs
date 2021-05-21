//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/startup/public/nsIAppStartup.idl
//


/// `interface nsIAppStartup : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAppStartup {
    vtable: *const nsIAppStartupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAppStartup.
unsafe impl XpCom for nsIAppStartup {
    const IID: nsIID = nsID(0x6621f6d5, 0x6c04, 0x4a0e,
        [0x9e, 0x74, 0x44, 0x7d, 0xb2, 0x21, 0x48, 0x4e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAppStartup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAppStartup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAppStartupCoerce {
    /// Cheaply cast a value of this type from a `nsIAppStartup`.
    fn coerce_from(v: &nsIAppStartup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAppStartupCoerce for nsIAppStartup {
    #[inline]
    fn coerce_from(v: &nsIAppStartup) -> &Self {
        v
    }
}

impl nsIAppStartup {
    /// Cast this `nsIAppStartup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAppStartupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAppStartup {
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
impl<T: nsISupportsCoerce> nsIAppStartupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppStartup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAppStartup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAppStartupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void createHiddenWindow (); */
    pub CreateHiddenWindow: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* void destroyHiddenWindow (); */
    pub DestroyHiddenWindow: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* void run (); */
    pub Run: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* void enterLastWindowClosingSurvivalArea (); */
    pub EnterLastWindowClosingSurvivalArea: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* void exitLastWindowClosingSurvivalArea (); */
    pub ExitLastWindowClosingSurvivalArea: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* readonly attribute boolean automaticSafeModeNecessary; */
    pub GetAutomaticSafeModeNecessary: unsafe extern "system" fn (this: *const nsIAppStartup, aAutomaticSafeModeNecessary: *mut bool) -> ::nserror::nsresult,

    /* void restartInSafeMode (in uint32_t aQuitMode); */
    pub RestartInSafeMode: unsafe extern "system" fn (this: *const nsIAppStartup, aQuitMode: uint32_t) -> ::nserror::nsresult,

    /* void createInstanceWithProfile (in nsIToolkitProfile aProfile); */
    pub CreateInstanceWithProfile: unsafe extern "system" fn (this: *const nsIAppStartup, aProfile: *const nsIToolkitProfile) -> ::nserror::nsresult,

    /* bool trackStartupCrashBegin (); */
    pub TrackStartupCrashBegin: unsafe extern "system" fn (this: *const nsIAppStartup, _retval: *mut bool) -> ::nserror::nsresult,

    /* void trackStartupCrashEnd (); */
    pub TrackStartupCrashEnd: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* bool quit (in uint32_t aMode, [optional] in int32_t aExitCode); */
    pub Quit: unsafe extern "system" fn (this: *const nsIAppStartup, aMode: uint32_t, aExitCode: int32_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean shuttingDown; */
    pub GetShuttingDown: unsafe extern "system" fn (this: *const nsIAppStartup, aShuttingDown: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean startingUp; */
    pub GetStartingUp: unsafe extern "system" fn (this: *const nsIAppStartup, aStartingUp: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void doneStartingUp (); */
    pub DoneStartingUp: unsafe extern "system" fn (this: *const nsIAppStartup) -> ::nserror::nsresult,

    /* readonly attribute boolean restarting; */
    pub GetRestarting: unsafe extern "system" fn (this: *const nsIAppStartup, aRestarting: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean wasRestarted; */
    pub GetWasRestarted: unsafe extern "system" fn (this: *const nsIAppStartup, aWasRestarted: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute int64_t secondsSinceLastOSRestart; */
    pub GetSecondsSinceLastOSRestart: unsafe extern "system" fn (this: *const nsIAppStartup, aSecondsSinceLastOSRestart: *mut int64_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getStartupInfo (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetStartupInfo: *const ::libc::c_void,

    /* attribute boolean interrupted; */
    pub GetInterrupted: unsafe extern "system" fn (this: *const nsIAppStartup, aInterrupted: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean interrupted; */
    pub SetInterrupted: unsafe extern "system" fn (this: *const nsIAppStartup, aInterrupted: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAppStartup {
    /// ```text
    /// /**
    ///      * The following flags may be passed as the aMode parameter to the quit
    ///      * method.  One and only one of the "Quit" flags must be specified.  The
    ///      * eRestart flag may be bit-wise combined with one of the "Quit" flags to
    ///      * cause the application to restart after it quits.
    ///      */
    /// /**
    ///      * Attempt to quit if all windows are closed.
    ///      */
    /// ```
    ///

    pub const eConsiderQuit: i64 = 1;

    /// ```text
    /// /**
    ///      * Try to close all windows, then quit if successful.
    ///      */
    /// ```
    ///

    pub const eAttemptQuit: i64 = 2;

    /// ```text
    /// /**
    ///      * Quit, damnit!
    ///      */
    /// ```
    ///

    pub const eForceQuit: i64 = 3;

    /// ```text
    /// /**
    ///      * Restart the application after quitting.  The application will be
    ///      * restarted with the same profile and an empty command line.
    ///      */
    /// ```
    ///

    pub const eRestart: i64 = 16;

    /// ```text
    /// /**
    ///      * Create the hidden window.
    ///      */
    /// ```
    ///

    /// `void createHiddenWindow ();`
    #[inline]
    pub unsafe fn CreateHiddenWindow(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CreateHiddenWindow)(self, )
    }


    /// ```text
    /// /**
    ///      * Destroys the hidden window. This will have no effect if the hidden window
    ///      * has not yet been created.
    ///      */
    /// ```
    ///

    /// `void destroyHiddenWindow ();`
    #[inline]
    pub unsafe fn DestroyHiddenWindow(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DestroyHiddenWindow)(self, )
    }


    /// ```text
    /// /**
    ///      * Runs an application event loop: normally the main event pump which
    ///      * defines the lifetime of the application. If there are no windows open
    ///      * and no outstanding calls to enterLastWindowClosingSurvivalArea this
    ///      * method will exit immediately.
    ///      *
    ///      * @returnCode NS_SUCCESS_RESTART_APP
    ///      *             This return code indicates that the application should be
    ///      *             restarted because quit was called with the eRestart flag.
    ///      */
    /// ```
    ///

    /// `void run ();`
    #[inline]
    pub unsafe fn Run(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Run)(self, )
    }


    /// ```text
    /// /**
    ///      * There are situations where all application windows will be
    ///      * closed but we don't want to take this as a signal to quit the
    ///      * app. Bracket the code where the last window could close with
    ///      * these.
    ///      */
    /// ```
    ///

    /// `void enterLastWindowClosingSurvivalArea ();`
    #[inline]
    pub unsafe fn EnterLastWindowClosingSurvivalArea(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnterLastWindowClosingSurvivalArea)(self, )
    }



    /// `void exitLastWindowClosingSurvivalArea ();`
    #[inline]
    pub unsafe fn ExitLastWindowClosingSurvivalArea(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ExitLastWindowClosingSurvivalArea)(self, )
    }


    /// ```text
    /// /**
    ///      * Startup Crash Detection
    ///      *
    ///      * Keeps track of application startup begining and success using flags to
    ///      * determine whether the application is crashing on startup.
    ///      * When the number of crashes crosses the acceptable threshold, safe mode
    ///      * or other repair procedures are performed.
    ///      */
    /// /**
    ///      * Whether automatic safe mode is necessary at this time.  This gets set
    ///      * in trackStartupCrashBegin.
    ///      *
    ///      * @see trackStartupCrashBegin
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean automaticSafeModeNecessary;`
    #[inline]
    pub unsafe fn GetAutomaticSafeModeNecessary(&self, aAutomaticSafeModeNecessary: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAutomaticSafeModeNecessary)(self, aAutomaticSafeModeNecessary)
    }


    /// ```text
    /// /**
    ///      * Restart the application in safe mode
    ///      * @param aQuitMode
    ///      *        This parameter modifies how the app is shutdown.
    ///      * @see nsIAppStartup::quit
    ///      */
    /// ```
    ///

    /// `void restartInSafeMode (in uint32_t aQuitMode);`
    #[inline]
    pub unsafe fn RestartInSafeMode(&self, aQuitMode: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).RestartInSafeMode)(self, aQuitMode)
    }


    /// ```text
    /// /**
    ///      * Run a new instance of this app with a specified profile
    ///      * @param aProfile
    ///      *        The profile we want to use.
    ///      * @see nsIAppStartup::quit
    ///      */
    /// ```
    ///

    /// `void createInstanceWithProfile (in nsIToolkitProfile aProfile);`
    #[inline]
    pub unsafe fn CreateInstanceWithProfile(&self, aProfile: *const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).CreateInstanceWithProfile)(self, aProfile)
    }


    /// ```text
    /// /**
    ///      * If the last startup crashed then increment a counter.
    ///      * Set a flag so on next startup we can detect whether TrackStartupCrashEnd
    ///      * was called (and therefore the application crashed).
    ///      * @return whether safe mode is necessary
    ///      */
    /// ```
    ///

    /// `bool trackStartupCrashBegin ();`
    #[inline]
    pub unsafe fn TrackStartupCrashBegin(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).TrackStartupCrashBegin)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * We have succesfully started without crashing. Clear flags that were
    ///      * tracking past crashes.
    ///      */
    /// ```
    ///

    /// `void trackStartupCrashEnd ();`
    #[inline]
    pub unsafe fn TrackStartupCrashEnd(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TrackStartupCrashEnd)(self, )
    }


    /// ```text
    /// /**
    ///      * Exit the event loop, and shut down the app.
    ///      *
    ///      * @param aMode
    ///      *        This parameter modifies how the app is shutdown, and it is
    ///      *        constructed from the constants defined above.
    ///      * @param aExitCode
    ///      *        The exit code to return from the process. The precise code
    ///      *        returned by the process may vary depending on the platform. Only
    ///      *        values 0-255 should generallt be used. If not specified an exit
    ///      *        code of 0 will be used.
    ///      *
    ///      * @return false if the shutdown was cancelled due to the presence
    ///      *         of a hidden window or if the user disallowed a window
    ///      *         to be closed.
    ///      */
    /// ```
    ///

    /// `bool quit (in uint32_t aMode, [optional] in int32_t aExitCode);`
    #[inline]
    pub unsafe fn Quit(&self, aMode: uint32_t, aExitCode: int32_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Quit)(self, aMode, aExitCode, _retval)
    }


    /// ```text
    /// /**
    ///      * True if the application is in the process of shutting down.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean shuttingDown;`
    #[inline]
    pub unsafe fn GetShuttingDown(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetShuttingDown)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * True if the application is in the process of starting up.
    ///      *
    ///      * Startup is complete once all observers of final-ui-startup have returned.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean startingUp;`
    #[inline]
    pub unsafe fn GetStartingUp(&self, aStartingUp: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetStartingUp)(self, aStartingUp)
    }


    /// ```text
    /// /**
    ///      * Mark the startup as completed.
    ///      *
    ///      * Called at the end of startup by nsAppRunner.
    ///      */
    /// ```
    ///

    /// `[noscript] void doneStartingUp ();`
    #[inline]
    pub unsafe fn DoneStartingUp(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DoneStartingUp)(self, )
    }


    /// ```text
    /// /**
    ///      * True if the application is being restarted
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean restarting;`
    #[inline]
    pub unsafe fn GetRestarting(&self, aRestarting: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRestarting)(self, aRestarting)
    }


    /// ```text
    /// /**
    ///      * True if this is the startup following restart, i.e. if the application
    ///      * was restarted using quit(eRestart*).
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean wasRestarted;`
    #[inline]
    pub unsafe fn GetWasRestarted(&self, aWasRestarted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWasRestarted)(self, aWasRestarted)
    }


    /// ```text
    /// /**
    ///      * The number of seconds since the OS was last rebooted
    ///      */
    /// ```
    ///

    /// `readonly attribute int64_t secondsSinceLastOSRestart;`
    #[inline]
    pub unsafe fn GetSecondsSinceLastOSRestart(&self, aSecondsSinceLastOSRestart: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetSecondsSinceLastOSRestart)(self, aSecondsSinceLastOSRestart)
    }


    /// ```text
    /// /**
    ///      * Returns an object with main, process, firstPaint, sessionRestored properties.
    ///      * Properties may not be available depending on platform or application
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getStartupInfo ();`
    const _GetStartupInfo: () = ();

    /// ```text
    /// /**
    ///      * True if startup was interrupted by an interactive prompt.
    ///      */
    /// ```
    ///

    /// `attribute boolean interrupted;`
    #[inline]
    pub unsafe fn GetInterrupted(&self, aInterrupted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInterrupted)(self, aInterrupted)
    }


    /// ```text
    /// /**
    ///      * True if startup was interrupted by an interactive prompt.
    ///      */
    /// ```
    ///

    /// `attribute boolean interrupted;`
    #[inline]
    pub unsafe fn SetInterrupted(&self, aInterrupted: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetInterrupted)(self, aInterrupted)
    }


}


