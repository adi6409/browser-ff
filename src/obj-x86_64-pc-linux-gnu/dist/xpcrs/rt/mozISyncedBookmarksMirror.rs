//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozISyncedBookmarksMirror.idl
//


/// `interface mozISyncedBookmarksMirrorProgressListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISyncedBookmarksMirrorProgressListener {
    vtable: *const mozISyncedBookmarksMirrorProgressListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISyncedBookmarksMirrorProgressListener.
unsafe impl XpCom for mozISyncedBookmarksMirrorProgressListener {
    const IID: nsIID = nsID(0x6239ffe3, 0x6ffd, 0x49ac,
        [0x8b, 0x1d, 0x95, 0x84, 0x07, 0x39, 0x5b, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISyncedBookmarksMirrorProgressListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISyncedBookmarksMirrorProgressListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISyncedBookmarksMirrorProgressListenerCoerce {
    /// Cheaply cast a value of this type from a `mozISyncedBookmarksMirrorProgressListener`.
    fn coerce_from(v: &mozISyncedBookmarksMirrorProgressListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISyncedBookmarksMirrorProgressListenerCoerce for mozISyncedBookmarksMirrorProgressListener {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMirrorProgressListener) -> &Self {
        v
    }
}

impl mozISyncedBookmarksMirrorProgressListener {
    /// Cast this `mozISyncedBookmarksMirrorProgressListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISyncedBookmarksMirrorProgressListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISyncedBookmarksMirrorProgressListener {
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
impl<T: nsISupportsCoerce> mozISyncedBookmarksMirrorProgressListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMirrorProgressListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISyncedBookmarksMirrorProgressListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISyncedBookmarksMirrorProgressListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onFetchLocalTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems); */
    pub OnFetchLocalTree: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorProgressListener, took: i64, itemCount: i64, deletedCount: i64, problems: *const nsIPropertyBag) -> ::nserror::nsresult,

    /* void onFetchRemoteTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems); */
    pub OnFetchRemoteTree: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorProgressListener, took: i64, itemCount: i64, deletedCount: i64, problems: *const nsIPropertyBag) -> ::nserror::nsresult,

    /* void onMerge (in long long took, in nsIPropertyBag counts); */
    pub OnMerge: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorProgressListener, took: i64, counts: *const nsIPropertyBag) -> ::nserror::nsresult,

    /* void onApply (in long long took); */
    pub OnApply: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorProgressListener, took: i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISyncedBookmarksMirrorProgressListener {


    /// `void onFetchLocalTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems);`
    #[inline]
    pub unsafe fn OnFetchLocalTree(&self, took: i64, itemCount: i64, deletedCount: i64, problems: *const nsIPropertyBag) -> ::nserror::nsresult {
        ((*self.vtable).OnFetchLocalTree)(self, took, itemCount, deletedCount, problems)
    }



    /// `void onFetchRemoteTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems);`
    #[inline]
    pub unsafe fn OnFetchRemoteTree(&self, took: i64, itemCount: i64, deletedCount: i64, problems: *const nsIPropertyBag) -> ::nserror::nsresult {
        ((*self.vtable).OnFetchRemoteTree)(self, took, itemCount, deletedCount, problems)
    }



    /// `void onMerge (in long long took, in nsIPropertyBag counts);`
    #[inline]
    pub unsafe fn OnMerge(&self, took: i64, counts: *const nsIPropertyBag) -> ::nserror::nsresult {
        ((*self.vtable).OnMerge)(self, took, counts)
    }



    /// `void onApply (in long long took);`
    #[inline]
    pub unsafe fn OnApply(&self, took: i64) -> ::nserror::nsresult {
        ((*self.vtable).OnApply)(self, took)
    }


}


/// `interface mozISyncedBookmarksMirrorCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISyncedBookmarksMirrorCallback {
    vtable: *const mozISyncedBookmarksMirrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISyncedBookmarksMirrorCallback.
unsafe impl XpCom for mozISyncedBookmarksMirrorCallback {
    const IID: nsIID = nsID(0xd23fdfea, 0x92c8, 0x409d,
        [0xa5, 0x16, 0x08, 0xae, 0x39, 0x5d, 0x57, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISyncedBookmarksMirrorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISyncedBookmarksMirrorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISyncedBookmarksMirrorCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozISyncedBookmarksMirrorCallback`.
    fn coerce_from(v: &mozISyncedBookmarksMirrorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISyncedBookmarksMirrorCallbackCoerce for mozISyncedBookmarksMirrorCallback {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMirrorCallback) -> &Self {
        v
    }
}

impl mozISyncedBookmarksMirrorCallback {
    /// Cast this `mozISyncedBookmarksMirrorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISyncedBookmarksMirrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISyncedBookmarksMirrorCallback {
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
impl<T: nsISupportsCoerce> mozISyncedBookmarksMirrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMirrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISyncedBookmarksMirrorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISyncedBookmarksMirrorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleSuccess (in bool result); */
    pub HandleSuccess: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorCallback, result: bool) -> ::nserror::nsresult,

    /* void handleError (in nsresult code, in AString message); */
    pub HandleError: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorCallback, code: ::nserror::nsresult, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISyncedBookmarksMirrorCallback {


    /// `void handleSuccess (in bool result);`
    #[inline]
    pub unsafe fn HandleSuccess(&self, result: bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleSuccess)(self, result)
    }



    /// `void handleError (in nsresult code, in AString message);`
    #[inline]
    pub unsafe fn HandleError(&self, code: ::nserror::nsresult, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, code, message)
    }


}


/// `interface mozISyncedBookmarksMirrorLogger : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISyncedBookmarksMirrorLogger {
    vtable: *const mozISyncedBookmarksMirrorLoggerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISyncedBookmarksMirrorLogger.
unsafe impl XpCom for mozISyncedBookmarksMirrorLogger {
    const IID: nsIID = nsID(0x37485984, 0xa6ab, 0x46e3,
        [0x9b, 0x0c, 0xe8, 0xb6, 0x13, 0x41, 0x3e, 0xf3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISyncedBookmarksMirrorLogger {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISyncedBookmarksMirrorLogger.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISyncedBookmarksMirrorLoggerCoerce {
    /// Cheaply cast a value of this type from a `mozISyncedBookmarksMirrorLogger`.
    fn coerce_from(v: &mozISyncedBookmarksMirrorLogger) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISyncedBookmarksMirrorLoggerCoerce for mozISyncedBookmarksMirrorLogger {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMirrorLogger) -> &Self {
        v
    }
}

impl mozISyncedBookmarksMirrorLogger {
    /// Cast this `mozISyncedBookmarksMirrorLogger` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISyncedBookmarksMirrorLoggerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISyncedBookmarksMirrorLogger {
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
impl<T: nsISupportsCoerce> mozISyncedBookmarksMirrorLoggerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMirrorLogger) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISyncedBookmarksMirrorLogger
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISyncedBookmarksMirrorLoggerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute short maxLevel; */
    pub GetMaxLevel: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorLogger, aMaxLevel: *mut i16) -> ::nserror::nsresult,

    /* attribute short maxLevel; */
    pub SetMaxLevel: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorLogger, aMaxLevel: i16) -> ::nserror::nsresult,

    /* void error (in AString message); */
    pub Error: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorLogger, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void warn (in AString message); */
    pub Warn: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorLogger, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void debug (in AString message); */
    pub Debug: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorLogger, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void trace (in AString message); */
    pub Trace: unsafe extern "system" fn (this: *const mozISyncedBookmarksMirrorLogger, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISyncedBookmarksMirrorLogger {

    pub const LEVEL_OFF: i64 = 0;


    pub const LEVEL_ERROR: i64 = 1;


    pub const LEVEL_WARN: i64 = 2;


    pub const LEVEL_DEBUG: i64 = 3;


    pub const LEVEL_TRACE: i64 = 4;


    /// `attribute short maxLevel;`
    #[inline]
    pub unsafe fn GetMaxLevel(&self, aMaxLevel: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxLevel)(self, aMaxLevel)
    }



    /// `attribute short maxLevel;`
    #[inline]
    pub unsafe fn SetMaxLevel(&self, aMaxLevel: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetMaxLevel)(self, aMaxLevel)
    }



    /// `void error (in AString message);`
    #[inline]
    pub unsafe fn Error(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Error)(self, message)
    }



    /// `void warn (in AString message);`
    #[inline]
    pub unsafe fn Warn(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Warn)(self, message)
    }



    /// `void debug (in AString message);`
    #[inline]
    pub unsafe fn Debug(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Debug)(self, message)
    }



    /// `void trace (in AString message);`
    #[inline]
    pub unsafe fn Trace(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Trace)(self, message)
    }


}


/// `interface mozISyncedBookmarksMerger : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISyncedBookmarksMerger {
    vtable: *const mozISyncedBookmarksMergerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISyncedBookmarksMerger.
unsafe impl XpCom for mozISyncedBookmarksMerger {
    const IID: nsIID = nsID(0xf0a6217d, 0x8344, 0x4e68,
        [0x99, 0x95, 0xbb, 0xf5, 0x55, 0x4b, 0xe8, 0x6e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISyncedBookmarksMerger {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISyncedBookmarksMerger.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISyncedBookmarksMergerCoerce {
    /// Cheaply cast a value of this type from a `mozISyncedBookmarksMerger`.
    fn coerce_from(v: &mozISyncedBookmarksMerger) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISyncedBookmarksMergerCoerce for mozISyncedBookmarksMerger {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMerger) -> &Self {
        v
    }
}

impl mozISyncedBookmarksMerger {
    /// Cast this `mozISyncedBookmarksMerger` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISyncedBookmarksMergerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISyncedBookmarksMerger {
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
impl<T: nsISupportsCoerce> mozISyncedBookmarksMergerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISyncedBookmarksMerger) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISyncedBookmarksMerger
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISyncedBookmarksMergerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute mozIStorageConnection db; */
    pub GetDb: unsafe extern "system" fn (this: *const mozISyncedBookmarksMerger, aDb: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* attribute mozIStorageConnection db; */
    pub SetDb: unsafe extern "system" fn (this: *const mozISyncedBookmarksMerger, aDb: *const mozIStorageConnection) -> ::nserror::nsresult,

    /* attribute mozIServicesLogSink logger; */
    pub GetLogger: unsafe extern "system" fn (this: *const mozISyncedBookmarksMerger, aLogger: *mut *const mozIServicesLogSink) -> ::nserror::nsresult,

    /* attribute mozIServicesLogSink logger; */
    pub SetLogger: unsafe extern "system" fn (this: *const mozISyncedBookmarksMerger, aLogger: *const mozIServicesLogSink) -> ::nserror::nsresult,

    /* mozIPlacesPendingOperation merge (in long long localTimeSeconds, in long long remoteTimeSeconds, in Array<AString> weakUploads, in mozISyncedBookmarksMirrorCallback callback); */
    pub Merge: unsafe extern "system" fn (this: *const mozISyncedBookmarksMerger, localTimeSeconds: i64, remoteTimeSeconds: i64, weakUploads: *const thin_vec::ThinVec<::nsstring::nsString>, callback: *const mozISyncedBookmarksMirrorCallback, _retval: *mut*const mozIPlacesPendingOperation) -> ::nserror::nsresult,

    /* void reset (); */
    pub Reset: unsafe extern "system" fn (this: *const mozISyncedBookmarksMerger) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISyncedBookmarksMerger {

    pub const KIND_BOOKMARK: i64 = 1;


    pub const KIND_QUERY: i64 = 2;


    pub const KIND_FOLDER: i64 = 3;


    pub const KIND_LIVEMARK: i64 = 4;


    pub const KIND_SEPARATOR: i64 = 5;


    pub const VALIDITY_VALID: i64 = 1;


    pub const VALIDITY_REUPLOAD: i64 = 2;


    pub const VALIDITY_REPLACE: i64 = 3;


    /// `attribute mozIStorageConnection db;`
    #[inline]
    pub unsafe fn GetDb(&self, aDb: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).GetDb)(self, aDb)
    }



    /// `attribute mozIStorageConnection db;`
    #[inline]
    pub unsafe fn SetDb(&self, aDb: *const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).SetDb)(self, aDb)
    }



    /// `attribute mozIServicesLogSink logger;`
    #[inline]
    pub unsafe fn GetLogger(&self, aLogger: *mut *const mozIServicesLogSink) -> ::nserror::nsresult {
        ((*self.vtable).GetLogger)(self, aLogger)
    }



    /// `attribute mozIServicesLogSink logger;`
    #[inline]
    pub unsafe fn SetLogger(&self, aLogger: *const mozIServicesLogSink) -> ::nserror::nsresult {
        ((*self.vtable).SetLogger)(self, aLogger)
    }



    /// `mozIPlacesPendingOperation merge (in long long localTimeSeconds, in long long remoteTimeSeconds, in Array<AString> weakUploads, in mozISyncedBookmarksMirrorCallback callback);`
    #[inline]
    pub unsafe fn Merge(&self, localTimeSeconds: i64, remoteTimeSeconds: i64, weakUploads: *const thin_vec::ThinVec<::nsstring::nsString>, callback: *const mozISyncedBookmarksMirrorCallback, _retval: *mut*const mozIPlacesPendingOperation) -> ::nserror::nsresult {
        ((*self.vtable).Merge)(self, localTimeSeconds, remoteTimeSeconds, weakUploads, callback, _retval)
    }



    /// `void reset ();`
    #[inline]
    pub unsafe fn Reset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, )
    }


}


