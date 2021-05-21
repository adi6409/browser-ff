//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCache.idl
//


/// `interface nsIApplicationCacheNamespace : nsISupports`
///

/// ```text
/// /**
///  * Application caches can store a set of namespace entries that affect
///  * loads from the application cache.  If a load from the cache fails
///  * to match an exact cache entry, namespaces entries will be searched
///  * for a substring match, and should be applied appropriately.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationCacheNamespace {
    vtable: *const nsIApplicationCacheNamespaceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationCacheNamespace.
unsafe impl XpCom for nsIApplicationCacheNamespace {
    const IID: nsIID = nsID(0x96e4c264, 0x2065, 0x4ce9,
        [0x93, 0xbb, 0x43, 0x73, 0x4c, 0x62, 0xc4, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationCacheNamespace {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationCacheNamespace.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationCacheNamespaceCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationCacheNamespace`.
    fn coerce_from(v: &nsIApplicationCacheNamespace) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationCacheNamespaceCoerce for nsIApplicationCacheNamespace {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheNamespace) -> &Self {
        v
    }
}

impl nsIApplicationCacheNamespace {
    /// Cast this `nsIApplicationCacheNamespace` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationCacheNamespaceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationCacheNamespace {
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
impl<T: nsISupportsCoerce> nsIApplicationCacheNamespaceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheNamespace) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationCacheNamespace
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationCacheNamespaceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data); */
    pub Init: unsafe extern "system" fn (this: *const nsIApplicationCacheNamespace, itemType: u32, namespaceSpec: *const ::nsstring::nsACString, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long itemType; */
    pub GetItemType: unsafe extern "system" fn (this: *const nsIApplicationCacheNamespace, aItemType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute ACString namespaceSpec; */
    pub GetNamespaceSpec: unsafe extern "system" fn (this: *const nsIApplicationCacheNamespace, aNamespaceSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIApplicationCacheNamespace, aData: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationCacheNamespace {
    /// ```text
    /// /**
    ///      * Items matching this namespace can be fetched from the network
    ///      * when loading from this cache.  The "data" attribute is unused.
    ///      */
    /// ```
    ///

    pub const NAMESPACE_BYPASS: i64 = 1;

    /// ```text
    /// /**
    ///      * Items matching this namespace can be fetched from the network
    ///      * when loading from this cache.  If the load fails, the cache entry
    ///      * specified by the "data" attribute should be loaded instead.
    ///      */
    /// ```
    ///

    pub const NAMESPACE_FALLBACK: i64 = 2;

    /// ```text
    /// /**
    ///      * Items matching this namespace should be cached
    ///      * opportunistically.  Successful toplevel loads of documents
    ///      * in this namespace should be placed in the application cache.
    ///      * Namespaces specifying NAMESPACE_OPPORTUNISTIC may also specify
    ///      * NAMESPACE_FALLBACK to supply a fallback entry.
    ///      */
    /// ```
    ///

    pub const NAMESPACE_OPPORTUNISTIC: i64 = 4;

    /// ```text
    /// /**
    ///      * Initialize the namespace.
    ///      */
    /// ```
    ///

    /// `void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data);`
    #[inline]
    pub unsafe fn Init(&self, itemType: u32, namespaceSpec: *const ::nsstring::nsACString, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, itemType, namespaceSpec, data)
    }


    /// ```text
    /// /**
    ///      * The namespace type.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long itemType;`
    #[inline]
    pub unsafe fn GetItemType(&self, aItemType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetItemType)(self, aItemType)
    }


    /// ```text
    /// /**
    ///      * The prefix of this namespace.  This should be the asciiSpec of the
    ///      * URI prefix.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString namespaceSpec;`
    #[inline]
    pub unsafe fn GetNamespaceSpec(&self, aNamespaceSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNamespaceSpec)(self, aNamespaceSpec)
    }


    /// ```text
    /// /**
    ///      * Data associated with this namespace, such as a fallback.  URI data should
    ///      * use the asciiSpec of the URI.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


}


/// `interface nsIApplicationCache : nsISupports`
///

/// ```text
/// /**
///  * Application caches store resources for offline use.  Each
///  * application cache has a unique client ID for use with
///  * nsICacheService::openSession() to access the cache's entries.
///  *
///  * Each entry in the application cache can be marked with a set of
///  * types, as discussed in the WHAT-WG offline applications
///  * specification.
///  *
///  * All application caches with the same group ID belong to a cache
///  * group.  Each group has one "active" cache that will service future
///  * loads.  Inactive caches will be removed from the cache when they are
///  * no longer referenced.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationCache {
    vtable: *const nsIApplicationCacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationCache.
unsafe impl XpCom for nsIApplicationCache {
    const IID: nsIID = nsID(0x06568dae, 0xc374, 0x4383,
        [0xa1, 0x22, 0x0c, 0xc9, 0x6c, 0x71, 0x77, 0xf2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationCache {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationCache.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationCacheCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationCache`.
    fn coerce_from(v: &nsIApplicationCache) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationCacheCoerce for nsIApplicationCache {
    #[inline]
    fn coerce_from(v: &nsIApplicationCache) -> &Self {
        v
    }
}

impl nsIApplicationCache {
    /// Cast this `nsIApplicationCache` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationCacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationCache {
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
impl<T: nsISupportsCoerce> nsIApplicationCacheCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCache) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationCache
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationCacheVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void initAsHandle (in ACString groupId, in ACString clientId); */
    pub InitAsHandle: unsafe extern "system" fn (this: *const nsIApplicationCache, groupId: *const ::nsstring::nsACString, clientId: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIURI manifestURI; */
    pub GetManifestURI: unsafe extern "system" fn (this: *const nsIApplicationCache, aManifestURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute ACString groupID; */
    pub GetGroupID: unsafe extern "system" fn (this: *const nsIApplicationCache, aGroupID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString clientID; */
    pub GetClientID: unsafe extern "system" fn (this: *const nsIApplicationCache, aClientID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean active; */
    pub GetActive: unsafe extern "system" fn (this: *const nsIApplicationCache, aActive: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long usage; */
    pub GetUsage: unsafe extern "system" fn (this: *const nsIApplicationCache, aUsage: *mut u32) -> ::nserror::nsresult,

    /* void activate (); */
    pub Activate: unsafe extern "system" fn (this: *const nsIApplicationCache) -> ::nserror::nsresult,

    /* void discard (); */
    pub Discard: unsafe extern "system" fn (this: *const nsIApplicationCache) -> ::nserror::nsresult,

    /* void markEntry (in ACString key, in unsigned long typeBits); */
    pub MarkEntry: unsafe extern "system" fn (this: *const nsIApplicationCache, key: *const ::nsstring::nsACString, typeBits: u32) -> ::nserror::nsresult,

    /* void unmarkEntry (in ACString key, in unsigned long typeBits); */
    pub UnmarkEntry: unsafe extern "system" fn (this: *const nsIApplicationCache, key: *const ::nsstring::nsACString, typeBits: u32) -> ::nserror::nsresult,

    /* unsigned long getTypes (in ACString key); */
    pub GetTypes: unsafe extern "system" fn (this: *const nsIApplicationCache, key: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult,

    /* Array<ACString> gatherEntries (in uint32_t typeBits); */
    pub GatherEntries: unsafe extern "system" fn (this: *const nsIApplicationCache, typeBits: uint32_t, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void addNamespaces (in nsIArray namespaces); */
    pub AddNamespaces: unsafe extern "system" fn (this: *const nsIApplicationCache, namespaces: *const nsIArray) -> ::nserror::nsresult,

    /* nsIApplicationCacheNamespace getMatchingNamespace (in ACString key); */
    pub GetMatchingNamespace: unsafe extern "system" fn (this: *const nsIApplicationCache, key: *const ::nsstring::nsACString, _retval: *mut *const nsIApplicationCacheNamespace) -> ::nserror::nsresult,

    /* readonly attribute nsIFile profileDirectory; */
    pub GetProfileDirectory: unsafe extern "system" fn (this: *const nsIApplicationCache, aProfileDirectory: *mut*const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationCache {
    /// ```text
    /// /**
    ///      * Entries in an application cache can be marked as one or more of
    ///      * the following types.
    ///      */
    /// ```
    ///

    pub const ITEM_MANIFEST: i64 = 1;


    pub const ITEM_EXPLICIT: i64 = 2;


    pub const ITEM_IMPLICIT: i64 = 4;


    pub const ITEM_DYNAMIC: i64 = 8;


    pub const ITEM_FOREIGN: i64 = 16;


    pub const ITEM_FALLBACK: i64 = 32;


    pub const ITEM_OPPORTUNISTIC: i64 = 64;

    /// ```text
    /// /**
    ///      * Init this application cache instance to just hold the group ID and
    ///      * the client ID to work just as a handle to the real cache. Used on
    ///      * content process to simplify the application cache code.
    ///      */
    /// ```
    ///

    /// `void initAsHandle (in ACString groupId, in ACString clientId);`
    #[inline]
    pub unsafe fn InitAsHandle(&self, groupId: *const ::nsstring::nsACString, clientId: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).InitAsHandle)(self, groupId, clientId)
    }


    /// ```text
    /// /**
    ///      * URI of the manfiest specifying this application cache.
    ///      **/
    /// ```
    ///

    /// `readonly attribute nsIURI manifestURI;`
    #[inline]
    pub unsafe fn GetManifestURI(&self, aManifestURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetManifestURI)(self, aManifestURI)
    }


    /// ```text
    /// /**
    ///      * The group ID for this cache group.  It is an internally generated string
    ///      * and cannot be used as manifest URL spec.
    ///      **/
    /// ```
    ///

    /// `readonly attribute ACString groupID;`
    #[inline]
    pub unsafe fn GetGroupID(&self, aGroupID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetGroupID)(self, aGroupID)
    }


    /// ```text
    /// /**
    ///      * The client ID for this application cache.  Clients can open a
    ///      * session with nsICacheService::createSession() using this client
    ///      * ID and a storage policy of STORE_OFFLINE to access this cache.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString clientID;`
    #[inline]
    pub unsafe fn GetClientID(&self, aClientID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetClientID)(self, aClientID)
    }


    /// ```text
    /// /**
    ///      * TRUE if the cache is the active cache for this group.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean active;`
    #[inline]
    pub unsafe fn GetActive(&self, aActive: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetActive)(self, aActive)
    }


    /// ```text
    /// /**
    ///      * The disk usage of the application cache, in bytes.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long usage;`
    #[inline]
    pub unsafe fn GetUsage(&self, aUsage: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetUsage)(self, aUsage)
    }


    /// ```text
    /// /**
    ///      * Makes this cache the active application cache for this group.
    ///      * Future loads associated with this group will come from this
    ///      * cache.  Other caches from this cache group will be deactivated.
    ///      */
    /// ```
    ///

    /// `void activate ();`
    #[inline]
    pub unsafe fn Activate(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Activate)(self, )
    }


    /// ```text
    /// /**
    ///      * Discard this application cache.  Removes all cached resources
    ///      * for this cache.  If this is the active application cache for the
    ///      * group, the group will be removed.
    ///      */
    /// ```
    ///

    /// `void discard ();`
    #[inline]
    pub unsafe fn Discard(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Discard)(self, )
    }


    /// ```text
    /// /**
    ///      * Adds item types to a given entry.
    ///      */
    /// ```
    ///

    /// `void markEntry (in ACString key, in unsigned long typeBits);`
    #[inline]
    pub unsafe fn MarkEntry(&self, key: *const ::nsstring::nsACString, typeBits: u32) -> ::nserror::nsresult {
        ((*self.vtable).MarkEntry)(self, key, typeBits)
    }


    /// ```text
    /// /**
    ///      * Removes types from a given entry.  If the resulting entry has
    ///      * no types left, the entry is removed.
    ///      */
    /// ```
    ///

    /// `void unmarkEntry (in ACString key, in unsigned long typeBits);`
    #[inline]
    pub unsafe fn UnmarkEntry(&self, key: *const ::nsstring::nsACString, typeBits: u32) -> ::nserror::nsresult {
        ((*self.vtable).UnmarkEntry)(self, key, typeBits)
    }


    /// ```text
    /// /**
    ///      * Gets the types for a given entry.
    ///      */
    /// ```
    ///

    /// `unsigned long getTypes (in ACString key);`
    #[inline]
    pub unsafe fn GetTypes(&self, key: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTypes)(self, key, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns any entries in the application cache whose type matches
    ///      * one or more of the bits in typeBits.
    ///      */
    /// ```
    ///

    /// `Array<ACString> gatherEntries (in uint32_t typeBits);`
    #[inline]
    pub unsafe fn GatherEntries(&self, typeBits: uint32_t, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GatherEntries)(self, typeBits, _retval)
    }


    /// ```text
    /// /**
    ///      * Add a set of namespace entries to the application cache.
    ///      * @param namespaces
    ///      *        An nsIArray of nsIApplicationCacheNamespace entries.
    ///      */
    /// ```
    ///

    /// `void addNamespaces (in nsIArray namespaces);`
    #[inline]
    pub unsafe fn AddNamespaces(&self, namespaces: *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).AddNamespaces)(self, namespaces)
    }


    /// ```text
    /// /**
    ///      * Get the most specific namespace matching a given key.
    ///      */
    /// ```
    ///

    /// `nsIApplicationCacheNamespace getMatchingNamespace (in ACString key);`
    #[inline]
    pub unsafe fn GetMatchingNamespace(&self, key: *const ::nsstring::nsACString, _retval: *mut *const nsIApplicationCacheNamespace) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchingNamespace)(self, key, _retval)
    }


    /// ```text
    /// /**
    ///      * If set, this offline cache is placed in a different directory
    ///      * than the current application profile.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile profileDirectory;`
    #[inline]
    pub unsafe fn GetProfileDirectory(&self, aProfileDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetProfileDirectory)(self, aProfileDirectory)
    }


}


