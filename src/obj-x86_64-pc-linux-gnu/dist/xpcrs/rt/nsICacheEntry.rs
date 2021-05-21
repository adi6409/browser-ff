//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntry.idl
//


/// `interface nsICacheEntry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheEntry {
    vtable: *const nsICacheEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheEntry.
unsafe impl XpCom for nsICacheEntry {
    const IID: nsIID = nsID(0x607c2a2c, 0x0a48, 0x40b9,
        [0xa9, 0x56, 0x8c, 0xf2, 0xbb, 0x98, 0x57, 0xcf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheEntryCoerce {
    /// Cheaply cast a value of this type from a `nsICacheEntry`.
    fn coerce_from(v: &nsICacheEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheEntryCoerce for nsICacheEntry {
    #[inline]
    fn coerce_from(v: &nsICacheEntry) -> &Self {
        v
    }
}

impl nsICacheEntry {
    /// Cast this `nsICacheEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheEntry {
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
impl<T: nsISupportsCoerce> nsICacheEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString key; */
    pub GetKey: unsafe extern "system" fn (this: *const nsICacheEntry, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute uint64_t cacheEntryId; */
    pub GetCacheEntryId: unsafe extern "system" fn (this: *const nsICacheEntry, aCacheEntryId: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute boolean persistent; */
    pub GetPersistent: unsafe extern "system" fn (this: *const nsICacheEntry, aPersistent: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long fetchCount; */
    pub GetFetchCount: unsafe extern "system" fn (this: *const nsICacheEntry, aFetchCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute uint32_t lastFetched; */
    pub GetLastFetched: unsafe extern "system" fn (this: *const nsICacheEntry, aLastFetched: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t lastModified; */
    pub GetLastModified: unsafe extern "system" fn (this: *const nsICacheEntry, aLastModified: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t expirationTime; */
    pub GetExpirationTime: unsafe extern "system" fn (this: *const nsICacheEntry, aExpirationTime: *mut uint32_t) -> ::nserror::nsresult,

    /* void setExpirationTime (in uint32_t expirationTime); */
    pub SetExpirationTime: unsafe extern "system" fn (this: *const nsICacheEntry, expirationTime: uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint64_t onStartTime; */
    pub GetOnStartTime: unsafe extern "system" fn (this: *const nsICacheEntry, aOnStartTime: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute uint64_t onStopTime; */
    pub GetOnStopTime: unsafe extern "system" fn (this: *const nsICacheEntry, aOnStopTime: *mut uint64_t) -> ::nserror::nsresult,

    /* void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime); */
    pub SetNetworkTimes: unsafe extern "system" fn (this: *const nsICacheEntry, onStartTime: uint64_t, onStopTime: uint64_t) -> ::nserror::nsresult,

    /* void setContentType (in uint8_t contentType); */
    pub SetContentType: unsafe extern "system" fn (this: *const nsICacheEntry, contentType: uint8_t) -> ::nserror::nsresult,

    /* void forceValidFor (in unsigned long aSecondsToTheFuture); */
    pub ForceValidFor: unsafe extern "system" fn (this: *const nsICacheEntry, aSecondsToTheFuture: u32) -> ::nserror::nsresult,

    /* readonly attribute boolean isForcedValid; */
    pub GetIsForcedValid: unsafe extern "system" fn (this: *const nsICacheEntry, aIsForcedValid: *mut bool) -> ::nserror::nsresult,

    /* nsIInputStream openInputStream (in long long offset); */
    pub OpenInputStream: unsafe extern "system" fn (this: *const nsICacheEntry, offset: i64, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* nsIOutputStream openOutputStream (in long long offset, in long long predictedSize); */
    pub OpenOutputStream: unsafe extern "system" fn (this: *const nsICacheEntry, offset: i64, predictedSize: i64, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* attribute nsISupports securityInfo; */
    pub GetSecurityInfo: unsafe extern "system" fn (this: *const nsICacheEntry, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsISupports securityInfo; */
    pub SetSecurityInfo: unsafe extern "system" fn (this: *const nsICacheEntry, aSecurityInfo: *const nsISupports) -> ::nserror::nsresult,

    /* readonly attribute unsigned long storageDataSize; */
    pub GetStorageDataSize: unsafe extern "system" fn (this: *const nsICacheEntry, aStorageDataSize: *mut u32) -> ::nserror::nsresult,

    /* void asyncDoom (in nsICacheEntryDoomCallback listener); */
    pub AsyncDoom: unsafe extern "system" fn (this: *const nsICacheEntry, listener: *const nsICacheEntryDoomCallback) -> ::nserror::nsresult,

    /* string getMetaDataElement (in string key); */
    pub GetMetaDataElement: unsafe extern "system" fn (this: *const nsICacheEntry, key: *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* void setMetaDataElement (in string key, in string value); */
    pub SetMetaDataElement: unsafe extern "system" fn (this: *const nsICacheEntry, key: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult,

    /* void visitMetaData (in nsICacheEntryMetaDataVisitor visitor); */
    pub VisitMetaData: unsafe extern "system" fn (this: *const nsICacheEntry, visitor: *const nsICacheEntryMetaDataVisitor) -> ::nserror::nsresult,

    /* void metaDataReady (); */
    pub MetaDataReady: unsafe extern "system" fn (this: *const nsICacheEntry) -> ::nserror::nsresult,

    /* void setValid (); */
    pub SetValid: unsafe extern "system" fn (this: *const nsICacheEntry) -> ::nserror::nsresult,

    /* void dismiss (); */
    pub Dismiss: unsafe extern "system" fn (this: *const nsICacheEntry) -> ::nserror::nsresult,

    /* readonly attribute uint32_t diskStorageSizeInKB; */
    pub GetDiskStorageSizeInKB: unsafe extern "system" fn (this: *const nsICacheEntry, aDiskStorageSizeInKB: *mut uint32_t) -> ::nserror::nsresult,

    /* nsICacheEntry recreate ([optional] in boolean aMemoryOnly); */
    pub Recreate: unsafe extern "system" fn (this: *const nsICacheEntry, aMemoryOnly: bool, _retval: *mut *const nsICacheEntry) -> ::nserror::nsresult,

    /* readonly attribute long long dataSize; */
    pub GetDataSize: unsafe extern "system" fn (this: *const nsICacheEntry, aDataSize: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long altDataSize; */
    pub GetAltDataSize: unsafe extern "system" fn (this: *const nsICacheEntry, aAltDataSize: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute ACString altDataType; */
    pub GetAltDataType: unsafe extern "system" fn (this: *const nsICacheEntry, aAltDataType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize); */
    pub OpenAlternativeOutputStream: unsafe extern "system" fn (this: *const nsICacheEntry, type_: *const ::nsstring::nsACString, predictedSize: i64, _retval: *mut*const nsIAsyncOutputStream) -> ::nserror::nsresult,

    /* nsIInputStream openAlternativeInputStream (in ACString type); */
    pub OpenAlternativeInputStream: unsafe extern "system" fn (this: *const nsICacheEntry, type_: *const ::nsstring::nsACString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* readonly attribute nsILoadContextInfo loadContextInfo; */
    pub GetLoadContextInfo: unsafe extern "system" fn (this: *const nsICacheEntry, aLoadContextInfo: *mut*const nsILoadContextInfo) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsICacheEntry) -> ::nserror::nsresult,

    /* void markValid (); */
    pub MarkValid: unsafe extern "system" fn (this: *const nsICacheEntry) -> ::nserror::nsresult,

    /* void maybeMarkValid (); */
    pub MaybeMarkValid: unsafe extern "system" fn (this: *const nsICacheEntry) -> ::nserror::nsresult,

    /* boolean hasWriteAccess (in boolean aWriteAllowed); */
    pub HasWriteAccess: unsafe extern "system" fn (this: *const nsICacheEntry, aWriteAllowed: bool, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheEntry {

    pub const CONTENT_TYPE_UNKNOWN: i64 = 0;


    pub const CONTENT_TYPE_OTHER: i64 = 1;


    pub const CONTENT_TYPE_JAVASCRIPT: i64 = 2;


    pub const CONTENT_TYPE_IMAGE: i64 = 3;


    pub const CONTENT_TYPE_MEDIA: i64 = 4;


    pub const CONTENT_TYPE_STYLESHEET: i64 = 5;


    pub const CONTENT_TYPE_WASM: i64 = 6;

    /// ```text
    /// /**
    ///    * Content type that is used internally to check whether the value parsed
    ///    * from disk is within allowed limits. Don't pass CONTENT_TYPE_LAST to
    ///    * setContentType method.
    ///    */
    /// ```
    ///

    pub const CONTENT_TYPE_LAST: i64 = 7;

    /// ```text
    /// /**
    ///    * Placeholder for the initial value of expiration time.
    ///    */
    /// ```
    ///

    pub const NO_EXPIRATION_TIME: i64 = 4294967295;

    /// ```text
    /// /**
    ///    * Get the key identifying the cache entry.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString key;`
    #[inline]
    pub unsafe fn GetKey(&self, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKey)(self, aKey)
    }


    /// ```text
    /// /**
    ///    * The unique ID for every nsICacheEntry instance, which can be used to check
    ///    * whether two pieces of information are from the same nsICacheEntry instance.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint64_t cacheEntryId;`
    #[inline]
    pub unsafe fn GetCacheEntryId(&self, aCacheEntryId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheEntryId)(self, aCacheEntryId)
    }


    /// ```text
    /// /**
    ///    * Whether the entry is memory/only or persisted to disk.
    ///    * Note: private browsing entries are reported as persistent for consistency
    ///    * while are not actually persisted to disk.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean persistent;`
    #[inline]
    pub unsafe fn GetPersistent(&self, aPersistent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPersistent)(self, aPersistent)
    }


    /// ```text
    /// /**
    ///    * Get the number of times the cache entry has been opened.
    ///    */
    /// ```
    ///

    /// `readonly attribute long fetchCount;`
    #[inline]
    pub unsafe fn GetFetchCount(&self, aFetchCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetFetchCount)(self, aFetchCount)
    }


    /// ```text
    /// /**
    ///    * Get the last time the cache entry was opened (in seconds since the Epoch).
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t lastFetched;`
    #[inline]
    pub unsafe fn GetLastFetched(&self, aLastFetched: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastFetched)(self, aLastFetched)
    }


    /// ```text
    /// /**
    ///    * Get the last time the cache entry was modified (in seconds since the Epoch).
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t lastModified;`
    #[inline]
    pub unsafe fn GetLastModified(&self, aLastModified: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModified)(self, aLastModified)
    }


    /// ```text
    /// /**
    ///    * Get the expiration time of the cache entry (in seconds since the Epoch).
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t expirationTime;`
    #[inline]
    pub unsafe fn GetExpirationTime(&self, aExpirationTime: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetExpirationTime)(self, aExpirationTime)
    }


    /// ```text
    /// /**
    ///    * Set the time at which the cache entry should be considered invalid (in
        ///    * seconds since the Epoch).
    ///    */
    /// ```
    ///

    /// `void setExpirationTime (in uint32_t expirationTime);`
    #[inline]
    pub unsafe fn SetExpirationTime(&self, expirationTime: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetExpirationTime)(self, expirationTime)
    }


    /// ```text
    /// /**
    ///    * Get the last network response times for onStartReqeust/onStopRequest (in ms).
    ///    * @throws
    ///    *    - NS_ERROR_NOT_AVAILABLE if onStartTime/onStopTime does not exist.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint64_t onStartTime;`
    #[inline]
    pub unsafe fn GetOnStartTime(&self, aOnStartTime: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetOnStartTime)(self, aOnStartTime)
    }



    /// `readonly attribute uint64_t onStopTime;`
    #[inline]
    pub unsafe fn GetOnStopTime(&self, aOnStopTime: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetOnStopTime)(self, aOnStopTime)
    }


    /// ```text
    /// /**
    ///    * Set the network response times for onStartReqeust/onStopRequest (in ms).
    ///    */
    /// ```
    ///

    /// `void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime);`
    #[inline]
    pub unsafe fn SetNetworkTimes(&self, onStartTime: uint64_t, onStopTime: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetNetworkTimes)(self, onStartTime, onStopTime)
    }


    /// ```text
    /// /**
    ///    * Set content type. Available types are defined at the begining of this file.
    ///    * The content type is used internally for cache partitioning and telemetry
    ///    * purposes so there is no getter.
    ///    */
    /// ```
    ///

    /// `void setContentType (in uint8_t contentType);`
    #[inline]
    pub unsafe fn SetContentType(&self, contentType: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).SetContentType)(self, contentType)
    }


    /// ```text
    /// /**
    ///    * This method is intended to override the per-spec cache validation
    ///    * decisions for a duration specified in seconds. The current state can
    ///    * be examined with isForcedValid (see below). This value is not persisted,
    ///    * so it will not survive session restart. Cache entries that are forced valid
    ///    * will not be evicted from the cache for the duration of forced validity.
    ///    * This means that there is a potential problem if the number of forced valid
    ///    * entries grows to take up more space than the cache size allows.
    ///    *
    ///    * NOTE: entries that have been forced valid will STILL be ignored by HTTP
    ///    * channels if they have expired AND the resource in question requires
    ///    * validation after expiring. This is to avoid using known-stale content.
    ///    *
    ///    * @param aSecondsToTheFuture
    ///    *        the number of seconds the default cache validation behavior will be
    ///    *        overridden before it returns to normal
    ///    */
    /// ```
    ///

    /// `void forceValidFor (in unsigned long aSecondsToTheFuture);`
    #[inline]
    pub unsafe fn ForceValidFor(&self, aSecondsToTheFuture: u32) -> ::nserror::nsresult {
        ((*self.vtable).ForceValidFor)(self, aSecondsToTheFuture)
    }


    /// ```text
    /// /**
    ///    * The state variable for whether this entry is currently forced valid.
    ///    * Defaults to false for normal cache validation behavior, and will return
    ///    * true if the number of seconds set by forceValidFor() has yet to be reached.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isForcedValid;`
    #[inline]
    pub unsafe fn GetIsForcedValid(&self, aIsForcedValid: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsForcedValid)(self, aIsForcedValid)
    }


    /// ```text
    /// /**
    ///    * Open blocking input stream to cache data.  Use the stream transport
    ///    * service to asynchronously read this stream on a background thread.
    ///    * The returned stream MAY implement nsISeekableStream.
    ///    *
    ///    * @param offset
    ///    *        read starting from this offset into the cached data.  an offset
    ///    *        beyond the end of the stream has undefined consequences.
    ///    *
    ///    * @return non-blocking, buffered input stream.
    ///    */
    /// ```
    ///

    /// `nsIInputStream openInputStream (in long long offset);`
    #[inline]
    pub unsafe fn OpenInputStream(&self, offset: i64, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenInputStream)(self, offset, _retval)
    }


    /// ```text
    /// /**
    ///    * Open non-blocking output stream to cache data.  The returned stream
    ///    * MAY implement nsISeekableStream.
    ///    *
    ///    * If opening an output stream to existing cached data, the data will be
    ///    * truncated to the specified offset.
    ///    *
    ///    * @param offset
    ///    *        write starting from this offset into the cached data.  an offset
    ///    *        beyond the end of the stream has undefined consequences.
    ///    * @param predictedSize
    ///    *        Predicted size of the data that will be written. It's used to decide
    ///    *        whether the resulting entry would exceed size limit, in which case
    ///    *        an error is thrown. If the size isn't known in advance, -1 should be
    ///    *        passed.
    ///    *
    ///    * @return blocking, buffered output stream.
    ///    */
    /// ```
    ///

    /// `nsIOutputStream openOutputStream (in long long offset, in long long predictedSize);`
    #[inline]
    pub unsafe fn OpenOutputStream(&self, offset: i64, predictedSize: i64, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenOutputStream)(self, offset, predictedSize, _retval)
    }


    /// ```text
    /// /**
    ///    * Get/set security info on the cache entry for this descriptor.
    ///    */
    /// ```
    ///

    /// `attribute nsISupports securityInfo;`
    #[inline]
    pub unsafe fn GetSecurityInfo(&self, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetSecurityInfo)(self, aSecurityInfo)
    }


    /// ```text
    /// /**
    ///    * Get/set security info on the cache entry for this descriptor.
    ///    */
    /// ```
    ///

    /// `attribute nsISupports securityInfo;`
    #[inline]
    pub unsafe fn SetSecurityInfo(&self, aSecurityInfo: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetSecurityInfo)(self, aSecurityInfo)
    }


    /// ```text
    /// /**
    ///    * Get the size of the cache entry data, as stored. This may differ
    ///    * from the entry's dataSize, if the entry is compressed.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long storageDataSize;`
    #[inline]
    pub unsafe fn GetStorageDataSize(&self, aStorageDataSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageDataSize)(self, aStorageDataSize)
    }


    /// ```text
    /// /**
    ///    * Asynchronously doom an entry. Listener will be notified about the status
    ///    * of the operation. Null may be passed if caller doesn't care about the
    ///    * result.
    ///    */
    /// ```
    ///

    /// `void asyncDoom (in nsICacheEntryDoomCallback listener);`
    #[inline]
    pub unsafe fn AsyncDoom(&self, listener: *const nsICacheEntryDoomCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncDoom)(self, listener)
    }


    /// ```text
    /// /**
    ///    * Methods for accessing meta data.  Meta data is a table of key/value
    ///    * string pairs.  The strings do not have to conform to any particular
    ///    * charset, but they must be null terminated.
    ///    */
    /// ```
    ///

    /// `string getMetaDataElement (in string key);`
    #[inline]
    pub unsafe fn GetMetaDataElement(&self, key: *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetMetaDataElement)(self, key, _retval)
    }



    /// `void setMetaDataElement (in string key, in string value);`
    #[inline]
    pub unsafe fn SetMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetMetaDataElement)(self, key, value)
    }


    /// ```text
    /// /**
    ///    * Obtain the list of metadata keys this entry keeps.
    ///    *
    ///    * NOTE: The callback is invoked under the CacheFile's lock.  It means
    ///    * there should not be made any calls to the entry from the visitor and
    ///    * if the values need to be processed somehow, it's better to cache them
    ///    * and process outside the callback.
    ///    */
    /// ```
    ///

    /// `void visitMetaData (in nsICacheEntryMetaDataVisitor visitor);`
    #[inline]
    pub unsafe fn VisitMetaData(&self, visitor: *const nsICacheEntryMetaDataVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitMetaData)(self, visitor)
    }


    /// ```text
    /// /**
    ///    * Claims that all metadata on this entry are up-to-date and this entry
    ///    * now can be delivered to other waiting consumers.
    ///    *
    ///    * We need such method since metadata must be delivered synchronously.
    ///    */
    /// ```
    ///

    /// `void metaDataReady ();`
    #[inline]
    pub unsafe fn MetaDataReady(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MetaDataReady)(self, )
    }


    /// ```text
    /// /**
    ///    * Called by consumer upon 304/206 response from the server.  This marks
    ///    * the entry content as positively revalidated.
    ///    * Consumer uses this method after the consumer has returned ENTRY_NEEDS_REVALIDATION
    ///    * result from onCacheEntryCheck and after successfull revalidation with the server.
    ///    */
    /// ```
    ///

    /// `void setValid ();`
    #[inline]
    pub unsafe fn SetValid(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetValid)(self, )
    }


    /// ```text
    /// /**
    ///    * Explicitly tell the cache backend this consumer is no longer going to modify
    ///    * this cache entry data or metadata.  In case the consumer was responsible to
    ///    * either of writing the cache entry or revalidating it, calling this method
    ///    * reverts the state to initial (as never written) or as not-validated and
    ///    * immediately notifies the next consumer in line waiting for this entry.
    ///    * This is the way to prevent deadlocks when someone else than the responsible
    ///    * channel references the cache entry being in a non-written or revalidating
    ///    * state.
    ///    */
    /// ```
    ///

    /// `void dismiss ();`
    #[inline]
    pub unsafe fn Dismiss(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Dismiss)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns the size in kilobytes used to store the cache entry on disk.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t diskStorageSizeInKB;`
    #[inline]
    pub unsafe fn GetDiskStorageSizeInKB(&self, aDiskStorageSizeInKB: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetDiskStorageSizeInKB)(self, aDiskStorageSizeInKB)
    }


    /// ```text
    /// /**
    ///    * Doom this entry and open a new, empty, entry for write.  Consumer has
    ///    * to exchange the entry this method is called on for the newly created.
    ///    * Used on 200 responses to conditional requests.
    ///    *
    ///    * @param aMemoryOnly
    ///    *    - whether the entry is to be created as memory/only regardless how
    ///    *      the entry being recreated persistence is set
    ///    * @returns
    ///    *    - an entry that can be used to write to
    ///    * @throws
    ///    *    - NS_ERROR_NOT_AVAILABLE when the entry cannot be from some reason
    ///    *      recreated for write
    ///    */
    /// ```
    ///

    /// `nsICacheEntry recreate ([optional] in boolean aMemoryOnly);`
    #[inline]
    pub unsafe fn Recreate(&self, aMemoryOnly: bool, _retval: *mut *const nsICacheEntry) -> ::nserror::nsresult {
        ((*self.vtable).Recreate)(self, aMemoryOnly, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the length of data this entry holds.
    ///    * @throws
    ///    *    NS_ERROR_IN_PROGRESS when the write is still in progress.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long dataSize;`
    #[inline]
    pub unsafe fn GetDataSize(&self, aDataSize: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetDataSize)(self, aDataSize)
    }


    /// ```text
    /// /**
    ///   * Returns the length of data this entry holds.
    ///   * @throws
    ///   *    - NS_ERROR_IN_PROGRESS when a write is still in progress (either real
        ///                               content or alt data).
    ///   *    - NS_ERROR_NOT_AVAILABLE if alt data does not exist.
    ///   */
    /// ```
    ///

    /// `readonly attribute long long altDataSize;`
    #[inline]
    pub unsafe fn GetAltDataSize(&self, aAltDataSize: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetAltDataSize)(self, aAltDataSize)
    }


    /// ```text
    /// /**
    ///   * Returns the type of the saved alt data.
    ///   * @throws
    ///   *    - NS_ERROR_NOT_AVAILABLE if alt data does not exist.
    ///   */
    /// ```
    ///

    /// `readonly attribute ACString altDataType;`
    #[inline]
    pub unsafe fn GetAltDataType(&self, aAltDataType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAltDataType)(self, aAltDataType)
    }


    /// ```text
    /// /**
    ///    * Opens and returns an output stream that a consumer may use to save an
    ///    * alternate representation of the data.
    ///    *
    ///    * @param type
    ///    *        type of the alternative data representation
    ///    * @param predictedSize
    ///    *        Predicted size of the data that will be written. It's used to decide
    ///    *        whether the resulting entry would exceed size limit, in which case
    ///    *        an error is thrown. If the size isn't known in advance, -1 should be
    ///    *        passed.
    ///    *
    ///    * @throws
    ///    *    - NS_ERROR_NOT_AVAILABLE if the real data hasn't been written.
    ///    *    - NS_ERROR_IN_PROGRESS when the writing regular content or alt-data to
    ///    *      the cache entry is still in progress.
    ///    *
    ///    * If there is alt-data already saved, it will be overwritten.
    ///    */
    /// ```
    ///

    /// `nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize);`
    #[inline]
    pub unsafe fn OpenAlternativeOutputStream(&self, type_: *const ::nsstring::nsACString, predictedSize: i64, _retval: *mut*const nsIAsyncOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenAlternativeOutputStream)(self, type_, predictedSize, _retval)
    }


    /// ```text
    /// /**
    ///    * Opens and returns an input stream that can be used to read the alternative
    ///    * representation previously saved in the cache.
    ///    * If this call is made while writing alt-data is still in progress, it is
    ///    * still possible to read content from the input stream as it's being written.
    ///    * @throws
    ///    *    - NS_ERROR_NOT_AVAILABLE if the alt-data representation doesn't exist at
    ///    *      all or if alt-data of the given type doesn't exist.
    ///    */
    /// ```
    ///

    /// `nsIInputStream openAlternativeInputStream (in ACString type);`
    #[inline]
    pub unsafe fn OpenAlternativeInputStream(&self, type_: *const ::nsstring::nsACString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenAlternativeInputStream)(self, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the nsILoadContextInfo of the cache entry
    ///    */
    /// ```
    ///

    /// `readonly attribute nsILoadContextInfo loadContextInfo;`
    #[inline]
    pub unsafe fn GetLoadContextInfo(&self, aLoadContextInfo: *mut*const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadContextInfo)(self, aLoadContextInfo)
    }


    /// ```text
    /// /****************************************************************************
    ///    * The following methods might be added to some nsICacheEntryInternal
    ///    * interface since we want to remove them as soon as the old cache backend is
    ///    * completely removed.
    ///    */
    /// /**
    ///    * @deprecated
    ///    * FOR BACKWARD COMPATIBILITY ONLY
    ///    * When the old cache backend is eventually removed, this method
    ///    * can be removed too.
    ///    *
    ///    * In the new backend: this method is no-op
    ///    * In the old backend: this method delegates to nsICacheEntryDescriptor.close()
    ///    */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///    * @deprecated
    ///    * FOR BACKWARD COMPATIBILITY ONLY
    ///    * Marks the entry as valid so that others can use it and get only readonly
    ///    * access when the entry is held by the 1st writer.
    ///    */
    /// ```
    ///

    /// `void markValid ();`
    #[inline]
    pub unsafe fn MarkValid(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MarkValid)(self, )
    }


    /// ```text
    /// /**
    ///    * @deprecated
    ///    * FOR BACKWARD COMPATIBILITY ONLY
    ///    * Marks the entry as valid when write access is acquired.
    ///    */
    /// ```
    ///

    /// `void maybeMarkValid ();`
    #[inline]
    pub unsafe fn MaybeMarkValid(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MaybeMarkValid)(self, )
    }


    /// ```text
    /// /**
    ///    * @deprecated
    ///    * FOR BACKWARD COMPATIBILITY ONLY / KINDA HACK
    ///    * @param aWriteAllowed
    ///    *    Consumer indicates whether write to the entry is allowed for it.
    ///    *    Depends on implementation how the flag is handled.
    ///    * @returns
    ///    *    true when write access is acquired for this entry,
    ///    *    false otherwise
    ///    */
    /// ```
    ///

    /// `boolean hasWriteAccess (in boolean aWriteAllowed);`
    #[inline]
    pub unsafe fn HasWriteAccess(&self, aWriteAllowed: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasWriteAccess)(self, aWriteAllowed, _retval)
    }


}


/// `interface nsICacheEntryMetaDataVisitor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheEntryMetaDataVisitor {
    vtable: *const nsICacheEntryMetaDataVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheEntryMetaDataVisitor.
unsafe impl XpCom for nsICacheEntryMetaDataVisitor {
    const IID: nsIID = nsID(0xfea3e276, 0x6ba5, 0x4ceb,
        [0xa5, 0x81, 0x80, 0x7d, 0x1f, 0x43, 0xf6, 0xd0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheEntryMetaDataVisitor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheEntryMetaDataVisitor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheEntryMetaDataVisitorCoerce {
    /// Cheaply cast a value of this type from a `nsICacheEntryMetaDataVisitor`.
    fn coerce_from(v: &nsICacheEntryMetaDataVisitor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheEntryMetaDataVisitorCoerce for nsICacheEntryMetaDataVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheEntryMetaDataVisitor) -> &Self {
        v
    }
}

impl nsICacheEntryMetaDataVisitor {
    /// Cast this `nsICacheEntryMetaDataVisitor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheEntryMetaDataVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheEntryMetaDataVisitor {
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
impl<T: nsISupportsCoerce> nsICacheEntryMetaDataVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryMetaDataVisitor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheEntryMetaDataVisitor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheEntryMetaDataVisitorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onMetaDataElement (in string key, in string value); */
    pub OnMetaDataElement: unsafe extern "system" fn (this: *const nsICacheEntryMetaDataVisitor, key: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheEntryMetaDataVisitor {

    /// ```text
    /// /**
    ///  * Argument for nsICacheEntry.visitMetaData, provides access to all metadata
    ///  * keys and values stored on the entry.
    ///  */
    /// /**
    ///    * Called over each key / value pair.
    ///    */
    /// ```
    ///

    /// `void onMetaDataElement (in string key, in string value);`
    #[inline]
    pub unsafe fn OnMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).OnMetaDataElement)(self, key, value)
    }


}


