//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsICrashReporter.idl
//


/// `interface nsICrashReporter : nsISupports`
///

/// ```text
/// /**
///  * Provides access to crash reporting functionality.
///  *
///  * @status UNSTABLE - This interface is not frozen and will probably change in
///  *                    future releases.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICrashReporter {
    vtable: *const nsICrashReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICrashReporter.
unsafe impl XpCom for nsICrashReporter {
    const IID: nsIID = nsID(0x4b74c39a, 0xcf69, 0x4a8a,
        [0x8e, 0x63, 0x16, 0x9d, 0x81, 0xad, 0x1e, 0xcf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICrashReporter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICrashReporter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICrashReporterCoerce {
    /// Cheaply cast a value of this type from a `nsICrashReporter`.
    fn coerce_from(v: &nsICrashReporter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICrashReporterCoerce for nsICrashReporter {
    #[inline]
    fn coerce_from(v: &nsICrashReporter) -> &Self {
        v
    }
}

impl nsICrashReporter {
    /// Cast this `nsICrashReporter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICrashReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICrashReporter {
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
impl<T: nsISupportsCoerce> nsICrashReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICrashReporter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICrashReporter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICrashReporterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean enabled; */
    pub GetEnabled: unsafe extern "system" fn (this: *const nsICrashReporter, aEnabled: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void setEnabled (in bool enabled); */
    pub SetEnabled: unsafe extern "system" fn (this: *const nsICrashReporter, enabled: bool) -> ::nserror::nsresult,

    /* attribute nsIURL serverURL; */
    pub GetServerURL: unsafe extern "system" fn (this: *const nsICrashReporter, aServerURL: *mut*const nsIURL) -> ::nserror::nsresult,

    /* attribute nsIURL serverURL; */
    pub SetServerURL: unsafe extern "system" fn (this: *const nsICrashReporter, aServerURL: *const nsIURL) -> ::nserror::nsresult,

    /* attribute nsIFile minidumpPath; */
    pub GetMinidumpPath: unsafe extern "system" fn (this: *const nsICrashReporter, aMinidumpPath: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsIFile minidumpPath; */
    pub SetMinidumpPath: unsafe extern "system" fn (this: *const nsICrashReporter, aMinidumpPath: *const nsIFile) -> ::nserror::nsresult,

    /* nsIFile getMinidumpForID (in AString id); */
    pub GetMinidumpForID: unsafe extern "system" fn (this: *const nsICrashReporter, id: *const ::nsstring::nsAString, _retval: *mut*const nsIFile) -> ::nserror::nsresult,

    /* nsIFile getExtraFileForID (in AString id); */
    pub GetExtraFileForID: unsafe extern "system" fn (this: *const nsICrashReporter, id: *const ::nsstring::nsAString, _retval: *mut*const nsIFile) -> ::nserror::nsresult,

    /* void annotateCrashReport (in AUTF8String key, in AUTF8String data); */
    pub AnnotateCrashReport: unsafe extern "system" fn (this: *const nsICrashReporter, key: *const ::nsstring::nsACString, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void removeCrashReportAnnotation (in AUTF8String key); */
    pub RemoveCrashReportAnnotation: unsafe extern "system" fn (this: *const nsICrashReporter, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean isAnnotationWhitelistedForPing (in ACString value); */
    pub IsAnnotationWhitelistedForPing: unsafe extern "system" fn (this: *const nsICrashReporter, value: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void appendAppNotesToCrashReport (in ACString data); */
    pub AppendAppNotesToCrashReport: unsafe extern "system" fn (this: *const nsICrashReporter, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void registerAppMemory (in unsigned long long ptr, in unsigned long long size); */
    pub RegisterAppMemory: unsafe extern "system" fn (this: *const nsICrashReporter, ptr: u64, size: u64) -> ::nserror::nsresult,

    /* [noscript] void writeMinidumpForException (in voidPtr aExceptionInfo); */
    pub WriteMinidumpForException: unsafe extern "system" fn (this: *const nsICrashReporter, aExceptionInfo: *const libc::c_void) -> ::nserror::nsresult,

    /* [noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException); */
    pub AppendObjCExceptionInfoToAppNotes: unsafe extern "system" fn (this: *const nsICrashReporter, aException: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute boolean submitReports; */
    pub GetSubmitReports: unsafe extern "system" fn (this: *const nsICrashReporter, aSubmitReports: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean submitReports; */
    pub SetSubmitReports: unsafe extern "system" fn (this: *const nsICrashReporter, aSubmitReports: bool) -> ::nserror::nsresult,

    /* void UpdateCrashEventsDir (); */
    pub UpdateCrashEventsDir: unsafe extern "system" fn (this: *const nsICrashReporter) -> ::nserror::nsresult,

    /* void saveMemoryReport (); */
    pub SaveMemoryReport: unsafe extern "system" fn (this: *const nsICrashReporter) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICrashReporter {

    /// ```text
    /// /**
    ///    * Get the enabled status of the crash reporter.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean enabled;`
    #[inline]
    pub unsafe fn GetEnabled(&self, aEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEnabled)(self, aEnabled)
    }


    /// ```text
    /// /**
    ///    * Enable or disable crash reporting at runtime. Not available to script
    ///    * because the JS engine relies on proper exception handler chaining.
    ///    */
    /// ```
    ///

    /// `[noscript] void setEnabled (in bool enabled);`
    #[inline]
    pub unsafe fn SetEnabled(&self, enabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEnabled)(self, enabled)
    }


    /// ```text
    /// /**
    ///    * Get or set the URL to which crash reports will be submitted.
    ///    * Only https and http URLs are allowed, as the submission is handled
    ///    * by OS-native networking libraries.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting is not initialized
    ///    * @throw NS_ERROR_INVALID_ARG on set if a non-http(s) URL is assigned
    ///    * @throw NS_ERROR_FAILURE on get if no URL is set
    ///    */
    /// ```
    ///

    /// `attribute nsIURL serverURL;`
    #[inline]
    pub unsafe fn GetServerURL(&self, aServerURL: *mut*const nsIURL) -> ::nserror::nsresult {
        ((*self.vtable).GetServerURL)(self, aServerURL)
    }


    /// ```text
    /// /**
    ///    * Get or set the URL to which crash reports will be submitted.
    ///    * Only https and http URLs are allowed, as the submission is handled
    ///    * by OS-native networking libraries.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting is not initialized
    ///    * @throw NS_ERROR_INVALID_ARG on set if a non-http(s) URL is assigned
    ///    * @throw NS_ERROR_FAILURE on get if no URL is set
    ///    */
    /// ```
    ///

    /// `attribute nsIURL serverURL;`
    #[inline]
    pub unsafe fn SetServerURL(&self, aServerURL: *const nsIURL) -> ::nserror::nsresult {
        ((*self.vtable).SetServerURL)(self, aServerURL)
    }


    /// ```text
    /// /**
    ///    * Get or set the path on the local system to which minidumps will be
    ///    * written when a crash happens.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting is not initialized
    ///    */
    /// ```
    ///

    /// `attribute nsIFile minidumpPath;`
    #[inline]
    pub unsafe fn GetMinidumpPath(&self, aMinidumpPath: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetMinidumpPath)(self, aMinidumpPath)
    }


    /// ```text
    /// /**
    ///    * Get or set the path on the local system to which minidumps will be
    ///    * written when a crash happens.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting is not initialized
    ///    */
    /// ```
    ///

    /// `attribute nsIFile minidumpPath;`
    #[inline]
    pub unsafe fn SetMinidumpPath(&self, aMinidumpPath: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetMinidumpPath)(self, aMinidumpPath)
    }


    /// ```text
    /// /**
    ///    * Get the minidump file corresponding to the specified ID.
    ///    *
    ///    * @param id
    ///    *        ID of the crash. Likely a UUID.
    ///    *
    ///    * @return The minidump file associated with the ID.
    ///    *
    ///    * @throw NS_ERROR_FILE_NOT_FOUND if the minidump could not be found
    ///    */
    /// ```
    ///

    /// `nsIFile getMinidumpForID (in AString id);`
    #[inline]
    pub unsafe fn GetMinidumpForID(&self, id: *const ::nsstring::nsAString, _retval: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetMinidumpForID)(self, id, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the extra file corresponding to the specified ID.
    ///    *
    ///    * @param id
    ///    *        ID of the crash. Likely a UUID.
    ///    *
    ///    * @return The extra file associated with the ID.
    ///    *
    ///    * @throw NS_ERROR_FILE_NOT_FOUND if the extra file could not be found
    ///    */
    /// ```
    ///

    /// `nsIFile getExtraFileForID (in AString id);`
    #[inline]
    pub unsafe fn GetExtraFileForID(&self, id: *const ::nsstring::nsAString, _retval: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetExtraFileForID)(self, id, _retval)
    }


    /// ```text
    /// /**
    ///    * Add some extra data to be submitted with a crash report.
    ///    *
    ///    * @param key
    ///    *        Name of a known crash annotation constant.
    ///    * @param data
    ///    *        Data to be added.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting not initialized
    ///    * @throw NS_ERROR_INVALID_ARG if key contains an invalid value or data
    ///    *                             contains invalid characters.  Invalid
    ///    *                             character for data is '\0'.
    ///    */
    /// ```
    ///

    /// `void annotateCrashReport (in AUTF8String key, in AUTF8String data);`
    #[inline]
    pub unsafe fn AnnotateCrashReport(&self, key: *const ::nsstring::nsACString, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AnnotateCrashReport)(self, key, data)
    }


    /// ```text
    /// /**
    ///    * Remove a crash report annotation.
    ///    *
    ///    * @param key
    ///    *        Name of a known crash annotation constant.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting not initialized
    ///    * @throw NS_ERROR_INVALID_ARG if key contains an invalid value.
    ///    */
    /// ```
    ///

    /// `void removeCrashReportAnnotation (in AUTF8String key);`
    #[inline]
    pub unsafe fn RemoveCrashReportAnnotation(&self, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCrashReportAnnotation)(self, key)
    }


    /// ```text
    /// /**
    ///    * Checks if an annotation is whitelisted for inclusion in the crash ping.
    ///    *
    ///    * @param key
    ///    *        Name of a known crash annotation constant.
    ///    *
    ///    * @return True if the specified value is a valid annotation and can be
    ///              included in the crash ping, false otherwise.
    ///    * @throw NS_ERROR_INVALID_ARG if key contains an invalid value.
    ///    */
    /// ```
    ///

    /// `boolean isAnnotationWhitelistedForPing (in ACString value);`
    #[inline]
    pub unsafe fn IsAnnotationWhitelistedForPing(&self, value: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsAnnotationWhitelistedForPing)(self, value, _retval)
    }


    /// ```text
    /// /**
    ///    * Append some data to the "Notes" field, to be submitted with a crash report.
    ///    * Unlike annotateCrashReport, this method will append to existing data.
    ///    *
    ///    * @param data
    ///    *        Data to be added.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting not initialized
    ///    * @throw NS_ERROR_INVALID_ARG if data contains invalid characters.
    ///    *                             The only invalid character is '\0'.
    ///    */
    /// ```
    ///

    /// `void appendAppNotesToCrashReport (in ACString data);`
    #[inline]
    pub unsafe fn AppendAppNotesToCrashReport(&self, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AppendAppNotesToCrashReport)(self, data)
    }


    /// ```text
    /// /**
    ///    * Register a given memory range to be included in the crash report.
    ///    *
    ///    * @param ptr
    ///    *        The starting address for the bytes.
    ///    * @param size
    ///    *        The number of bytes to include.
    ///    *
    ///    * @throw NS_ERROR_NOT_INITIALIZED if crash reporting not initialized
    ///    * @throw NS_ERROR_NOT_IMPLEMENTED if unavailable on the current OS
    ///    */
    /// ```
    ///

    /// `void registerAppMemory (in unsigned long long ptr, in unsigned long long size);`
    #[inline]
    pub unsafe fn RegisterAppMemory(&self, ptr: u64, size: u64) -> ::nserror::nsresult {
        ((*self.vtable).RegisterAppMemory)(self, ptr, size)
    }


    /// ```text
    /// /**
    ///    * Write a minidump immediately, with the user-supplied exception
    ///    * information. This is implemented on Windows only, because
    ///    * SEH (structured exception handling) exists on Windows only.
    ///    *
    ///    * @param aExceptionInfo  EXCEPTION_INFO* provided by Window's SEH
    ///    */
    /// ```
    ///

    /// `[noscript] void writeMinidumpForException (in voidPtr aExceptionInfo);`
    #[inline]
    pub unsafe fn WriteMinidumpForException(&self, aExceptionInfo: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).WriteMinidumpForException)(self, aExceptionInfo)
    }


    /// ```text
    /// /**
    ///    * Append note containing an Obj-C exception's info.
    ///    *
    ///    * @param aException  NSException object to append note for
    ///    */
    /// ```
    ///

    /// `[noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException);`
    #[inline]
    pub unsafe fn AppendObjCExceptionInfoToAppNotes(&self, aException: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).AppendObjCExceptionInfoToAppNotes)(self, aException)
    }


    /// ```text
    /// /**
    ///    * User preference for submitting crash reports.
    ///    */
    /// ```
    ///

    /// `attribute boolean submitReports;`
    #[inline]
    pub unsafe fn GetSubmitReports(&self, aSubmitReports: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSubmitReports)(self, aSubmitReports)
    }


    /// ```text
    /// /**
    ///    * User preference for submitting crash reports.
    ///    */
    /// ```
    ///

    /// `attribute boolean submitReports;`
    #[inline]
    pub unsafe fn SetSubmitReports(&self, aSubmitReports: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSubmitReports)(self, aSubmitReports)
    }


    /// ```text
    /// /**
    ///    * Cause the crash reporter to re-evaluate where crash events should go.
    ///    *
    ///    * This should be called during application startup and whenever profiles
    ///    * change.
    ///    */
    /// ```
    ///

    /// `void UpdateCrashEventsDir ();`
    #[inline]
    pub unsafe fn UpdateCrashEventsDir(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpdateCrashEventsDir)(self, )
    }


    /// ```text
    /// /**
    ///    * Save an anonymized memory report file for inclusion in a future crash
    ///    * report in this session.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if crash reporting is disabled.
    ///    */
    /// ```
    ///

    /// `void saveMemoryReport ();`
    #[inline]
    pub unsafe fn SaveMemoryReport(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SaveMemoryReport)(self, )
    }


}


