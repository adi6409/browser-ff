//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIBridgedSyncEngine.idl
//


/// `interface mozIBridgedSyncEngineCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIBridgedSyncEngineCallback {
    vtable: *const mozIBridgedSyncEngineCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIBridgedSyncEngineCallback.
unsafe impl XpCom for mozIBridgedSyncEngineCallback {
    const IID: nsIID = nsID(0x9b7dd2a3, 0xdf99, 0x4469,
        [0x9e, 0xa9, 0x61, 0xb2, 0x22, 0x09, 0x86, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIBridgedSyncEngineCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIBridgedSyncEngineCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIBridgedSyncEngineCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIBridgedSyncEngineCallback`.
    fn coerce_from(v: &mozIBridgedSyncEngineCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIBridgedSyncEngineCallbackCoerce for mozIBridgedSyncEngineCallback {
    #[inline]
    fn coerce_from(v: &mozIBridgedSyncEngineCallback) -> &Self {
        v
    }
}

impl mozIBridgedSyncEngineCallback {
    /// Cast this `mozIBridgedSyncEngineCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIBridgedSyncEngineCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIBridgedSyncEngineCallback {
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
impl<T: nsISupportsCoerce> mozIBridgedSyncEngineCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIBridgedSyncEngineCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIBridgedSyncEngineCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIBridgedSyncEngineCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleSuccess (in nsIVariant result); */
    pub HandleSuccess: unsafe extern "system" fn (this: *const mozIBridgedSyncEngineCallback, result: *const nsIVariant) -> ::nserror::nsresult,

    /* void handleError (in nsresult code, in AUTF8String message); */
    pub HandleError: unsafe extern "system" fn (this: *const mozIBridgedSyncEngineCallback, code: ::nserror::nsresult, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIBridgedSyncEngineCallback {


    /// `void handleSuccess (in nsIVariant result);`
    #[inline]
    pub unsafe fn HandleSuccess(&self, result: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).HandleSuccess)(self, result)
    }



    /// `void handleError (in nsresult code, in AUTF8String message);`
    #[inline]
    pub unsafe fn HandleError(&self, code: ::nserror::nsresult, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, code, message)
    }


}


/// `interface mozIBridgedSyncEngineApplyCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIBridgedSyncEngineApplyCallback {
    vtable: *const mozIBridgedSyncEngineApplyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIBridgedSyncEngineApplyCallback.
unsafe impl XpCom for mozIBridgedSyncEngineApplyCallback {
    const IID: nsIID = nsID(0x2776cdd5, 0x799a, 0x4009,
        [0xb2, 0xf3, 0x35, 0x6d, 0x94, 0x0a, 0x52, 0x44]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIBridgedSyncEngineApplyCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIBridgedSyncEngineApplyCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIBridgedSyncEngineApplyCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIBridgedSyncEngineApplyCallback`.
    fn coerce_from(v: &mozIBridgedSyncEngineApplyCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIBridgedSyncEngineApplyCallbackCoerce for mozIBridgedSyncEngineApplyCallback {
    #[inline]
    fn coerce_from(v: &mozIBridgedSyncEngineApplyCallback) -> &Self {
        v
    }
}

impl mozIBridgedSyncEngineApplyCallback {
    /// Cast this `mozIBridgedSyncEngineApplyCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIBridgedSyncEngineApplyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIBridgedSyncEngineApplyCallback {
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
impl<T: nsISupportsCoerce> mozIBridgedSyncEngineApplyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIBridgedSyncEngineApplyCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIBridgedSyncEngineApplyCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIBridgedSyncEngineApplyCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleSuccess (in Array<AUTF8String> outgoingEnvelopesAsJSON); */
    pub HandleSuccess: unsafe extern "system" fn (this: *const mozIBridgedSyncEngineApplyCallback, outgoingEnvelopesAsJSON: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void handleError (in nsresult code, in AUTF8String message); */
    pub HandleError: unsafe extern "system" fn (this: *const mozIBridgedSyncEngineApplyCallback, code: ::nserror::nsresult, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIBridgedSyncEngineApplyCallback {


    /// `void handleSuccess (in Array<AUTF8String> outgoingEnvelopesAsJSON);`
    #[inline]
    pub unsafe fn HandleSuccess(&self, outgoingEnvelopesAsJSON: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).HandleSuccess)(self, outgoingEnvelopesAsJSON)
    }



    /// `void handleError (in nsresult code, in AUTF8String message);`
    #[inline]
    pub unsafe fn HandleError(&self, code: ::nserror::nsresult, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, code, message)
    }


}


/// `interface mozIBridgedSyncEngine : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIBridgedSyncEngine {
    vtable: *const mozIBridgedSyncEngineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIBridgedSyncEngine.
unsafe impl XpCom for mozIBridgedSyncEngine {
    const IID: nsIID = nsID(0x3b2b80be, 0xc30e, 0x4498,
        [0x80, 0x65, 0x01, 0x80, 0x9c, 0xfe, 0x8d, 0x47]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIBridgedSyncEngine {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIBridgedSyncEngine.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIBridgedSyncEngineCoerce {
    /// Cheaply cast a value of this type from a `mozIBridgedSyncEngine`.
    fn coerce_from(v: &mozIBridgedSyncEngine) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIBridgedSyncEngineCoerce for mozIBridgedSyncEngine {
    #[inline]
    fn coerce_from(v: &mozIBridgedSyncEngine) -> &Self {
        v
    }
}

impl mozIBridgedSyncEngine {
    /// Cast this `mozIBridgedSyncEngine` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIBridgedSyncEngineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIBridgedSyncEngine {
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
impl<T: nsISupportsCoerce> mozIBridgedSyncEngineCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIBridgedSyncEngine) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIBridgedSyncEngine
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIBridgedSyncEngineVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long storageVersion; */
    pub GetStorageVersion: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, aStorageVersion: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute boolean allowSkippedRecord; */
    pub GetAllowSkippedRecord: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, aAllowSkippedRecord: *mut bool) -> ::nserror::nsresult,

    /* attribute mozIServicesLogSink logger; */
    pub GetLogger: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, aLogger: *mut *const mozIServicesLogSink) -> ::nserror::nsresult,

    /* attribute mozIServicesLogSink logger; */
    pub SetLogger: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, aLogger: *const mozIServicesLogSink) -> ::nserror::nsresult,

    /* void getLastSync (in mozIBridgedSyncEngineCallback callback); */
    pub GetLastSync: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void setLastSync (in long long lastSyncMillis, in mozIBridgedSyncEngineCallback callback); */
    pub SetLastSync: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, lastSyncMillis: i64, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void getSyncId (in mozIBridgedSyncEngineCallback callback); */
    pub GetSyncId: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void resetSyncId (in mozIBridgedSyncEngineCallback callback); */
    pub ResetSyncId: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void ensureCurrentSyncId (in AUTF8String newSyncId, in mozIBridgedSyncEngineCallback callback); */
    pub EnsureCurrentSyncId: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, newSyncId: *const ::nsstring::nsACString, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void syncStarted (in mozIBridgedSyncEngineCallback callback); */
    pub SyncStarted: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void storeIncoming (in Array<AUTF8String> incomingEnvelopesAsJSON, in mozIBridgedSyncEngineCallback callback); */
    pub StoreIncoming: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, incomingEnvelopesAsJSON: *const thin_vec::ThinVec<::nsstring::nsCString>, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void apply (in mozIBridgedSyncEngineApplyCallback callback); */
    pub Apply: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineApplyCallback) -> ::nserror::nsresult,

    /* void setUploaded (in long long newTimestampMillis, in Array<AUTF8String> uploadedIds, in mozIBridgedSyncEngineCallback callback); */
    pub SetUploaded: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, newTimestampMillis: i64, uploadedIds: *const thin_vec::ThinVec<::nsstring::nsCString>, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void syncFinished (in mozIBridgedSyncEngineCallback callback); */
    pub SyncFinished: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void reset (in mozIBridgedSyncEngineCallback callback); */
    pub Reset: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,

    /* void wipe (in mozIBridgedSyncEngineCallback callback); */
    pub Wipe: unsafe extern "system" fn (this: *const mozIBridgedSyncEngine, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIBridgedSyncEngine {


    /// `readonly attribute long storageVersion;`
    #[inline]
    pub unsafe fn GetStorageVersion(&self, aStorageVersion: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageVersion)(self, aStorageVersion)
    }



    /// `readonly attribute boolean allowSkippedRecord;`
    #[inline]
    pub unsafe fn GetAllowSkippedRecord(&self, aAllowSkippedRecord: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowSkippedRecord)(self, aAllowSkippedRecord)
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



    /// `void getLastSync (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn GetLastSync(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetLastSync)(self, callback)
    }



    /// `void setLastSync (in long long lastSyncMillis, in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn SetLastSync(&self, lastSyncMillis: i64, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetLastSync)(self, lastSyncMillis, callback)
    }



    /// `void getSyncId (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn GetSyncId(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetSyncId)(self, callback)
    }



    /// `void resetSyncId (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn ResetSyncId(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).ResetSyncId)(self, callback)
    }



    /// `void ensureCurrentSyncId (in AUTF8String newSyncId, in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn EnsureCurrentSyncId(&self, newSyncId: *const ::nsstring::nsACString, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).EnsureCurrentSyncId)(self, newSyncId, callback)
    }



    /// `void syncStarted (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn SyncStarted(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).SyncStarted)(self, callback)
    }



    /// `void storeIncoming (in Array<AUTF8String> incomingEnvelopesAsJSON, in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn StoreIncoming(&self, incomingEnvelopesAsJSON: *const thin_vec::ThinVec<::nsstring::nsCString>, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).StoreIncoming)(self, incomingEnvelopesAsJSON, callback)
    }



    /// `void apply (in mozIBridgedSyncEngineApplyCallback callback);`
    #[inline]
    pub unsafe fn Apply(&self, callback: *const mozIBridgedSyncEngineApplyCallback) -> ::nserror::nsresult {
        ((*self.vtable).Apply)(self, callback)
    }



    /// `void setUploaded (in long long newTimestampMillis, in Array<AUTF8String> uploadedIds, in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn SetUploaded(&self, newTimestampMillis: i64, uploadedIds: *const thin_vec::ThinVec<::nsstring::nsCString>, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetUploaded)(self, newTimestampMillis, uploadedIds, callback)
    }



    /// `void syncFinished (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn SyncFinished(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).SyncFinished)(self, callback)
    }



    /// `void reset (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn Reset(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, callback)
    }



    /// `void wipe (in mozIBridgedSyncEngineCallback callback);`
    #[inline]
    pub unsafe fn Wipe(&self, callback: *const mozIBridgedSyncEngineCallback) -> ::nserror::nsresult {
        ((*self.vtable).Wipe)(self, callback)
    }


}


