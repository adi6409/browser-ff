//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheSession.idl
//


/// `interface nsICacheSession : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheSession {
    vtable: *const nsICacheSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheSession.
unsafe impl XpCom for nsICacheSession {
    const IID: nsIID = nsID(0x1dd7708c, 0xde48, 0x4ffe,
        [0xb5, 0xaa, 0xcd, 0x21, 0x8c, 0x76, 0x28, 0x87]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheSession {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheSession.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheSessionCoerce {
    /// Cheaply cast a value of this type from a `nsICacheSession`.
    fn coerce_from(v: &nsICacheSession) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheSessionCoerce for nsICacheSession {
    #[inline]
    fn coerce_from(v: &nsICacheSession) -> &Self {
        v
    }
}

impl nsICacheSession {
    /// Cast this `nsICacheSession` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheSession {
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
impl<T: nsISupportsCoerce> nsICacheSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheSession) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheSession
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheSessionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean doomEntriesIfExpired; */
    pub GetDoomEntriesIfExpired: unsafe extern "system" fn (this: *const nsICacheSession, aDoomEntriesIfExpired: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean doomEntriesIfExpired; */
    pub SetDoomEntriesIfExpired: unsafe extern "system" fn (this: *const nsICacheSession, aDoomEntriesIfExpired: bool) -> ::nserror::nsresult,

    /* attribute nsIFile profileDirectory; */
    pub GetProfileDirectory: unsafe extern "system" fn (this: *const nsICacheSession, aProfileDirectory: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsIFile profileDirectory; */
    pub SetProfileDirectory: unsafe extern "system" fn (this: *const nsICacheSession, aProfileDirectory: *const nsIFile) -> ::nserror::nsresult,

    /* nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode); */
    pub OpenCacheEntry: unsafe extern "system" fn (this: *const nsICacheSession, key: *const ::nsstring::nsACString, accessRequested: nsCacheAccessMode, blockingMode: bool, _retval: *mut*const nsICacheEntryDescriptor) -> ::nserror::nsresult,

    /* void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait); */
    pub AsyncOpenCacheEntry: unsafe extern "system" fn (this: *const nsICacheSession, key: *const ::nsstring::nsACString, accessRequested: nsCacheAccessMode, listener: *const nsICacheListener, noWait: bool) -> ::nserror::nsresult,

    /* void evictEntries (); */
    pub EvictEntries: unsafe extern "system" fn (this: *const nsICacheSession) -> ::nserror::nsresult,

    /* boolean isStorageEnabled (); */
    pub IsStorageEnabled: unsafe extern "system" fn (this: *const nsICacheSession, _retval: *mut bool) -> ::nserror::nsresult,

    /* void doomEntry (in ACString key, in nsICacheListener listener); */
    pub DoomEntry: unsafe extern "system" fn (this: *const nsICacheSession, key: *const ::nsstring::nsACString, listener: *const nsICacheListener) -> ::nserror::nsresult,

    /* attribute boolean isPrivate; */
    pub GetIsPrivate: unsafe extern "system" fn (this: *const nsICacheSession, aIsPrivate: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isPrivate; */
    pub SetIsPrivate: unsafe extern "system" fn (this: *const nsICacheSession, aIsPrivate: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheSession {

    /// ```text
    /// /**
    ///      * Expired entries will be doomed or evicted if this attribute is set to
    ///      * true.  If false, expired entries will be returned (useful for offline-
        ///      * mode and clients, such as HTTP, that can update the valid lifetime of
        ///      * cached content).  This attribute defaults to true.
    ///      */
    /// ```
    ///

    /// `attribute boolean doomEntriesIfExpired;`
    #[inline]
    pub unsafe fn GetDoomEntriesIfExpired(&self, aDoomEntriesIfExpired: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDoomEntriesIfExpired)(self, aDoomEntriesIfExpired)
    }


    /// ```text
    /// /**
    ///      * Expired entries will be doomed or evicted if this attribute is set to
    ///      * true.  If false, expired entries will be returned (useful for offline-
        ///      * mode and clients, such as HTTP, that can update the valid lifetime of
        ///      * cached content).  This attribute defaults to true.
    ///      */
    /// ```
    ///

    /// `attribute boolean doomEntriesIfExpired;`
    #[inline]
    pub unsafe fn SetDoomEntriesIfExpired(&self, aDoomEntriesIfExpired: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDoomEntriesIfExpired)(self, aDoomEntriesIfExpired)
    }


    /// ```text
    /// /**
    ///      * When set, entries created with this session will be placed to a cache
    ///      * based at this directory.  Use when storing entries to a different
    ///      * profile than the active profile of the the current running application
    ///      * process.
    ///      */
    /// ```
    ///

    /// `attribute nsIFile profileDirectory;`
    #[inline]
    pub unsafe fn GetProfileDirectory(&self, aProfileDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetProfileDirectory)(self, aProfileDirectory)
    }


    /// ```text
    /// /**
    ///      * When set, entries created with this session will be placed to a cache
    ///      * based at this directory.  Use when storing entries to a different
    ///      * profile than the active profile of the the current running application
    ///      * process.
    ///      */
    /// ```
    ///

    /// `attribute nsIFile profileDirectory;`
    #[inline]
    pub unsafe fn SetProfileDirectory(&self, aProfileDirectory: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetProfileDirectory)(self, aProfileDirectory)
    }


    /// ```text
    /// /**
    ///      * A cache session can only give out one descriptor with WRITE access
    ///      * to a given cache entry at a time.  Until the client calls MarkValid on
    ///      * its descriptor, other attempts to open the same cache entry will block.
    ///      */
    /// /**
    ///      * Synchronous cache access. This method fails if it is called on the main
    ///      * thread. Use asyncOpenCacheEntry() instead. This returns a unique
    ///      * descriptor each time it is called, even if the same key is specified.
    ///      * When called by multiple threads for write access, only one writable
    ///      * descriptor will be granted.  If 'blockingMode' is set to false, it will
    ///      * return NS_ERROR_CACHE_WAIT_FOR_VALIDATION rather than block when another
    ///      * descriptor has been given WRITE access but hasn't validated the entry yet.
    ///      */
    /// ```
    ///

    /// `nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode);`
    #[inline]
    pub unsafe fn OpenCacheEntry(&self, key: *const ::nsstring::nsACString, accessRequested: nsCacheAccessMode, blockingMode: bool, _retval: *mut*const nsICacheEntryDescriptor) -> ::nserror::nsresult {
        ((*self.vtable).OpenCacheEntry)(self, key, accessRequested, blockingMode, _retval)
    }


    /// ```text
    /// /**
    ///      * Asynchronous cache access. Does not block the calling thread. Instead,
    ///      * the listener will be notified when the descriptor is available. If
    ///      * 'noWait' is set to true, the listener will be notified immediately with
    ///      * status NS_ERROR_CACHE_WAIT_FOR_VALIDATION rather than queuing the request
    ///      * when another descriptor has been given WRITE access but hasn't validated
    ///      * the entry yet.
    ///      */
    /// ```
    ///

    /// `void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait);`
    #[inline]
    pub unsafe fn AsyncOpenCacheEntry(&self, key: *const ::nsstring::nsACString, accessRequested: nsCacheAccessMode, listener: *const nsICacheListener, noWait: bool) -> ::nserror::nsresult {
        ((*self.vtable).AsyncOpenCacheEntry)(self, key, accessRequested, listener, noWait)
    }


    /// ```text
    /// /**
    ///      * Evict all entries for this session's clientID according to its storagePolicy.
    ///      */
    /// ```
    ///

    /// `void evictEntries ();`
    #[inline]
    pub unsafe fn EvictEntries(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EvictEntries)(self, )
    }


    /// ```text
    /// /**
    ///      * Return whether any of the cache devices implied by the session storage policy
    ///      * are currently enabled for instantiation if they don't already exist.
    ///      */
    /// ```
    ///

    /// `boolean isStorageEnabled ();`
    #[inline]
    pub unsafe fn IsStorageEnabled(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsStorageEnabled)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Asynchronously doom an entry specified by the key. Listener will be
    ///      * notified about the status of the operation. Null may be passed if caller
    ///      * doesn't care about the result.
    ///      */
    /// ```
    ///

    /// `void doomEntry (in ACString key, in nsICacheListener listener);`
    #[inline]
    pub unsafe fn DoomEntry(&self, key: *const ::nsstring::nsACString, listener: *const nsICacheListener) -> ::nserror::nsresult {
        ((*self.vtable).DoomEntry)(self, key, listener)
    }


    /// ```text
    /// /**
    ///      * Private entries will be doomed when the last private browsing session
    ///      * finishes.
    ///      */
    /// ```
    ///

    /// `attribute boolean isPrivate;`
    #[inline]
    pub unsafe fn GetIsPrivate(&self, aIsPrivate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsPrivate)(self, aIsPrivate)
    }


    /// ```text
    /// /**
    ///      * Private entries will be doomed when the last private browsing session
    ///      * finishes.
    ///      */
    /// ```
    ///

    /// `attribute boolean isPrivate;`
    #[inline]
    pub unsafe fn SetIsPrivate(&self, aIsPrivate: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsPrivate)(self, aIsPrivate)
    }


}


