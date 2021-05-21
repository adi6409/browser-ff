//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheStorageService.idl
//


/// `interface nsICacheStorageService : nsISupports`
///

/// ```text
/// /**
///  * Provides access to particual cache storages of the network URI cache.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheStorageService {
    vtable: *const nsICacheStorageServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheStorageService.
unsafe impl XpCom for nsICacheStorageService {
    const IID: nsIID = nsID(0xae29c44b, 0xfbc3, 0x4552,
        [0xaf, 0xaf, 0x0a, 0x15, 0x7c, 0xe7, 0x71, 0xe7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheStorageService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheStorageService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheStorageServiceCoerce {
    /// Cheaply cast a value of this type from a `nsICacheStorageService`.
    fn coerce_from(v: &nsICacheStorageService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheStorageServiceCoerce for nsICacheStorageService {
    #[inline]
    fn coerce_from(v: &nsICacheStorageService) -> &Self {
        v
    }
}

impl nsICacheStorageService {
    /// Cast this `nsICacheStorageService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheStorageServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheStorageService {
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
impl<T: nsISupportsCoerce> nsICacheStorageServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorageService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheStorageService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheStorageServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    pub MemoryCacheStorage: unsafe extern "system" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult,

    /* nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache); */
    pub DiskCacheStorage: unsafe extern "system" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, aLookupAppCache: bool, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult,

    /* nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    pub PinningCacheStorage: unsafe extern "system" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult,

    /* nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache); */
    pub AppCacheStorage: unsafe extern "system" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, aApplicationCache: *const nsIApplicationCache, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult,

    /* nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    pub SynthesizedCacheStorage: unsafe extern "system" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult,

    /* void clearOrigin (in nsIPrincipal aPrincipal); */
    pub ClearOrigin: unsafe extern "system" fn (this: *const nsICacheStorageService, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* void clearOriginAttributes (in AString aOriginAttributes); */
    pub ClearOriginAttributes: unsafe extern "system" fn (this: *const nsICacheStorageService, aOriginAttributes: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void clear (); */
    pub Clear: unsafe extern "system" fn (this: *const nsICacheStorageService) -> ::nserror::nsresult,

    /* void purgeFromMemory (in uint32_t aWhat); */
    pub PurgeFromMemory: unsafe extern "system" fn (this: *const nsICacheStorageService, aWhat: uint32_t) -> ::nserror::nsresult,

    /* readonly attribute nsIEventTarget ioTarget; */
    pub GetIoTarget: unsafe extern "system" fn (this: *const nsICacheStorageService, aIoTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,

    /* void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver); */
    pub AsyncGetDiskConsumption: unsafe extern "system" fn (this: *const nsICacheStorageService, aObserver: *const nsICacheStorageConsumptionObserver) -> ::nserror::nsresult,

    /* void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
    pub AsyncVisitAllStorages: unsafe extern "system" fn (this: *const nsICacheStorageService, aVisitor: *const nsICacheStorageVisitor, aVisitEntries: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheStorageService {
    /// ```text
    /// /**
    ///    * Purge only data of disk backed entries.  Metadata are left for
    ///    * performance purposes.
    ///    */
    /// ```
    ///

    pub const PURGE_DISK_DATA_ONLY: i64 = 1;

    /// ```text
    /// /**
    ///    * Purge whole disk backed entries from memory.  Disk files will
    ///    * be left unattended.
    ///    */
    /// ```
    ///

    pub const PURGE_DISK_ALL: i64 = 2;

    /// ```text
    /// /**
    ///    * Purge all entries we keep in memory, including memory-storage
    ///    * entries.  This may be dangerous to use.
    ///    */
    /// ```
    ///

    pub const PURGE_EVERYTHING: i64 = 3;

    /// ```text
    /// /**
    ///    * Get storage where entries will only remain in memory, never written
    ///    * to the disk.
    ///    *
    ///    * NOTE: Any existing disk entry for [URL|id-extension] will be doomed
    ///    * prior opening an entry using this memory-only storage.  Result of
    ///    * AsyncOpenURI will be a new and empty memory-only entry.  Using
    ///    * OPEN_READONLY open flag has no effect on this behavior.
    ///    *
    ///    * @param aLoadContextInfo
    ///    *    Information about the loading context, this focuses the storage JAR and
    ///    *    respects separate storage for private browsing.
    ///    */
    /// ```
    ///

    /// `nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo);`
    #[inline]
    pub unsafe fn MemoryCacheStorage(&self, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult {
        ((*self.vtable).MemoryCacheStorage)(self, aLoadContextInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Get storage where entries will be written to disk when not forbidden by
    ///    * response headers.
    ///    *
    ///    * @param aLookupAppCache
    ///    *    When set true (for top level document loading channels) app cache will
    ///    *    be first to check on to find entries in.
    ///    */
    /// ```
    ///

    /// `nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache);`
    #[inline]
    pub unsafe fn DiskCacheStorage(&self, aLoadContextInfo: *const nsILoadContextInfo, aLookupAppCache: bool, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult {
        ((*self.vtable).DiskCacheStorage)(self, aLoadContextInfo, aLookupAppCache, _retval)
    }


    /// ```text
    /// /**
    ///    * Get storage where entries will be written to disk and marked as pinned.
    ///    * These pinned entries are immune to over limit eviction and call of clear()
    ///    * on this service.
    ///    */
    /// ```
    ///

    /// `nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo);`
    #[inline]
    pub unsafe fn PinningCacheStorage(&self, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult {
        ((*self.vtable).PinningCacheStorage)(self, aLoadContextInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Get storage for a specified application cache obtained using some different
    ///    * mechanism.
    ///    *
    ///    * @param aLoadContextInfo
    ///    *    Mandatory reference to a load context information.
    ///    * @param aApplicationCache
    ///    *    Optional reference to an existing appcache.  When left null, this will
    ///    *    work with offline cache as a whole.
    ///    */
    /// ```
    ///

    /// `nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache);`
    #[inline]
    pub unsafe fn AppCacheStorage(&self, aLoadContextInfo: *const nsILoadContextInfo, aApplicationCache: *const nsIApplicationCache, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult {
        ((*self.vtable).AppCacheStorage)(self, aLoadContextInfo, aApplicationCache, _retval)
    }


    /// ```text
    /// /**
    ///    * Get storage for synthesized cache entries that we currently use for ServiceWorker interception in non-e10s mode.
    ///    *
    ///    * This cache storage has no limits on file size to allow the ServiceWorker to intercept large files.
    ///    */
    /// ```
    ///

    /// `nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo);`
    #[inline]
    pub unsafe fn SynthesizedCacheStorage(&self, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut*const nsICacheStorage) -> ::nserror::nsresult {
        ((*self.vtable).SynthesizedCacheStorage)(self, aLoadContextInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Evict any cache entry having the same origin of aPrincipal.
    ///    *
    ///    * @param aPrincipal
    ///    *   The principal to compare the entries with.
    ///    */
    /// ```
    ///

    /// `void clearOrigin (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn ClearOrigin(&self, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).ClearOrigin)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///    * Evict any cache entry having the same originAttributes.
    ///    *
    ///    * @param aOriginAttributes
    ///    *   The origin attributes in string format to compare the entries with.
    ///    */
    /// ```
    ///

    /// `void clearOriginAttributes (in AString aOriginAttributes);`
    #[inline]
    pub unsafe fn ClearOriginAttributes(&self, aOriginAttributes: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ClearOriginAttributes)(self, aOriginAttributes)
    }


    /// ```text
    /// /**
    ///    * Evict the whole cache.
    ///    */
    /// ```
    ///

    /// `void clear ();`
    #[inline]
    pub unsafe fn Clear(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, )
    }


    /// ```text
    /// /**
    ///    * Purges data we keep warmed in memory.  Use for tests and for
    ///    * saving memory.
    ///    */
    /// ```
    ///

    /// `void purgeFromMemory (in uint32_t aWhat);`
    #[inline]
    pub unsafe fn PurgeFromMemory(&self, aWhat: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).PurgeFromMemory)(self, aWhat)
    }


    /// ```text
    /// /**
    ///    * I/O thread target to use for any operations on disk
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIEventTarget ioTarget;`
    #[inline]
    pub unsafe fn GetIoTarget(&self, aIoTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetIoTarget)(self, aIoTarget)
    }


    /// ```text
    /// /**
    ///    * Asynchronously determine how many bytes of the disk space the cache takes.
    ///    * @see nsICacheStorageConsumptionObserver
    ///    * @param aObserver
    ///    *    A mandatory (weak referred) observer.  Documented at
    ///    *    nsICacheStorageConsumptionObserver.
    ///    *    NOTE: the observer MUST implement nsISupportsWeakReference.
    ///    */
    /// ```
    ///

    /// `void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver);`
    #[inline]
    pub unsafe fn AsyncGetDiskConsumption(&self, aObserver: *const nsICacheStorageConsumptionObserver) -> ::nserror::nsresult {
        ((*self.vtable).AsyncGetDiskConsumption)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Asynchronously visits all storages of the disk cache and memory cache.
    ///    * @see nsICacheStorageVisitor
    ///    * @param aVisitor
    ///    *   A visitor callback.
    ///    * @param aVisitEntries
    ///    *   A boolean indicates whether visits entries.
    ///    */
    /// ```
    ///

    /// `void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries);`
    #[inline]
    pub unsafe fn AsyncVisitAllStorages(&self, aVisitor: *const nsICacheStorageVisitor, aVisitEntries: bool) -> ::nserror::nsresult {
        ((*self.vtable).AsyncVisitAllStorages)(self, aVisitor, aVisitEntries)
    }


}


/// `interface nsICacheStorageConsumptionObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheStorageConsumptionObserver {
    vtable: *const nsICacheStorageConsumptionObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheStorageConsumptionObserver.
unsafe impl XpCom for nsICacheStorageConsumptionObserver {
    const IID: nsIID = nsID(0x7728ab5b, 0x4c01, 0x4483,
        [0xa6, 0x06, 0x32, 0xbf, 0x5b, 0x81, 0x36, 0xcb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheStorageConsumptionObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheStorageConsumptionObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheStorageConsumptionObserverCoerce {
    /// Cheaply cast a value of this type from a `nsICacheStorageConsumptionObserver`.
    fn coerce_from(v: &nsICacheStorageConsumptionObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheStorageConsumptionObserverCoerce for nsICacheStorageConsumptionObserver {
    #[inline]
    fn coerce_from(v: &nsICacheStorageConsumptionObserver) -> &Self {
        v
    }
}

impl nsICacheStorageConsumptionObserver {
    /// Cast this `nsICacheStorageConsumptionObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheStorageConsumptionObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheStorageConsumptionObserver {
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
impl<T: nsISupportsCoerce> nsICacheStorageConsumptionObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorageConsumptionObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheStorageConsumptionObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheStorageConsumptionObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onNetworkCacheDiskConsumption (in int64_t aDiskSize); */
    pub OnNetworkCacheDiskConsumption: unsafe extern "system" fn (this: *const nsICacheStorageConsumptionObserver, aDiskSize: int64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheStorageConsumptionObserver {

    /// ```text
    /// /**
    ///    * Callback invoked to answer asyncGetDiskConsumption call. Always triggered
    ///    * on the main thread.
    ///    * NOTE: implementers must also implement nsISupportsWeakReference.
    ///    *
    ///    * @param aDiskSize
    ///    *    The disk consumption in bytes.
    ///    */
    /// ```
    ///

    /// `void onNetworkCacheDiskConsumption (in int64_t aDiskSize);`
    #[inline]
    pub unsafe fn OnNetworkCacheDiskConsumption(&self, aDiskSize: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).OnNetworkCacheDiskConsumption)(self, aDiskSize)
    }


}


