//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheService.idl
//


/// `interface nsIApplicationCacheService : nsISupports`
///

/// ```text
/// /**
///  * The application cache service manages the set of application cache
///  * groups.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationCacheService {
    vtable: *const nsIApplicationCacheServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationCacheService.
unsafe impl XpCom for nsIApplicationCacheService {
    const IID: nsIID = nsID(0xb8b6546c, 0x6cec, 0x4bda,
        [0x82, 0xdf, 0x08, 0xe0, 0x06, 0xa9, 0x7b, 0x56]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationCacheService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationCacheService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationCacheServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationCacheService`.
    fn coerce_from(v: &nsIApplicationCacheService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationCacheServiceCoerce for nsIApplicationCacheService {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheService) -> &Self {
        v
    }
}

impl nsIApplicationCacheService {
    /// Cast this `nsIApplicationCacheService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationCacheServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationCacheService {
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
impl<T: nsISupportsCoerce> nsIApplicationCacheServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationCacheService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationCacheServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString buildGroupIDForInfo (in nsIURI aManifestURL, in nsILoadContextInfo aLoadContextInfo); */
    pub BuildGroupIDForInfo: unsafe extern "system" fn (this: *const nsIApplicationCacheService, aManifestURL: *const nsIURI, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString buildGroupIDForSuffix (in nsIURI aManifestURL, in ACString aOriginSuffix); */
    pub BuildGroupIDForSuffix: unsafe extern "system" fn (this: *const nsIApplicationCacheService, aManifestURL: *const nsIURI, aOriginSuffix: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIApplicationCache createApplicationCache (in ACString group); */
    pub CreateApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheService, group: *const ::nsstring::nsACString, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* nsIApplicationCache createCustomApplicationCache (in ACString group, in nsIFile profileDir, in int32_t quota); */
    pub CreateCustomApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheService, group: *const ::nsstring::nsACString, profileDir: *const nsIFile, quota: int32_t, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* nsIApplicationCache getApplicationCache (in ACString clientID); */
    pub GetApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheService, clientID: *const ::nsstring::nsACString, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* nsIApplicationCache getActiveCache (in ACString group); */
    pub GetActiveCache: unsafe extern "system" fn (this: *const nsIApplicationCacheService, group: *const ::nsstring::nsACString, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* void deactivateGroup (in ACString group); */
    pub DeactivateGroup: unsafe extern "system" fn (this: *const nsIApplicationCacheService, group: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void evict (in nsILoadContextInfo aLoadContextInfo); */
    pub Evict: unsafe extern "system" fn (this: *const nsIApplicationCacheService, aLoadContextInfo: *const nsILoadContextInfo) -> ::nserror::nsresult,

    /* void evictMatchingOriginAttributes (in AString aPattern); */
    pub EvictMatchingOriginAttributes: unsafe extern "system" fn (this: *const nsIApplicationCacheService, aPattern: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIApplicationCache chooseApplicationCache (in ACString key, [optional] in nsILoadContextInfo aLoadContextInfo); */
    pub ChooseApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheService, key: *const ::nsstring::nsACString, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* void cacheOpportunistically (in nsIApplicationCache cache, in ACString key); */
    pub CacheOpportunistically: unsafe extern "system" fn (this: *const nsIApplicationCacheService, cache: *const nsIApplicationCache, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* Array<ACString> getGroups (); */
    pub GetGroups: unsafe extern "system" fn (this: *const nsIApplicationCacheService, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* Array<ACString> getGroupsTimeOrdered (); */
    pub GetGroupsTimeOrdered: unsafe extern "system" fn (this: *const nsIApplicationCacheService, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationCacheService {

    /// ```text
    /// /**
    ///      * Create group string identifying cache group according the manifest
    ///      * URL and the given principal.
    ///      */
    /// ```
    ///

    /// `ACString buildGroupIDForInfo (in nsIURI aManifestURL, in nsILoadContextInfo aLoadContextInfo);`
    #[inline]
    pub unsafe fn BuildGroupIDForInfo(&self, aManifestURL: *const nsIURI, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BuildGroupIDForInfo)(self, aManifestURL, aLoadContextInfo, _retval)
    }



    /// `ACString buildGroupIDForSuffix (in nsIURI aManifestURL, in ACString aOriginSuffix);`
    #[inline]
    pub unsafe fn BuildGroupIDForSuffix(&self, aManifestURL: *const nsIURI, aOriginSuffix: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BuildGroupIDForSuffix)(self, aManifestURL, aOriginSuffix, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a new, empty application cache for the given cache
    ///      * group.
    ///      */
    /// ```
    ///

    /// `nsIApplicationCache createApplicationCache (in ACString group);`
    #[inline]
    pub unsafe fn CreateApplicationCache(&self, group: *const ::nsstring::nsACString, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).CreateApplicationCache)(self, group, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a new, empty application cache for the given cache
    ///      * group residing in a custom directory with a custom quota.
    ///      *
    ///      * @param group
    ///      *    URL of the manifest
    ///      * @param directory
    ///      *    Actually a reference to a profile directory where to
    ///      *    create the OfflineCache sub-dir.
    ///      * @param quota
    ///      *    Optional override of the default quota.
    ///      */
    /// ```
    ///

    /// `nsIApplicationCache createCustomApplicationCache (in ACString group, in nsIFile profileDir, in int32_t quota);`
    #[inline]
    pub unsafe fn CreateCustomApplicationCache(&self, group: *const ::nsstring::nsACString, profileDir: *const nsIFile, quota: int32_t, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).CreateCustomApplicationCache)(self, group, profileDir, quota, _retval)
    }


    /// ```text
    /// /**
    ///      * Get an application cache object for the given client ID.
    ///      */
    /// ```
    ///

    /// `nsIApplicationCache getApplicationCache (in ACString clientID);`
    #[inline]
    pub unsafe fn GetApplicationCache(&self, clientID: *const ::nsstring::nsACString, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationCache)(self, clientID, _retval)
    }


    /// ```text
    /// /**
    ///      * Get the currently active cache object for a cache group.
    ///      */
    /// ```
    ///

    /// `nsIApplicationCache getActiveCache (in ACString group);`
    #[inline]
    pub unsafe fn GetActiveCache(&self, group: *const ::nsstring::nsACString, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveCache)(self, group, _retval)
    }


    /// ```text
    /// /**
    ///      * Deactivate the currently-active cache object for a cache group.
    ///      */
    /// ```
    ///

    /// `void deactivateGroup (in ACString group);`
    #[inline]
    pub unsafe fn DeactivateGroup(&self, group: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).DeactivateGroup)(self, group)
    }


    /// ```text
    /// /**
    ///      * Evict offline cache entries, either all of them or those belonging
    ///      * to the given origin.
    ///      */
    /// ```
    ///

    /// `void evict (in nsILoadContextInfo aLoadContextInfo);`
    #[inline]
    pub unsafe fn Evict(&self, aLoadContextInfo: *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).Evict)(self, aLoadContextInfo)
    }


    /// ```text
    /// /**
    ///      * Delete caches whom origin attributes matches the given pattern.
    ///      */
    /// ```
    ///

    /// `void evictMatchingOriginAttributes (in AString aPattern);`
    #[inline]
    pub unsafe fn EvictMatchingOriginAttributes(&self, aPattern: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).EvictMatchingOriginAttributes)(self, aPattern)
    }


    /// ```text
    /// /**
    ///      * Try to find the best application cache to serve a resource.
    ///      */
    /// ```
    ///

    /// `nsIApplicationCache chooseApplicationCache (in ACString key, [optional] in nsILoadContextInfo aLoadContextInfo);`
    #[inline]
    pub unsafe fn ChooseApplicationCache(&self, key: *const ::nsstring::nsACString, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).ChooseApplicationCache)(self, key, aLoadContextInfo, _retval)
    }


    /// ```text
    /// /**
    ///      * Flags the key as being opportunistically cached.
    ///      *
    ///      * This method should also propagate the entry to other
    ///      * application caches with the same opportunistic namespace, but
    ///      * this is not currently implemented.
    ///      *
    ///      * @param cache
    ///      *        The cache in which the entry is cached now.
    ///      * @param key
    ///      *        The cache entry key.
    ///      */
    /// ```
    ///

    /// `void cacheOpportunistically (in nsIApplicationCache cache, in ACString key);`
    #[inline]
    pub unsafe fn CacheOpportunistically(&self, cache: *const nsIApplicationCache, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).CacheOpportunistically)(self, cache, key)
    }


    /// ```text
    /// /**
    ///      * Get the list of application cache groups.
    ///      */
    /// ```
    ///

    /// `Array<ACString> getGroups ();`
    #[inline]
    pub unsafe fn GetGroups(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetGroups)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Get the list of application cache groups in the order of
    ///      * activating time.
    ///      */
    /// ```
    ///

    /// `Array<ACString> getGroupsTimeOrdered ();`
    #[inline]
    pub unsafe fn GetGroupsTimeOrdered(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetGroupsTimeOrdered)(self, _retval)
    }


}


