//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/bitsdownload/nsIBits.idl
//


/// `typedef int32_t  nsProxyUsage;`
///


pub type nsProxyUsage = i32;


/// `typedef int32_t  nsBitsErrorType;`
///


pub type nsBitsErrorType = i32;


/// `typedef int32_t  nsBitsErrorAction;`
///


pub type nsBitsErrorAction = i32;


/// `typedef int32_t  nsBitsErrorStage;`
///


pub type nsBitsErrorStage = i32;


/// `interface nsIBits : nsISupports`
///

/// ```text
/// /**
///  * An interface for interacting with Windows Background Intelligent Transfer
///  * Service. This should only be used on Windows.
///  *
///  * It would be preferable for the functions in this interface to return
///  * Promises, but this interface is implemented in Rust, which does not yet have
///  * support for Promises. There is a JS wrapper around this class that should be
///  * preferred over using this interface directly, located in Bits.jsm.
///  *
///  * Methods of this class that take a nsIBitsNewRequestCallback do not return or
///  * throw errors. All errors will be reported through the callback. The only
///  * things that should cause methods to directly throw errors are null arguments.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBits {
    vtable: *const nsIBitsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBits.
unsafe impl XpCom for nsIBits {
    const IID: nsIID = nsID(0x495d6f3d, 0x9748, 0x4d30,
        [0x8c, 0xe5, 0x02, 0x90, 0xc0, 0x00, 0x1e, 0xdf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBits {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBits.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBitsCoerce {
    /// Cheaply cast a value of this type from a `nsIBits`.
    fn coerce_from(v: &nsIBits) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBitsCoerce for nsIBits {
    #[inline]
    fn coerce_from(v: &nsIBits) -> &Self {
        v
    }
}

impl nsIBits {
    /// Cast this `nsIBits` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBitsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBits {
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
impl<T: nsISupportsCoerce> nsIBitsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBits) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBits
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBitsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean initialized; */
    pub GetInitialized: unsafe extern "system" fn (this: *const nsIBits, aInitialized: *mut bool) -> ::nserror::nsresult,

    /* void init (in AUTF8String jobName, in AUTF8String savePathPrefix, in unsigned long monitorTimeoutMs); */
    pub Init: unsafe extern "system" fn (this: *const nsIBits, jobName: *const ::nsstring::nsACString, savePathPrefix: *const ::nsstring::nsACString, monitorTimeoutMs: u32) -> ::nserror::nsresult,

    /* void startDownload (in AUTF8String downloadURL, in AUTF8String saveRelativePath, in nsProxyUsage proxy, in unsigned long noProgressTimeoutSecs, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback); */
    pub StartDownload: unsafe extern "system" fn (this: *const nsIBits, downloadURL: *const ::nsstring::nsACString, saveRelativePath: *const ::nsstring::nsACString, proxy: nsProxyUsage, noProgressTimeoutSecs: u32, monitorIntervalMs: u32, observer: *const nsIRequestObserver, context: *const nsISupports, callback: *const nsIBitsNewRequestCallback) -> ::nserror::nsresult,

    /* void monitorDownload (in AUTF8String id, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback); */
    pub MonitorDownload: unsafe extern "system" fn (this: *const nsIBits, id: *const ::nsstring::nsACString, monitorIntervalMs: u32, observer: *const nsIRequestObserver, context: *const nsISupports, callback: *const nsIBitsNewRequestCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBits {
    /// ```text
    /// /**
    ///    * nsBitsErrorType values
    ///    * The BITS interface returns many error codes. These are intended to help
    ///    * determine appropriate fallback actions and to report to telemetry.
    ///    */
    /// ```
    ///

    pub const ERROR_TYPE_SUCCESS: i64 = 0;


    pub const ERROR_TYPE_UNKNOWN: i64 = 1;


    pub const ERROR_TYPE_METHOD_THREW: i64 = 2;


    pub const ERROR_TYPE_METHOD_TIMEOUT: i64 = 3;


    pub const ERROR_TYPE_NULL_ARGUMENT: i64 = 4;


    pub const ERROR_TYPE_INVALID_ARGUMENT: i64 = 5;


    pub const ERROR_TYPE_NOT_INITIALIZED: i64 = 6;


    pub const ERROR_TYPE_NO_UTF8_CONVERSION: i64 = 7;


    pub const ERROR_TYPE_INVALID_GUID: i64 = 8;


    pub const ERROR_TYPE_PIPE_NOT_CONNECTED: i64 = 9;


    pub const ERROR_TYPE_PIPE_TIMEOUT: i64 = 10;


    pub const ERROR_TYPE_PIPE_BAD_WRITE_COUNT: i64 = 11;


    pub const ERROR_TYPE_PIPE_API_ERROR: i64 = 12;


    pub const ERROR_TYPE_FAILED_TO_CREATE_BITS_JOB: i64 = 13;


    pub const ERROR_TYPE_FAILED_TO_ADD_FILE_TO_JOB: i64 = 14;


    pub const ERROR_TYPE_FAILED_TO_APPLY_BITS_JOB_SETTINGS: i64 = 15;


    pub const ERROR_TYPE_FAILED_TO_RESUME_BITS_JOB: i64 = 16;


    pub const ERROR_TYPE_OTHER_BITS_ERROR: i64 = 17;


    pub const ERROR_TYPE_OTHER_BITS_CLIENT_ERROR: i64 = 18;


    pub const ERROR_TYPE_BITS_JOB_NOT_FOUND: i64 = 19;


    pub const ERROR_TYPE_FAILED_TO_GET_BITS_JOB: i64 = 20;


    pub const ERROR_TYPE_FAILED_TO_SUSPEND_BITS_JOB: i64 = 21;


    pub const ERROR_TYPE_FAILED_TO_COMPLETE_BITS_JOB: i64 = 22;


    pub const ERROR_TYPE_PARTIALLY_COMPLETED_BITS_JOB: i64 = 23;


    pub const ERROR_TYPE_FAILED_TO_CANCEL_BITS_JOB: i64 = 24;


    pub const ERROR_TYPE_MISSING_RESULT_DATA: i64 = 25;


    pub const ERROR_TYPE_MISSING_CALLBACK: i64 = 26;


    pub const ERROR_TYPE_CALLBACK_ON_WRONG_THREAD: i64 = 27;


    pub const ERROR_TYPE_MISSING_BITS_SERVICE: i64 = 28;


    pub const ERROR_TYPE_BITS_SERVICE_ON_WRONG_THREAD: i64 = 29;


    pub const ERROR_TYPE_MISSING_BITS_REQUEST: i64 = 30;


    pub const ERROR_TYPE_BITS_REQUEST_ON_WRONG_THREAD: i64 = 31;


    pub const ERROR_TYPE_MISSING_OBSERVER: i64 = 32;


    pub const ERROR_TYPE_OBSERVER_ON_WRONG_THREAD: i64 = 33;


    pub const ERROR_TYPE_MISSING_CONTEXT: i64 = 34;


    pub const ERROR_TYPE_CONTEXT_ON_WRONG_THREAD: i64 = 35;


    pub const ERROR_TYPE_FAILED_TO_START_THREAD: i64 = 36;


    pub const ERROR_TYPE_FAILED_TO_CONSTRUCT_TASK_RUNNABLE: i64 = 37;


    pub const ERROR_TYPE_FAILED_TO_DISPATCH_RUNNABLE: i64 = 38;


    pub const ERROR_TYPE_TRANSFER_ALREADY_COMPLETE: i64 = 39;


    pub const ERROR_TYPE_OPERATION_ALREADY_IN_PROGRESS: i64 = 40;


    pub const ERROR_TYPE_MISSING_BITS_CLIENT: i64 = 41;


    pub const ERROR_TYPE_FAILED_TO_GET_JOB_STATUS: i64 = 42;


    pub const ERROR_TYPE_BITS_STATE_ERROR: i64 = 43;


    pub const ERROR_TYPE_BITS_STATE_TRANSIENT_ERROR: i64 = 44;


    pub const ERROR_TYPE_BITS_STATE_CANCELLED: i64 = 45;


    pub const ERROR_TYPE_BITS_STATE_UNEXPECTED: i64 = 46;


    pub const ERROR_TYPE_VERIFICATION_FAILURE: i64 = 47;


    pub const ERROR_TYPE_ACCESS_DENIED_EXPECTED: i64 = 48;


    pub const ERROR_TYPE_FAILED_TO_CONNECT_TO_BCM: i64 = 49;


    pub const ERROR_TYPE_USE_AFTER_REQUEST_SHUTDOWN: i64 = 50;

    /// ```text
    /// /**
    ///    * nsBitsErrorAction values
    ///    * These values indicate where the error occurred.
    ///    */
    /// ```
    ///

    pub const ERROR_ACTION_UNKNOWN: i64 = 1;


    pub const ERROR_ACTION_NONE: i64 = 2;


    pub const ERROR_ACTION_START_DOWNLOAD: i64 = 3;


    pub const ERROR_ACTION_MONITOR_DOWNLOAD: i64 = 4;


    pub const ERROR_ACTION_CHANGE_MONITOR_INTERVAL: i64 = 5;


    pub const ERROR_ACTION_CANCEL: i64 = 6;


    pub const ERROR_ACTION_SET_PRIORITY: i64 = 7;


    pub const ERROR_ACTION_COMPLETE: i64 = 8;


    pub const ERROR_ACTION_SUSPEND: i64 = 9;


    pub const ERROR_ACTION_RESUME: i64 = 10;


    pub const ERROR_ACTION_SET_NO_PROGRESS_TIMEOUT: i64 = 11;

    /// ```text
    /// /**
    ///    * nsBitsErrorStage values
    ///    * These values allow the caller to determine at what point in the download
    ///    * mechanism a failure occurred.
    ///    */
    /// ```
    ///

    pub const ERROR_STAGE_UNKNOWN: i64 = 1;


    pub const ERROR_STAGE_PRETASK: i64 = 2;


    pub const ERROR_STAGE_COMMAND_THREAD: i64 = 3;


    pub const ERROR_STAGE_AGENT_COMMUNICATION: i64 = 4;


    pub const ERROR_STAGE_BITS_CLIENT: i64 = 5;


    pub const ERROR_STAGE_MAIN_THREAD: i64 = 6;


    pub const ERROR_STAGE_MONITOR: i64 = 7;


    pub const ERROR_STAGE_VERIFICATION: i64 = 8;

    /// ```text
    /// /**
    ///    * These values indicate what type of error code was returned. These are used
    ///    * to allow the different types taken by the different callback failure
    ///    * functions to be made into one generic error type in Javascript.
    ///    */
    /// ```
    ///

    pub const ERROR_CODE_TYPE_NONE: i64 = 1;


    pub const ERROR_CODE_TYPE_NSRESULT: i64 = 2;


    pub const ERROR_CODE_TYPE_HRESULT: i64 = 3;


    pub const ERROR_CODE_TYPE_STRING: i64 = 4;


    pub const ERROR_CODE_TYPE_EXCEPTION: i64 = 5;


    pub const PROXY_NONE: i64 = 1;


    pub const PROXY_PRECONFIG: i64 = 2;


    pub const PROXY_AUTODETECT: i64 = 3;

    /// ```text
    /// /**
    ///    * Indicates whether init() has been called.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean initialized;`
    #[inline]
    pub unsafe fn GetInitialized(&self, aInitialized: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInitialized)(self, aInitialized)
    }


    /// ```text
    /// /**
    ///    * Initializes the BITS interface. Unlike other functions here, this happens
    ///    * synchronously.
    ///    * init() should only be called only once.
    ///    *
    ///    * @param jobName
    ///    *        The name of the BITS job. This is used both to set the name during
    ///    *        job creation and to verify that a job is ours.
    ///    * @param savePathPrefix
    ///    *        The directory that downloads will be saved to. Providing a safe
    ///    *        directory here ensures that the download path cannot be manipulated
    ///    *        to save files to a malicious location. Downloads are guaranteed to
    ///    *        be saved to this directory or a subdirectory.
    ///    * @param monitorTimeoutMs
    ///    *        The amount of time to wait between download monitor notifications.
    ///    *        This should be larger than the largest monitorIntervalMs that will
    ///    *        be passed to startDownload(), monitorDownload(), or
    ///    *        changeMonitorInterval(). This value may not be 0.
    ///    */
    /// ```
    ///

    /// `void init (in AUTF8String jobName, in AUTF8String savePathPrefix, in unsigned long monitorTimeoutMs);`
    #[inline]
    pub unsafe fn Init(&self, jobName: *const ::nsstring::nsACString, savePathPrefix: *const ::nsstring::nsACString, monitorTimeoutMs: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, jobName, savePathPrefix, monitorTimeoutMs)
    }


    /// ```text
    /// /**
    ///    * Downloads the specified URL to the specified location within the
    ///    * savePathPrefix passed to init().
    ///    *
    ///    * @param downloadURL
    ///    *        The URL to be downloaded.
    ///    * @param saveRelativePath
    ///    *        The location to download to. The path given should be a path
    ///    *        relative to the savePathPrefix passed to init(). If this attempts to
    ///    *        escape the directory specified by savePathPrefix, this call will
    ///    *        fail (ex: Don't pass "../filename").
    ///    * @param proxy
    ///    *        Specifies what proxy to use when downloading. Valid values are
    ///    *        listed below.
    ///    * @param noProgressTimeoutSecs
    ///    *        The number of seconds for the "no progress" timeout. After there has
    ///    *        been no download progress for this long, BITS will not retry the job
    ///    *        following a transient error, producing instead a permanent error.
    ///    * @param monitorIntervalMs
    ///    *        The number of milliseconds between download status notifications.
    ///    * @param observer
    ///    *        An observer to be notified of various events. OnStartRequest is
    ///    *        called once the BITS job has been created. OnStopRequest is called
    ///    *        when the file transfer has completed or when an error occurs. If
    ///    *        this object implements nsIProgressEventSink, then its OnProgress
    ///    *        method will be called as data is transferred.
    ///    *        IMPORTANT NOTE: When OnStopRequest is called, the download has
    ///    *                        completed, but nsIBitsRequest::complete() still
    ///    *                        needs to be called to save the file to the
    ///    *                        filesystem.
    ///    * @param context
    ///    *        User defined object forwarded to the observer's onProgress method.
    ///    *        This parameter, unlike others for this interface, can be passed a
    ///    *        null pointer.
    ///    * @param callback
    ///    *        The callback used to relay the response from BITS.
    ///    */
    /// ```
    ///

    /// `void startDownload (in AUTF8String downloadURL, in AUTF8String saveRelativePath, in nsProxyUsage proxy, in unsigned long noProgressTimeoutSecs, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback);`
    #[inline]
    pub unsafe fn StartDownload(&self, downloadURL: *const ::nsstring::nsACString, saveRelativePath: *const ::nsstring::nsACString, proxy: nsProxyUsage, noProgressTimeoutSecs: u32, monitorIntervalMs: u32, observer: *const nsIRequestObserver, context: *const nsISupports, callback: *const nsIBitsNewRequestCallback) -> ::nserror::nsresult {
        ((*self.vtable).StartDownload)(self, downloadURL, saveRelativePath, proxy, noProgressTimeoutSecs, monitorIntervalMs, observer, context, callback)
    }


    /// ```text
    /// /**
    ///    * Similar to startDownload, but connects to a BITS transfer that has already
    ///    * been started.
    ///    *
    ///    * @param id
    ///    *        The GUID of the download to monitor.
    ///    * @param monitorIntervalMs
    ///    *        The number of milliseconds between download status notifications.
    ///    * @param observer
    ///    *        An observer to be notified of various events. OnStartRequest is
    ///    *        called once the BITS job has been created. OnStopRequest is called
    ///    *        when the file transfer has completed or when an error occurs. If
    ///    *        this object implements nsIProgressEventSink, then its OnProgress
    ///    *        method will be called as data is transferred.
    ///    *        IMPORTANT NOTE: When OnStopRequest is called, the download has
    ///    *                        completed, but nsIBitsRequest::complete() still
    ///    *                        needs to be called to save the file to the
    ///    *                        filesystem.
    ///    * @param context
    ///    *        User defined object forwarded to the observer's onProgress method.
    ///    *        This parameter, unlike others for this interface, can be passed a
    ///    *        null pointer.
    ///    * @param callback
    ///    *        The callback used to relay the response from BITS.
    ///    */
    /// ```
    ///

    /// `void monitorDownload (in AUTF8String id, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback);`
    #[inline]
    pub unsafe fn MonitorDownload(&self, id: *const ::nsstring::nsACString, monitorIntervalMs: u32, observer: *const nsIRequestObserver, context: *const nsISupports, callback: *const nsIBitsNewRequestCallback) -> ::nserror::nsresult {
        ((*self.vtable).MonitorDownload)(self, id, monitorIntervalMs, observer, context, callback)
    }


}


/// `interface nsIBitsNewRequestCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBitsNewRequestCallback {
    vtable: *const nsIBitsNewRequestCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBitsNewRequestCallback.
unsafe impl XpCom for nsIBitsNewRequestCallback {
    const IID: nsIID = nsID(0xaa12e433, 0x5b9f, 0x452d,
        [0xb5, 0xc9, 0x84, 0x0a, 0x95, 0x41, 0x32, 0x8b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBitsNewRequestCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBitsNewRequestCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBitsNewRequestCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIBitsNewRequestCallback`.
    fn coerce_from(v: &nsIBitsNewRequestCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBitsNewRequestCallbackCoerce for nsIBitsNewRequestCallback {
    #[inline]
    fn coerce_from(v: &nsIBitsNewRequestCallback) -> &Self {
        v
    }
}

impl nsIBitsNewRequestCallback {
    /// Cast this `nsIBitsNewRequestCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBitsNewRequestCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBitsNewRequestCallback {
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
impl<T: nsISupportsCoerce> nsIBitsNewRequestCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBitsNewRequestCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBitsNewRequestCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBitsNewRequestCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void success (in nsIBitsRequest request); */
    pub Success: unsafe extern "system" fn (this: *const nsIBitsNewRequestCallback, request: *const nsIBitsRequest) -> ::nserror::nsresult,

    /* void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage); */
    pub Failure: unsafe extern "system" fn (this: *const nsIBitsNewRequestCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage) -> ::nserror::nsresult,

    /* void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode); */
    pub FailureNsresult: unsafe extern "system" fn (this: *const nsIBitsNewRequestCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode); */
    pub FailureHresult: unsafe extern "system" fn (this: *const nsIBitsNewRequestCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: i32) -> ::nserror::nsresult,

    /* void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage); */
    pub FailureString: unsafe extern "system" fn (this: *const nsIBitsNewRequestCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorMessage: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBitsNewRequestCallback {

    /// ```text
    /// /**
    ///  * This callback interface is for use by the nsIBits interface for returning
    ///  * results asynchronously to the caller.
    ///  */
    /// ```
    ///

    /// `void success (in nsIBitsRequest request);`
    #[inline]
    pub unsafe fn Success(&self, request: *const nsIBitsRequest) -> ::nserror::nsresult {
        ((*self.vtable).Success)(self, request)
    }



    /// `void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage);`
    #[inline]
    pub unsafe fn Failure(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage) -> ::nserror::nsresult {
        ((*self.vtable).Failure)(self, errorType, errorAction, errorStage)
    }



    /// `void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode);`
    #[inline]
    pub unsafe fn FailureNsresult(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).FailureNsresult)(self, errorType, errorAction, errorStage, errorCode)
    }



    /// `void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode);`
    #[inline]
    pub unsafe fn FailureHresult(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).FailureHresult)(self, errorType, errorAction, errorStage, errorCode)
    }



    /// `void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage);`
    #[inline]
    pub unsafe fn FailureString(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorMessage: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).FailureString)(self, errorType, errorAction, errorStage, errorMessage)
    }


}


/// `interface nsIBitsRequest : nsIRequest`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBitsRequest {
    vtable: *const nsIBitsRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBitsRequest.
unsafe impl XpCom for nsIBitsRequest {
    const IID: nsIID = nsID(0xab9da0e9, 0x06bf, 0x4e73,
        [0xbb, 0x1b, 0xc0, 0xf2, 0xea, 0x9e, 0xcc, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBitsRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBitsRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBitsRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIBitsRequest`.
    fn coerce_from(v: &nsIBitsRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBitsRequestCoerce for nsIBitsRequest {
    #[inline]
    fn coerce_from(v: &nsIBitsRequest) -> &Self {
        v
    }
}

impl nsIBitsRequest {
    /// Cast this `nsIBitsRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBitsRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBitsRequest {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestCoerce> nsIBitsRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBitsRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBitsRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBitsRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* readonly attribute AUTF8String bitsId; */
    pub GetBitsId: unsafe extern "system" fn (this: *const nsIBitsRequest, aBitsId: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsBitsErrorType transferError; */
    pub GetTransferError: unsafe extern "system" fn (this: *const nsIBitsRequest, aTransferError: *mut nsBitsErrorType) -> ::nserror::nsresult,

    /* void changeMonitorInterval (in unsigned long monitorIntervalMs, in nsIBitsCallback callback); */
    pub ChangeMonitorInterval: unsafe extern "system" fn (this: *const nsIBitsRequest, monitorIntervalMs: u32, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void cancelAsync (in nsresult status, in nsIBitsCallback callback); */
    pub CancelAsync: unsafe extern "system" fn (this: *const nsIBitsRequest, status: ::nserror::nsresult, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void setPriorityHigh (in nsIBitsCallback callback); */
    pub SetPriorityHigh: unsafe extern "system" fn (this: *const nsIBitsRequest, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void setPriorityLow (in nsIBitsCallback callback); */
    pub SetPriorityLow: unsafe extern "system" fn (this: *const nsIBitsRequest, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void setNoProgressTimeout (in unsigned long timeoutSecs, in nsIBitsCallback callback); */
    pub SetNoProgressTimeout: unsafe extern "system" fn (this: *const nsIBitsRequest, timeoutSecs: u32, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void complete (in nsIBitsCallback callback); */
    pub Complete: unsafe extern "system" fn (this: *const nsIBitsRequest, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void suspendAsync (in nsIBitsCallback callback); */
    pub SuspendAsync: unsafe extern "system" fn (this: *const nsIBitsRequest, callback: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void resumeAsync (in nsIBitsCallback callback); */
    pub ResumeAsync: unsafe extern "system" fn (this: *const nsIBitsRequest, callback: *const nsIBitsCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBitsRequest {

    /// ```text
    /// /**
    ///    * The BITS id of the download. This will be a string representing a UUID.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String bitsId;`
    #[inline]
    pub unsafe fn GetBitsId(&self, aBitsId: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBitsId)(self, aBitsId)
    }


    /// ```text
    /// /**
    ///    * The transfer result of the download, meant to be accessed after the
    ///    * transfer has stopped (i.e. after the observer's onStopRequest method has
        ///    * been called). Will be nsIBits::ERROR_TYPE_SUCCESS if the transfer is
    ///    * successful (and before transfer completion). If the transfer failed, this
    ///    * will be a different nsBitsErrorType value indicating the cause of the
    ///    * failure.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsBitsErrorType transferError;`
    #[inline]
    pub unsafe fn GetTransferError(&self, aTransferError: *mut nsBitsErrorType) -> ::nserror::nsresult {
        ((*self.vtable).GetTransferError)(self, aTransferError)
    }


    /// ```text
    /// /**
    ///    * Requests a change to the frequency that Firefox is receiving download
    ///    * status notifications.
    ///    *
    ///    * @param monitorIntervalMs
    ///    *        The new number of milliseconds between download status
    ///    *        notifications.
    ///    * @param callback
    ///    *        The callback function used to relay success or failure.
    ///    */
    /// ```
    ///

    /// `void changeMonitorInterval (in unsigned long monitorIntervalMs, in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn ChangeMonitorInterval(&self, monitorIntervalMs: u32, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).ChangeMonitorInterval)(self, monitorIntervalMs, callback)
    }


    /// ```text
    /// /**
    ///    * Cancels the download. This function is named this way to avoid conflict
    ///    * with nsIRequest::cancel.
    ///    *
    ///    * @param status
    ///    *        The reason for cancelling the request. This must be a failure code
    ///    *        rather than a success code like NS_OK.
    ///    * @param callback
    ///    *        The callback function used to relay success or failure.
    ///    */
    /// ```
    ///

    /// `void cancelAsync (in nsresult status, in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn CancelAsync(&self, status: ::nserror::nsresult, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).CancelAsync)(self, status, callback)
    }


    /// ```text
    /// /**
    ///    * Sets the priority of the BITS job to high (i.e. foreground download).
    ///    *
    ///    * @param callback
    ///    *        The callback function used to relay success or failure.
    ///    */
    /// ```
    ///

    /// `void setPriorityHigh (in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn SetPriorityHigh(&self, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetPriorityHigh)(self, callback)
    }


    /// ```text
    /// /**
    ///    * Sets the priority of the BITS job to low (i.e. background download).
    ///    *
    ///    * @param callback
    ///    *        The callback function used to relay success or failure.
    ///    */
    /// ```
    ///

    /// `void setPriorityLow (in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn SetPriorityLow(&self, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetPriorityLow)(self, callback)
    }


    /// ```text
    /// /**
    ///    * Sets the BITS "no progress" timeout for the job.
    ///    *
    ///    * @param timeoutSecs
    ///    *        The new number of seconds for the timeout. After there has been
    ///    *        no progress for this long, BITS will not retry the job following
    ///    *        a transient error, producing instead a permanent error.
    ///    * @param callback
    ///    *        The callback function used to relay success or failure.
    ///    */
    /// ```
    ///

    /// `void setNoProgressTimeout (in unsigned long timeoutSecs, in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn SetNoProgressTimeout(&self, timeoutSecs: u32, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetNoProgressTimeout)(self, timeoutSecs, callback)
    }



    /// `void complete (in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn Complete(&self, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, callback)
    }



    /// `void suspendAsync (in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn SuspendAsync(&self, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).SuspendAsync)(self, callback)
    }



    /// `void resumeAsync (in nsIBitsCallback callback);`
    #[inline]
    pub unsafe fn ResumeAsync(&self, callback: *const nsIBitsCallback) -> ::nserror::nsresult {
        ((*self.vtable).ResumeAsync)(self, callback)
    }


}


/// `interface nsIBitsCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBitsCallback {
    vtable: *const nsIBitsCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBitsCallback.
unsafe impl XpCom for nsIBitsCallback {
    const IID: nsIID = nsID(0xea657e66, 0x6bad, 0x4e41,
        [0x84, 0xd9, 0xc6, 0xd1, 0x07, 0xe9, 0x79, 0x9d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBitsCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBitsCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBitsCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIBitsCallback`.
    fn coerce_from(v: &nsIBitsCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBitsCallbackCoerce for nsIBitsCallback {
    #[inline]
    fn coerce_from(v: &nsIBitsCallback) -> &Self {
        v
    }
}

impl nsIBitsCallback {
    /// Cast this `nsIBitsCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBitsCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBitsCallback {
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
impl<T: nsISupportsCoerce> nsIBitsCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBitsCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBitsCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBitsCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void success (); */
    pub Success: unsafe extern "system" fn (this: *const nsIBitsCallback) -> ::nserror::nsresult,

    /* void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage); */
    pub Failure: unsafe extern "system" fn (this: *const nsIBitsCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage) -> ::nserror::nsresult,

    /* void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode); */
    pub FailureNsresult: unsafe extern "system" fn (this: *const nsIBitsCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode); */
    pub FailureHresult: unsafe extern "system" fn (this: *const nsIBitsCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: i32) -> ::nserror::nsresult,

    /* void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage); */
    pub FailureString: unsafe extern "system" fn (this: *const nsIBitsCallback, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorMessage: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBitsCallback {

    /// ```text
    /// /**
    ///  * This callback interface is for use by the nsIBitsRequest interface for
    ///  * returning results asynchronously to the caller.
    ///  */
    /// ```
    ///

    /// `void success ();`
    #[inline]
    pub unsafe fn Success(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Success)(self, )
    }



    /// `void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage);`
    #[inline]
    pub unsafe fn Failure(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage) -> ::nserror::nsresult {
        ((*self.vtable).Failure)(self, errorType, errorAction, errorStage)
    }



    /// `void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode);`
    #[inline]
    pub unsafe fn FailureNsresult(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).FailureNsresult)(self, errorType, errorAction, errorStage, errorCode)
    }



    /// `void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode);`
    #[inline]
    pub unsafe fn FailureHresult(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).FailureHresult)(self, errorType, errorAction, errorStage, errorCode)
    }



    /// `void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage);`
    #[inline]
    pub unsafe fn FailureString(&self, errorType: nsBitsErrorType, errorAction: nsBitsErrorAction, errorStage: nsBitsErrorStage, errorMessage: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).FailureString)(self, errorType, errorAction, errorStage, errorMessage)
    }


}


