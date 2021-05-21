//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorage.idl
//


/// `interface nsICacheStorage : nsISupports`
///

/// ```text
/// /**
///  * Representation of a cache storage. There can be just-in-mem,
///  * in-mem+on-disk, in-mem+on-disk+app-cache or just a specific
///  * app-cache storage.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheStorage {
    vtable: *const nsICacheStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheStorage.
unsafe impl XpCom for nsICacheStorage {
    const IID: nsIID = nsID(0x35d104a6, 0xd252, 0x4fd4,
        [0x8a, 0x56, 0x3c, 0x14, 0x65, 0x7c, 0xad, 0x3b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheStorage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheStorage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheStorageCoerce {
    /// Cheaply cast a value of this type from a `nsICacheStorage`.
    fn coerce_from(v: &nsICacheStorage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheStorageCoerce for nsICacheStorage {
    #[inline]
    fn coerce_from(v: &nsICacheStorage) -> &Self {
        v
    }
}

impl nsICacheStorage {
    /// Cast this `nsICacheStorage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheStorage {
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
impl<T: nsISupportsCoerce> nsICacheStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheStorage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheStorageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback); */
    pub AsyncOpenURI: unsafe extern "system" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, aFlags: uint32_t, aCallback: *const nsICacheEntryOpenCallback) -> ::nserror::nsresult,

    /* nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension); */
    pub OpenTruncate: unsafe extern "system" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, _retval: *mut*const nsICacheEntry) -> ::nserror::nsresult,

    /* boolean exists (in nsIURI aURI, in ACString aIdExtension); */
    pub Exists: unsafe extern "system" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB); */
    pub GetCacheIndexEntryAttrs: unsafe extern "system" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, aHasAltData: *mut bool, aSizeInKB: *mut uint32_t) -> ::nserror::nsresult,

    /* void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback); */
    pub AsyncDoomURI: unsafe extern "system" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, aCallback: *const nsICacheEntryDoomCallback) -> ::nserror::nsresult,

    /* void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback); */
    pub AsyncEvictStorage: unsafe extern "system" fn (this: *const nsICacheStorage, aCallback: *const nsICacheEntryDoomCallback) -> ::nserror::nsresult,

    /* void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
    pub AsyncVisitStorage: unsafe extern "system" fn (this: *const nsICacheStorage, aVisitor: *const nsICacheStorageVisitor, aVisitEntries: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheStorage {
    /// ```text
    /// /**
    ///    * Placeholder for specifying "no special flags" during open.
    ///    */
    /// ```
    ///

    pub const OPEN_NORMALLY: i64 = 0;

    /// ```text
    /// /**
    ///    * Rewrite any existing data when opening a URL.
    ///    */
    /// ```
    ///

    pub const OPEN_TRUNCATE: i64 = 1;

    /// ```text
    /// /**
    ///    * Only open an existing entry.  Don't create a new one.
    ///    */
    /// ```
    ///

    pub const OPEN_READONLY: i64 = 2;

    /// ```text
    /// /**
    ///    * Use for first-paint blocking loads.
    ///    */
    /// ```
    ///

    pub const OPEN_PRIORITY: i64 = 4;

    /// ```text
    /// /**
    ///    * Bypass the cache load when write is still in progress.
    ///    */
    /// ```
    ///

    pub const OPEN_BYPASS_IF_BUSY: i64 = 8;

    /// ```text
    /// /**
    ///    * Perform the cache entry check (onCacheEntryCheck invocation) on any thread
    ///    * for optimal perfomance optimization.  If this flag is not specified it is
    ///    * ensured that onCacheEntryCheck is called on the same thread as respective
    ///    * asyncOpen has been called.
    ///    */
    /// ```
    ///

    pub const CHECK_MULTITHREADED: i64 = 16;

    /// ```text
    /// /**
    ///    * Don't automatically update any 'last used' metadata of the entry.
    ///    */
    /// ```
    ///

    pub const OPEN_SECRETLY: i64 = 32;

    /// ```text
    /// /**
    ///    * Entry is being opened as part of a service worker interception.  Do not
    ///    * allow the cache to be disabled in this case.
    ///    */
    /// ```
    ///

    pub const OPEN_INTERCEPTED: i64 = 64;

    /// ```text
    /// /**
    ///    * Asynchronously opens a cache entry for the specified URI.
    ///    * Result is fetched asynchronously via the callback.
    ///    *
    ///    * @param aURI
    ///    *    The URI to search in cache or to open for writting.
    ///    * @param aIdExtension
    ///    *    Any string that will extend (distinguish) the entry.  Two entries
    ///    *    with the same aURI but different aIdExtension will be comletely
    ///    *    different entries.  If you don't know what aIdExtension should be
    ///    *    leave it empty.
    ///    * @param aFlags
    ///    *    OPEN_NORMALLY - open cache entry normally for read and write
    ///    *    OPEN_TRUNCATE - delete any existing entry before opening it
    ///    *    OPEN_READONLY - don't create an entry if there is none
    ///    *    OPEN_PRIORITY - give this request a priority over others
    ///    *    OPEN_BYPASS_IF_BUSY - backward compatibility only, LOAD_BYPASS_LOCAL_CACHE_IF_BUSY
    ///    *    CHECK_MULTITHREADED - onCacheEntryCheck may be called on any thread, consumer
    ///    *                          implementation is thread-safe
    ///    * @param aCallback
    ///    *    The consumer that receives the result.
    ///    *    IMPORTANT: The callback may be called sooner the method returns.
    ///    */
    /// ```
    ///

    /// `void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback);`
    #[inline]
    pub unsafe fn AsyncOpenURI(&self, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, aFlags: uint32_t, aCallback: *const nsICacheEntryOpenCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncOpenURI)(self, aURI, aIdExtension, aFlags, aCallback)
    }


    /// ```text
    /// /**
    ///    * Immediately opens a new and empty cache entry in the storage, any existing
    ///    * entries are immediately doomed.  This is similar to the recreate() method
    ///    * on nsICacheEntry.
    ///    *
    ///    * Storage may not implement this method and throw NS_ERROR_NOT_IMPLEMENTED.
    ///    * In that case consumer must use asyncOpen with OPEN_TRUNCATE flag and get
    ///    * the new entry via a callback.
    ///    *
    ///    * @param aURI @see asyncOpenURI
    ///    * @param aIdExtension @see asyncOpenURI
    ///    */
    /// ```
    ///

    /// `nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension);`
    #[inline]
    pub unsafe fn OpenTruncate(&self, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, _retval: *mut*const nsICacheEntry) -> ::nserror::nsresult {
        ((*self.vtable).OpenTruncate)(self, aURI, aIdExtension, _retval)
    }


    /// ```text
    /// /**
    ///    * Synchronously check on existance of an entry.  In case of disk entries
    ///    * this uses information from the cache index.  When the index data are not
    ///    * up to date or index is still building, NS_ERROR_NOT_AVAILABLE is thrown.
    ///    * The same error may throw any storage implementation that cannot determine
    ///    * entry state without blocking the caller.
    ///    */
    /// ```
    ///

    /// `boolean exists (in nsIURI aURI, in ACString aIdExtension);`
    #[inline]
    pub unsafe fn Exists(&self, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Exists)(self, aURI, aIdExtension, _retval)
    }


    /// ```text
    /// /**
    ///    * Synchronously check on existance of alternative data and size of the
    ///    * content. When the index data are not up to date or index is still building,
    ///    * NS_ERROR_NOT_AVAILABLE is thrown. The same error may throw any storage
    ///    * implementation that cannot determine entry state without blocking the caller.
    ///    */
    /// ```
    ///

    /// `void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB);`
    #[inline]
    pub unsafe fn GetCacheIndexEntryAttrs(&self, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, aHasAltData: *mut bool, aSizeInKB: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheIndexEntryAttrs)(self, aURI, aIdExtension, aHasAltData, aSizeInKB)
    }


    /// ```text
    /// /**
    ///    * Asynchronously removes an entry belonging to the URI from the cache.
    ///    */
    /// ```
    ///

    /// `void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback);`
    #[inline]
    pub unsafe fn AsyncDoomURI(&self, aURI: *const nsIURI, aIdExtension: *const ::nsstring::nsACString, aCallback: *const nsICacheEntryDoomCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncDoomURI)(self, aURI, aIdExtension, aCallback)
    }


    /// ```text
    /// /**
    ///    * Asynchronously removes all cached entries under this storage.
    ///    * NOTE: Disk storage also evicts memory storage.
    ///    */
    /// ```
    ///

    /// `void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback);`
    #[inline]
    pub unsafe fn AsyncEvictStorage(&self, aCallback: *const nsICacheEntryDoomCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncEvictStorage)(self, aCallback)
    }


    /// ```text
    /// /**
    ///    * Visits the storage and its entries.
    ///    * NOTE: Disk storage also visits memory storage.
    ///    */
    /// ```
    ///

    /// `void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries);`
    #[inline]
    pub unsafe fn AsyncVisitStorage(&self, aVisitor: *const nsICacheStorageVisitor, aVisitEntries: bool) -> ::nserror::nsresult {
        ((*self.vtable).AsyncVisitStorage)(self, aVisitor, aVisitEntries)
    }


}


