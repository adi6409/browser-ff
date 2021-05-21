//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICache.idl
//


/// `typedef int32_t  nsCacheStoragePolicy;`
///


pub type nsCacheStoragePolicy = i32;


/// `typedef int32_t  nsCacheAccessMode;`
///


pub type nsCacheAccessMode = i32;


/// `interface nsICache : nsISupports`
///

/// ```text
/// /**
///  * nsICache is a namespace for various cache constants.  It does not represent
///  * an actual object.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICache {
    vtable: *const nsICacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICache.
unsafe impl XpCom for nsICache {
    const IID: nsIID = nsID(0xd6c67f38, 0xb39a, 0x4582,
        [0x8a, 0x48, 0x4c, 0x4f, 0x8a, 0x56, 0xdf, 0xd0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICache {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICache.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheCoerce {
    /// Cheaply cast a value of this type from a `nsICache`.
    fn coerce_from(v: &nsICache) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheCoerce for nsICache {
    #[inline]
    fn coerce_from(v: &nsICache) -> &Self {
        v
    }
}

impl nsICache {
    /// Cast this `nsICache` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICache {
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
impl<T: nsISupportsCoerce> nsICacheCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICache) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICache
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICache {
    /// ```text
    /// /**
    ///      * Access Modes
    ///      *
    ///      *
    ///      * Mode Requested | Not Cached          | Cached
    ///      * ------------------------------------------------------------------------
    ///      * READ           | KEY_NOT_FOUND       | NS_OK
    ///      *                | Mode = NONE         | Mode = READ
    ///      *                | No Descriptor       | Descriptor
    ///      * ------------------------------------------------------------------------
    ///      * WRITE          | NS_OK               | NS_OK            (Cache service
        ///      *                | Mode = WRITE        | Mode = WRITE      dooms existing
        ///      *                | Descriptor          | Descriptor        cache entry)
    ///      * ------------------------------------------------------------------------
    ///      * READ_WRITE     | NS_OK               | NS_OK
    ///      * (1st req.)     | Mode = WRITE        | Mode = READ_WRITE
    ///      *                | Descriptor          | Descriptor
    ///      * ------------------------------------------------------------------------
    ///      * READ_WRITE     | N/A                 | NS_OK
    ///      * (Nth req.)     |                     | Mode = READ
    ///      *                |                     | Descriptor
    ///      * ------------------------------------------------------------------------
    ///      *
    ///      *
    ///      * Access Requested:
    ///      *
    ///      * READ	       - I only want to READ, if there isn't an entry just fail
    ///      * WRITE       - I have something new I want to write into the cache, make
    ///      *               me a new entry and doom the old one, if any.
    ///      * READ_WRITE  - I want to READ, but I'm willing to update an existing
    ///      *               entry if necessary, or create a new one if none exists.
    ///      *
    ///      *
    ///      * Access Granted:
    ///      *
    ///      * NONE        - No descriptor is provided. You get zilch. Nada. Nothing.
    ///      * READ		   - You can READ from this descriptor.
    ///      * WRITE	   - You must WRITE to this descriptor because the cache entry
    ///      *               was just created for you.
    ///      * READ_WRITE  - You can READ the descriptor to determine if it's valid,
    ///      *               you may WRITE if it needs updating.
    ///      *
    ///      *
    ///      * Comments:
    ///      *
    ///      * If you think that you might need to modify cached data or meta data,
    ///      * then you must open a cache entry requesting WRITE access.  Only one
    ///      * cache entry descriptor, per cache entry, will be granted WRITE access.
    ///      *
    ///      * Usually, you will request READ_WRITE access in order to first test the
    ///      * meta data and informational fields to determine if a write (ie. going
        ///      * to the net) may actually be necessary.  If you determine that it is
    ///      * not, then you would mark the cache entry as valid (using MarkValid) and
    ///      * then simply read the data from the cache.
    ///      *
    ///      * A descriptor granted WRITE access has exclusive access to the cache
    ///      * entry up to the point at which it marks it as valid.  Once the cache
    ///      * entry has been "validated", other descriptors with READ access may be
    ///      * opened to the cache entry.
    ///      *
    ///      * If you make a request for READ_WRITE access to a cache entry, the cache
    ///      * service will downgrade your access to READ if there is already a
    ///      * cache entry descriptor open with WRITE access.
    ///      *
    ///      * If you make a request for only WRITE access to a cache entry and another
    ///      * descriptor with WRITE access is currently open, then the existing cache
    ///      * entry will be 'doomed', and you will be given a descriptor (with WRITE
        ///      * access only) to a new cache entry.
    ///      *
    ///      */
    /// ```
    ///

    pub const ACCESS_NONE: i64 = 0;


    pub const ACCESS_READ: i64 = 1;


    pub const ACCESS_WRITE: i64 = 2;


    pub const ACCESS_READ_WRITE: i64 = 3;

    /// ```text
    /// /**
    ///      * Storage Policy
    ///      *
    ///      * The storage policy of a cache entry determines the device(s) to which
    ///      * it belongs.  See nsICacheSession and nsICacheEntryDescriptor for more
    ///      * details.
    ///      *
    ///      * STORE_ANYWHERE        - Allows the cache entry to be stored in any device.
    ///      *                         The cache service decides which cache device to use
    ///      *                         based on "some resource management calculation."
    ///      * STORE_IN_MEMORY       - Requires the cache entry to reside in non-persistent
    ///      *                         storage (ie. typically in system RAM).
    ///      * STORE_ON_DISK         - Requires the cache entry to reside in persistent
    ///      *                         storage (ie. typically on a system's hard disk).
    ///      * STORE_OFFLINE         - Requires the cache entry to reside in persistent,
    ///      *                         reliable storage for offline use.
    ///      */
    /// ```
    ///

    pub const STORE_ANYWHERE: i64 = 0;


    pub const STORE_IN_MEMORY: i64 = 1;


    pub const STORE_ON_DISK: i64 = 2;


    pub const STORE_OFFLINE: i64 = 4;

    /// ```text
    /// /**
    ///      * All entries for a cache session are stored as streams of data or
    ///      * as objects.  These constant my be used to specify the type of entries
    ///      * when calling nsICacheService::CreateSession().
    ///      */
    /// ```
    ///

    pub const NOT_STREAM_BASED: i64 = 0;


    pub const STREAM_BASED: i64 = 1;

    /// ```text
    /// /**
    ///      * The synchronous OpenCacheEntry() may be blocking or non-blocking.  If a cache entry is
    ///      * waiting to be validated by another cache descriptor (so no new cache descriptors for that
        ///      * key can be created, OpenCacheEntry() will return NS_ERROR_CACHE_WAIT_FOR_VALIDATION in
        ///      * non-blocking mode.  In blocking mode, it will wait until the cache entry for the key has
        ///      * been validated or doomed.  If the cache entry is validated, then a descriptor for that
        ///      * entry will be created and returned.  If the cache entry was doomed, then a descriptor
        ///      * will be created for a new cache entry for the key.
        ///      */
        /// ```
        ///

        pub const NON_BLOCKING: i64 = 0;


        pub const BLOCKING: i64 = 1;

        /// ```text
        /// /**
        ///      * Constant meaning no expiration time.
        ///      */
        /// ```
        ///

        pub const NO_EXPIRATION_TIME: i64 = 4294967295;


    }


