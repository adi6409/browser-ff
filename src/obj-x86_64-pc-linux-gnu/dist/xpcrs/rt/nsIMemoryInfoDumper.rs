//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemoryInfoDumper.idl
//


/// `interface nsIFinishDumpingCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFinishDumpingCallback {
    vtable: *const nsIFinishDumpingCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFinishDumpingCallback.
unsafe impl XpCom for nsIFinishDumpingCallback {
    const IID: nsIID = nsID(0x2dea18fc, 0xfbfa, 0x4bf7,
        [0xad, 0x45, 0x0e, 0xfa, 0xf5, 0x49, 0x5f, 0x5e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFinishDumpingCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFinishDumpingCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFinishDumpingCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFinishDumpingCallback`.
    fn coerce_from(v: &nsIFinishDumpingCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFinishDumpingCallbackCoerce for nsIFinishDumpingCallback {
    #[inline]
    fn coerce_from(v: &nsIFinishDumpingCallback) -> &Self {
        v
    }
}

impl nsIFinishDumpingCallback {
    /// Cast this `nsIFinishDumpingCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFinishDumpingCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFinishDumpingCallback {
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
impl<T: nsISupportsCoerce> nsIFinishDumpingCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFinishDumpingCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFinishDumpingCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFinishDumpingCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (in nsISupports data); */
    pub Callback: unsafe extern "system" fn (this: *const nsIFinishDumpingCallback, data: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFinishDumpingCallback {


    /// `void callback (in nsISupports data);`
    #[inline]
    pub unsafe fn Callback(&self, data: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, data)
    }


}


/// `interface nsIDumpGCAndCCLogsCallback : nsISupports`
///

/// ```text
/// /**
///  * Callback interface for |dumpGCAndCCLogsToFile|, below.  Note that
///  * these method calls can occur before |dumpGCAndCCLogsToFile|
///  * returns.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDumpGCAndCCLogsCallback {
    vtable: *const nsIDumpGCAndCCLogsCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDumpGCAndCCLogsCallback.
unsafe impl XpCom for nsIDumpGCAndCCLogsCallback {
    const IID: nsIID = nsID(0xdc1b2b24, 0x65bd, 0x441b,
        [0xb6, 0xbd, 0xcb, 0x58, 0x25, 0xa7, 0xed, 0x14]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDumpGCAndCCLogsCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDumpGCAndCCLogsCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDumpGCAndCCLogsCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIDumpGCAndCCLogsCallback`.
    fn coerce_from(v: &nsIDumpGCAndCCLogsCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDumpGCAndCCLogsCallbackCoerce for nsIDumpGCAndCCLogsCallback {
    #[inline]
    fn coerce_from(v: &nsIDumpGCAndCCLogsCallback) -> &Self {
        v
    }
}

impl nsIDumpGCAndCCLogsCallback {
    /// Cast this `nsIDumpGCAndCCLogsCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDumpGCAndCCLogsCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDumpGCAndCCLogsCallback {
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
impl<T: nsISupportsCoerce> nsIDumpGCAndCCLogsCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDumpGCAndCCLogsCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDumpGCAndCCLogsCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDumpGCAndCCLogsCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent); */
    pub OnDump: unsafe extern "system" fn (this: *const nsIDumpGCAndCCLogsCallback, aGCLog: *const nsIFile, aCCLog: *const nsIFile, aIsParent: bool) -> ::nserror::nsresult,

    /* void onFinish (); */
    pub OnFinish: unsafe extern "system" fn (this: *const nsIDumpGCAndCCLogsCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDumpGCAndCCLogsCallback {

    /// ```text
    /// /**
    ///    * Called whenever a process has successfully finished dumping its GC/CC logs.
    ///    * Incomplete dumps (e.g., if the child crashes or is killed due to memory
        ///    * exhaustion) are not reported.
    ///    *
    ///    * @param aGCLog The file that the GC log was written to.
    ///    *
    ///    * @param aCCLog The file that the CC log was written to.
    ///    *
    ///    * @param aIsParent indicates whether this log file pair is from the
    ///    * parent process.
    ///    */
    /// ```
    ///

    /// `void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent);`
    #[inline]
    pub unsafe fn OnDump(&self, aGCLog: *const nsIFile, aCCLog: *const nsIFile, aIsParent: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnDump)(self, aGCLog, aCCLog, aIsParent)
    }


    /// ```text
    /// /**
    ///    * Called when GC/CC logging has finished, after all calls to |onDump|.
    ///    */
    /// ```
    ///

    /// `void onFinish ();`
    #[inline]
    pub unsafe fn OnFinish(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnFinish)(self, )
    }


}


/// `interface nsIMemoryInfoDumper : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMemoryInfoDumper {
    vtable: *const nsIMemoryInfoDumperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMemoryInfoDumper.
unsafe impl XpCom for nsIMemoryInfoDumper {
    const IID: nsIID = nsID(0x48541b74, 0x47ee, 0x4a62,
        [0x95, 0x57, 0x7f, 0x4b, 0x80, 0x9b, 0xda, 0x5c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMemoryInfoDumper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMemoryInfoDumper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMemoryInfoDumperCoerce {
    /// Cheaply cast a value of this type from a `nsIMemoryInfoDumper`.
    fn coerce_from(v: &nsIMemoryInfoDumper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMemoryInfoDumperCoerce for nsIMemoryInfoDumper {
    #[inline]
    fn coerce_from(v: &nsIMemoryInfoDumper) -> &Self {
        v
    }
}

impl nsIMemoryInfoDumper {
    /// Cast this `nsIMemoryInfoDumper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMemoryInfoDumperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMemoryInfoDumper {
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
impl<T: nsISupportsCoerce> nsIMemoryInfoDumperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryInfoDumper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMemoryInfoDumper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMemoryInfoDumperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
    pub DumpMemoryReportsToNamedFile: unsafe extern "system" fn (this: *const nsIMemoryInfoDumper, aFilename: *const ::nsstring::nsAString, aFinishDumping: *const nsIFinishDumpingCallback, aFinishDumpingData: *const nsISupports, aAnonymize: bool, aMinimizeMemoryUsage: bool) -> ::nserror::nsresult,

    /* void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
    pub DumpMemoryInfoToTempDir: unsafe extern "system" fn (this: *const nsIMemoryInfoDumper, aIdentifier: *const ::nsstring::nsAString, aAnonymize: bool, aMinimizeMemoryUsage: bool) -> ::nserror::nsresult,

    /* void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback); */
    pub DumpGCAndCCLogsToFile: unsafe extern "system" fn (this: *const nsIMemoryInfoDumper, aIdentifier: *const ::nsstring::nsAString, aDumpAllTraces: bool, aDumpChildProcesses: bool, aCallback: *const nsIDumpGCAndCCLogsCallback) -> ::nserror::nsresult,

    /* void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink); */
    pub DumpGCAndCCLogsToSink: unsafe extern "system" fn (this: *const nsIMemoryInfoDumper, aDumpAllTraces: bool, aSink: *const nsICycleCollectorLogSink) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMemoryInfoDumper {

    /// ```text
    /// /**
    ///    * This dumps gzipped memory reports for this process and its child
    ///    * processes.  If a file of the given name exists, it will be overwritten.
    ///    *
    ///    * @param aFilename The output file.
    ///    *
    ///    * @param aFinishDumping The callback called on completion.
    ///    *
    ///    * @param aFinishDumpingData The environment for the callback.
    ///    *
    ///    * @param aAnonymize Should the reports be anonymized?
    ///    *
    ///    * @param aMinimizeMemoryUsage indicates whether we should run a series of
    ///    *   GC/CC's in an attempt to reduce our memory usage before collecting our
    ///    *   memory report.
    ///    *
    ///    * Sample output, annotated with comments for explanatory purposes.
    ///    *
    ///    * {
        ///    *   // The version number of the format, which will be incremented each time
        ///    *   // backwards-incompatible changes are made. A mandatory integer.
        ///    *   "version": 1
        ///    *
        ///    *   // Equal to nsIMemoryReporterManager::hasMozMallocUsableSize. A
        ///    *   // mandatory boolean.
        ///    *   "hasMozMallocUsableSize": true,
        ///    *
        ///    *   // The memory reports. A mandatory array.
        ///    *   "reports": [
            ///    *     // The properties correspond to the arguments of
            ///    *     // nsIHandleReportCallback::callback. Every one is mandatory.
            ///    *     {"process":"Main Process (pid 12345)", "path":"explicit/foo/bar",
                ///    *      "kind":1, "units":0, "amount":2000000, "description":"Foo bar."},
            ///    *     {"process":"Main Process (pid 12345)", "path":"heap-allocated",
                ///    *      "kind":1, "units":0, "amount":3000000, "description":"Heap allocated."},
            ///    *     {"process":"Main Process (pid 12345)", "path":"vsize",
                ///    *      "kind":1, "units":0, "amount":10000000, "description":"Vsize."}
            ///    *   ]
        ///    * }
    ///    */
    /// ```
    ///

    /// `void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize, in boolean aMinimizeMemoryUsage);`
    #[inline]
    pub unsafe fn DumpMemoryReportsToNamedFile(&self, aFilename: *const ::nsstring::nsAString, aFinishDumping: *const nsIFinishDumpingCallback, aFinishDumpingData: *const nsISupports, aAnonymize: bool, aMinimizeMemoryUsage: bool) -> ::nserror::nsresult {
        ((*self.vtable).DumpMemoryReportsToNamedFile)(self, aFilename, aFinishDumping, aFinishDumpingData, aAnonymize, aMinimizeMemoryUsage)
    }


    /// ```text
    /// /**
    ///    * Similar to dumpMemoryReportsToNamedFile, this method dumps gzipped memory
    ///    * reports for this process and its child processes to files in the tmp
    ///    * directory called memory-reports-<identifier>-<pid>.json.gz (or something
        ///    * similar, such as memory-reports-<identifier>-<pid>-1.json.gz; no existing
        ///    * file will be overwritten).
    ///    *
    ///    * If DMD is enabled, this method also dumps gzipped DMD output for this
    ///    * process and its child processes to files in the tmp directory called
    ///    * dmd-<identifier>-<pid>.txt.gz (or something similar; again, no existing
        ///    * file will be overwritten).
    ///    *
    ///    * @param aIdentifier this identifier will appear in the filename of our
    ///    *   about:memory dump and those of our children.
    ///    *
    ///    *   If the identifier is empty, the implementation may set it arbitrarily
    ///    *   and use that new value for its own dump and the dumps of its child
    ///    *   processes.  For example, the implementation may set |aIdentifier| to the
    ///    *   number of seconds since the epoch.
    ///    *
    ///    * @param aAnonymize Should the reports be anonymized?
    ///    *
    ///    * @param aMinimizeMemoryUsage indicates whether we should run a series of
    ///    *   GC/CC's in an attempt to reduce our memory usage before collecting our
    ///    *   memory report.
    ///    */
    /// ```
    ///

    /// `void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage);`
    #[inline]
    pub unsafe fn DumpMemoryInfoToTempDir(&self, aIdentifier: *const ::nsstring::nsAString, aAnonymize: bool, aMinimizeMemoryUsage: bool) -> ::nserror::nsresult {
        ((*self.vtable).DumpMemoryInfoToTempDir)(self, aIdentifier, aAnonymize, aMinimizeMemoryUsage)
    }


    /// ```text
    /// /**
    ///    * Dump GC and CC logs to files in the OS's temp directory (or in
        ///    * $MOZ_CC_LOG_DIRECTORY, if that environment variable is specified).
    ///    *
    ///    * @param aIdentifier If aIdentifier is non-empty, this string will appear in
    ///    *   the filenames of the logs we create (both for this process and, if
        ///    *   aDumpChildProcesses is true, for our child processes).
    ///    *
    ///    *   If aIdentifier is empty, the implementation may set it to an
    ///    *   arbitrary value; for example, it may set aIdentifier to the number
    ///    *   of seconds since the epoch.
    ///    *
    ///    * @param aDumpAllTraces indicates whether we should run an all-traces CC
    ///    *   log.  An all-traces log visits all objects currently eligible for cycle
    ///    *   collection, while a non-all-traces log avoids visiting some objects
    ///    *   which we know are reachable.
    ///    *
    ///    *   All-traces logs are much bigger than the alternative, but they may be
    ///    *   helpful when trying to understand why a particular object is alive.  For
    ///    *   example, a non-traces-log will skip references held by an active
    ///    *   document; if your object is being held alive by such a document, you
    ///    *   probably want to see those references.
    ///    *
    ///    * @param aDumpChildProcesses indicates whether we should call
    ///    *   DumpGCAndCCLogsToFile in our child processes.  If so, the child processes
    ///    *   will dump their children, and so on.
    ///    *
    ///    */
    /// ```
    ///

    /// `void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback);`
    #[inline]
    pub unsafe fn DumpGCAndCCLogsToFile(&self, aIdentifier: *const ::nsstring::nsAString, aDumpAllTraces: bool, aDumpChildProcesses: bool, aCallback: *const nsIDumpGCAndCCLogsCallback) -> ::nserror::nsresult {
        ((*self.vtable).DumpGCAndCCLogsToFile)(self, aIdentifier, aDumpAllTraces, aDumpChildProcesses, aCallback)
    }


    /// ```text
    /// /**
    ///    * Like |dumpGCAndCCLogsToFile|, but sends the logs to the given log
    ///    * sink object instead of accessing the filesystem directly, and
    ///    * dumps the current process only.
    ///    */
    /// ```
    ///

    /// `void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink);`
    #[inline]
    pub unsafe fn DumpGCAndCCLogsToSink(&self, aDumpAllTraces: bool, aSink: *const nsICycleCollectorLogSink) -> ::nserror::nsresult {
        ((*self.vtable).DumpGCAndCCLogsToSink)(self, aDumpAllTraces, aSink)
    }


}


