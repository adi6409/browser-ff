//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICachingChannel.idl
//


/// `interface nsICachingChannel : nsICacheInfoChannel`
///

/// ```text
/// /**
///  * A channel may optionally implement this interface to allow clients
///  * to affect its behavior with respect to how it uses the cache service.
///  *
///  * This interface provides:
///  *   1) Support for "stream as file" semantics (for JAR and plugins).
///  *   2) Support for "pinning" cached data in the cache (for printing and save-as).
///  *   3) Support for uniquely identifying cached data in cases when the URL
///  *      is insufficient (e.g., HTTP form submission).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICachingChannel {
vtable: *const nsICachingChannelVTable,

/// This field is a phantomdata to ensure that the VTable type and any
/// struct containing it is not safe to send across threads, as XPCOM is
/// generally not threadsafe.
///
/// XPCOM interfaces in general are not safe to send across threads.
__nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICachingChannel.
unsafe impl XpCom for nsICachingChannel {
const IID: nsIID = nsID(0xdd1d6122, 0x5ecf, 0x4fe4,
[0x8f, 0x0f, 0x99, 0x5e, 0x7a, 0xb3, 0x12, 0x1a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICachingChannel {
#[inline]
unsafe fn addref(&self) {
self.AddRef();
}
#[inline]
unsafe fn release(&self) {
self.Release();
}
}

// This trait is implemented on all types which can be coerced to from nsICachingChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICachingChannelCoerce {
/// Cheaply cast a value of this type from a `nsICachingChannel`.
fn coerce_from(v: &nsICachingChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICachingChannelCoerce for nsICachingChannel {
#[inline]
fn coerce_from(v: &nsICachingChannel) -> &Self {
v
}
}

impl nsICachingChannel {
/// Cast this `nsICachingChannel` to one of its base interfaces.
#[inline]
pub fn coerce<T: nsICachingChannelCoerce>(&self) -> &T {
T::coerce_from(self)
}
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICachingChannel {
type Target = nsICacheInfoChannel;
#[inline]
fn deref(&self) -> &nsICacheInfoChannel {
unsafe {
::std::mem::transmute(self)
}
}
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICacheInfoChannelCoerce> nsICachingChannelCoerce for T {
#[inline]
fn coerce_from(v: &nsICachingChannel) -> &Self {
T::coerce_from(v)
}
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICachingChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICachingChannelVTable {
/// We need to include the members from the base interface's vtable at the start
/// of the VTable definition.
pub __base: nsICacheInfoChannelVTable,

/* attribute nsISupports cacheToken; */
pub GetCacheToken: unsafe extern "system" fn (this: *const nsICachingChannel, aCacheToken: *mut *const nsISupports) -> ::nserror::nsresult,

/* attribute nsISupports cacheToken; */
pub SetCacheToken: unsafe extern "system" fn (this: *const nsICachingChannel, aCacheToken: *const nsISupports) -> ::nserror::nsresult,

/* attribute nsISupports offlineCacheToken; */
pub GetOfflineCacheToken: unsafe extern "system" fn (this: *const nsICachingChannel, aOfflineCacheToken: *mut *const nsISupports) -> ::nserror::nsresult,

/* attribute nsISupports offlineCacheToken; */
pub SetOfflineCacheToken: unsafe extern "system" fn (this: *const nsICachingChannel, aOfflineCacheToken: *const nsISupports) -> ::nserror::nsresult,

/* attribute boolean cacheOnlyMetadata; */
pub GetCacheOnlyMetadata: unsafe extern "system" fn (this: *const nsICachingChannel, aCacheOnlyMetadata: *mut bool) -> ::nserror::nsresult,

/* attribute boolean cacheOnlyMetadata; */
pub SetCacheOnlyMetadata: unsafe extern "system" fn (this: *const nsICachingChannel, aCacheOnlyMetadata: bool) -> ::nserror::nsresult,

/* attribute boolean pin; */
pub GetPin: unsafe extern "system" fn (this: *const nsICachingChannel, aPin: *mut bool) -> ::nserror::nsresult,

/* attribute boolean pin; */
pub SetPin: unsafe extern "system" fn (this: *const nsICachingChannel, aPin: bool) -> ::nserror::nsresult,

/* void forceCacheEntryValidFor (in unsigned long aSecondsToTheFuture); */
pub ForceCacheEntryValidFor: unsafe extern "system" fn (this: *const nsICachingChannel, aSecondsToTheFuture: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICachingChannel {
/// ```text
/// /**************************************************************************
///      * Caching channel specific load flags:
///      */
/// /**
///      * This load flag inhibits fetching from the net.  An error of
///      * NS_ERROR_DOCUMENT_NOT_CACHED will be sent to the listener's
///      * onStopRequest if network IO is necessary to complete the request.
///      *
///      * This flag can be used to find out whether fetching this URL would
///      * cause validation of the cache entry via the network.
///      *
///      * Combining this flag with LOAD_BYPASS_LOCAL_CACHE will cause all
///      * loads to fail.
///      */
/// ```
///

pub const LOAD_NO_NETWORK_IO: i64 = 67108864;

/// ```text
/// /**
///      * This load flag causes the offline cache to be checked when fetching
///      * a request.  It will be set automatically if the browser is offline.
///      *
///      * This flag will not be transferred through a redirect.
///      */
/// ```
///

pub const LOAD_CHECK_OFFLINE_CACHE: i64 = 134217728;

/// ```text
/// /**
///      * This load flag causes the local cache to be skipped when fetching a
///      * request.  Unlike LOAD_BYPASS_CACHE, it does not force an end-to-end load
///      * (i.e., it does not affect proxy caches).
///      */
/// ```
///

pub const LOAD_BYPASS_LOCAL_CACHE: i64 = 268435456;

/// ```text
/// /**
///      * This load flag causes the local cache to be skipped if the request
///      * would otherwise block waiting to access the cache.
///      */
/// ```
///

pub const LOAD_BYPASS_LOCAL_CACHE_IF_BUSY: i64 = 536870912;

/// ```text
/// /**
///      * This load flag inhibits fetching from the net if the data in the cache
///      * has been evicted.  An error of NS_ERROR_DOCUMENT_NOT_CACHED will be sent
///      * to the listener's onStopRequest in this case.  This flag is set
///      * automatically when the application is offline.
///      */
/// ```
///

pub const LOAD_ONLY_FROM_CACHE: i64 = 1073741824;

/// ```text
/// /**
///      * This load flag controls what happens when a document would be loaded
///      * from the cache to satisfy a call to AsyncOpen.  If this attribute is
///      * set to TRUE, then the document will not be loaded from the cache.  A
///      * stream listener can check nsICachingChannel::isFromCache to determine
///      * if the AsyncOpen will actually result in data being streamed.
///      *
///      * If this flag has been set, and the request can be satisfied via the
///      * cache, then the OnDataAvailable events will be skipped.  The listener
///      * will only see OnStartRequest followed by OnStopRequest.
///      */
/// ```
///

pub const LOAD_ONLY_IF_MODIFIED: i64 = 2147483648;

/// ```text
/// /**
///      * Set/get the cache token... uniquely identifies the data in the cache.
///      * Holding a reference to this token prevents the cached data from being
///      * removed.
///      *
///      * A cache token retrieved from a particular instance of nsICachingChannel
///      * could be set on another instance of nsICachingChannel provided the
///      * underlying implementations are compatible.  The implementation of
///      * nsICachingChannel would be expected to only read from the cache entry
///      * identified by the cache token and not try to validate it.
///      *
///      * The cache token can be QI'd to a nsICacheEntryInfo if more detail
///      * about the cache entry is needed (e.g., expiration time).
///      */
/// ```
///

/// `attribute nsISupports cacheToken;`
#[inline]
pub unsafe fn GetCacheToken(&self, aCacheToken: *mut *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).GetCacheToken)(self, aCacheToken)
}


/// ```text
/// /**
///      * Set/get the cache token... uniquely identifies the data in the cache.
///      * Holding a reference to this token prevents the cached data from being
///      * removed.
///      *
///      * A cache token retrieved from a particular instance of nsICachingChannel
///      * could be set on another instance of nsICachingChannel provided the
///      * underlying implementations are compatible.  The implementation of
///      * nsICachingChannel would be expected to only read from the cache entry
///      * identified by the cache token and not try to validate it.
///      *
///      * The cache token can be QI'd to a nsICacheEntryInfo if more detail
///      * about the cache entry is needed (e.g., expiration time).
///      */
/// ```
///

/// `attribute nsISupports cacheToken;`
#[inline]
pub unsafe fn SetCacheToken(&self, aCacheToken: *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).SetCacheToken)(self, aCacheToken)
}


/// ```text
/// /**
///      * The same as above but accessing the offline app cache token if there
///      * is any.
///      *
///      * @throws
///      *      NS_ERROR_NOT_AVAILABLE when there is not offline cache token
///      */
/// ```
///

/// `attribute nsISupports offlineCacheToken;`
#[inline]
pub unsafe fn GetOfflineCacheToken(&self, aOfflineCacheToken: *mut *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).GetOfflineCacheToken)(self, aOfflineCacheToken)
}


/// ```text
/// /**
///      * The same as above but accessing the offline app cache token if there
///      * is any.
///      *
///      * @throws
///      *      NS_ERROR_NOT_AVAILABLE when there is not offline cache token
///      */
/// ```
///

/// `attribute nsISupports offlineCacheToken;`
#[inline]
pub unsafe fn SetOfflineCacheToken(&self, aOfflineCacheToken: *const nsISupports) -> ::nserror::nsresult {
((*self.vtable).SetOfflineCacheToken)(self, aOfflineCacheToken)
}


/// ```text
/// /**
///      * Instructs the channel to only store the metadata of the entry, and not
///      * the content. When reading an existing entry, this automatically sets
///      * LOAD_ONLY_IF_MODIFIED flag.
///      * Must be called before asyncOpen().
///      */
/// ```
///

/// `attribute boolean cacheOnlyMetadata;`
#[inline]
pub unsafe fn GetCacheOnlyMetadata(&self, aCacheOnlyMetadata: *mut bool) -> ::nserror::nsresult {
((*self.vtable).GetCacheOnlyMetadata)(self, aCacheOnlyMetadata)
}


/// ```text
/// /**
///      * Instructs the channel to only store the metadata of the entry, and not
///      * the content. When reading an existing entry, this automatically sets
///      * LOAD_ONLY_IF_MODIFIED flag.
///      * Must be called before asyncOpen().
///      */
/// ```
///

/// `attribute boolean cacheOnlyMetadata;`
#[inline]
pub unsafe fn SetCacheOnlyMetadata(&self, aCacheOnlyMetadata: bool) -> ::nserror::nsresult {
((*self.vtable).SetCacheOnlyMetadata)(self, aCacheOnlyMetadata)
}


/// ```text
/// /**
///      * Tells the channel to use the pinning storage.
///      */
/// ```
///

/// `attribute boolean pin;`
#[inline]
pub unsafe fn GetPin(&self, aPin: *mut bool) -> ::nserror::nsresult {
((*self.vtable).GetPin)(self, aPin)
}


/// ```text
/// /**
///      * Tells the channel to use the pinning storage.
///      */
/// ```
///

/// `attribute boolean pin;`
#[inline]
pub unsafe fn SetPin(&self, aPin: bool) -> ::nserror::nsresult {
((*self.vtable).SetPin)(self, aPin)
}


/// ```text
/// /**
///      * Overrides cache validation for a time specified in seconds.
///      *
///      * @param aSecondsToTheFuture
///      *
///      */
/// ```
///

/// `void forceCacheEntryValidFor (in unsigned long aSecondsToTheFuture);`
#[inline]
pub unsafe fn ForceCacheEntryValidFor(&self, aSecondsToTheFuture: u32) -> ::nserror::nsresult {
((*self.vtable).ForceCacheEntryValidFor)(self, aSecondsToTheFuture)
}


}


