//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemoryReporter.idl
//


/// `interface nsIHandleReportCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHandleReportCallback {
    vtable: *const nsIHandleReportCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHandleReportCallback.
unsafe impl XpCom for nsIHandleReportCallback {
    const IID: nsIID = nsID(0x62ef0e1c, 0xdbd6, 0x11e3,
        [0xaa, 0x75, 0x3c, 0x97, 0x0e, 0x9f, 0x42, 0x38]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHandleReportCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHandleReportCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHandleReportCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIHandleReportCallback`.
    fn coerce_from(v: &nsIHandleReportCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHandleReportCallbackCoerce for nsIHandleReportCallback {
    #[inline]
    fn coerce_from(v: &nsIHandleReportCallback) -> &Self {
        v
    }
}

impl nsIHandleReportCallback {
    /// Cast this `nsIHandleReportCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHandleReportCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHandleReportCallback {
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
impl<T: nsISupportsCoerce> nsIHandleReportCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandleReportCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHandleReportCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHandleReportCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (in ACString process, in AUTF8String path, in int32_t kind, in int32_t units, in int64_t amount, in AUTF8String description, in nsISupports data); */
    pub Callback: unsafe extern "system" fn (this: *const nsIHandleReportCallback, process: *const ::nsstring::nsACString, path: *const ::nsstring::nsACString, kind: int32_t, units: int32_t, amount: int64_t, description: *const ::nsstring::nsACString, data: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHandleReportCallback {


    /// `void callback (in ACString process, in AUTF8String path, in int32_t kind, in int32_t units, in int64_t amount, in AUTF8String description, in nsISupports data);`
    #[inline]
    pub unsafe fn Callback(&self, process: *const ::nsstring::nsACString, path: *const ::nsstring::nsACString, kind: int32_t, units: int32_t, amount: int64_t, description: *const ::nsstring::nsACString, data: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, process, path, kind, units, amount, description, data)
    }


}


/// `interface nsIMemoryReporter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMemoryReporter {
    vtable: *const nsIMemoryReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMemoryReporter.
unsafe impl XpCom for nsIMemoryReporter {
    const IID: nsIID = nsID(0x92a36db1, 0x46bd, 0x4fe6,
        [0x98, 0x8e, 0x47, 0xdb, 0x47, 0x23, 0x6d, 0x8b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMemoryReporter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMemoryReporter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMemoryReporterCoerce {
    /// Cheaply cast a value of this type from a `nsIMemoryReporter`.
    fn coerce_from(v: &nsIMemoryReporter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMemoryReporterCoerce for nsIMemoryReporter {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporter) -> &Self {
        v
    }
}

impl nsIMemoryReporter {
    /// Cast this `nsIMemoryReporter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMemoryReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMemoryReporter {
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
impl<T: nsISupportsCoerce> nsIMemoryReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMemoryReporter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMemoryReporterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void collectReports (in nsIHandleReportCallback callback, in nsISupports data, in boolean anonymize); */
    pub CollectReports: unsafe extern "system" fn (this: *const nsIMemoryReporter, callback: *const nsIHandleReportCallback, data: *const nsISupports, anonymize: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMemoryReporter {

    pub const KIND_NONHEAP: i64 = 0;


    pub const KIND_HEAP: i64 = 1;


    pub const KIND_OTHER: i64 = 2;


    pub const UNITS_BYTES: i64 = 0;


    pub const UNITS_COUNT: i64 = 1;


    pub const UNITS_COUNT_CUMULATIVE: i64 = 2;


    pub const UNITS_PERCENTAGE: i64 = 3;


    /// `void collectReports (in nsIHandleReportCallback callback, in nsISupports data, in boolean anonymize);`
    #[inline]
    pub unsafe fn CollectReports(&self, callback: *const nsIHandleReportCallback, data: *const nsISupports, anonymize: bool) -> ::nserror::nsresult {
        ((*self.vtable).CollectReports)(self, callback, data, anonymize)
    }


}


/// `interface nsIFinishReportingCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFinishReportingCallback {
    vtable: *const nsIFinishReportingCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFinishReportingCallback.
unsafe impl XpCom for nsIFinishReportingCallback {
    const IID: nsIID = nsID(0x548b3909, 0xc04d, 0x4ca6,
        [0x84, 0x66, 0xb8, 0xbe, 0xe3, 0x83, 0x74, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFinishReportingCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFinishReportingCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFinishReportingCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFinishReportingCallback`.
    fn coerce_from(v: &nsIFinishReportingCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFinishReportingCallbackCoerce for nsIFinishReportingCallback {
    #[inline]
    fn coerce_from(v: &nsIFinishReportingCallback) -> &Self {
        v
    }
}

impl nsIFinishReportingCallback {
    /// Cast this `nsIFinishReportingCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFinishReportingCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFinishReportingCallback {
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
impl<T: nsISupportsCoerce> nsIFinishReportingCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFinishReportingCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFinishReportingCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFinishReportingCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (in nsISupports data); */
    pub Callback: unsafe extern "system" fn (this: *const nsIFinishReportingCallback, data: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFinishReportingCallback {


    /// `void callback (in nsISupports data);`
    #[inline]
    pub unsafe fn Callback(&self, data: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, data)
    }


}


/// `interface nsIHeapAllocatedCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHeapAllocatedCallback {
    vtable: *const nsIHeapAllocatedCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHeapAllocatedCallback.
unsafe impl XpCom for nsIHeapAllocatedCallback {
    const IID: nsIID = nsID(0x1a80cd0f, 0x0d9e, 0x4397,
        [0xbe, 0x69, 0x68, 0xad, 0x28, 0xfe, 0x51, 0x75]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHeapAllocatedCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHeapAllocatedCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHeapAllocatedCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIHeapAllocatedCallback`.
    fn coerce_from(v: &nsIHeapAllocatedCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHeapAllocatedCallbackCoerce for nsIHeapAllocatedCallback {
    #[inline]
    fn coerce_from(v: &nsIHeapAllocatedCallback) -> &Self {
        v
    }
}

impl nsIHeapAllocatedCallback {
    /// Cast this `nsIHeapAllocatedCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHeapAllocatedCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHeapAllocatedCallback {
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
impl<T: nsISupportsCoerce> nsIHeapAllocatedCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHeapAllocatedCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHeapAllocatedCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHeapAllocatedCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (in int64_t bytesAllocated); */
    pub Callback: unsafe extern "system" fn (this: *const nsIHeapAllocatedCallback, bytesAllocated: int64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHeapAllocatedCallback {


    /// `void callback (in int64_t bytesAllocated);`
    #[inline]
    pub unsafe fn Callback(&self, bytesAllocated: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, bytesAllocated)
    }


}


/// `interface nsIMemoryReporterManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMemoryReporterManager {
    vtable: *const nsIMemoryReporterManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMemoryReporterManager.
unsafe impl XpCom for nsIMemoryReporterManager {
    const IID: nsIID = nsID(0x2998574d, 0x8993, 0x407a,
        [0xb1, 0xa5, 0x8a, 0xd7, 0x41, 0x76, 0x53, 0xe1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMemoryReporterManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMemoryReporterManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMemoryReporterManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIMemoryReporterManager`.
    fn coerce_from(v: &nsIMemoryReporterManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMemoryReporterManagerCoerce for nsIMemoryReporterManager {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporterManager) -> &Self {
        v
    }
}

impl nsIMemoryReporterManager {
    /// Cast this `nsIMemoryReporterManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMemoryReporterManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMemoryReporterManager {
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
impl<T: nsISupportsCoerce> nsIMemoryReporterManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporterManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMemoryReporterManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMemoryReporterManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void init (); */
    pub Init: unsafe extern "system" fn (this: *const nsIMemoryReporterManager) -> ::nserror::nsresult,

    /* void registerStrongReporter (in nsIMemoryReporter reporter); */
    pub RegisterStrongReporter: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void registerStrongAsyncReporter (in nsIMemoryReporter reporter); */
    pub RegisterStrongAsyncReporter: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void registerWeakReporter (in nsIMemoryReporter reporter); */
    pub RegisterWeakReporter: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void registerWeakAsyncReporter (in nsIMemoryReporter reporter); */
    pub RegisterWeakAsyncReporter: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void unregisterStrongReporter (in nsIMemoryReporter reporter); */
    pub UnregisterStrongReporter: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void unregisterWeakReporter (in nsIMemoryReporter reporter); */
    pub UnregisterWeakReporter: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void blockRegistrationAndHideExistingReporters (); */
    pub BlockRegistrationAndHideExistingReporters: unsafe extern "system" fn (this: *const nsIMemoryReporterManager) -> ::nserror::nsresult,

    /* void unblockRegistrationAndRestoreOriginalReporters (); */
    pub UnblockRegistrationAndRestoreOriginalReporters: unsafe extern "system" fn (this: *const nsIMemoryReporterManager) -> ::nserror::nsresult,

    /* void registerStrongReporterEvenIfBlocked (in nsIMemoryReporter aReporter); */
    pub RegisterStrongReporterEvenIfBlocked: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aReporter: *const nsIMemoryReporter) -> ::nserror::nsresult,

    /* void getReports (in nsIHandleReportCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize); */
    pub GetReports: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, handleReport: *const nsIHandleReportCallback, handleReportData: *const nsISupports, finishReporting: *const nsIFinishReportingCallback, finishReportingData: *const nsISupports, anonymize: bool) -> ::nserror::nsresult,

    /* [noscript] void getReportsExtended (in nsIHandleReportCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize, in boolean minimizeMemoryUsage, in AString DMDDumpIdent); */
    pub GetReportsExtended: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, handleReport: *const nsIHandleReportCallback, handleReportData: *const nsISupports, finishReporting: *const nsIFinishReportingCallback, finishReportingData: *const nsISupports, anonymize: bool, minimizeMemoryUsage: bool, DMDDumpIdent: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void getReportsForThisProcessExtended (in nsIHandleReportCallback handleReport, in nsISupports handleReportData, in boolean anonymize, in FILE DMDFile, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData); */
    /// Unable to generate binding because `native type FILE unsupported`
    pub GetReportsForThisProcessExtended: *const ::libc::c_void,

    /* [noscript] void endReport (); */
    pub EndReport: unsafe extern "system" fn (this: *const nsIMemoryReporterManager) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t vsize; */
    pub GetVsize: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aVsize: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t vsizeMaxContiguous; */
    pub GetVsizeMaxContiguous: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aVsizeMaxContiguous: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t resident; */
    pub GetResident: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aResident: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t residentFast; */
    pub GetResidentFast: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aResidentFast: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t residentPeak; */
    pub GetResidentPeak: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aResidentPeak: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t residentUnique; */
    pub GetResidentUnique: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aResidentUnique: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t heapAllocated; */
    pub GetHeapAllocated: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aHeapAllocated: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t heapOverheadFraction; */
    pub GetHeapOverheadFraction: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aHeapOverheadFraction: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeGCHeap; */
    pub GetJSMainRuntimeGCHeap: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeGCHeap: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeTemporaryPeak; */
    pub GetJSMainRuntimeTemporaryPeak: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeTemporaryPeak: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeCompartmentsSystem; */
    pub GetJSMainRuntimeCompartmentsSystem: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeCompartmentsSystem: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeCompartmentsUser; */
    pub GetJSMainRuntimeCompartmentsUser: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeCompartmentsUser: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeRealmsSystem; */
    pub GetJSMainRuntimeRealmsSystem: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeRealmsSystem: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeRealmsUser; */
    pub GetJSMainRuntimeRealmsUser: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeRealmsUser: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t imagesContentUsedUncompressed; */
    pub GetImagesContentUsedUncompressed: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aImagesContentUsedUncompressed: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t storageSQLite; */
    pub GetStorageSQLite: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aStorageSQLite: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t lowMemoryEventsVirtual; */
    pub GetLowMemoryEventsVirtual: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aLowMemoryEventsVirtual: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t lowMemoryEventsCommitSpace; */
    pub GetLowMemoryEventsCommitSpace: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aLowMemoryEventsCommitSpace: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t lowMemoryEventsPhysical; */
    pub GetLowMemoryEventsPhysical: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aLowMemoryEventsPhysical: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t ghostWindows; */
    pub GetGhostWindows: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aGhostWindows: *mut int64_t) -> ::nserror::nsresult,

    /* [must_use] readonly attribute int64_t pageFaultsHard; */
    pub GetPageFaultsHard: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aPageFaultsHard: *mut int64_t) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean hasMozMallocUsableSize; */
    pub GetHasMozMallocUsableSize: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aHasMozMallocUsableSize: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isDMDEnabled; */
    pub GetIsDMDEnabled: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aIsDMDEnabled: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isDMDRunning; */
    pub GetIsDMDRunning: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, aIsDMDRunning: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void minimizeMemoryUsage (in nsIRunnable callback); */
    pub MinimizeMemoryUsage: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, callback: *const nsIRunnable) -> ::nserror::nsresult,

    /* [must_use] void sizeOfTab (in mozIDOMWindowProxy window, out int64_t jsObjectsSize, out int64_t jsStringsSize, out int64_t jsOtherSize, out int64_t domSize, out int64_t styleSize, out int64_t otherSize, out int64_t totalSize, out double jsMilliseconds, out double nonJSMilliseconds); */
    pub SizeOfTab: unsafe extern "system" fn (this: *const nsIMemoryReporterManager, window: *const mozIDOMWindowProxy, jsObjectsSize: *mut int64_t, jsStringsSize: *mut int64_t, jsOtherSize: *mut int64_t, domSize: *mut int64_t, styleSize: *mut int64_t, otherSize: *mut int64_t, totalSize: *mut int64_t, jsMilliseconds: *mut libc::c_double, nonJSMilliseconds: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMemoryReporterManager {


    /// `[must_use] void init ();`
    #[inline]
    pub unsafe fn Init(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, )
    }



    /// `void registerStrongReporter (in nsIMemoryReporter reporter);`
    #[inline]
    pub unsafe fn RegisterStrongReporter(&self, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).RegisterStrongReporter)(self, reporter)
    }



    /// `void registerStrongAsyncReporter (in nsIMemoryReporter reporter);`
    #[inline]
    pub unsafe fn RegisterStrongAsyncReporter(&self, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).RegisterStrongAsyncReporter)(self, reporter)
    }



    /// `void registerWeakReporter (in nsIMemoryReporter reporter);`
    #[inline]
    pub unsafe fn RegisterWeakReporter(&self, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).RegisterWeakReporter)(self, reporter)
    }



    /// `void registerWeakAsyncReporter (in nsIMemoryReporter reporter);`
    #[inline]
    pub unsafe fn RegisterWeakAsyncReporter(&self, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).RegisterWeakAsyncReporter)(self, reporter)
    }



    /// `void unregisterStrongReporter (in nsIMemoryReporter reporter);`
    #[inline]
    pub unsafe fn UnregisterStrongReporter(&self, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterStrongReporter)(self, reporter)
    }



    /// `void unregisterWeakReporter (in nsIMemoryReporter reporter);`
    #[inline]
    pub unsafe fn UnregisterWeakReporter(&self, reporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterWeakReporter)(self, reporter)
    }



    /// `void blockRegistrationAndHideExistingReporters ();`
    #[inline]
    pub unsafe fn BlockRegistrationAndHideExistingReporters(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BlockRegistrationAndHideExistingReporters)(self, )
    }



    /// `void unblockRegistrationAndRestoreOriginalReporters ();`
    #[inline]
    pub unsafe fn UnblockRegistrationAndRestoreOriginalReporters(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UnblockRegistrationAndRestoreOriginalReporters)(self, )
    }



    /// `void registerStrongReporterEvenIfBlocked (in nsIMemoryReporter aReporter);`
    #[inline]
    pub unsafe fn RegisterStrongReporterEvenIfBlocked(&self, aReporter: *const nsIMemoryReporter) -> ::nserror::nsresult {
        ((*self.vtable).RegisterStrongReporterEvenIfBlocked)(self, aReporter)
    }



    /// `void getReports (in nsIHandleReportCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize);`
    #[inline]
    pub unsafe fn GetReports(&self, handleReport: *const nsIHandleReportCallback, handleReportData: *const nsISupports, finishReporting: *const nsIFinishReportingCallback, finishReportingData: *const nsISupports, anonymize: bool) -> ::nserror::nsresult {
        ((*self.vtable).GetReports)(self, handleReport, handleReportData, finishReporting, finishReportingData, anonymize)
    }



    /// `[noscript] void getReportsExtended (in nsIHandleReportCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize, in boolean minimizeMemoryUsage, in AString DMDDumpIdent);`
    #[inline]
    pub unsafe fn GetReportsExtended(&self, handleReport: *const nsIHandleReportCallback, handleReportData: *const nsISupports, finishReporting: *const nsIFinishReportingCallback, finishReportingData: *const nsISupports, anonymize: bool, minimizeMemoryUsage: bool, DMDDumpIdent: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetReportsExtended)(self, handleReport, handleReportData, finishReporting, finishReportingData, anonymize, minimizeMemoryUsage, DMDDumpIdent)
    }



    /// `[noscript] void getReportsForThisProcessExtended (in nsIHandleReportCallback handleReport, in nsISupports handleReportData, in boolean anonymize, in FILE DMDFile, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData);`
    const _GetReportsForThisProcessExtended: () = ();


    /// `[noscript] void endReport ();`
    #[inline]
    pub unsafe fn EndReport(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EndReport)(self, )
    }



    /// `[must_use] readonly attribute int64_t vsize;`
    #[inline]
    pub unsafe fn GetVsize(&self, aVsize: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetVsize)(self, aVsize)
    }



    /// `[must_use] readonly attribute int64_t vsizeMaxContiguous;`
    #[inline]
    pub unsafe fn GetVsizeMaxContiguous(&self, aVsizeMaxContiguous: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetVsizeMaxContiguous)(self, aVsizeMaxContiguous)
    }



    /// `[must_use] readonly attribute int64_t resident;`
    #[inline]
    pub unsafe fn GetResident(&self, aResident: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetResident)(self, aResident)
    }



    /// `[must_use] readonly attribute int64_t residentFast;`
    #[inline]
    pub unsafe fn GetResidentFast(&self, aResidentFast: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetResidentFast)(self, aResidentFast)
    }



    /// `[must_use] readonly attribute int64_t residentPeak;`
    #[inline]
    pub unsafe fn GetResidentPeak(&self, aResidentPeak: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetResidentPeak)(self, aResidentPeak)
    }



    /// `[must_use] readonly attribute int64_t residentUnique;`
    #[inline]
    pub unsafe fn GetResidentUnique(&self, aResidentUnique: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetResidentUnique)(self, aResidentUnique)
    }



    /// `[must_use] readonly attribute int64_t heapAllocated;`
    #[inline]
    pub unsafe fn GetHeapAllocated(&self, aHeapAllocated: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetHeapAllocated)(self, aHeapAllocated)
    }



    /// `[must_use] readonly attribute int64_t heapOverheadFraction;`
    #[inline]
    pub unsafe fn GetHeapOverheadFraction(&self, aHeapOverheadFraction: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetHeapOverheadFraction)(self, aHeapOverheadFraction)
    }



    /// `[must_use] readonly attribute int64_t JSMainRuntimeGCHeap;`
    #[inline]
    pub unsafe fn GetJSMainRuntimeGCHeap(&self, aJSMainRuntimeGCHeap: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetJSMainRuntimeGCHeap)(self, aJSMainRuntimeGCHeap)
    }



    /// `[must_use] readonly attribute int64_t JSMainRuntimeTemporaryPeak;`
    #[inline]
    pub unsafe fn GetJSMainRuntimeTemporaryPeak(&self, aJSMainRuntimeTemporaryPeak: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetJSMainRuntimeTemporaryPeak)(self, aJSMainRuntimeTemporaryPeak)
    }



    /// `[must_use] readonly attribute int64_t JSMainRuntimeCompartmentsSystem;`
    #[inline]
    pub unsafe fn GetJSMainRuntimeCompartmentsSystem(&self, aJSMainRuntimeCompartmentsSystem: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetJSMainRuntimeCompartmentsSystem)(self, aJSMainRuntimeCompartmentsSystem)
    }



    /// `[must_use] readonly attribute int64_t JSMainRuntimeCompartmentsUser;`
    #[inline]
    pub unsafe fn GetJSMainRuntimeCompartmentsUser(&self, aJSMainRuntimeCompartmentsUser: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetJSMainRuntimeCompartmentsUser)(self, aJSMainRuntimeCompartmentsUser)
    }



    /// `[must_use] readonly attribute int64_t JSMainRuntimeRealmsSystem;`
    #[inline]
    pub unsafe fn GetJSMainRuntimeRealmsSystem(&self, aJSMainRuntimeRealmsSystem: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetJSMainRuntimeRealmsSystem)(self, aJSMainRuntimeRealmsSystem)
    }



    /// `[must_use] readonly attribute int64_t JSMainRuntimeRealmsUser;`
    #[inline]
    pub unsafe fn GetJSMainRuntimeRealmsUser(&self, aJSMainRuntimeRealmsUser: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetJSMainRuntimeRealmsUser)(self, aJSMainRuntimeRealmsUser)
    }



    /// `[must_use] readonly attribute int64_t imagesContentUsedUncompressed;`
    #[inline]
    pub unsafe fn GetImagesContentUsedUncompressed(&self, aImagesContentUsedUncompressed: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetImagesContentUsedUncompressed)(self, aImagesContentUsedUncompressed)
    }



    /// `[must_use] readonly attribute int64_t storageSQLite;`
    #[inline]
    pub unsafe fn GetStorageSQLite(&self, aStorageSQLite: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageSQLite)(self, aStorageSQLite)
    }



    /// `[must_use] readonly attribute int64_t lowMemoryEventsVirtual;`
    #[inline]
    pub unsafe fn GetLowMemoryEventsVirtual(&self, aLowMemoryEventsVirtual: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLowMemoryEventsVirtual)(self, aLowMemoryEventsVirtual)
    }



    /// `[must_use] readonly attribute int64_t lowMemoryEventsCommitSpace;`
    #[inline]
    pub unsafe fn GetLowMemoryEventsCommitSpace(&self, aLowMemoryEventsCommitSpace: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLowMemoryEventsCommitSpace)(self, aLowMemoryEventsCommitSpace)
    }



    /// `[must_use] readonly attribute int64_t lowMemoryEventsPhysical;`
    #[inline]
    pub unsafe fn GetLowMemoryEventsPhysical(&self, aLowMemoryEventsPhysical: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLowMemoryEventsPhysical)(self, aLowMemoryEventsPhysical)
    }



    /// `[must_use] readonly attribute int64_t ghostWindows;`
    #[inline]
    pub unsafe fn GetGhostWindows(&self, aGhostWindows: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetGhostWindows)(self, aGhostWindows)
    }



    /// `[must_use] readonly attribute int64_t pageFaultsHard;`
    #[inline]
    pub unsafe fn GetPageFaultsHard(&self, aPageFaultsHard: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPageFaultsHard)(self, aPageFaultsHard)
    }



    /// `[infallible] readonly attribute boolean hasMozMallocUsableSize;`
    #[inline]
    pub unsafe fn GetHasMozMallocUsableSize(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetHasMozMallocUsableSize)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean isDMDEnabled;`
    #[inline]
    pub unsafe fn GetIsDMDEnabled(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsDMDEnabled)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean isDMDRunning;`
    #[inline]
    pub unsafe fn GetIsDMDRunning(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsDMDRunning)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[must_use] void minimizeMemoryUsage (in nsIRunnable callback);`
    #[inline]
    pub unsafe fn MinimizeMemoryUsage(&self, callback: *const nsIRunnable) -> ::nserror::nsresult {
        ((*self.vtable).MinimizeMemoryUsage)(self, callback)
    }



    /// `[must_use] void sizeOfTab (in mozIDOMWindowProxy window, out int64_t jsObjectsSize, out int64_t jsStringsSize, out int64_t jsOtherSize, out int64_t domSize, out int64_t styleSize, out int64_t otherSize, out int64_t totalSize, out double jsMilliseconds, out double nonJSMilliseconds);`
    #[inline]
    pub unsafe fn SizeOfTab(&self, window: *const mozIDOMWindowProxy, jsObjectsSize: *mut int64_t, jsStringsSize: *mut int64_t, jsOtherSize: *mut int64_t, domSize: *mut int64_t, styleSize: *mut int64_t, otherSize: *mut int64_t, totalSize: *mut int64_t, jsMilliseconds: *mut libc::c_double, nonJSMilliseconds: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SizeOfTab)(self, window, jsObjectsSize, jsStringsSize, jsOtherSize, domSize, styleSize, otherSize, totalSize, jsMilliseconds, nonJSMilliseconds)
    }


}


