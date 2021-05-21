//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIXULRuntime.idl
//


/// `interface nsIXULRuntime : nsISupports`
///

/// ```text
/// /**
///  * Provides information about the XUL runtime.
///  * @status UNSTABLE - This interface is not frozen and will probably change in
///  *                    future releases. If you need this functionality to be
///  *                    stable/frozen, please contact Benjamin Smedberg.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXULRuntime {
    vtable: *const nsIXULRuntimeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXULRuntime.
unsafe impl XpCom for nsIXULRuntime {
    const IID: nsIID = nsID(0x03602fac, 0xfa3f, 0x4a50,
        [0x9b, 0xaa, 0xb8, 0x84, 0x56, 0xfb, 0x4a, 0x0f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXULRuntime {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXULRuntime.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXULRuntimeCoerce {
    /// Cheaply cast a value of this type from a `nsIXULRuntime`.
    fn coerce_from(v: &nsIXULRuntime) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXULRuntimeCoerce for nsIXULRuntime {
    #[inline]
    fn coerce_from(v: &nsIXULRuntime) -> &Self {
        v
    }
}

impl nsIXULRuntime {
    /// Cast this `nsIXULRuntime` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXULRuntimeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXULRuntime {
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
impl<T: nsISupportsCoerce> nsIXULRuntimeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULRuntime) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXULRuntime
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXULRuntimeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean inSafeMode; */
    pub GetInSafeMode: unsafe extern "system" fn (this: *const nsIXULRuntime, aInSafeMode: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean fissionAutostart; */
    pub GetFissionAutostart: unsafe extern "system" fn (this: *const nsIXULRuntime, aFissionAutostart: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIXULRuntime_ExperimentStatus fissionExperimentStatus; */
    pub GetFissionExperimentStatus: unsafe extern "system" fn (this: *const nsIXULRuntime, aFissionExperimentStatus: *mut u8) -> ::nserror::nsresult,

    /* readonly attribute nsIXULRuntime_FissionDecisionStatus fissionDecisionStatus; */
    pub GetFissionDecisionStatus: unsafe extern "system" fn (this: *const nsIXULRuntime, aFissionDecisionStatus: *mut u8) -> ::nserror::nsresult,

    /* readonly attribute ACString fissionDecisionStatusString; */
    pub GetFissionDecisionStatusString: unsafe extern "system" fn (this: *const nsIXULRuntime, aFissionDecisionStatusString: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean sessionHistoryInParent; */
    pub GetSessionHistoryInParent: unsafe extern "system" fn (this: *const nsIXULRuntime, aSessionHistoryInParent: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean logConsoleErrors; */
    pub GetLogConsoleErrors: unsafe extern "system" fn (this: *const nsIXULRuntime, aLogConsoleErrors: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean logConsoleErrors; */
    pub SetLogConsoleErrors: unsafe extern "system" fn (this: *const nsIXULRuntime, aLogConsoleErrors: bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String OS; */
    pub GetOS: unsafe extern "system" fn (this: *const nsIXULRuntime, aOS: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String XPCOMABI; */
    pub GetXPCOMABI: unsafe extern "system" fn (this: *const nsIXULRuntime, aXPCOMABI: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String widgetToolkit; */
    pub GetWidgetToolkit: unsafe extern "system" fn (this: *const nsIXULRuntime, aWidgetToolkit: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long processType; */
    pub GetProcessType: unsafe extern "system" fn (this: *const nsIXULRuntime, aProcessType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long processID; */
    pub GetProcessID: unsafe extern "system" fn (this: *const nsIXULRuntime, aProcessID: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute uint64_t uniqueProcessID; */
    pub GetUniqueProcessID: unsafe extern "system" fn (this: *const nsIXULRuntime, aUniqueProcessID: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String remoteType; */
    pub GetRemoteType: unsafe extern "system" fn (this: *const nsIXULRuntime, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean browserTabsRemoteAutostart; */
    pub GetBrowserTabsRemoteAutostart: unsafe extern "system" fn (this: *const nsIXULRuntime, aBrowserTabsRemoteAutostart: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute uint32_t maxWebProcessCount; */
    pub GetMaxWebProcessCount: unsafe extern "system" fn (this: *const nsIXULRuntime, aMaxWebProcessCount: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean accessibilityEnabled; */
    pub GetAccessibilityEnabled: unsafe extern "system" fn (this: *const nsIXULRuntime, aAccessibilityEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean accessibleHandlerUsed; */
    pub GetAccessibleHandlerUsed: unsafe extern "system" fn (this: *const nsIXULRuntime, aAccessibleHandlerUsed: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString accessibilityInstantiator; */
    pub GetAccessibilityInstantiator: unsafe extern "system" fn (this: *const nsIXULRuntime, aAccessibilityInstantiator: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean shouldBlockIncompatJaws; */
    pub GetShouldBlockIncompatJaws: unsafe extern "system" fn (this: *const nsIXULRuntime, aShouldBlockIncompatJaws: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean is64Bit; */
    pub GetIs64Bit: unsafe extern "system" fn (this: *const nsIXULRuntime, aIs64Bit: *mut bool) -> ::nserror::nsresult,

    /* void invalidateCachesOnRestart (); */
    pub InvalidateCachesOnRestart: unsafe extern "system" fn (this: *const nsIXULRuntime) -> ::nserror::nsresult,

    /* void ensureContentProcess (); */
    pub EnsureContentProcess: unsafe extern "system" fn (this: *const nsIXULRuntime) -> ::nserror::nsresult,

    /* readonly attribute PRTime replacedLockTime; */
    pub GetReplacedLockTime: unsafe extern "system" fn (this: *const nsIXULRuntime, aReplacedLockTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute boolean isReleaseOrBeta; */
    pub GetIsReleaseOrBeta: unsafe extern "system" fn (this: *const nsIXULRuntime, aIsReleaseOrBeta: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isOfficialBranding; */
    pub GetIsOfficialBranding: unsafe extern "system" fn (this: *const nsIXULRuntime, aIsOfficialBranding: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String defaultUpdateChannel; */
    pub GetDefaultUpdateChannel: unsafe extern "system" fn (this: *const nsIXULRuntime, aDefaultUpdateChannel: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String distributionID; */
    pub GetDistributionID: unsafe extern "system" fn (this: *const nsIXULRuntime, aDistributionID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean windowsDLLBlocklistStatus; */
    pub GetWindowsDLLBlocklistStatus: unsafe extern "system" fn (this: *const nsIXULRuntime, aWindowsDLLBlocklistStatus: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean restartedByOS; */
    pub GetRestartedByOS: unsafe extern "system" fn (this: *const nsIXULRuntime, aRestartedByOS: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString processStartupShortcut; */
    pub GetProcessStartupShortcut: unsafe extern "system" fn (this: *const nsIXULRuntime, aProcessStartupShortcut: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t launcherProcessState; */
    pub GetLauncherProcessState: unsafe extern "system" fn (this: *const nsIXULRuntime, aLauncherProcessState: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute ACString lastAppVersion; */
    pub GetLastAppVersion: unsafe extern "system" fn (this: *const nsIXULRuntime, aLastAppVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString lastAppBuildID; */
    pub GetLastAppBuildID: unsafe extern "system" fn (this: *const nsIXULRuntime, aLastAppBuildID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXULRuntime {
    /// ```text
    /// /**
    ///    * The legal values of processType.
    ///    */
    /// ```
    ///

    pub const PROCESS_TYPE_DEFAULT: i64 = 0;


    pub const PROCESS_TYPE_PLUGIN: i64 = 1;


    pub const PROCESS_TYPE_CONTENT: i64 = 2;


    pub const PROCESS_TYPE_IPDLUNITTEST: i64 = 3;


    pub const PROCESS_TYPE_GMPLUGIN: i64 = 4;


    pub const PROCESS_TYPE_GPU: i64 = 5;


    pub const PROCESS_TYPE_VR: i64 = 6;


    pub const PROCESS_TYPE_RDD: i64 = 7;


    pub const PROCESS_TYPE_SOCKET: i64 = 8;


    pub const PROCESS_TYPE_SANDBOX_BROKER: i64 = 9;


    pub const PROCESS_TYPE_FORKSERVER: i64 = 10;

    /// ```text
    /// /**
    ///    * The current e10s-multi experiment number. Set dom.ipc.multiOptOut to (at
        ///    * least) this to disable it until the next experiment.
    ///    */
    /// ```
    ///

    pub const E10S_MULTI_EXPERIMENT: i64 = 1;

    /// ```text
    /// /**
    ///    * Whether the application was launched in safe mode.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean inSafeMode;`
    #[inline]
    pub unsafe fn GetInSafeMode(&self, aInSafeMode: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInSafeMode)(self, aInSafeMode)
    }


    /// ```text
    /// /**
    ///    * Whether Fission should be automatically enabled for new browser windows.
    ///    * This may not match the value of the 'fission.autostart' pref.
    ///    *
    ///    * This value is guaranteed to remain constant for the length of a browser
    ///    * session.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean fissionAutostart;`
    #[inline]
    pub unsafe fn GetFissionAutostart(&self, aFissionAutostart: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFissionAutostart)(self, aFissionAutostart)
    }


    /// ```text
    /// /**
    ///    * The user's enrollment status in the Fission experiment at process startup.
    ///    * See `ExperimentStatus` for the potential values.
    ///    *
    ///    * Only available in the parent process.
    ///    *
    ///    * This value is guaranteed to remain constant for the length of a browser
    ///    * session.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIXULRuntime_ExperimentStatus fissionExperimentStatus;`
    #[inline]
    pub unsafe fn GetFissionExperimentStatus(&self, aFissionExperimentStatus: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetFissionExperimentStatus)(self, aFissionExperimentStatus)
    }


    /// ```text
    /// /**
    ///    * The deciding factor which caused Fission to be enabled or disabled in
    ///    * this session. The string version is the same of the name of the constant,
    ///    * without the leading `eFission`, and with an initial lower-case letter.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIXULRuntime_FissionDecisionStatus fissionDecisionStatus;`
    #[inline]
    pub unsafe fn GetFissionDecisionStatus(&self, aFissionDecisionStatus: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetFissionDecisionStatus)(self, aFissionDecisionStatus)
    }



    /// `readonly attribute ACString fissionDecisionStatusString;`
    #[inline]
    pub unsafe fn GetFissionDecisionStatusString(&self, aFissionDecisionStatusString: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFissionDecisionStatusString)(self, aFissionDecisionStatusString)
    }


    /// ```text
    /// /**
    ///    * Whether session history is stored in the parent process.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean sessionHistoryInParent;`
    #[inline]
    pub unsafe fn GetSessionHistoryInParent(&self, aSessionHistoryInParent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSessionHistoryInParent)(self, aSessionHistoryInParent)
    }


    /// ```text
    /// /**
    ///    * Whether to write console errors to a log file. If a component
    ///    * encounters startup errors that might prevent the app from showing
    ///    * proper UI, it should set this flag to "true".
    ///    */
    /// ```
    ///

    /// `attribute boolean logConsoleErrors;`
    #[inline]
    pub unsafe fn GetLogConsoleErrors(&self, aLogConsoleErrors: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLogConsoleErrors)(self, aLogConsoleErrors)
    }


    /// ```text
    /// /**
    ///    * Whether to write console errors to a log file. If a component
    ///    * encounters startup errors that might prevent the app from showing
    ///    * proper UI, it should set this flag to "true".
    ///    */
    /// ```
    ///

    /// `attribute boolean logConsoleErrors;`
    #[inline]
    pub unsafe fn SetLogConsoleErrors(&self, aLogConsoleErrors: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetLogConsoleErrors)(self, aLogConsoleErrors)
    }


    /// ```text
    /// /**
    ///    * A string tag identifying the current operating system. This is taken
    ///    * from the OS_TARGET configure variable. It will always be available.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String OS;`
    #[inline]
    pub unsafe fn GetOS(&self, aOS: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOS)(self, aOS)
    }


    /// ```text
    /// /**
    ///    * A string tag identifying the binary ABI of the current processor and
    ///    * compiler vtable. This is taken from the TARGET_XPCOM_ABI configure
    ///    * variable. It may not be available on all platforms, especially
    ///    * unusual processor or compiler combinations.
    ///    *
    ///    * The result takes the form <processor>-<compilerABI>, for example:
    ///    *   x86-msvc
    ///    *   ppc-gcc3
    ///    *
    ///    * This value should almost always be used in combination with "OS".
    ///    *
    ///    * @throw NS_ERROR_NOT_AVAILABLE if not available.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String XPCOMABI;`
    #[inline]
    pub unsafe fn GetXPCOMABI(&self, aXPCOMABI: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetXPCOMABI)(self, aXPCOMABI)
    }


    /// ```text
    /// /**
    ///    * A string tag identifying the target widget toolkit in use.
    ///    * This is taken from the MOZ_WIDGET_TOOLKIT configure variable.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String widgetToolkit;`
    #[inline]
    pub unsafe fn GetWidgetToolkit(&self, aWidgetToolkit: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetWidgetToolkit)(self, aWidgetToolkit)
    }


    /// ```text
    /// /**
    ///    * The type of the caller's process.  Returns one of the values above.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long processType;`
    #[inline]
    pub unsafe fn GetProcessType(&self, aProcessType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessType)(self, aProcessType)
    }


    /// ```text
    /// /**
    ///    * The system process ID of the caller's process.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long processID;`
    #[inline]
    pub unsafe fn GetProcessID(&self, aProcessID: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessID)(self, aProcessID)
    }


    /// ```text
    /// /**
    ///    * A globally unique and non-recycled ID of the caller's process.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint64_t uniqueProcessID;`
    #[inline]
    pub unsafe fn GetUniqueProcessID(&self, aUniqueProcessID: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetUniqueProcessID)(self, aUniqueProcessID)
    }


    /// ```text
    /// /**
    ///    * The type of remote content process we're running in.
    ///    * null if we're in the parent/chrome process. This can contain
    ///    * a URI if Fission is enabled, so don't use it for any kind of
    ///    * telemetry.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String remoteType;`
    #[inline]
    pub unsafe fn GetRemoteType(&self, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteType)(self, aRemoteType)
    }


    /// ```text
    /// /**
    ///    * If true, browser tabs may be opened by default in a different process
    ///    * from the main browser UI.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean browserTabsRemoteAutostart;`
    #[inline]
    pub unsafe fn GetBrowserTabsRemoteAutostart(&self, aBrowserTabsRemoteAutostart: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowserTabsRemoteAutostart)(self, aBrowserTabsRemoteAutostart)
    }


    /// ```text
    /// /**
    ///    * Returns the number of content processes to use for normal web pages. If
    ///    * this value is > 1, then e10s-multi should be considered to be "on".
    ///    *
    ///    * NB: If browserTabsRemoteAutostart is false, then this value has no
    ///    * meaning and e10s should be considered to be "off"!
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t maxWebProcessCount;`
    #[inline]
    pub unsafe fn GetMaxWebProcessCount(&self, aMaxWebProcessCount: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxWebProcessCount)(self, aMaxWebProcessCount)
    }


    /// ```text
    /// /**
    ///    * If true, the accessibility service is running.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean accessibilityEnabled;`
    #[inline]
    pub unsafe fn GetAccessibilityEnabled(&self, aAccessibilityEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessibilityEnabled)(self, aAccessibilityEnabled)
    }


    /// ```text
    /// /**
    ///    * If true, the AccessibleHandler dll is used.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean accessibleHandlerUsed;`
    #[inline]
    pub unsafe fn GetAccessibleHandlerUsed(&self, aAccessibleHandlerUsed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessibleHandlerUsed)(self, aAccessibleHandlerUsed)
    }


    /// ```text
    /// /**
    ///    * Executable of Windows service that activated accessibility.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString accessibilityInstantiator;`
    #[inline]
    pub unsafe fn GetAccessibilityInstantiator(&self, aAccessibilityInstantiator: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessibilityInstantiator)(self, aAccessibilityInstantiator)
    }


    /// ```text
    /// /**
    ///    * Temporary, do not use. Indicates if an incompat version of JAWS
    ///    * screen reader software is loaded in our process space.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean shouldBlockIncompatJaws;`
    #[inline]
    pub unsafe fn GetShouldBlockIncompatJaws(&self, aShouldBlockIncompatJaws: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetShouldBlockIncompatJaws)(self, aShouldBlockIncompatJaws)
    }


    /// ```text
    /// /**
    ///    * Indicates whether the current Firefox build is 64-bit.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean is64Bit;`
    #[inline]
    pub unsafe fn GetIs64Bit(&self, aIs64Bit: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIs64Bit)(self, aIs64Bit)
    }


    /// ```text
    /// /**
    ///    * Signal the apprunner to invalidate caches on the next restart.
    ///    * This will cause components to be autoregistered and all
    ///    * fastload data to be re-created.
    ///    */
    /// ```
    ///

    /// `void invalidateCachesOnRestart ();`
    #[inline]
    pub unsafe fn InvalidateCachesOnRestart(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).InvalidateCachesOnRestart)(self, )
    }


    /// ```text
    /// /**
    ///    * Starts a child process. This method is intented to pre-start a
    ///    * content child process so that when it is actually needed, it is
    ///    * ready to go.
    ///    *
    ///    * @throw NS_ERROR_NOT_AVAILABLE if not available.
    ///    */
    /// ```
    ///

    /// `void ensureContentProcess ();`
    #[inline]
    pub unsafe fn EnsureContentProcess(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnsureContentProcess)(self, )
    }


    /// ```text
    /// /**
    ///    * Modification time of the profile lock before the profile was locked on
    ///    * this startup. Used to know the last time the profile was used and not
    ///    * closed cleanly. This is set to 0 if there was no existing profile lock.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime replacedLockTime;`
    #[inline]
    pub unsafe fn GetReplacedLockTime(&self, aReplacedLockTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetReplacedLockTime)(self, aReplacedLockTime)
    }


    /// ```text
    /// /**
    ///    * True if this is RELEASE_OR_BETA.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isReleaseOrBeta;`
    #[inline]
    pub unsafe fn GetIsReleaseOrBeta(&self, aIsReleaseOrBeta: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsReleaseOrBeta)(self, aIsReleaseOrBeta)
    }


    /// ```text
    /// /**
    ///    * True if this build uses official branding (MOZ_OFFICIAL_BRANDING).
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isOfficialBranding;`
    #[inline]
    pub unsafe fn GetIsOfficialBranding(&self, aIsOfficialBranding: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsOfficialBranding)(self, aIsOfficialBranding)
    }


    /// ```text
    /// /**
    ///    * The default update channel (MOZ_UPDATE_CHANNEL).
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String defaultUpdateChannel;`
    #[inline]
    pub unsafe fn GetDefaultUpdateChannel(&self, aDefaultUpdateChannel: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultUpdateChannel)(self, aDefaultUpdateChannel)
    }


    /// ```text
    /// /**
    ///    * The distribution ID for this build (MOZ_DISTRIBUTION_ID).
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String distributionID;`
    #[inline]
    pub unsafe fn GetDistributionID(&self, aDistributionID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDistributionID)(self, aDistributionID)
    }


    /// ```text
    /// /**
    ///    * True if Windows DLL blocklist initialized correctly. This is
    ///    * primarily for automated testing purposes.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean windowsDLLBlocklistStatus;`
    #[inline]
    pub unsafe fn GetWindowsDLLBlocklistStatus(&self, aWindowsDLLBlocklistStatus: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowsDLLBlocklistStatus)(self, aWindowsDLLBlocklistStatus)
    }


    /// ```text
    /// /**
    ///    * True if this application was started by the OS as part of an automatic
    ///    * restart mechanism (such as RegisterApplicationRestart on Windows).
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean restartedByOS;`
    #[inline]
    pub unsafe fn GetRestartedByOS(&self, aRestartedByOS: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRestartedByOS)(self, aRestartedByOS)
    }


    /// ```text
    /// /**
    ///    * The path of the shortcut used to start the current process, or "" if none.
    ///    *
    ///    * Windows Main process only, otherwise throws NS_ERROR_NOT_AVAILABLE
    ///    *
    ///    * May be mising in some cases where the user did launch from a shortcut:
    ///    * - If the updater ran on startup
    ///    * - If the AUMID was set before the shortcut could be saved
    ///    *
    ///    * @throw NS_ERROR_NOT_AVAILABLE if not available.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString processStartupShortcut;`
    #[inline]
    pub unsafe fn GetProcessStartupShortcut(&self, aProcessStartupShortcut: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessStartupShortcut)(self, aProcessStartupShortcut)
    }


    /// ```text
    /// /**
    ///    * Returns a value corresponding to one of the
    ///    * |mozilla::LauncherRegistryInfo::EnabledState| values.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t launcherProcessState;`
    #[inline]
    pub unsafe fn GetLauncherProcessState(&self, aLauncherProcessState: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLauncherProcessState)(self, aLauncherProcessState)
    }


    /// ```text
    /// /**
    ///    * Returns the last application version that used the current profile or null
    ///    * if the last version could not be found (compatibility.ini was either
        ///    * missing or invalid). Throws NS_ERROR_UNAVAILABLE if called from a content
    ///    * process.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString lastAppVersion;`
    #[inline]
    pub unsafe fn GetLastAppVersion(&self, aLastAppVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLastAppVersion)(self, aLastAppVersion)
    }


    /// ```text
    /// /**
    ///    * Returns the last application build ID that used the current profile or null
    ///    * if the last build ID could not be found (compatibility.ini was either
        ///    * missing or invalid). Throws NS_ERROR_UNAVAILABLE if called from a content
    ///    * process.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString lastAppBuildID;`
    #[inline]
    pub unsafe fn GetLastAppBuildID(&self, aLastAppBuildID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLastAppBuildID)(self, aLastAppBuildID)
    }


}


