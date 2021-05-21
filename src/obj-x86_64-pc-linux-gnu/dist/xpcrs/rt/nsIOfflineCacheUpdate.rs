//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/prefetch/nsIOfflineCacheUpdate.idl
//


/// `interface nsIOfflineCacheUpdateObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOfflineCacheUpdateObserver {
    vtable: *const nsIOfflineCacheUpdateObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOfflineCacheUpdateObserver.
unsafe impl XpCom for nsIOfflineCacheUpdateObserver {
    const IID: nsIID = nsID(0x47360d57, 0x8ef4, 0x4a5d,
        [0x88, 0x65, 0x1a, 0x27, 0xa7, 0x39, 0xad, 0x1a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOfflineCacheUpdateObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOfflineCacheUpdateObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOfflineCacheUpdateObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIOfflineCacheUpdateObserver`.
    fn coerce_from(v: &nsIOfflineCacheUpdateObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOfflineCacheUpdateObserverCoerce for nsIOfflineCacheUpdateObserver {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateObserver) -> &Self {
        v
    }
}

impl nsIOfflineCacheUpdateObserver {
    /// Cast this `nsIOfflineCacheUpdateObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOfflineCacheUpdateObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOfflineCacheUpdateObserver {
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
impl<T: nsISupportsCoerce> nsIOfflineCacheUpdateObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOfflineCacheUpdateObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOfflineCacheUpdateObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state); */
    pub UpdateStateChanged: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateObserver, aUpdate: *const nsIOfflineCacheUpdate, state: uint32_t) -> ::nserror::nsresult,

    /* void applicationCacheAvailable (in nsIApplicationCache applicationCache); */
    pub ApplicationCacheAvailable: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateObserver, applicationCache: *const nsIApplicationCache) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOfflineCacheUpdateObserver {

    pub const STATE_ERROR: i64 = 1;


    pub const STATE_CHECKING: i64 = 2;


    pub const STATE_NOUPDATE: i64 = 3;


    pub const STATE_OBSOLETE: i64 = 4;


    pub const STATE_DOWNLOADING: i64 = 5;


    pub const STATE_ITEMSTARTED: i64 = 6;


    pub const STATE_ITEMCOMPLETED: i64 = 7;


    pub const STATE_ITEMPROGRESS: i64 = 8;


    pub const STATE_FINISHED: i64 = 10;

    /// ```text
    /// /**
    ///    * aUpdate has changed its state.
    ///    *
    ///    * @param aUpdate
    ///    *        The nsIOfflineCacheUpdate being processed.
    ///    * @param event
    ///    *        See enumeration above
    ///    */
    /// ```
    ///

    /// `void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state);`
    #[inline]
    pub unsafe fn UpdateStateChanged(&self, aUpdate: *const nsIOfflineCacheUpdate, state: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).UpdateStateChanged)(self, aUpdate, state)
    }


    /// ```text
    /// /**
    ///    * Informs the observer about an application being available to associate.
    ///    *
    ///    * @param applicationCache
    ///    *        The application cache instance that has been created or found by the
    ///    *        update to associate with
    ///    */
    /// ```
    ///

    /// `void applicationCacheAvailable (in nsIApplicationCache applicationCache);`
    #[inline]
    pub unsafe fn ApplicationCacheAvailable(&self, applicationCache: *const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).ApplicationCacheAvailable)(self, applicationCache)
    }


}


/// `interface nsIOfflineCacheUpdate : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOfflineCacheUpdate {
    vtable: *const nsIOfflineCacheUpdateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOfflineCacheUpdate.
unsafe impl XpCom for nsIOfflineCacheUpdate {
    const IID: nsIID = nsID(0x6e3e26ea, 0x45b2, 0x4db7,
        [0x9e, 0x4a, 0x93, 0xb9, 0x65, 0x67, 0x92, 0x98]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOfflineCacheUpdate {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOfflineCacheUpdate.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOfflineCacheUpdateCoerce {
    /// Cheaply cast a value of this type from a `nsIOfflineCacheUpdate`.
    fn coerce_from(v: &nsIOfflineCacheUpdate) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOfflineCacheUpdateCoerce for nsIOfflineCacheUpdate {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdate) -> &Self {
        v
    }
}

impl nsIOfflineCacheUpdate {
    /// Cast this `nsIOfflineCacheUpdate` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOfflineCacheUpdateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOfflineCacheUpdate {
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
impl<T: nsISupportsCoerce> nsIOfflineCacheUpdateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdate) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOfflineCacheUpdate
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOfflineCacheUpdateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short status; */
    pub GetStatus: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aStatus: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute boolean partial; */
    pub GetPartial: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aPartial: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isUpgrade; */
    pub GetIsUpgrade: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aIsUpgrade: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute ACString updateDomain; */
    pub GetUpdateDomain: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aUpdateDomain: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIURI manifestURI; */
    pub GetManifestURI: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal loadingPrincipal; */
    pub GetLoadingPrincipal: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aLoadingPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute boolean succeeded; */
    pub GetSucceeded: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aSucceeded: *mut bool) -> ::nserror::nsresult,

    /* void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument, [optional] in nsIFile aCustomProfileDir); */
    pub Init: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aDocument: *const libc::c_void, aCustomProfileDir: *const nsIFile) -> ::nserror::nsresult,

    /* void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal, in nsICookieJarSettings aCookieJarSettings); */
    pub InitPartial: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *const nsIURI, aClientID: *const ::nsstring::nsACString, aDocumentURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCookieJarSettings: *const nsICookieJarSettings) -> ::nserror::nsresult,

    /* void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
    pub InitForUpdateCheck: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void addDynamicURI (in nsIURI aURI); */
    pub AddDynamicURI: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* void schedule (); */
    pub Schedule: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate) -> ::nserror::nsresult,

    /* void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aObserver: *const nsIOfflineCacheUpdateObserver, aHoldWeak: bool) -> ::nserror::nsresult,

    /* void removeObserver (in nsIOfflineCacheUpdateObserver aObserver); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aObserver: *const nsIOfflineCacheUpdateObserver) -> ::nserror::nsresult,

    /* void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate) -> ::nserror::nsresult,

    /* readonly attribute uint64_t byteProgress; */
    pub GetByteProgress: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdate, aByteProgress: *mut uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOfflineCacheUpdate {

    /// ```text
    /// /**
    ///  * An nsIOfflineCacheUpdate is used to update an application's offline
    ///  * resources.
    ///  *
    ///  * It can be used to perform partial or complete updates.
    ///  *
    ///  * One update object will be updating at a time.  The active object will
    ///  * load its items one by one, sending itemCompleted() to any registered
    ///  * observers.
    ///  */
    /// /**
    ///    * Fetch the status of the running update.  This will return a value
    ///    * defined in OfflineResourceList.webidl.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned short status;`
    #[inline]
    pub unsafe fn GetStatus(&self, aStatus: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetStatus)(self, aStatus)
    }


    /// ```text
    /// /**
    ///    * TRUE if the update is being used to add specific resources.
    ///    * FALSE if the complete cache update process is happening.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean partial;`
    #[inline]
    pub unsafe fn GetPartial(&self, aPartial: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPartial)(self, aPartial)
    }


    /// ```text
    /// /**
    ///    * TRUE if this is an upgrade attempt, FALSE if it is a new cache
    ///    * attempt.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isUpgrade;`
    #[inline]
    pub unsafe fn GetIsUpgrade(&self, aIsUpgrade: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsUpgrade)(self, aIsUpgrade)
    }


    /// ```text
    /// /**
    ///    * The domain being updated, and the domain that will own any URIs added
    ///    * with this update.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString updateDomain;`
    #[inline]
    pub unsafe fn GetUpdateDomain(&self, aUpdateDomain: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdateDomain)(self, aUpdateDomain)
    }


    /// ```text
    /// /**
    ///    * The manifest for the offline application being updated.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI manifestURI;`
    #[inline]
    pub unsafe fn GetManifestURI(&self, aManifestURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetManifestURI)(self, aManifestURI)
    }


    /// ```text
    /// /**
    ///    * The principal of the page that is requesting the update.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrincipal loadingPrincipal;`
    #[inline]
    pub unsafe fn GetLoadingPrincipal(&self, aLoadingPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadingPrincipal)(self, aLoadingPrincipal)
    }


    /// ```text
    /// /**
    ///    * TRUE if the cache update completed successfully.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean succeeded;`
    #[inline]
    pub unsafe fn GetSucceeded(&self, aSucceeded: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSucceeded)(self, aSucceeded)
    }


    /// ```text
    /// /**
    ///    * Initialize the update.
    ///    *
    ///    * @param aManifestURI
    ///    *        The manifest URI to be checked.
    ///    * @param aDocumentURI
    ///    *        The page that is requesting the update.
    ///    * @param aLoadingPrincipal
    ///    *        The principal of the page that is requesting the update.
    ///    */
    /// ```
    ///

    /// `void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument, [optional] in nsIFile aCustomProfileDir);`
    #[inline]
    pub unsafe fn Init(&self, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aDocument: *const libc::c_void, aCustomProfileDir: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aManifestURI, aDocumentURI, aLoadingPrincipal, aDocument, aCustomProfileDir)
    }


    /// ```text
    /// /**
    ///    * Initialize the update for partial processing.
    ///    *
    ///    * @param aManifestURI
    ///    *        The manifest URI of the related cache.
    ///    * @param aClientID
    ///    *        Client  ID of the cache to store resource to. This ClientID
    ///    *        must be ID of cache in the cache group identified by
    ///    *        the manifest URI passed in the first parameter.
    ///    * @param aDocumentURI
    ///    *        The page that is requesting the update. May be null
    ///    *        when this information is unknown.
    ///    * @param aCookieJarSettings
    ///    *        The cookie jar settings belonging to the page that is requesting
    ///    *        the update.
    ///    */
    /// ```
    ///

    /// `void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal, in nsICookieJarSettings aCookieJarSettings);`
    #[inline]
    pub unsafe fn InitPartial(&self, aManifestURI: *const nsIURI, aClientID: *const ::nsstring::nsACString, aDocumentURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCookieJarSettings: *const nsICookieJarSettings) -> ::nserror::nsresult {
        ((*self.vtable).InitPartial)(self, aManifestURI, aClientID, aDocumentURI, aPrincipal, aCookieJarSettings)
    }


    /// ```text
    /// /**
    ///    * Initialize the update to only check whether there is an update
    ///    * to the manifest available (if it has actually changed on the server).
    ///    *
    ///    * @param aManifestURI
    ///    *        The manifest URI of the related cache.
    ///    * @param aObserver
    ///    *        nsIObserver implementation that receives the result.
    ///    *        When aTopic == "offline-cache-update-available" there is an update to
    ///    *        to download. Update of the app cache will lead to a new version
    ///    *        download.
    ///    *        When aTopic == "offline-cache-update-unavailable" then there is no
    ///    *        update available (the manifest has not changed on the server).
    ///    */
    /// ```
    ///

    /// `void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn InitForUpdateCheck(&self, aManifestURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).InitForUpdateCheck)(self, aManifestURI, aLoadingPrincipal, aObserver)
    }


    /// ```text
    /// /**
    ///    * Add a dynamic URI to the offline cache as part of the update.
    ///    *
    ///    * @param aURI
    ///    *        The URI to add.
    ///    */
    /// ```
    ///

    /// `void addDynamicURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn AddDynamicURI(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).AddDynamicURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///    * Add the update to the offline update queue.  An offline-cache-update-added
    ///    * event will be sent to the observer service.
    ///    */
    /// ```
    ///

    /// `void schedule ();`
    #[inline]
    pub unsafe fn Schedule(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Schedule)(self, )
    }


    /// ```text
    /// /**
    ///    * Observe loads that are added to the update.
    ///    *
    ///    * @param aObserver
    ///    *        object that notifications will be sent to.
    ///    * @param aHoldWeak
    ///    *        TRUE if you want the update to hold a weak reference to the
    ///    *        observer, FALSE for a strong reference.
    ///    */
    /// ```
    ///

    /// `void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak);`
    #[inline]
    pub unsafe fn AddObserver(&self, aObserver: *const nsIOfflineCacheUpdateObserver, aHoldWeak: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, aObserver, aHoldWeak)
    }


    /// ```text
    /// /**
    ///    * Remove an observer from the update.
    ///    *
    ///    * @param aObserver
    ///    *        the observer to remove.
    ///    */
    /// ```
    ///

    /// `void removeObserver (in nsIOfflineCacheUpdateObserver aObserver);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, aObserver: *const nsIOfflineCacheUpdateObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Cancel the update when still in progress. This stops all running resource
    ///    * downloads and discards the downloaded cache version. Throws when update
    ///    * has already finished and made the new cache version active.
    ///    */
    /// ```
    ///

    /// `void cancel ();`
    #[inline]
    pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, )
    }


    /// ```text
    /// /**
    ///    * Return the number of bytes downloaded so far
    ///    */
    /// ```
    ///

    /// `readonly attribute uint64_t byteProgress;`
    #[inline]
    pub unsafe fn GetByteProgress(&self, aByteProgress: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetByteProgress)(self, aByteProgress)
    }


}


/// `interface nsIOfflineCacheUpdateService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOfflineCacheUpdateService {
    vtable: *const nsIOfflineCacheUpdateServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOfflineCacheUpdateService.
unsafe impl XpCom for nsIOfflineCacheUpdateService {
    const IID: nsIID = nsID(0x44971e74, 0x37e4, 0x4140,
        [0x86, 0x77, 0xa4, 0xcf, 0x21, 0x3a, 0x3f, 0x4b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOfflineCacheUpdateService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOfflineCacheUpdateService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOfflineCacheUpdateServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIOfflineCacheUpdateService`.
    fn coerce_from(v: &nsIOfflineCacheUpdateService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOfflineCacheUpdateServiceCoerce for nsIOfflineCacheUpdateService {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateService) -> &Self {
        v
    }
}

impl nsIOfflineCacheUpdateService {
    /// Cast this `nsIOfflineCacheUpdateService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOfflineCacheUpdateServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOfflineCacheUpdateService {
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
impl<T: nsISupportsCoerce> nsIOfflineCacheUpdateServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOfflineCacheUpdateService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOfflineCacheUpdateServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long numUpdates; */
    pub GetNumUpdates: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aNumUpdates: *mut u32) -> ::nserror::nsresult,

    /* nsIOfflineCacheUpdate getUpdate (in unsigned long index); */
    pub GetUpdate: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, index: u32, _retval: *mut *const nsIOfflineCacheUpdate) -> ::nserror::nsresult,

    /* nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow); */
    pub ScheduleUpdate: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aWindow: *const mozIDOMWindow, _retval: *mut *const nsIOfflineCacheUpdate) -> ::nserror::nsresult,

    /* nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir); */
    pub ScheduleAppUpdate: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aProfileDir: *const nsIFile, _retval: *mut *const nsIOfflineCacheUpdate) -> ::nserror::nsresult,

    /* void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument); */
    pub ScheduleOnDocumentStop: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aDocument: *const libc::c_void) -> ::nserror::nsresult,

    /* void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
    pub CheckForUpdate: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* boolean offlineAppAllowed (in nsIPrincipal aPrincipal); */
    pub OfflineAppAllowed: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aPrincipal: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean offlineAppAllowedForURI (in nsIURI aURI); */
    pub OfflineAppAllowedForURI: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* void allowOfflineApp (in nsIPrincipal aPrincipal); */
    pub AllowOfflineApp: unsafe extern "system" fn (this: *const nsIOfflineCacheUpdateService, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOfflineCacheUpdateService {
    /// ```text
    /// /**
    ///      * Constants for the offline-app permission.
    ///      *
    ///      * XXX: This isn't a great place for this, but it's really the only
    ///      * private offline-app-related interface
    ///      */
    /// /**
    ///      * Allow the domain to use offline APIs, and don't warn about excessive
    ///      * usage.
    ///      */
    /// ```
    ///

    pub const ALLOW_NO_WARN: i64 = 3;

    /// ```text
    /// /**
    ///      * Access to the list of cache updates that have been scheduled.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long numUpdates;`
    #[inline]
    pub unsafe fn GetNumUpdates(&self, aNumUpdates: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumUpdates)(self, aNumUpdates)
    }



    /// `nsIOfflineCacheUpdate getUpdate (in unsigned long index);`
    #[inline]
    pub unsafe fn GetUpdate(&self, index: u32, _retval: *mut *const nsIOfflineCacheUpdate) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdate)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///      * Schedule a cache update for a given offline manifest.  If an
    ///      * existing update is scheduled or running, that update will be returned.
    ///      * Otherwise a new update will be scheduled.
    ///      */
    /// ```
    ///

    /// `nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn ScheduleUpdate(&self, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aWindow: *const mozIDOMWindow, _retval: *mut *const nsIOfflineCacheUpdate) -> ::nserror::nsresult {
        ((*self.vtable).ScheduleUpdate)(self, aManifestURI, aDocumentURI, aLoadingPrincipal, aWindow, _retval)
    }


    /// ```text
    /// /**
    ///      * Schedule a cache update for a given offline manifest using app cache
    ///      * bound to the given appID flag.  If an existing update is scheduled or
    ///      * running, that update will be returned. Otherwise a new update will be
    ///      * scheduled.
    ///      */
    /// ```
    ///

    /// `nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir);`
    #[inline]
    pub unsafe fn ScheduleAppUpdate(&self, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aProfileDir: *const nsIFile, _retval: *mut *const nsIOfflineCacheUpdate) -> ::nserror::nsresult {
        ((*self.vtable).ScheduleAppUpdate)(self, aManifestURI, aDocumentURI, aLoadingPrincipal, aProfileDir, _retval)
    }


    /// ```text
    /// /**
    ///      * Schedule a cache update for a manifest when the document finishes
    ///      * loading.
    ///      */
    /// ```
    ///

    /// `void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in Document aDocument);`
    #[inline]
    pub unsafe fn ScheduleOnDocumentStop(&self, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aDocument: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ScheduleOnDocumentStop)(self, aManifestURI, aDocumentURI, aLoadingPrincipal, aDocument)
    }


    /// ```text
    /// /**
    ///      * Schedule a check to see if an update is available.
    ///      *
    ///      * This will not update or make any changes to the appcache.
    ///      * It only notifies the observer to indicate whether the manifest has
    ///      * changed on the server (or not): a changed manifest means that an
    ///      * update is available.
    ///      *
    ///      * For arguments see nsIOfflineCacheUpdate.initForUpdateCheck() method
    ///      * description.
    ///      */
    /// ```
    ///

    /// `void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn CheckForUpdate(&self, aManifestURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).CheckForUpdate)(self, aManifestURI, aLoadingPrincipal, aObserver)
    }


    /// ```text
    /// /**
    ///      * Checks whether a principal should have access to the offline
    ///      * cache.
    ///      * @param aPrincipal
    ///      *        The principal to check.
    ///      */
    /// ```
    ///

    /// `boolean offlineAppAllowed (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn OfflineAppAllowed(&self, aPrincipal: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OfflineAppAllowed)(self, aPrincipal, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks whether a document at the given URI should have access
    ///      * to the offline cache.
    ///      * @param aURI
    ///      *        The URI to check
    ///      */
    /// ```
    ///

    /// `boolean offlineAppAllowedForURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn OfflineAppAllowedForURI(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OfflineAppAllowedForURI)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Sets the "offline-app" permission for the principal.
    ///      * In the single process model calls directly on permission manager.
    ///      * In the multi process model dispatches to the parent process.
    ///      */
    /// ```
    ///

    /// `void allowOfflineApp (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn AllowOfflineApp(&self, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).AllowOfflineApp)(self, aPrincipal)
    }


}


