//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/telemetry/core/nsITelemetry.idl
//


/// `interface nsIFetchTelemetryDataCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFetchTelemetryDataCallback {
    vtable: *const nsIFetchTelemetryDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFetchTelemetryDataCallback.
unsafe impl XpCom for nsIFetchTelemetryDataCallback {
    const IID: nsIID = nsID(0x3d3b9075, 0x5549, 0x4244,
        [0x9c, 0x08, 0xb6, 0x4f, 0xef, 0xa1, 0xdd, 0x60]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFetchTelemetryDataCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFetchTelemetryDataCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFetchTelemetryDataCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFetchTelemetryDataCallback`.
    fn coerce_from(v: &nsIFetchTelemetryDataCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFetchTelemetryDataCallbackCoerce for nsIFetchTelemetryDataCallback {
    #[inline]
    fn coerce_from(v: &nsIFetchTelemetryDataCallback) -> &Self {
        v
    }
}

impl nsIFetchTelemetryDataCallback {
    /// Cast this `nsIFetchTelemetryDataCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFetchTelemetryDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFetchTelemetryDataCallback {
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
impl<T: nsISupportsCoerce> nsIFetchTelemetryDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFetchTelemetryDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFetchTelemetryDataCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFetchTelemetryDataCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (); */
    pub Complete: unsafe extern "system" fn (this: *const nsIFetchTelemetryDataCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFetchTelemetryDataCallback {


    /// `void complete ();`
    #[inline]
    pub unsafe fn Complete(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, )
    }


}


/// `interface nsITelemetry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITelemetry {
    vtable: *const nsITelemetryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITelemetry.
unsafe impl XpCom for nsITelemetry {
    const IID: nsIID = nsID(0x273d2dd0, 0x6c63, 0x475a,
        [0xb8, 0x64, 0xcb, 0x65, 0x16, 0x0a, 0x19, 0x09]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITelemetry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITelemetry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITelemetryCoerce {
    /// Cheaply cast a value of this type from a `nsITelemetry`.
    fn coerce_from(v: &nsITelemetry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITelemetryCoerce for nsITelemetry {
    #[inline]
    fn coerce_from(v: &nsITelemetry) -> &Self {
        v
    }
}

impl nsITelemetry {
    /// Cast this `nsITelemetry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITelemetryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITelemetry {
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
impl<T: nsISupportsCoerce> nsITelemetryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITelemetry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITelemetry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITelemetryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval getCategoricalLabels (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetCategoricalLabels: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSnapshotForHistograms ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetSnapshotForHistograms: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSnapshotForKeyedHistograms ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetSnapshotForKeyedHistograms: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSnapshotForScalars ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetSnapshotForScalars: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSnapshotForKeyedScalars ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetSnapshotForKeyedScalars: *const ::libc::c_void,

    /* readonly attribute uint32_t lastShutdownDuration; */
    pub GetLastShutdownDuration: unsafe extern "system" fn (this: *const nsITelemetry, aLastShutdownDuration: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t failedProfileLockCount; */
    pub GetFailedProfileLockCount: unsafe extern "system" fn (this: *const nsITelemetry, aFailedProfileLockCount: *mut uint32_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval slowSQL; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetSlowSQL: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval debugSlowSQL; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetDebugSlowSQL: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getUntrustedModuleLoadEvents ([optional] in unsigned long aFlags); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetUntrustedModuleLoadEvents: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getLoadedModules (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetLoadedModules: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval lateWrites; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetLateWrites: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getHistogramById (in ACString id); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetHistogramById: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getKeyedHistogramById (in ACString id); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetKeyedHistogramById: *const ::libc::c_void,

    /* attribute boolean canRecordBase; */
    pub GetCanRecordBase: unsafe extern "system" fn (this: *const nsITelemetry, aCanRecordBase: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean canRecordBase; */
    pub SetCanRecordBase: unsafe extern "system" fn (this: *const nsITelemetry, aCanRecordBase: bool) -> ::nserror::nsresult,

    /* attribute boolean canRecordExtended; */
    pub GetCanRecordExtended: unsafe extern "system" fn (this: *const nsITelemetry, aCanRecordExtended: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean canRecordExtended; */
    pub SetCanRecordExtended: unsafe extern "system" fn (this: *const nsITelemetry, aCanRecordExtended: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean canRecordReleaseData; */
    pub GetCanRecordReleaseData: unsafe extern "system" fn (this: *const nsITelemetry, aCanRecordReleaseData: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean canRecordPrereleaseData; */
    pub GetCanRecordPrereleaseData: unsafe extern "system" fn (this: *const nsITelemetry, aCanRecordPrereleaseData: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isOfficialTelemetry; */
    pub GetIsOfficialTelemetry: unsafe extern "system" fn (this: *const nsITelemetry, aIsOfficialTelemetry: *mut bool) -> ::nserror::nsresult,

    /* void setHistogramRecordingEnabled (in ACString id, in boolean enabled); */
    pub SetHistogramRecordingEnabled: unsafe extern "system" fn (this: *const nsITelemetry, id: *const ::nsstring::nsACString, enabled: bool) -> ::nserror::nsresult,

    /* void asyncFetchTelemetryData (in nsIFetchTelemetryDataCallback aCallback); */
    pub AsyncFetchTelemetryData: unsafe extern "system" fn (this: *const nsITelemetry, aCallback: *const nsIFetchTelemetryDataCallback) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval fileIOReports; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetFileIOReports: *const ::libc::c_void,

    /* double msSinceProcessStart (); */
    pub MsSinceProcessStart: unsafe extern "system" fn (this: *const nsITelemetry, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* double msSinceProcessStartIncludingSuspend (); */
    pub MsSinceProcessStartIncludingSuspend: unsafe extern "system" fn (this: *const nsITelemetry, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* double msSinceProcessStartExcludingSuspend (); */
    pub MsSinceProcessStartExcludingSuspend: unsafe extern "system" fn (this: *const nsITelemetry, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* double msSystemNow (); */
    pub MsSystemNow: unsafe extern "system" fn (this: *const nsITelemetry, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* [implicit_jscontext] void scalarAdd (in ACString aName, in jsval aValue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ScalarAdd: *const ::libc::c_void,

    /* [implicit_jscontext] void scalarSet (in ACString aName, in jsval aValue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ScalarSet: *const ::libc::c_void,

    /* [implicit_jscontext] void scalarSetMaximum (in ACString aName, in jsval aValue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ScalarSetMaximum: *const ::libc::c_void,

    /* [implicit_jscontext] void keyedScalarAdd (in ACString aName, in AString aKey, in jsval aValue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub KeyedScalarAdd: *const ::libc::c_void,

    /* [implicit_jscontext] void keyedScalarSet (in ACString aName, in AString aKey, in jsval aValue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub KeyedScalarSet: *const ::libc::c_void,

    /* [implicit_jscontext] void keyedScalarSetMaximum (in ACString aName, in AString aKey, in jsval aValue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub KeyedScalarSetMaximum: *const ::libc::c_void,

    /* void clearScalars (); */
    pub ClearScalars: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,

    /* void flushBatchedChildTelemetry (); */
    pub FlushBatchedChildTelemetry: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,

    /* [implicit_jscontext,optional_argc] void recordEvent (in ACString aCategory, in ACString aMethod, in ACString aObject, [optional] in jsval aValue, [optional] in jsval extra); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RecordEvent: *const ::libc::c_void,

    /* void setEventRecordingEnabled (in ACString aCategory, in boolean aEnabled); */
    pub SetEventRecordingEnabled: unsafe extern "system" fn (this: *const nsITelemetry, aCategory: *const ::nsstring::nsACString, aEnabled: bool) -> ::nserror::nsresult,

    /* [implicit_jscontext,optional_argc] jsval snapshotEvents (in uint32_t aDataset, [optional] in boolean aClear, [optional] in uint32_t aEventLimit); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub SnapshotEvents: *const ::libc::c_void,

    /* [implicit_jscontext] void registerEvents (in ACString aCategory, in jsval aEventData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RegisterEvents: *const ::libc::c_void,

    /* [implicit_jscontext] void registerBuiltinEvents (in ACString aCategory, in jsval aEventData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RegisterBuiltinEvents: *const ::libc::c_void,

    /* [implicit_jscontext] void registerScalars (in ACString aCategoryName, in jsval aScalarData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RegisterScalars: *const ::libc::c_void,

    /* [implicit_jscontext] void registerBuiltinScalars (in ACString aCategoryName, in jsval aScalarData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RegisterBuiltinScalars: *const ::libc::c_void,

    /* void clearEvents (); */
    pub ClearEvents: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getAllStores (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetAllStores: *const ::libc::c_void,

    /* void earlyInit (); */
    pub EarlyInit: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,

    /* void delayedInit (); */
    pub DelayedInit: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,

    /* void shutdown (); */
    pub Shutdown: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,

    /* [implicit_jscontext] Promise gatherMemory (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GatherMemory: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getOriginSnapshot ([optional] in boolean aClear); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetOriginSnapshot: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getEncodedOriginSnapshot ([optional] in boolean aClear); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetEncodedOriginSnapshot: *const ::libc::c_void,

    /* void clearOrigins (); */
    pub ClearOrigins: unsafe extern "system" fn (this: *const nsITelemetry) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITelemetry {
    /// ```text
    /// /**
    ///    * Histogram types:
    ///    * HISTOGRAM_EXPONENTIAL - buckets increase exponentially
    ///    * HISTOGRAM_LINEAR - buckets increase linearly
    ///    * HISTOGRAM_BOOLEAN - For storing 0/1 values
    ///    * HISTOGRAM_FLAG - For storing a single value; its count is always == 1.
    ///    * HISTOGRAM_COUNT - For storing counter values without bucketing.
    ///    * HISTOGRAM_CATEGORICAL - For storing enumerated values by label.
    ///    */
    /// ```
    ///

    pub const HISTOGRAM_EXPONENTIAL: i64 = 0;


    pub const HISTOGRAM_LINEAR: i64 = 1;


    pub const HISTOGRAM_BOOLEAN: i64 = 2;


    pub const HISTOGRAM_FLAG: i64 = 3;


    pub const HISTOGRAM_COUNT: i64 = 4;


    pub const HISTOGRAM_CATEGORICAL: i64 = 5;

    /// ```text
    /// /**
    ///    * Scalar types:
    ///    * SCALAR_TYPE_COUNT - for storing a numeric value
    ///    * SCALAR_TYPE_STRING - for storing a string value
    ///    * SCALAR_TYPE_BOOLEAN - for storing a boolean value
    ///    */
    /// ```
    ///

    pub const SCALAR_TYPE_COUNT: i64 = 0;


    pub const SCALAR_TYPE_STRING: i64 = 1;


    pub const SCALAR_TYPE_BOOLEAN: i64 = 2;

    /// ```text
    /// /**
    ///    * Dataset types:
    ///    * DATASET_ALL_CHANNELS - the basic dataset that is on-by-default on all channels
    ///    * DATASET_PRERELEASE_CHANNELS - the extended dataset that is opt-in on release,
    ///    *                                 opt-out on pre-release channels.
    ///    */
    /// ```
    ///

    pub const DATASET_ALL_CHANNELS: i64 = 0;


    pub const DATASET_PRERELEASE_CHANNELS: i64 = 1;

    /// ```text
    /// /**
    ///    * Flags for getUntrustedModuleLoadEvents.
    ///    */
    /// /**
    ///    * This flag is set to retrieve all data including instances which have been
    ///    * retrieved before.  If not set, only new instances since the last call
    ///    * will be returned.
    ///    * If this flag is set, KEEP_LOADEVENTS_NEW must not be set unless
    ///    * EXCLUDE_STACKINFO_FROM_LOADEVENTS is set.
    ///    * (See also MultiGetUntrustedModulesData::Serialize.)
    ///    */
    /// ```
    ///

    pub const INCLUDE_OLD_LOADEVENTS: i64 = 1;

    /// ```text
    /// /**
    ///    * This flag is set to keep the returned instances as if they were not
    ///    * retrieved, meaning those instances will be returned by a next method
    ///    * call without INCLUDE_OLD_LOADEVENTS.  If not set, the returned instances
    ///    * can be re-retrieved only when INCLUDE_OLD_LOADEVENTS is specified.
    ///    * If this flag is set, INCLUDE_OLD_LOADEVENTS must not be set unless
    ///    * EXCLUDE_STACKINFO_FROM_LOADEVENTS is set.
    ///    * (See also MultiGetUntrustedModulesData::Serialize.)
    ///    */
    /// ```
    ///

    pub const KEEP_LOADEVENTS_NEW: i64 = 2;

    /// ```text
    /// /**
    ///    * This flag is set to include private fields.
    ///    * Do not specify this flag to retrieve data to be submitted.
    ///    */
    /// ```
    ///

    pub const INCLUDE_PRIVATE_FIELDS_IN_LOADEVENTS: i64 = 4;

    /// ```text
    /// /**
    ///    * This flag is set to exclude the "combinedStacks" field.
    ///    * Without this flag, the flags INCLUDE_OLD_LOADEVENTS and KEEP_LOADEVENTS_NEW
    ///    * cannot be set at the same time.
    ///    */
    /// ```
    ///

    pub const EXCLUDE_STACKINFO_FROM_LOADEVENTS: i64 = 8;

    /// ```text
    /// /**
    ///    * Serializes the histogram labels for categorical hitograms.
    ///    * The returned structure looks like:
    ///    *   { "histogram1": [ "histogram1_label1", "histogram1_label2", ...],
        ///    *     "histogram2": [ "histogram2_label1", "histogram2_label2", ...]
        ///    *     ...
        ///    *     }
    ///    *
    ///    * Note that this function should only be used in tests and about:telemetry.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getCategoricalLabels ();`
    const _GetCategoricalLabels: () = ();

    /// ```text
    /// /**
    ///    * Serializes the histograms from the given store to a JSON-style object.
    ///    * The returned structure looks like:
    ///    *   { "process": { "name1": histogramData1, "name2": histogramData2 }, ... }
    ///    *
    ///    * Each histogram is represented in a packed format and has the following properties:
    ///    *   bucket_count - Number of buckets of this histogram
    ///    *   histogram_type - HISTOGRAM_EXPONENTIAL, HISTOGRAM_LINEAR, HISTOGRAM_BOOLEAN,
    ///    *                    HISTOGRAM_FLAG, HISTOGRAM_COUNT, or HISTOGRAM_CATEGORICAL
    ///    *   sum - sum of the bucket contents
    ///    *   range - A 2-item array of minimum and maximum bucket size
    ///    *   values - Map from bucket to the bucket's count
    ///    *
    ///    * @param aStoreName The name of the store to snapshot.
    ///    *                   Defaults to "main".
    ///    *                   Custom stores are available when probes have them defined.
    ///    *                   See the `record_into_store` attribute on histograms.
    ///    *                   @see https://firefox-source-docs.mozilla.org/toolkit/components/telemetry/telemetry/collection/histograms.html#record-into-store
    ///    * @param aClearStore Whether to clear out the histograms in the named store after snapshotting.
    ///    *                    Defaults to false.
    ///    * @param aFilterTest If true, `TELEMETRY_TEST_` histograms will be filtered out.
    ///                         Filtered histograms are still cleared if `aClearStore` is true.
    ///    *                    Defaults to false.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getSnapshotForHistograms ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest);`
    const _GetSnapshotForHistograms: () = ();

    /// ```text
    /// /**
    ///    * Serializes the keyed histograms from the given store to a JSON-style object.
    ///    * The returned structure looks like:
    ///    *   { "process": { "name1": { "key_1": histogramData1, "key_2": histogramData2 }, ...}, ... }
    ///    *
    ///    * @param aStoreName The name of the store to snapshot.
    ///    *                   Defaults to "main".
    ///    *                   Custom stores are available when probes have them defined.
    ///    *                   See the `record_into_store` attribute on histograms.
    ///    *                   @see https://firefox-source-docs.mozilla.org/toolkit/components/telemetry/telemetry/collection/histograms.html#record-into-store
    ///    * @param aClearStore Whether to clear out the keyed histograms in the named store after snapshotting.
    ///    *                    Defaults to false.
    ///    * @param aFilterTest If true, `TELEMETRY_TEST_` histograms will be filtered out.
    ///                         Filtered histograms are still cleared if `aClearStore` is true.
    ///    *                    Defaults to false.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getSnapshotForKeyedHistograms ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest);`
    const _GetSnapshotForKeyedHistograms: () = ();

    /// ```text
    /// /**
    ///    * Serializes the scalars from the given store to a JSON-style object.
    ///    * The returned structure looks like:
    ///    *   { "process": { "category1.probe": 1,"category1.other_probe": false, ... }, ... }.
    ///    *
    ///    * @param aStoreName The name of the store to snapshot.
    ///    *                   Defaults to "main".
    ///    *                   Custom stores are available when probes have them defined.
    ///    *                   See the `record_into_store` attribute on scalars.
    ///    *                   @see https://firefox-source-docs.mozilla.org/toolkit/components/telemetry/telemetry/collection/scalars.html#optional-fields
    ///    * @param aClearStore Whether to clear out the scalars in the named store after snapshotting.
    ///    *                    Defaults to false.
    ///    * @param aFilterTest If true, `telemetry.test` scalars will be filtered out.
    ///                         Filtered scalars are still cleared if `aClearStore` is true.
    ///    *                    Defaults to false.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getSnapshotForScalars ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest);`
    const _GetSnapshotForScalars: () = ();

    /// ```text
    /// /**
    ///    * Serializes the keyed scalars from the given store to a JSON-style object.
    ///    * The returned structure looks like:
    ///    *   { "process": { "category1.probe": { "key_1": 2, "key_2": 1, ... }, ... }, ... }
    ///    *
    ///    * @param aStoreName The name of the store to snapshot.
    ///    *                   Defaults to "main".
    ///    *                   Custom stores are available when probes have them defined.
    ///    *                   See the `record_into_store` attribute on scalars.
    ///    *                   @see https://firefox-source-docs.mozilla.org/toolkit/components/telemetry/telemetry/collection/scalars.html#optional-fields
    ///    * @param aClearStore Whether to clear out the keyed scalars in the named store after snapshotting.
    ///    *                    Defaults to false.
    ///    * @param aFilterTest If true, `telemetry.test` scalars will be filtered out.
    ///                         Filtered scalars are still cleared if `aClearStore` is true.
    ///    *                    Defaults to false.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getSnapshotForKeyedScalars ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest);`
    const _GetSnapshotForKeyedScalars: () = ();

    /// ```text
    /// /**
    ///    * The amount of time, in milliseconds, that the last session took
    ///    * to shutdown.  Reads as 0 to indicate failure.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t lastShutdownDuration;`
    #[inline]
    pub unsafe fn GetLastShutdownDuration(&self, aLastShutdownDuration: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastShutdownDuration)(self, aLastShutdownDuration)
    }


    /// ```text
    /// /**
    ///    * The number of failed profile lock attempts that have occurred prior to
    ///    * successfully locking the profile
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t failedProfileLockCount;`
    #[inline]
    pub unsafe fn GetFailedProfileLockCount(&self, aFailedProfileLockCount: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFailedProfileLockCount)(self, aFailedProfileLockCount)
    }



    /// `[implicit_jscontext] readonly attribute jsval slowSQL;`
    const _GetSlowSQL: () = ();


    /// `[implicit_jscontext] readonly attribute jsval debugSlowSQL;`
    const _GetDebugSlowSQL: () = ();


    /// `[implicit_jscontext] Promise getUntrustedModuleLoadEvents ([optional] in unsigned long aFlags);`
    const _GetUntrustedModuleLoadEvents: () = ();


    /// `[implicit_jscontext] Promise getLoadedModules ();`
    const _GetLoadedModules: () = ();


    /// `[implicit_jscontext] readonly attribute jsval lateWrites;`
    const _GetLateWrites: () = ();

    /// ```text
    /// /**
    ///    * Create and return a histogram registered in TelemetryHistograms.h.
    ///    *
    ///    * @param id - unique identifier from TelemetryHistograms.h
    ///    * The returned object has the following functions:
    ///    *   add(value) - Adds a sample of `value` to the histogram.
    ///                     `value` may be a categorical histogram's label as a string,
    ///                     a boolean histogram's value as a boolean,
    ///                     or a number that fits inside a uint32_t.
    ///    *   snapshot([optional] {store}) - Returns a snapshot of the histogram with the same data fields
    ///                                       as in getSnapshotForHistograms().
    ///                                       Defaults to the "main" store.
    ///    *   clear([optional] {store}) - Zeros out the histogram's buckets and sum.
    ///                                    Defaults to the "main" store.
    ///                                    Note: This is intended to be only used in tests.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getHistogramById (in ACString id);`
    const _GetHistogramById: () = ();

    /// ```text
    /// /**
    ///    * Create and return a histogram registered in TelemetryHistograms.h.
    ///    *
    ///    * @param id - unique identifier from TelemetryHistograms.h
    ///    * The returned object has the following functions:
    ///    *   add(string key, [optional] value) - Adds a sample of `value` to the histogram for that key.
    ///                                          If no histogram for that key exists yet, it is created.
    ///                                          `value` may be a categorical histogram's label as a string,
    ///                                          a boolean histogram's value as a boolean,
    ///                                          or a number that fits inside a uint32_t.
    ///    *   snapshot([optional] {store}) - Returns the snapshots of all the registered keys in the form
    ///                                       {key1: snapshot1, key2: snapshot2, ...} in the specified store.
    ///    *                                  Defaults to the "main" store.
    ///    *   keys([optional] {store}) - Returns an array with the string keys of the currently registered
    ///                                   histograms in the given store.
    ///                                   Defaults to "main".
    ///    *   clear([optional] {store}) - Clears the registered histograms from this.
    ///    *                               Defaults to the "main" store.
    ///    *                               Note: This is intended to be only used in tests.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getKeyedHistogramById (in ACString id);`
    const _GetKeyedHistogramById: () = ();

    /// ```text
    /// /**
    ///    * A flag indicating if Telemetry can record base data (FHR data). This is true if the
    ///    * FHR data reporting service or self-support are enabled.
    ///    *
    ///    * In the unlikely event that adding a new base probe is needed, please check the data
    ///    * collection wiki at https://wiki.mozilla.org/Firefox/Data_Collection and talk to the
    ///    * Telemetry team.
    ///    */
    /// ```
    ///

    /// `attribute boolean canRecordBase;`
    #[inline]
    pub unsafe fn GetCanRecordBase(&self, aCanRecordBase: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanRecordBase)(self, aCanRecordBase)
    }


    /// ```text
    /// /**
    ///    * A flag indicating if Telemetry can record base data (FHR data). This is true if the
    ///    * FHR data reporting service or self-support are enabled.
    ///    *
    ///    * In the unlikely event that adding a new base probe is needed, please check the data
    ///    * collection wiki at https://wiki.mozilla.org/Firefox/Data_Collection and talk to the
    ///    * Telemetry team.
    ///    */
    /// ```
    ///

    /// `attribute boolean canRecordBase;`
    #[inline]
    pub unsafe fn SetCanRecordBase(&self, aCanRecordBase: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCanRecordBase)(self, aCanRecordBase)
    }


    /// ```text
    /// /**
    ///    * A flag indicating if Telemetry is allowed to record extended data. Returns false if
    ///    * the user hasn't opted into "extended Telemetry" on the Release channel, when the
    ///    * user has explicitly opted out of Telemetry on Nightly/Aurora/Beta or if manually
    ///    * set to false during tests.
    ///    *
    ///    * Set this to false in tests to disable gathering of extended telemetry statistics.
    ///    */
    /// ```
    ///

    /// `attribute boolean canRecordExtended;`
    #[inline]
    pub unsafe fn GetCanRecordExtended(&self, aCanRecordExtended: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanRecordExtended)(self, aCanRecordExtended)
    }


    /// ```text
    /// /**
    ///    * A flag indicating if Telemetry is allowed to record extended data. Returns false if
    ///    * the user hasn't opted into "extended Telemetry" on the Release channel, when the
    ///    * user has explicitly opted out of Telemetry on Nightly/Aurora/Beta or if manually
    ///    * set to false during tests.
    ///    *
    ///    * Set this to false in tests to disable gathering of extended telemetry statistics.
    ///    */
    /// ```
    ///

    /// `attribute boolean canRecordExtended;`
    #[inline]
    pub unsafe fn SetCanRecordExtended(&self, aCanRecordExtended: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCanRecordExtended)(self, aCanRecordExtended)
    }


    /// ```text
    /// /**
    ///    * A flag indicating whether Telemetry is recording release data, which is a
    ///    * smallish subset of our usage data that we're prepared to handle from our
    ///    * largish release population.
    ///    *
    ///    * This is true most of the time.
    ///    *
    ///    * This does not indicate whether Telemetry will send any data. That is
    ///    * governed by user preference and other mechanisms.
    ///    *
    ///    * You may use this to determine if it's okay to record your data.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canRecordReleaseData;`
    #[inline]
    pub unsafe fn GetCanRecordReleaseData(&self, aCanRecordReleaseData: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanRecordReleaseData)(self, aCanRecordReleaseData)
    }


    /// ```text
    /// /**
    ///    * A flag indicating whether Telemetry is recording prerelease data, which is
    ///    * a largish amount of usage data that we're prepared to handle from our
    ///    * smallish pre-release population.
    ///    *
    ///    * This is true on pre-release branches of Firefox.
    ///    *
    ///    * This does not indicate whether Telemetry will send any data. That is
    ///    * governed by user preference and other mechanisms.
    ///    *
    ///    * You may use this to determine if it's okay to record your data.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canRecordPrereleaseData;`
    #[inline]
    pub unsafe fn GetCanRecordPrereleaseData(&self, aCanRecordPrereleaseData: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanRecordPrereleaseData)(self, aCanRecordPrereleaseData)
    }


    /// ```text
    /// /**
    ///    * A flag indicating whether Telemetry can submit official results (for base or extended
        ///    * data). This is true on official, non-debug builds with built in support for Mozilla
    ///    * Telemetry reporting.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isOfficialTelemetry;`
    #[inline]
    pub unsafe fn GetIsOfficialTelemetry(&self, aIsOfficialTelemetry: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsOfficialTelemetry)(self, aIsOfficialTelemetry)
    }


    /// ```text
    /// /**
    ///    * Enable/disable recording for this histogram at runtime.
    ///    * Recording is enabled by default, unless listed at kRecordingInitiallyDisabledIDs[].
    ///    * Name must be a valid Histogram identifier, otherwise an assertion will be triggered.
    ///    *
    ///    * @param id - unique identifier from histograms.json
    ///    * @param enabled - whether or not to enable recording from now on.
    ///    */
    /// ```
    ///

    /// `void setHistogramRecordingEnabled (in ACString id, in boolean enabled);`
    #[inline]
    pub unsafe fn SetHistogramRecordingEnabled(&self, id: *const ::nsstring::nsACString, enabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHistogramRecordingEnabled)(self, id, enabled)
    }


    /// ```text
    /// /**
    ///    * Read data from the previous run. After the callback is called, the last
    ///    * shutdown time is available in lastShutdownDuration and any late
    ///    * writes in lateWrites.
    ///    */
    /// ```
    ///

    /// `void asyncFetchTelemetryData (in nsIFetchTelemetryDataCallback aCallback);`
    #[inline]
    pub unsafe fn AsyncFetchTelemetryData(&self, aCallback: *const nsIFetchTelemetryDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncFetchTelemetryData)(self, aCallback)
    }


    /// ```text
    /// /**
    ///    * Get statistics of file IO reports, null, if not recorded.
    ///    *
    ///    * The statistics are returned as an object whose propoerties are the names
    ///    * of the files that have been accessed and whose corresponding values are
    ///    * arrays of size three, representing startup, normal, and shutdown stages.
    ///    * Each stage's entry is either null or an array with the layout
    ///    * [total_time, #creates, #reads, #writes, #fsyncs, #stats]
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval fileIOReports;`
    const _GetFileIOReports: () = ();

    /// ```text
    /// /**
    ///    * Return the number of milliseconds since process start using monotonic
    ///    * timestamps (unaffected by system clock changes). On Windows, this includes
    ///    * the period of time the device was suspended. On Linux and macOS, this does
    ///    * not include the period of time the device was suspneded.
    ///    */
    /// ```
    ///

    /// `double msSinceProcessStart ();`
    #[inline]
    pub unsafe fn MsSinceProcessStart(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).MsSinceProcessStart)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the number of milliseconds since process start using monotonic
    ///    * timestamps (unaffected by system clock changes), including the periods of
    ///    * time the device was suspended.
    ///    * @throws NS_ERROR_NOT_AVAILABLE if unavailable.
    ///    */
    /// ```
    ///

    /// `double msSinceProcessStartIncludingSuspend ();`
    #[inline]
    pub unsafe fn MsSinceProcessStartIncludingSuspend(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).MsSinceProcessStartIncludingSuspend)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the number of milliseconds since process start using monotonic
    ///    * timestamps (unaffected by system clock changes), excluding the periods of
    ///    * time the device was suspended.
    ///    * @throws NS_ERROR_NOT_AVAILABLE if unavailable.
    ///    */
    /// ```
    ///

    /// `double msSinceProcessStartExcludingSuspend ();`
    #[inline]
    pub unsafe fn MsSinceProcessStartExcludingSuspend(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).MsSinceProcessStartExcludingSuspend)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Time since the system wide epoch. This is not a monotonic timer but
    ///    * can be used across process boundaries.
    ///    */
    /// ```
    ///

    /// `double msSystemNow ();`
    #[inline]
    pub unsafe fn MsSystemNow(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).MsSystemNow)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds the value to the given scalar.
    ///    *
    ///    * @param aName The scalar name.
    ///    * @param aValue The numeric value to add to the scalar. Only unsigned integers supported.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void scalarAdd (in ACString aName, in jsval aValue);`
    const _ScalarAdd: () = ();

    /// ```text
    /// /**
    ///    * Sets the scalar to the given value.
    ///    *
    ///    * @param aName The scalar name.
    ///    * @param aValue The value to set the scalar to. If the type of aValue doesn't match the
    ///    *        type of the scalar, the function will fail. For scalar string types, the this
    ///    *        is truncated to 50 characters.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void scalarSet (in ACString aName, in jsval aValue);`
    const _ScalarSet: () = ();

    /// ```text
    /// /**
    ///    * Sets the scalar to the maximum of the current and the passed value.
    ///    *
    ///    * @param aName The scalar name.
    ///    * @param aValue The numeric value to set the scalar to. Only unsigned integers supported.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void scalarSetMaximum (in ACString aName, in jsval aValue);`
    const _ScalarSetMaximum: () = ();

    /// ```text
    /// /**
    ///    * Adds the value to the given keyed scalar.
    ///    *
    ///    * @param aName The scalar name.
    ///    * @param aKey The key name.
    ///    * @param aValue The numeric value to add to the scalar. Only unsigned integers supported.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void keyedScalarAdd (in ACString aName, in AString aKey, in jsval aValue);`
    const _KeyedScalarAdd: () = ();

    /// ```text
    /// /**
    ///    * Sets the keyed scalar to the given value.
    ///    *
    ///    * @param aName The scalar name.
    ///    * @param aKey The key name.
    ///    * @param aValue The value to set the scalar to. If the type of aValue doesn't match the
    ///    *        type of the scalar, the function will fail.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void keyedScalarSet (in ACString aName, in AString aKey, in jsval aValue);`
    const _KeyedScalarSet: () = ();

    /// ```text
    /// /**
    ///    * Sets the keyed scalar to the maximum of the current and the passed value.
    ///    *
    ///    * @param aName The scalar name.
    ///    * @param aKey The key name.
    ///    * @param aValue The numeric value to set the scalar to. Only unsigned integers supported.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void keyedScalarSetMaximum (in ACString aName, in AString aKey, in jsval aValue);`
    const _KeyedScalarSetMaximum: () = ();

    /// ```text
    /// /**
    ///    * Resets all the stored scalars. This is intended to be only used in tests.
    ///    */
    /// ```
    ///

    /// `void clearScalars ();`
    #[inline]
    pub unsafe fn ClearScalars(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearScalars)(self, )
    }


    /// ```text
    /// /**
    ///    * Immediately sends any Telemetry batched on this process to the parent
    ///    * process. This is intended only to be used on process shutdown.
    ///    */
    /// ```
    ///

    /// `void flushBatchedChildTelemetry ();`
    #[inline]
    pub unsafe fn FlushBatchedChildTelemetry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FlushBatchedChildTelemetry)(self, )
    }


    /// ```text
    /// /**
    ///    * Record an event in Telemetry.
    ///    *
    ///    * @param aCategory The category name.
    ///    * @param aMethod The method name.
    ///    * @param aObject The object name.
    ///    * @param aValue An optional string value to record.
    ///    * @param aExtra An optional object of the form (string -> string).
    ///    *               It should only contain registered extra keys.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG When trying to record an unknown event.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,optional_argc] void recordEvent (in ACString aCategory, in ACString aMethod, in ACString aObject, [optional] in jsval aValue, [optional] in jsval extra);`
    const _RecordEvent: () = ();

    /// ```text
    /// /**
    ///    * Enable recording of events in a category.
    ///    * Events default to recording disabled. This allows to toggle recording for all events
    ///    * in the specified category.
    ///    *
    ///    * @param aCategory The category name.
    ///    * @param aEnabled Whether recording is enabled for events in that category.
    ///    */
    /// ```
    ///

    /// `void setEventRecordingEnabled (in ACString aCategory, in boolean aEnabled);`
    #[inline]
    pub unsafe fn SetEventRecordingEnabled(&self, aCategory: *const ::nsstring::nsACString, aEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEventRecordingEnabled)(self, aCategory, aEnabled)
    }


    /// ```text
    /// /**
    ///    * Serializes the recorded events to a JSON-appropriate array and optionally resets them.
    ///    * The returned structure looks like this:
    ///    *   [
        ///    *     // [timestamp, category, method, object, stringValue, extraValues]
        ///    *     [43245, "category1", "method1", "object1", "string value", null],
        ///    *     [43258, "category1", "method2", "object1", null, {"key1": "string value"}],
        ///    *     ...
        ///    *   ]
    ///    *
    ///    * @param aDataset DATASET_ALL_CHANNELS or DATASET_PRERELEASE_CHANNELS.
    ///    * @param [aClear=false] Whether to clear out the flushed events after snapshotting.
    ///    * @param aEventLimit How many events per process to limit the snapshot to contain, all if unspecified.
    ///    *                    Even if aClear, the leftover event records are not cleared.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,optional_argc] jsval snapshotEvents (in uint32_t aDataset, [optional] in boolean aClear, [optional] in uint32_t aEventLimit);`
    const _SnapshotEvents: () = ();

    /// ```text
    /// /**
    ///    * Register new events to record them from addons. This allows registering multiple
    ///    * events for a category. They will be valid only for the current Firefox session.
    ///    * Note that events shipping in Firefox should be registered in Events.yaml.
    ///    *
    ///    * @param aCategory The unique category the events are registered in.
    ///    * @param aEventData An object that contains registration data for 1-N events of the form:
    ///    *   {
        ///    *     "categoryName": {
            ///    *       "methods": ["test1"],
            ///    *       "objects": ["object1"],
            ///    *       "record_on_release": false,
            ///    *       "extra_keys": ["key1", "key2"], // optional
            ///    *       "expired": false // optional, defaults to false.
            ///    *     },
        ///    *     ...
        ///    *   }
    ///    * @param aEventData.<name>.methods List of methods for this event entry.
    ///    * @param aEventData.<name>.objects List of objects for this event entry.
    ///    * @param aEventData.<name>.extra_keys Optional, list of allowed extra keys for this event entry.
    ///    * @param aEventData.<name>.record_on_release Optional, whether to record this data on release.
    ///    *                                            Defaults to false.
    ///    * @param aEventData.<name>.expired Optional, whether this event entry is expired. This allows
    ///    *                                  recording it without error, but it will be discarded. Defaults to false.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void registerEvents (in ACString aCategory, in jsval aEventData);`
    const _RegisterEvents: () = ();

    /// ```text
    /// /**
    ///    * Parent process only. Register dynamic builtin events. The parameters
    ///    * have the same meaning as the usual |registerEvents| function.
    ///    *
    ///    * This function is only meant to be used to support the "artifact build"/
    ///    * "build faster" developers by allowing to add new events without rebuilding
    ///    * the C++ components including the headers files.
    ///   */
    /// ```
    ///

    /// `[implicit_jscontext] void registerBuiltinEvents (in ACString aCategory, in jsval aEventData);`
    const _RegisterBuiltinEvents: () = ();

    /// ```text
    /// /**
    ///    * Parent process only. Register new scalars to record them from addons. This
    ///    * allows registering multiple scalars for a category. They will be valid only for
    ///    * the current Firefox session.
    ///    * Note that scalars shipping in Firefox should be registered in Scalars.yaml.
    ///    *
    ///    * @param aCategoryName The unique category the scalars are registered in.
    ///    * @param aScalarData An object that contains registration data for multiple scalars in the form:
    ///    *   {
        ///    *     "sample_scalar": {
            ///    *       "kind": Ci.nsITelemetry.SCALAR_TYPE_COUNT,
            ///    *       "keyed": true, //optional, defaults to false
            ///    *       "record_on_release: true, // optional, defaults to false
            ///    *       "expired": false // optional, defaults to false.
            ///    *     },
        ///    *     ...
        ///    *   }
    ///    * @param aScalarData.<name>.kind One of the scalar types defined in this file (SCALAR_TYPE_*)
    ///    * @param aScalarData.<name>.keyed Optional, whether this is a keyed scalar or not. Defaults to false.
    ///    * @param aScalarData.<name>.record_on_release Optional, whether to record this data on release.
    ///    *                                             Defaults to false.
    ///    * @param aScalarData.<name>.expired Optional, whether this scalar entry is expired. This allows
    ///    *                                   recording it without error, but it will be discarded. Defaults to false.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void registerScalars (in ACString aCategoryName, in jsval aScalarData);`
    const _RegisterScalars: () = ();

    /// ```text
    /// /**
    ///    * Parent process only. Register dynamic builtin scalars. The parameters
    ///    * have the same meaning as the usual |registerScalars| function.
    ///    *
    ///    * This function is only meant to be used to support the "artifact build"/
    ///    * "build faster" developers by allowing to add new scalars without rebuilding
    ///    * the C++ components including the headers files.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void registerBuiltinScalars (in ACString aCategoryName, in jsval aScalarData);`
    const _RegisterBuiltinScalars: () = ();

    /// ```text
    /// /**
    ///    * Resets all the stored events. This is intended to be only used in tests.
    ///    * Events recorded but not yet flushed to the parent process storage won't be cleared.
    ///    * Override the pref. `toolkit.telemetry.ipcBatchTimeout` to reduce the time to flush events.
    ///    */
    /// ```
    ///

    /// `void clearEvents ();`
    #[inline]
    pub unsafe fn ClearEvents(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearEvents)(self, )
    }


    /// ```text
    /// /**
    ///    * Get a list of all registered stores.
    ///    *
    ///    * The list is deduplicated, but unordered.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getAllStores ();`
    const _GetAllStores: () = ();

    /// ```text
    /// /**
    ///    * Does early, cheap initialization for native telemetry data providers.
    ///    * Currently, this includes only MemoryTelemetry.
    ///    */
    /// ```
    ///

    /// `void earlyInit ();`
    #[inline]
    pub unsafe fn EarlyInit(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EarlyInit)(self, )
    }


    /// ```text
    /// /**
    ///    * Does late, expensive initialization for native telemetry data providers.
    ///    * Currently, this includes only MemoryTelemetry.
    ///    *
    ///    * This should only be called after startup has completed and the event loop
    ///    * is idle.
    ///    */
    /// ```
    ///

    /// `void delayedInit ();`
    #[inline]
    pub unsafe fn DelayedInit(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DelayedInit)(self, )
    }


    /// ```text
    /// /**
    ///    * Shuts down native telemetry providers. Currently, this includes only
    ///    * MemoryTelemetry.
    ///    */
    /// ```
    ///

    /// `void shutdown ();`
    #[inline]
    pub unsafe fn Shutdown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Shutdown)(self, )
    }


    /// ```text
    /// /**
    ///    * Gathers telemetry data for memory usage and records it to the data store.
    ///    * Returns a promise which resolves when asynchronous data collection has
    ///    * completed and all data has been recorded.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise gatherMemory ();`
    const _GatherMemory: () = ();

    /// ```text
    /// /**
    ///    * Serializes the per-origin data in plain text, optionally clearing
    ///    * the storage. Only to be used by about:telemetry.
    ///    *
    ///    * The returned structure looks like:
    ///    *   { metric: {origin1: count1, origin2: count2, ...}, ...}
    ///    *
    ///    * @param aClear Whether to clear the storage. Default false.
    ///    * @return a snapshot of the per-origin data.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getOriginSnapshot ([optional] in boolean aClear);`
    const _GetOriginSnapshot: () = ();

    /// ```text
    /// /**
    ///    * Encodes the per-origin information then serializes it.
    ///    * Returns a Promise.
    ///    *
    ///    * @param aClear Whether to clear the storage. Default false.
    ///    * @return Promise that resolves to the serialized encoded data.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise getEncodedOriginSnapshot ([optional] in boolean aClear);`
    const _GetEncodedOriginSnapshot: () = ();

    /// ```text
    /// /**
    ///    * Clears Firefox Origin Telemetry. Only to be used in tests.
    ///    */
    /// ```
    ///

    /// `void clearOrigins ();`
    #[inline]
    pub unsafe fn ClearOrigins(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearOrigins)(self, )
    }


}


