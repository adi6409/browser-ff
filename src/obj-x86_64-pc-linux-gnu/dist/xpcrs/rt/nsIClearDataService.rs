//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/cleardata/nsIClearDataService.idl
//


/// `interface nsIClearDataService : nsISupports`
///

/// ```text
/// /**
///  * nsIClearDataService
///  *
///  * Provides methods for cleaning data from a nsIPrincipal and/or from a time
///  * range.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClearDataService {
    vtable: *const nsIClearDataServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClearDataService.
unsafe impl XpCom for nsIClearDataService {
    const IID: nsIID = nsID(0x6ef3ef16, 0xa502, 0x4576,
        [0x9f, 0xb4, 0x91, 0x9f, 0x1c, 0x40, 0xbf, 0x61]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClearDataService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClearDataService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClearDataServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIClearDataService`.
    fn coerce_from(v: &nsIClearDataService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClearDataServiceCoerce for nsIClearDataService {
    #[inline]
    fn coerce_from(v: &nsIClearDataService) -> &Self {
        v
    }
}

impl nsIClearDataService {
    /// Cast this `nsIClearDataService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClearDataServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClearDataService {
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
impl<T: nsISupportsCoerce> nsIClearDataServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClearDataService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClearDataService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClearDataServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void deleteDataFromLocalFiles (in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
    pub DeleteDataFromLocalFiles: unsafe extern "system" fn (this: *const nsIClearDataService, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult,

    /* void deleteDataFromHost (in AUTF8String aHost, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
    pub DeleteDataFromHost: unsafe extern "system" fn (this: *const nsIClearDataService, aHost: *const ::nsstring::nsACString, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult,

    /* void deleteDataFromPrincipal (in nsIPrincipal aPrincipal, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
    pub DeleteDataFromPrincipal: unsafe extern "system" fn (this: *const nsIClearDataService, aPrincipal: *const nsIPrincipal, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult,

    /* void deleteDataInTimeRange (in PRTime aFrom, in PRTime aTo, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
    pub DeleteDataInTimeRange: unsafe extern "system" fn (this: *const nsIClearDataService, aFrom: PRTime, aTo: PRTime, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult,

    /* void deleteData (in uint32_t aFlags, in nsIClearDataCallback aCallback); */
    pub DeleteData: unsafe extern "system" fn (this: *const nsIClearDataService, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult,

    /* void deleteDataFromOriginAttributesPattern (in jsval aOriginAttributesPattern, [optional] in nsIClearDataCallback aCallback); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub DeleteDataFromOriginAttributesPattern: *const ::libc::c_void,

    /* void deleteUserInteractionForClearingHistory (in Array<nsIPrincipal> aPrincipalsWithStorage, [optional] in PRTime aFrom, [optional] in nsIClearDataCallback aCallback); */
    pub DeleteUserInteractionForClearingHistory: unsafe extern "system" fn (this: *const nsIClearDataService, aPrincipalsWithStorage: *const thin_vec::ThinVec<RefPtr<nsIPrincipal>>, aFrom: PRTime, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClearDataService {
    /// ```text
    /// /**************************************************************************
    ///    * Listed below are the various flags which may be or'd together.
    ///    */
    /// /**
    ///    * Delete cookies.
    ///    */
    /// ```
    ///

    pub const CLEAR_COOKIES: i64 = 1;

    /// ```text
    /// /**
    ///    * Network Cache.
    ///    */
    /// ```
    ///

    pub const CLEAR_NETWORK_CACHE: i64 = 2;

    /// ```text
    /// /**
    ///    * Image cache.
    ///    */
    /// ```
    ///

    pub const CLEAR_IMAGE_CACHE: i64 = 4;

    /// ```text
    /// /**
    ///    * Data stored by external plugins.
    ///    */
    /// ```
    ///

    pub const CLEAR_PLUGIN_DATA: i64 = 8;

    /// ```text
    /// /**
    ///    * Completed downloads.
    ///    */
    /// ```
    ///

    pub const CLEAR_DOWNLOADS: i64 = 16;

    /// ```text
    /// /**
    ///    * Stored passwords.
    ///    */
    /// ```
    ///

    pub const CLEAR_PASSWORDS: i64 = 32;

    /// ```text
    /// /**
    ///    * Media devices.
    ///    */
    /// ```
    ///

    pub const CLEAR_MEDIA_DEVICES: i64 = 64;

    /// ```text
    /// /**
    ///    * AppCache.
    ///    */
    /// ```
    ///

    pub const CLEAR_APPCACHE: i64 = 128;

    /// ```text
    /// /**
    ///    * LocalStorage, IndexedDB, ServiceWorkers, DOM Cache and so on.
    ///    */
    /// ```
    ///

    pub const CLEAR_DOM_QUOTA: i64 = 256;

    /// ```text
    /// /**
    ///    * Predictor network data
    ///    */
    /// ```
    ///

    pub const CLEAR_PREDICTOR_NETWORK_DATA: i64 = 512;

    /// ```text
    /// /**
    ///    * DOM Push notifications
    ///    */
    /// ```
    ///

    pub const CLEAR_DOM_PUSH_NOTIFICATIONS: i64 = 1024;

    /// ```text
    /// /**
    ///    * Places history
    ///    */
    /// ```
    ///

    pub const CLEAR_HISTORY: i64 = 2048;

    /// ```text
    /// /**
    ///    * Session history
    ///    */
    /// ```
    ///

    pub const CLEAR_SESSION_HISTORY: i64 = 4096;

    /// ```text
    /// /**
    ///    * Auth tokens
    ///    */
    /// ```
    ///

    pub const CLEAR_AUTH_TOKENS: i64 = 8192;

    /// ```text
    /// /**
    ///    * Login cache
    ///    */
    /// ```
    ///

    pub const CLEAR_AUTH_CACHE: i64 = 16384;

    /// ```text
    /// /**
    ///    * Site permissions
    ///    */
    /// ```
    ///

    pub const CLEAR_PERMISSIONS: i64 = 32768;

    /// ```text
    /// /**
    ///    * Site preferences
    ///    */
    /// ```
    ///

    pub const CLEAR_CONTENT_PREFERENCES: i64 = 65536;

    /// ```text
    /// /**
    ///    * Secure site settings
    ///    */
    /// ```
    ///

    pub const CLEAR_SECURITY_SETTINGS: i64 = 131072;

    /// ```text
    /// /**
    ///    * Media plugin data
    ///    */
    /// ```
    ///

    pub const CLEAR_EME: i64 = 262144;

    /// ```text
    /// /**
    ///    * Reporting API reports.
    ///    */
    /// ```
    ///

    pub const CLEAR_REPORTS: i64 = 524288;

    /// ```text
    /// /**
    ///    * StorageAccessAPI flag, which indicates user interaction.
    ///    */
    /// ```
    ///

    pub const CLEAR_STORAGE_ACCESS: i64 = 1048576;

    /// ```text
    /// /**
    ///    * Clear Cert Exceptions.
    ///    */
    /// ```
    ///

    pub const CLEAR_CERT_EXCEPTIONS: i64 = 2097152;

    /// ```text
    /// /**
    ///    * Clear entries in the content blocking database.
    ///    */
    /// ```
    ///

    pub const CLEAR_CONTENT_BLOCKING_RECORDS: i64 = 4194304;

    /// ```text
    /// /**
    ///    * Clear the in-memory CSS cache.
    ///    */
    /// ```
    ///

    pub const CLEAR_CSS_CACHE: i64 = 8388608;

    /// ```text
    /// /**
    ///    * Use this value to delete all the data.
    ///    */
    /// ```
    ///

    pub const CLEAR_ALL: i64 = 16777215;

    /// ```text
    /// /**************************************************************************
    ///    * The following flags are helpers: they combine some of the previous flags
    ///    * in a more convenient way.
    ///    */
    /// /**
    ///    * Delete all the possible caches.
    ///    */
    /// ```
    ///

    pub const CLEAR_ALL_CACHES: i64 = 8388614;

    /// ```text
    /// /**
    ///    * Delete all DOM storages
    ///    */
    /// ```
    ///

    pub const CLEAR_DOM_STORAGES: i64 = 525696;

    /// ```text
    /// /**
    ///    * Helper flag for forget about site
    ///    */
    /// ```
    ///

    pub const CLEAR_FORGET_ABOUT_SITE: i64 = 11509695;

    /// ```text
    /// /**
    ///    * Delete data owned by local files or other hostless schemes.
    ///    * @param aIsUserRequest true if this request comes from a user interaction.
    ///    *        This information is important because if true, it's probably better
    ///    *        to remove more than less, for privacy reason. If false (e.g.
        ///    *        Clear-Site-Data header), we don't want to delete more than what is
    ///    *        strictly required.
    ///    * @param aFlags List of flags. See below the accepted values.
    ///                    Note that not all flags will make sense (e.g. we can't clear
        ///                    certificates for local files). Nonsensical flags will be
    ///                    ignored.
    ///    * @param aCallback this callback will be executed when the operation is
    ///    *                  completed.
    ///    */
    /// ```
    ///

    /// `void deleteDataFromLocalFiles (in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback);`
    #[inline]
    pub unsafe fn DeleteDataFromLocalFiles(&self, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).DeleteDataFromLocalFiles)(self, aIsUserRequest, aFlags, aCallback)
    }


    /// ```text
    /// /**
    ///    * Delete data owned by a host. For instance: mozilla.org. Data from any
    ///    * possible originAttributes will be deleted.
    ///    * @param aHost the host to be used.
    ///    * @param aIsUserRequest true if this request comes from a user interaction.
    ///    *        This information is important because if true, it's probably better
    ///    *        to remove more than less, for privacy reason. If false (e.g.
        ///    *        Clear-Site-Data header), we don't want to delete more than what is
    ///    *        strictly required.
    ///    * @param aFlags List of flags. See below the accepted values.
    ///    * @param aCallback this callback will be executed when the operation is
    ///    *                  completed.
    ///    */
    /// ```
    ///

    /// `void deleteDataFromHost (in AUTF8String aHost, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback);`
    #[inline]
    pub unsafe fn DeleteDataFromHost(&self, aHost: *const ::nsstring::nsACString, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).DeleteDataFromHost)(self, aHost, aIsUserRequest, aFlags, aCallback)
    }


    /// ```text
    /// /**
    ///    * Delete data owned by a principal.
    ///    * @param aPrincipal the nsIPrincipal to be used.
    ///    * @param aIsUserRequest true if this request comes from a user interaction.
    ///    *        This information is important because if true, it's probably better
    ///    *        to remove more than less, for privacy reason. If false (e.g.
        ///    *        Clear-Site-Data header), we don't want to delete more than what is
    ///    *        strictly required.
    ///    * @param aFlags List of flags. See below the accepted values.
    ///    * @param aCallback ths callback will be executed when the operation is
    ///    *                  completed.
    ///    */
    /// ```
    ///

    /// `void deleteDataFromPrincipal (in nsIPrincipal aPrincipal, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback);`
    #[inline]
    pub unsafe fn DeleteDataFromPrincipal(&self, aPrincipal: *const nsIPrincipal, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).DeleteDataFromPrincipal)(self, aPrincipal, aIsUserRequest, aFlags, aCallback)
    }


    /// ```text
    /// /**
    ///    * Delete all data in a time range. Limit excluded.
    ///    * @param aFrom microseconds from the epoch
    ///    * @param aTo microseconds from the epoch
    ///    * @param aIsUserRequest true if this request comes from a user interaction.
    ///    *        This information is important because if true, it's probably better
    ///    *        to remove more than less, for privacy reason. If false (e.g.
        ///    *        Clear-Site-Data header), we don't want to delete more than what is
    ///    *        strictly required.
    ///    * @param aFlags List of flags. See below the accepted values.
    ///    * @param aCallback ths callback will be executed when the operation is
    ///    *                  completed.
    ///    */
    /// ```
    ///

    /// `void deleteDataInTimeRange (in PRTime aFrom, in PRTime aTo, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback);`
    #[inline]
    pub unsafe fn DeleteDataInTimeRange(&self, aFrom: PRTime, aTo: PRTime, aIsUserRequest: bool, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).DeleteDataInTimeRange)(self, aFrom, aTo, aIsUserRequest, aFlags, aCallback)
    }


    /// ```text
    /// /**
    ///    * Delete all data from any host, in any time range.
    ///    * @param aFlags List of flags. See below the accepted values.
    ///    * @param aCallback ths callback will be executed when the operation is
    ///    *                  completed.
    ///    */
    /// ```
    ///

    /// `void deleteData (in uint32_t aFlags, in nsIClearDataCallback aCallback);`
    #[inline]
    pub unsafe fn DeleteData(&self, aFlags: uint32_t, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).DeleteData)(self, aFlags, aCallback)
    }


    /// ```text
    /// /**
    ///    * Delete all data from an OriginAttributesPatternDictionary.
    ///    * @param aOriginAttributesPattern the originAttributes dictionary.
    ///    * @param aCallback the optional callback will be executed when the operation
    ///    *                  is completed.
    ///    */
    /// ```
    ///

    /// `void deleteDataFromOriginAttributesPattern (in jsval aOriginAttributesPattern, [optional] in nsIClearDataCallback aCallback);`
    const _DeleteDataFromOriginAttributesPattern: () = ();

    /// ```text
    /// /**
    ///    * This is a helper function to clear storageAccessAPI permissions
    ///    * in a way that will not result in users getting logged out by
    ///    * cookie purging. To that end we only clear permissions for principals
    ///    * whose base domain does not have any storage associated with it.
    ///    *
    ///    * The principals to be considered will need to be passed by the API consumer.
    ///    * It is recommended to use PrincipalsCollector.jsm for that.
    ///    *
    ///    * @param aPrincipalsWithStorage principals to be excluded from clearing
    ///    * @param aFrom microseconds from the epoch
    ///    * @param aCallback the optional callback will be executed when the operation
    ///    *                  is completed.
    ///    */
    /// ```
    ///

    /// `void deleteUserInteractionForClearingHistory (in Array<nsIPrincipal> aPrincipalsWithStorage, [optional] in PRTime aFrom, [optional] in nsIClearDataCallback aCallback);`
    #[inline]
    pub unsafe fn DeleteUserInteractionForClearingHistory(&self, aPrincipalsWithStorage: *const thin_vec::ThinVec<RefPtr<nsIPrincipal>>, aFrom: PRTime, aCallback: *const nsIClearDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).DeleteUserInteractionForClearingHistory)(self, aPrincipalsWithStorage, aFrom, aCallback)
    }


}


/// `interface nsIClearDataCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClearDataCallback {
    vtable: *const nsIClearDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClearDataCallback.
unsafe impl XpCom for nsIClearDataCallback {
    const IID: nsIID = nsID(0xe225517b, 0x24c5, 0x498a,
        [0xb9, 0xfb, 0x99, 0x93, 0xe3, 0x41, 0xa3, 0x98]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClearDataCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClearDataCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClearDataCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIClearDataCallback`.
    fn coerce_from(v: &nsIClearDataCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClearDataCallbackCoerce for nsIClearDataCallback {
    #[inline]
    fn coerce_from(v: &nsIClearDataCallback) -> &Self {
        v
    }
}

impl nsIClearDataCallback {
    /// Cast this `nsIClearDataCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClearDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClearDataCallback {
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
impl<T: nsISupportsCoerce> nsIClearDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClearDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClearDataCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClearDataCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDataDeleted (in uint32_t aFailedFlags); */
    pub OnDataDeleted: unsafe extern "system" fn (this: *const nsIClearDataCallback, aFailedFlags: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClearDataCallback {

    /// ```text
    /// /**
    ///  * This is a companion interface for
    ///  * nsIClearDataService::deleteDataFromPrincipal().
    ///  */
    /// /**
    ///    * Called to indicate that the data cleaning is completed.
    ///    * @param aFailedFlags this value contains the flags that failed during the
    ///    *                     cleanup. If nothing failed, aFailedFlags will be 0.
    ///    */
    /// ```
    ///

    /// `void onDataDeleted (in uint32_t aFailedFlags);`
    #[inline]
    pub unsafe fn OnDataDeleted(&self, aFailedFlags: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).OnDataDeleted)(self, aFailedFlags)
    }


}


