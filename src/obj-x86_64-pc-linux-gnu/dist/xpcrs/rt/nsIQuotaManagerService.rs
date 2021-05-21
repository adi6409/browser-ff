//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaManagerService.idl
//


/// `interface nsIQuotaManagerService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaManagerService {
    vtable: *const nsIQuotaManagerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaManagerService.
unsafe impl XpCom for nsIQuotaManagerService {
    const IID: nsIID = nsID(0x1b3d0a38, 0x8151, 0x4cf9,
        [0x89, 0xfa, 0x4f, 0x92, 0xc2, 0xef, 0x0e, 0x7e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaManagerService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaManagerService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaManagerServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaManagerService`.
    fn coerce_from(v: &nsIQuotaManagerService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaManagerServiceCoerce for nsIQuotaManagerService {
    #[inline]
    fn coerce_from(v: &nsIQuotaManagerService) -> &Self {
        v
    }
}

impl nsIQuotaManagerService {
    /// Cast this `nsIQuotaManagerService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaManagerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaManagerService {
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
impl<T: nsISupportsCoerce> nsIQuotaManagerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaManagerService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaManagerService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaManagerServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] nsIQuotaRequest storageName (); */
    pub StorageName: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest storageInitialized (); */
    pub StorageInitialized: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest temporaryStorageInitialized (); */
    pub TemporaryStorageInitialized: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest init (); */
    pub Init: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest initTemporaryStorage (); */
    pub InitTemporaryStorage: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest initializePersistentOrigin (in nsIPrincipal aPrincipal); */
    pub InitializePersistentOrigin: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest initializeTemporaryOrigin (in ACString aPersistenceType, in nsIPrincipal aPrincipal); */
    pub InitializeTemporaryOrigin: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPersistenceType: *const ::nsstring::nsACString, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll); */
    pub GetUsage: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aCallback: *const nsIQuotaUsageCallback, aGetAll: bool, _retval: *mut*const nsIQuotaUsageRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aFromMemory); */
    pub GetUsageForPrincipal: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, aCallback: *const nsIQuotaUsageCallback, aFromMemory: bool, _retval: *mut*const nsIQuotaUsageRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest listOrigins (); */
    pub ListOrigins: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest clear (); */
    pub Clear: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest clearStoragesForOriginAttributesPattern (in AString aPattern); */
    pub ClearStoragesForOriginAttributesPattern: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPattern: *const ::nsstring::nsAString, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType, [optional] in boolean aClearAll); */
    pub ClearStoragesForPrincipal: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, aPersistenceType: *const ::nsstring::nsACString, aClientType: *const ::nsstring::nsAString, aClearAll: bool, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest reset (); */
    pub Reset: unsafe extern "system" fn (this: *const nsIQuotaManagerService, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest resetStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType); */
    pub ResetStoragesForPrincipal: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, aPersistenceType: *const ::nsstring::nsACString, aClientType: *const ::nsstring::nsAString, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal); */
    pub Persisted: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal); */
    pub Persist: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,

    /* [must_use] nsIQuotaRequest estimate (in nsIPrincipal aPrincipal); */
    pub Estimate: unsafe extern "system" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaManagerService {

    /// ```text
    /// /**
    ///    * Asynchronously retrieves storage name and returns it as a plain string.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest storageName ();`
    #[inline]
    pub unsafe fn StorageName(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).StorageName)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if storage is initialized.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest storageInitialized ();`
    #[inline]
    pub unsafe fn StorageInitialized(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).StorageInitialized)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if temporary storage is initialized.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest temporaryStorageInitialized ();`
    #[inline]
    pub unsafe fn TemporaryStorageInitialized(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).TemporaryStorageInitialized)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Initializes storage directory. This can be used in tests to verify
    ///    * upgrade methods.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest init ();`
    #[inline]
    pub unsafe fn Init(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Initializes temporary storage. This can be used in tests to verify
    ///    * temporary storage initialization.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest initTemporaryStorage ();`
    #[inline]
    pub unsafe fn InitTemporaryStorage(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).InitTemporaryStorage)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Initializes persistent origin directory for the given origin. This can be
    ///    * used in tests to verify origin initialization.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin whose directory is to be initialized.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest initializePersistentOrigin (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn InitializePersistentOrigin(&self, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).InitializePersistentOrigin)(self, aPrincipal, _retval)
    }


    /// ```text
    /// /**
    ///    * Initializes temporary origin directory for the given origin. This can be
    ///    * used in tests to verify origin initialization.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    *
    ///    * @param aPersistenceType
    ///    *        A string that tells what persistence type of origin will be
    ///    *        initialized (temporary or default).
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin whose directory is to be initialized.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest initializeTemporaryOrigin (in ACString aPersistenceType, in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn InitializeTemporaryOrigin(&self, aPersistenceType: *const ::nsstring::nsACString, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).InitializeTemporaryOrigin)(self, aPersistenceType, aPrincipal, _retval)
    }


    /// ```text
    /// /**
    ///    * Schedules an asynchronous callback that will inspect all origins and
    ///    * return the total amount of disk space being used by storages for each
    ///    * origin separately.
    ///    *
    ///    * @param aCallback
    ///    *        The callback that will be called when the usage is available.
    ///    * @param aGetAll
    ///    *        An optional boolean to indicate inspection of all origins,
    ///    *        including internal ones.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll);`
    #[inline]
    pub unsafe fn GetUsage(&self, aCallback: *const nsIQuotaUsageCallback, aGetAll: bool, _retval: *mut*const nsIQuotaUsageRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetUsage)(self, aCallback, aGetAll, _retval)
    }


    /// ```text
    /// /**
    ///    * Schedules an asynchronous callback that will return the total amount of
    ///    * disk space being used by storages for the given origin.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin whose usage is being queried.
    ///    * @param aCallback
    ///    *        The callback that will be called when the usage is available.
    ///    * @param aFromMemory
    ///    *        An optional flag to indicate whether the cached usage should be
    ///    *        obtained. The default value is false.  Note that this operation may
    ///    *        still be delayed by other operations on the QM I/O thread that are
    ///    *        peforming I/O.
    ///    * Note:  Origin usage here represents total usage of an origin. However,
    ///    *        cached usage here represents only non-persistent usage of an origin.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aFromMemory);`
    #[inline]
    pub unsafe fn GetUsageForPrincipal(&self, aPrincipal: *const nsIPrincipal, aCallback: *const nsIQuotaUsageCallback, aFromMemory: bool, _retval: *mut*const nsIQuotaUsageRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetUsageForPrincipal)(self, aPrincipal, aCallback, aFromMemory, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchronously lists all origins and returns them as plain strings.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest listOrigins ();`
    #[inline]
    pub unsafe fn ListOrigins(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).ListOrigins)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Removes all storages. The files may not be deleted immediately depending
    ///    * on prohibitive concurrent operations.
    ///    * Be careful, this removes *all* the data that has ever been stored!
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest clear ();`
    #[inline]
    pub unsafe fn Clear(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Removes all storages stored for the given pattern. The files may not be
    ///    * deleted immediately depending on prohibitive concurrent operations.  In
    ///    * terms of locks, it will get an exclusive multi directory lock for given
    ///    * pattern.  For example, given pattern {"userContextId":1007} and set of 3
    ///    * origins ["http://www.mozilla.org^userContextId=1007",
        ///    * "http://www.example.org^userContextId=1007",
        ///    * "http://www.example.org^userContextId=1008"], the method will only lock 2
    ///    * origins ["http://www.mozilla.org^userContextId=1007",
        ///    * "http://www.example.org^userContextId=1007"].
    ///    *
    ///    * @param aPattern
    ///    *        A pattern for the origins whose storages are to be cleared.
    ///    *        Currently this is expected to be a JSON representation of the
    ///    *        OriginAttributesPatternDictionary defined in ChromeUtils.webidl.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest clearStoragesForOriginAttributesPattern (in AString aPattern);`
    #[inline]
    pub unsafe fn ClearStoragesForOriginAttributesPattern(&self, aPattern: *const ::nsstring::nsAString, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).ClearStoragesForOriginAttributesPattern)(self, aPattern, _retval)
    }


    /// ```text
    /// /**
    ///    * Removes all storages stored for the given principal. The files may not be
    ///    * deleted immediately depending on prohibitive concurrent operations.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin whose storages are to be cleared.
    ///    * @param aPersistenceType
    ///    *        An optional string that tells what persistence type of storages
    ///    *        will be cleared.  If omitted (or void), all persistence types will
    ///    *        be cleared for the principal.  If a single persistence type
    ///    *        ("persistent", "temporary", or "default") is provided, then only
    ///    *        that persistence directory will be considered.  Note that
    ///    *        "persistent" is different than being "persisted" via persist() and
    ///    *        is only for chrome principals.  See bug 1354500 for more info.
    ///    *        In general, null is the right thing to pass here.
    ///    * @param aClientType
    ///    *        An optional string that tells what client type of storages
    ///    *        will be cleared.  If omitted (or void), all client types will be
    ///    *        cleared for the principal.  If a single client type is provided
    ///    *        from Client.h, then only that client's storage will be cleared.
    ///    *        If you want to clear multiple client types (but not all), then you
    ///    *        must call this method multiple times.
    ///    * @param aClearAll
    ///    *        An optional boolean to indicate clearing all storages under the
    ///    *        given origin.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType, [optional] in boolean aClearAll);`
    #[inline]
    pub unsafe fn ClearStoragesForPrincipal(&self, aPrincipal: *const nsIPrincipal, aPersistenceType: *const ::nsstring::nsACString, aClientType: *const ::nsstring::nsAString, aClearAll: bool, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).ClearStoragesForPrincipal)(self, aPrincipal, aPersistenceType, aClientType, aClearAll, _retval)
    }


    /// ```text
    /// /**
    ///    * Resets quota and storage management. This can be used to force
    ///    * reinitialization of the temp storage, for example when the pref for
    ///    * overriding the temp storage limit has changed.
    ///    * Be carefull, this invalidates all live storages!
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest reset ();`
    #[inline]
    pub unsafe fn Reset(&self, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Resets all storages stored for the given principal.
    ///    *
    ///    * If the dom.quotaManager.testing preference is not true the call will be
    ///    * a no-op.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin whose storages are to be reset.
    ///    * @param aPersistenceType
    ///    *        An optional string that tells what persistence type of storages
    ///    *        will be reset.  If omitted (or void), all persistence types will
    ///    *        be cleared for the principal.  If a single persistence type
    ///    *        ("persistent", "temporary", or "default") is provided, then only
    ///    *        that persistence directory will be considered.  Note that
    ///    *        "persistent" is different than being "persisted" via persist() and
    ///    *        is only for chrome principals.  See bug 1354500 for more info.
    ///    *        In general, null is the right thing to pass here.
    ///    * @param aClientType
    ///    *        An optional string that tells what client type of storages
    ///    *        will be reset.  If omitted (or void), all client types will be
    ///    *        cleared for the principal.  If a single client type is provided
    ///    *        from Client.h, then only that client's storage will be cleared.
    ///    *        If you want to clear multiple client types (but not all), then you
    ///    *        must call this method multiple times.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest resetStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in AString aClientType);`
    #[inline]
    pub unsafe fn ResetStoragesForPrincipal(&self, aPrincipal: *const nsIPrincipal, aPersistenceType: *const ::nsstring::nsACString, aClientType: *const ::nsstring::nsAString, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).ResetStoragesForPrincipal)(self, aPrincipal, aPersistenceType, aClientType, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if given origin is persisted.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin which we want to check.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn Persisted(&self, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).Persisted)(self, aPrincipal, _retval)
    }


    /// ```text
    /// /**
    ///    * Persist given origin.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin which we want to persist.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn Persist(&self, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).Persist)(self, aPrincipal, _retval)
    }


    /// ```text
    /// /**
    ///    * Given an origin, asynchronously calculate its group quota usage and quota
    ///    * limit. An origin's group is the set of all origins that share the same
    ///    * eTLD+1. This method is intended to be used for our implementation of the
    ///    * StorageManager.estimate() method. When we fix bug 1305665 and stop tracking
    ///    * quota limits on a group basis, this method will switch to operating on
    ///    * origins. Callers should strongly consider whether they want to be using
    ///    * getUsageForPrincipal() instead.
    ///    *
    ///    * This mechanism uses cached quota values and does not perform any I/O on its
    ///    * own, but it may be delayed by QuotaManager operations that do need to
    ///    * perform I/O on the QuotaManager I/O thread.
    ///    *
    ///    * @param aPrincipal
    ///    *        A principal for the origin (group) which we want to estimate.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIQuotaRequest estimate (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn Estimate(&self, aPrincipal: *const nsIPrincipal, _retval: *mut*const nsIQuotaRequest) -> ::nserror::nsresult {
        ((*self.vtable).Estimate)(self, aPrincipal, _retval)
    }


}


