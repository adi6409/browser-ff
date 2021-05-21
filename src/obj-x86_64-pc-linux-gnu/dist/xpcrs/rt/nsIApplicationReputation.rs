//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/reputationservice/nsIApplicationReputation.idl
//


/// `interface nsIApplicationReputationService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationReputationService {
    vtable: *const nsIApplicationReputationServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationReputationService.
unsafe impl XpCom for nsIApplicationReputationService {
    const IID: nsIID = nsID(0xc9f03479, 0xfd68, 0x4393,
        [0xac, 0xb2, 0xc8, 0x8d, 0x4f, 0x56, 0x31, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationReputationService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationReputationService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationReputationServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationReputationService`.
    fn coerce_from(v: &nsIApplicationReputationService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationReputationServiceCoerce for nsIApplicationReputationService {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationService) -> &Self {
        v
    }
}

impl nsIApplicationReputationService {
    /// Cast this `nsIApplicationReputationService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationReputationServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationReputationService {
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
impl<T: nsISupportsCoerce> nsIApplicationReputationServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationReputationService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationReputationServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback); */
    pub QueryReputation: unsafe extern "system" fn (this: *const nsIApplicationReputationService, aQuery: *const nsIApplicationReputationQuery, aCallback: *const nsIApplicationReputationCallback) -> ::nserror::nsresult,

    /* bool isBinary (in AUTF8String aFilename); */
    pub IsBinary: unsafe extern "system" fn (this: *const nsIApplicationReputationService, aFilename: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationReputationService {
    /// ```text
    /// /**
    ///    * Indicates the reason for the application reputation block.
    ///    */
    /// ```
    ///

    pub const VERDICT_SAFE: i64 = 0;


    pub const VERDICT_DANGEROUS: i64 = 1;


    pub const VERDICT_UNCOMMON: i64 = 2;


    pub const VERDICT_POTENTIALLY_UNWANTED: i64 = 3;


    pub const VERDICT_DANGEROUS_HOST: i64 = 4;

    /// ```text
    /// /**
    ///    * Start querying the application reputation service.
    ///    *
    ///    * @param aQuery
    ///    *        The nsIApplicationReputationQuery containing metadata of the
    ///    *        downloaded file.
    ///    *
    ///    * @param aCallback
    ///    *        The callback for receiving the results of the query.
    ///    *
    ///    * @remarks aCallback may not be null.  onComplete is guaranteed to be called
    ///    *          on aCallback. This function may not be called more than once with
    ///    *          the same query object. If any of the attributes of aQuery have
    ///    *          not been set or have been set with empty data (with the exception
        ///    *          of sourceURI), then a valid request can still be constructed and
    ///    *          will solicit a valid response, but won't produce any useful
    ///    *          information.
    ///    */
    /// ```
    ///

    /// `void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback);`
    #[inline]
    pub unsafe fn QueryReputation(&self, aQuery: *const nsIApplicationReputationQuery, aCallback: *const nsIApplicationReputationCallback) -> ::nserror::nsresult {
        ((*self.vtable).QueryReputation)(self, aQuery, aCallback)
    }


    /// ```text
    /// /**
    ///    * Check if a file with this name should be treated as a binary executable,
    ///    * and is therefore subject to application reputation checks.
    ///    * Will return true if the filename's extension is either:
    ///    * - in kBinaryFileExtensions in ApplicationReputation.cpp
    ///    * - in sExecutableExts in nsLocalFileCommon.h *and* not in
    ///    *   kNonBinaryExecutables in ApplicationReputation.cpp
    ///    *
    ///    * @param aFilename
    ///    *        The filename to check.
    ///    */
    /// ```
    ///

    /// `bool isBinary (in AUTF8String aFilename);`
    #[inline]
    pub unsafe fn IsBinary(&self, aFilename: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsBinary)(self, aFilename, _retval)
    }


}


/// `interface nsIApplicationReputationQuery : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationReputationQuery {
    vtable: *const nsIApplicationReputationQueryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationReputationQuery.
unsafe impl XpCom for nsIApplicationReputationQuery {
    const IID: nsIID = nsID(0x812d7509, 0xa9a3, 0x446e,
        [0xa6, 0x6f, 0x3e, 0xd8, 0xcc, 0x91, 0xeb, 0xd0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationReputationQuery {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationReputationQuery.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationReputationQueryCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationReputationQuery`.
    fn coerce_from(v: &nsIApplicationReputationQuery) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationReputationQueryCoerce for nsIApplicationReputationQuery {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationQuery) -> &Self {
        v
    }
}

impl nsIApplicationReputationQuery {
    /// Cast this `nsIApplicationReputationQuery` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationReputationQueryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationReputationQuery {
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
impl<T: nsISupportsCoerce> nsIApplicationReputationQueryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationQuery) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationReputationQuery
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationReputationQueryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI sourceURI; */
    pub GetSourceURI: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aSourceURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIReferrerInfo referrerInfo; */
    pub GetReferrerInfo: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String suggestedFileName; */
    pub GetSuggestedFileName: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aSuggestedFileName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long fileSize; */
    pub GetFileSize: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aFileSize: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute ACString sha256Hash; */
    pub GetSha256Hash: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aSha256Hash: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute Array<Array<Array<uint8_t>>> signatureInfo; */
    pub GetSignatureInfo: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aSignatureInfo: *mut thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>) -> ::nserror::nsresult,

    /* readonly attribute nsIArray redirects; */
    pub GetRedirects: unsafe extern "system" fn (this: *const nsIApplicationReputationQuery, aRedirects: *mut*const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationReputationQuery {

    /// ```text
    /// /**
    ///  * A single-use, write-once interface for recording the metadata of the
    ///  * downloaded file. nsIApplicationReputationService.Start() may only be called
    ///  * once with a single query.
    ///  */
    /// ```
    ///

    /// `readonly attribute nsIURI sourceURI;`
    #[inline]
    pub unsafe fn GetSourceURI(&self, aSourceURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceURI)(self, aSourceURI)
    }



    /// `readonly attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn GetReferrerInfo(&self, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerInfo)(self, aReferrerInfo)
    }



    /// `readonly attribute AUTF8String suggestedFileName;`
    #[inline]
    pub unsafe fn GetSuggestedFileName(&self, aSuggestedFileName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSuggestedFileName)(self, aSuggestedFileName)
    }



    /// `readonly attribute unsigned long fileSize;`
    #[inline]
    pub unsafe fn GetFileSize(&self, aFileSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFileSize)(self, aFileSize)
    }



    /// `readonly attribute ACString sha256Hash;`
    #[inline]
    pub unsafe fn GetSha256Hash(&self, aSha256Hash: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSha256Hash)(self, aSha256Hash)
    }



    /// `readonly attribute Array<Array<Array<uint8_t>>> signatureInfo;`
    #[inline]
    pub unsafe fn GetSignatureInfo(&self, aSignatureInfo: *mut thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>) -> ::nserror::nsresult {
        ((*self.vtable).GetSignatureInfo)(self, aSignatureInfo)
    }



    /// `readonly attribute nsIArray redirects;`
    #[inline]
    pub unsafe fn GetRedirects(&self, aRedirects: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetRedirects)(self, aRedirects)
    }


}


/// `interface nsIApplicationReputationCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationReputationCallback {
    vtable: *const nsIApplicationReputationCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationReputationCallback.
unsafe impl XpCom for nsIApplicationReputationCallback {
    const IID: nsIID = nsID(0x9a228470, 0xcfe5, 0x11e2,
        [0x8b, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationReputationCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationReputationCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationReputationCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationReputationCallback`.
    fn coerce_from(v: &nsIApplicationReputationCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationReputationCallbackCoerce for nsIApplicationReputationCallback {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationCallback) -> &Self {
        v
    }
}

impl nsIApplicationReputationCallback {
    /// Cast this `nsIApplicationReputationCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationReputationCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationReputationCallback {
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
impl<T: nsISupportsCoerce> nsIApplicationReputationCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationReputationCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationReputationCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict); */
    pub OnComplete: unsafe extern "system" fn (this: *const nsIApplicationReputationCallback, aShouldBlock: bool, aStatus: ::nserror::nsresult, aVerdict: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationReputationCallback {

    /// ```text
    /// /**
    ///    * Callback for the result of the application reputation query.
    ///    * @param aStatus
    ///    *        NS_OK if and only if the query succeeded. If it did, then
    ///    *        shouldBlock is meaningful (otherwise it defaults to false). This
    ///    *        may be NS_ERROR_FAILURE if the response cannot be parsed, or
    ///    *        NS_ERROR_NOT_AVAILABLE if the service has been disabled or is not
    ///    *        reachable.
    ///    * @param aShouldBlock
    ///    *        Whether or not the download should be blocked.
    ///    * @param aVerdict
    ///    *        Indicates the result of the lookup that determines whether the
    ///    *        download should be blocked, according to the "VERDICT_" constants.
    ///    *        This may be set to a value different than "VERDICT_SAFE" even if
    ///    *        aShouldBlock is false, so you should always check aShouldBlock.
    ///    */
    /// ```
    ///

    /// `void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict);`
    #[inline]
    pub unsafe fn OnComplete(&self, aShouldBlock: bool, aStatus: ::nserror::nsresult, aVerdict: u32) -> ::nserror::nsresult {
        ((*self.vtable).OnComplete)(self, aShouldBlock, aStatus, aVerdict)
    }


}


