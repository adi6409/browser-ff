//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIAsyncHistory.idl
//


/// `interface mozIVisitInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIVisitInfo {
    vtable: *const mozIVisitInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIVisitInfo.
unsafe impl XpCom for mozIVisitInfo {
    const IID: nsIID = nsID(0x41e4ccc9, 0xf0c8, 0x4cd7,
        [0x97, 0x53, 0x7a, 0x38, 0x51, 0x4b, 0x84, 0x88]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIVisitInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIVisitInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIVisitInfoCoerce {
    /// Cheaply cast a value of this type from a `mozIVisitInfo`.
    fn coerce_from(v: &mozIVisitInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIVisitInfoCoerce for mozIVisitInfo {
    #[inline]
    fn coerce_from(v: &mozIVisitInfo) -> &Self {
        v
    }
}

impl mozIVisitInfo {
    /// Cast this `mozIVisitInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIVisitInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIVisitInfo {
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
impl<T: nsISupportsCoerce> mozIVisitInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIVisitInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIVisitInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIVisitInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long long visitId; */
    pub GetVisitId: unsafe extern "system" fn (this: *const mozIVisitInfo, aVisitId: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute PRTime visitDate; */
    pub GetVisitDate: unsafe extern "system" fn (this: *const mozIVisitInfo, aVisitDate: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute unsigned long transitionType; */
    pub GetTransitionType: unsafe extern "system" fn (this: *const mozIVisitInfo, aTransitionType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsIURI referrerURI; */
    pub GetReferrerURI: unsafe extern "system" fn (this: *const mozIVisitInfo, aReferrerURI: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIVisitInfo {

    /// ```text
    /// /**
    ///    * The machine-local (internal) id of the visit.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long visitId;`
    #[inline]
    pub unsafe fn GetVisitId(&self, aVisitId: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetVisitId)(self, aVisitId)
    }


    /// ```text
    /// /**
    ///    * The time the visit occurred.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime visitDate;`
    #[inline]
    pub unsafe fn GetVisitDate(&self, aVisitDate: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetVisitDate)(self, aVisitDate)
    }


    /// ```text
    /// /**
    ///    * The transition type used to get to this visit.  One of the TRANSITION_TYPE
    ///    * constants on nsINavHistory.
    ///    *
    ///    * @see nsINavHistory.idl
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long transitionType;`
    #[inline]
    pub unsafe fn GetTransitionType(&self, aTransitionType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTransitionType)(self, aTransitionType)
    }


    /// ```text
    /// /**
    ///    * The referring URI of this visit.  This may be null.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI referrerURI;`
    #[inline]
    pub unsafe fn GetReferrerURI(&self, aReferrerURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerURI)(self, aReferrerURI)
    }


}


/// `interface mozIPlaceInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIPlaceInfo {
    vtable: *const mozIPlaceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIPlaceInfo.
unsafe impl XpCom for mozIPlaceInfo {
    const IID: nsIID = nsID(0xad83e137, 0xc92a, 0x4b7b,
        [0xb6, 0x7e, 0x0a, 0x31, 0x88, 0x11, 0xf9, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIPlaceInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIPlaceInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIPlaceInfoCoerce {
    /// Cheaply cast a value of this type from a `mozIPlaceInfo`.
    fn coerce_from(v: &mozIPlaceInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIPlaceInfoCoerce for mozIPlaceInfo {
    #[inline]
    fn coerce_from(v: &mozIPlaceInfo) -> &Self {
        v
    }
}

impl mozIPlaceInfo {
    /// Cast this `mozIPlaceInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIPlaceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIPlaceInfo {
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
impl<T: nsISupportsCoerce> mozIPlaceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPlaceInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIPlaceInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIPlaceInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long long placeId; */
    pub GetPlaceId: unsafe extern "system" fn (this: *const mozIPlaceInfo, aPlaceId: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute ACString guid; */
    pub GetGuid: unsafe extern "system" fn (this: *const mozIPlaceInfo, aGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIURI uri; */
    pub GetUri: unsafe extern "system" fn (this: *const mozIPlaceInfo, aUri: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const mozIPlaceInfo, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute long long frecency; */
    pub GetFrecency: unsafe extern "system" fn (this: *const mozIPlaceInfo, aFrecency: *mut i64) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval visits; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetVisits: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIPlaceInfo {

    /// ```text
    /// /**
    ///    * The machine-local (internal) id of the place.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long placeId;`
    #[inline]
    pub unsafe fn GetPlaceId(&self, aPlaceId: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetPlaceId)(self, aPlaceId)
    }


    /// ```text
    /// /**
    ///    * The globally unique id of the place.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString guid;`
    #[inline]
    pub unsafe fn GetGuid(&self, aGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetGuid)(self, aGuid)
    }


    /// ```text
    /// /**
    ///    * The URI of the place.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI uri;`
    #[inline]
    pub unsafe fn GetUri(&self, aUri: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetUri)(self, aUri)
    }


    /// ```text
    /// /**
    ///    * The title associated with the place.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///    * The frecency of the place.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long frecency;`
    #[inline]
    pub unsafe fn GetFrecency(&self, aFrecency: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetFrecency)(self, aFrecency)
    }


    /// ```text
    /// /**
    ///    * An array of mozIVisitInfo objects for the place.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval visits;`
    const _GetVisits: () = ();

}


/// `interface mozIVisitInfoCallback : nsISupports`
///

/// ```text
/// /**
///  * Shared Callback interface for mozIAsyncHistory methods. The semantics
///  * for each method are detailed in mozIAsyncHistory.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIVisitInfoCallback {
    vtable: *const mozIVisitInfoCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIVisitInfoCallback.
unsafe impl XpCom for mozIVisitInfoCallback {
    const IID: nsIID = nsID(0x1f266877, 0x2859, 0x418b,
        [0xa1, 0x1b, 0xec, 0x3a, 0xe4, 0xf4, 0xf9, 0x3d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIVisitInfoCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIVisitInfoCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIVisitInfoCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIVisitInfoCallback`.
    fn coerce_from(v: &mozIVisitInfoCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIVisitInfoCallbackCoerce for mozIVisitInfoCallback {
    #[inline]
    fn coerce_from(v: &mozIVisitInfoCallback) -> &Self {
        v
    }
}

impl mozIVisitInfoCallback {
    /// Cast this `mozIVisitInfoCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIVisitInfoCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIVisitInfoCallback {
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
impl<T: nsISupportsCoerce> mozIVisitInfoCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIVisitInfoCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIVisitInfoCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIVisitInfoCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo); */
    pub HandleError: unsafe extern "system" fn (this: *const mozIVisitInfoCallback, aResultCode: ::nserror::nsresult, aPlaceInfo: *const mozIPlaceInfo) -> ::nserror::nsresult,

    /* void handleResult (in mozIPlaceInfo aPlaceInfo); */
    pub HandleResult: unsafe extern "system" fn (this: *const mozIVisitInfoCallback, aPlaceInfo: *const mozIPlaceInfo) -> ::nserror::nsresult,

    /* void handleCompletion (in unsigned long aUpdatedItems); */
    pub HandleCompletion: unsafe extern "system" fn (this: *const mozIVisitInfoCallback, aUpdatedItems: u32) -> ::nserror::nsresult,

    /* readonly attribute bool ignoreResults; */
    pub GetIgnoreResults: unsafe extern "system" fn (this: *const mozIVisitInfoCallback, aIgnoreResults: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool ignoreErrors; */
    pub GetIgnoreErrors: unsafe extern "system" fn (this: *const mozIVisitInfoCallback, aIgnoreErrors: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIVisitInfoCallback {

    /// ```text
    /// /**
    ///    * Called when the given place could not be processed.
    ///    *
    ///    * @param aResultCode
    ///    *        nsresult indicating the failure reason.
    ///    * @param aPlaceInfo
    ///    *        The information that was given to the caller for the place.
    ///    */
    /// ```
    ///

    /// `void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo);`
    #[inline]
    pub unsafe fn HandleError(&self, aResultCode: ::nserror::nsresult, aPlaceInfo: *const mozIPlaceInfo) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, aResultCode, aPlaceInfo)
    }


    /// ```text
    /// /**
    ///    * Called for each place processed successfully.
    ///    *
    ///    * @param aPlaceInfo
    ///    *        The current info stored for the place.
    ///    */
    /// ```
    ///

    /// `void handleResult (in mozIPlaceInfo aPlaceInfo);`
    #[inline]
    pub unsafe fn HandleResult(&self, aPlaceInfo: *const mozIPlaceInfo) -> ::nserror::nsresult {
        ((*self.vtable).HandleResult)(self, aPlaceInfo)
    }


    /// ```text
    /// /**
    ///    * Called when all records were processed.
    ///    * @param aUpdatedItems
    ///    *        How many items were successfully updated.
    ///    */
    /// ```
    ///

    /// `void handleCompletion (in unsigned long aUpdatedItems);`
    #[inline]
    pub unsafe fn HandleCompletion(&self, aUpdatedItems: u32) -> ::nserror::nsresult {
        ((*self.vtable).HandleCompletion)(self, aUpdatedItems)
    }


    /// ```text
    /// /**
    ///    * These two attributes govern whether we attempt to call
    ///    * handleResult and handleError, respectively, if/once
    ///    * results/errors occur.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool ignoreResults;`
    #[inline]
    pub unsafe fn GetIgnoreResults(&self, aIgnoreResults: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIgnoreResults)(self, aIgnoreResults)
    }



    /// `readonly attribute bool ignoreErrors;`
    #[inline]
    pub unsafe fn GetIgnoreErrors(&self, aIgnoreErrors: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIgnoreErrors)(self, aIgnoreErrors)
    }


}


/// `interface mozIVisitedStatusCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIVisitedStatusCallback {
    vtable: *const mozIVisitedStatusCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIVisitedStatusCallback.
unsafe impl XpCom for mozIVisitedStatusCallback {
    const IID: nsIID = nsID(0x994092bf, 0x936f, 0x449b,
        [0x8d, 0xd6, 0x09, 0x41, 0xe0, 0x24, 0x36, 0x0d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIVisitedStatusCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIVisitedStatusCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIVisitedStatusCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIVisitedStatusCallback`.
    fn coerce_from(v: &mozIVisitedStatusCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIVisitedStatusCallbackCoerce for mozIVisitedStatusCallback {
    #[inline]
    fn coerce_from(v: &mozIVisitedStatusCallback) -> &Self {
        v
    }
}

impl mozIVisitedStatusCallback {
    /// Cast this `mozIVisitedStatusCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIVisitedStatusCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIVisitedStatusCallback {
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
impl<T: nsISupportsCoerce> mozIVisitedStatusCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIVisitedStatusCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIVisitedStatusCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIVisitedStatusCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void isVisited (in nsIURI aURI, in boolean aVisitedStatus); */
    pub IsVisited: unsafe extern "system" fn (this: *const mozIVisitedStatusCallback, aURI: *const nsIURI, aVisitedStatus: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIVisitedStatusCallback {

    /// ```text
    /// /**
    ///    * Notifies whether a certain URI has been visited.
    ///    *
    ///    * @param aURI
    ///    *        URI being notified about.
    ///    * @param aVisitedStatus
    ///    *        The visited status of aURI.
    ///    */
    /// ```
    ///

    /// `void isVisited (in nsIURI aURI, in boolean aVisitedStatus);`
    #[inline]
    pub unsafe fn IsVisited(&self, aURI: *const nsIURI, aVisitedStatus: bool) -> ::nserror::nsresult {
        ((*self.vtable).IsVisited)(self, aURI, aVisitedStatus)
    }


}


/// `interface mozIAsyncHistory : nsISupports`
///

/// ```text
/// /**
///  * This interface contains APIs for cpp consumers.
///  * Javascript consumers should look at History.jsm instead,
///  * that is exposed through PlacesUtils.history.
///  *
///  * If you're evaluating adding a new history API, it should
///  * usually go to History.jsm, unless it needs to do long and
///  * expensive work in a batch, then it could be worth doing
///  * that in History.cpp.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIAsyncHistory {
    vtable: *const mozIAsyncHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIAsyncHistory.
unsafe impl XpCom for mozIAsyncHistory {
    const IID: nsIID = nsID(0x1643efd2, 0xa329, 0x4733,
        [0xa3, 0x9d, 0x17, 0x06, 0x9c, 0x8d, 0x3b, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIAsyncHistory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIAsyncHistory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIAsyncHistoryCoerce {
    /// Cheaply cast a value of this type from a `mozIAsyncHistory`.
    fn coerce_from(v: &mozIAsyncHistory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIAsyncHistoryCoerce for mozIAsyncHistory {
    #[inline]
    fn coerce_from(v: &mozIAsyncHistory) -> &Self {
        v
    }
}

impl mozIAsyncHistory {
    /// Cast this `mozIAsyncHistory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIAsyncHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIAsyncHistory {
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
impl<T: nsISupportsCoerce> mozIAsyncHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIAsyncHistory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIAsyncHistory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIAsyncHistoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void updatePlaces (in jsval aPlaceInfo, [optional] in mozIVisitInfoCallback aCallback); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub UpdatePlaces: *const ::libc::c_void,

    /* void isURIVisited (in nsIURI aURI, in mozIVisitedStatusCallback aCallback); */
    pub IsURIVisited: unsafe extern "system" fn (this: *const mozIAsyncHistory, aURI: *const nsIURI, aCallback: *const mozIVisitedStatusCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIAsyncHistory {

    /// ```text
    /// /**
    ///    * Adds a set of visits for one or more mozIPlaceInfo objects, and updates
    ///    * each mozIPlaceInfo's title or guid.
    ///    *
    ///    * aCallback.handleResult is called for each visit added.
    ///    *
    ///    * @param aPlaceInfo
    ///    *        The mozIPlaceInfo object[s] containing the information to store or
    ///    *        update.  This can be a single object, or an array of objects.
    ///    * @param [optional] aCallback
    ///    *        A mozIVisitInfoCallback object which consists of callbacks to be
    ///    *        notified for successful and/or failed changes.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *         - Passing in NULL for aPlaceInfo.
    ///    *         - Not providing at least one valid guid, or uri for all
    ///    *           mozIPlaceInfo object[s].
    ///    *         - Not providing an array or nothing for the visits property of
    ///    *           mozIPlaceInfo.
    ///    *         - Not providing a visitDate and transitionType for each
    ///    *           mozIVisitInfo.
    ///    *         - Providing an invalid transitionType for a mozIVisitInfo.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void updatePlaces (in jsval aPlaceInfo, [optional] in mozIVisitInfoCallback aCallback);`
    const _UpdatePlaces: () = ();

    /// ```text
    /// /**
    ///    * Checks if a given URI has been visited.
    ///    *
    ///    * @param aURI
    ///    *        The URI to check for.
    ///    * @param aCallback
    ///    *        A mozIVisitStatusCallback object which receives the visited status.
    ///    */
    /// ```
    ///

    /// `void isURIVisited (in nsIURI aURI, in mozIVisitedStatusCallback aCallback);`
    #[inline]
    pub unsafe fn IsURIVisited(&self, aURI: *const nsIURI, aCallback: *const mozIVisitedStatusCallback) -> ::nserror::nsresult {
        ((*self.vtable).IsURIVisited)(self, aURI, aCallback)
    }


}


