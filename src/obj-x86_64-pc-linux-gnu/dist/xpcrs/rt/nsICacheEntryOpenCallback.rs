//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntryOpenCallback.idl
//


/// `interface nsICacheEntryOpenCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheEntryOpenCallback {
    vtable: *const nsICacheEntryOpenCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheEntryOpenCallback.
unsafe impl XpCom for nsICacheEntryOpenCallback {
    const IID: nsIID = nsID(0x1fc9fe11, 0xc6ac, 0x4748,
        [0x94, 0xbd, 0x85, 0x55, 0xa5, 0xa1, 0x2b, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheEntryOpenCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheEntryOpenCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheEntryOpenCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsICacheEntryOpenCallback`.
    fn coerce_from(v: &nsICacheEntryOpenCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheEntryOpenCallbackCoerce for nsICacheEntryOpenCallback {
    #[inline]
    fn coerce_from(v: &nsICacheEntryOpenCallback) -> &Self {
        v
    }
}

impl nsICacheEntryOpenCallback {
    /// Cast this `nsICacheEntryOpenCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheEntryOpenCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheEntryOpenCallback {
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
impl<T: nsISupportsCoerce> nsICacheEntryOpenCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryOpenCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheEntryOpenCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheEntryOpenCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* unsigned long onCacheEntryCheck (in nsICacheEntry aEntry, in nsIApplicationCache aApplicationCache); */
    pub OnCacheEntryCheck: unsafe extern "system" fn (this: *const nsICacheEntryOpenCallback, aEntry: *const nsICacheEntry, aApplicationCache: *const nsIApplicationCache, _retval: *mut u32) -> ::nserror::nsresult,

    /* void onCacheEntryAvailable (in nsICacheEntry aEntry, in boolean aNew, in nsIApplicationCache aApplicationCache, in nsresult aResult); */
    pub OnCacheEntryAvailable: unsafe extern "system" fn (this: *const nsICacheEntryOpenCallback, aEntry: *const nsICacheEntry, aNew: bool, aApplicationCache: *const nsIApplicationCache, aResult: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheEntryOpenCallback {
    /// ```text
    /// /**
    ///    * State of the entry determined by onCacheEntryCheck.
    ///    *
    ///    * ENTRY_WANTED - the consumer is interested in the entry, we will pass it.
    ///    * RECHECK_AFTER_WRITE_FINISHED - the consumer cannot use the entry while data is
    ///    *    still being written and wants to check it again after the current write is
    ///    *    finished. This actually prevents concurrent read/write and is used with
    ///    *    non-resumable HTTP responses.
    ///    * ENTRY_NEEDS_REVALIDATION - entry needs to be revalidated first with origin server,
    ///    *    this means the loading channel will decide whether to use the entry content
    ///    *    as is after it gets a positive response from the server about validity of the
    ///    *    content ; when a new content needs to be loaded from the server, the loading
    ///    *    channel opens a new entry with OPEN_TRUNCATE flag which dooms the one
    ///    *    this check has been made for.
    ///    * ENTRY_NOT_WANTED - the consumer is not interested in the entry, we will not pass it.
    ///    */
    /// ```
    ///

    pub const ENTRY_WANTED: i64 = 0;


    pub const RECHECK_AFTER_WRITE_FINISHED: i64 = 1;


    pub const ENTRY_NEEDS_REVALIDATION: i64 = 2;


    pub const ENTRY_NOT_WANTED: i64 = 3;

    /// ```text
    /// /**
    ///    * Callback to perform any validity checks before the entry should be used.
    ///    * Called before onCacheEntryAvailable callback, depending on the result it
    ///    * may be called more then one time.
    ///    *
    ///    * This callback is ensured to be called on the same thread on which asyncOpenURI
    ///    * has been called, unless nsICacheStorage.CHECK_MULTITHREADED flag has been specified.
    ///    * In that case this callback can be invoked on any thread, usually it is the cache I/O
    ///    * or cache management thread.
    ///    *
    ///    * IMPORTANT NOTE:
    ///    * This callback may be invoked sooner then respective asyncOpenURI call exits.
    ///    *
    ///    * @param aEntry
    ///    *    An entry to examine.  Consumer has a chance to decide whether the
    ///    *    entry is valid or not.
    ///    * @param aApplicationCache
    ///    *    Optional, application cache the entry has been found in, if any.
    ///    * @return
    ///    *    State of the entry, see the constants just above.
    ///    */
    /// ```
    ///

    /// `unsigned long onCacheEntryCheck (in nsICacheEntry aEntry, in nsIApplicationCache aApplicationCache);`
    #[inline]
    pub unsafe fn OnCacheEntryCheck(&self, aEntry: *const nsICacheEntry, aApplicationCache: *const nsIApplicationCache, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheEntryCheck)(self, aEntry, aApplicationCache, _retval)
    }


    /// ```text
    /// /**
    ///    * Callback giving actual result of asyncOpenURI.  It may give consumer the cache
    ///    * entry or a failure result when it's not possible to open it from some reason.
    ///    * This callback is ensured to be called on the same thread on which asyncOpenURI
    ///    * has been called.
    ///    *
    ///    * IMPORTANT NOTE:
    ///    * This callback may be invoked sooner then respective asyncOpenURI call exits.
    ///    *
    ///    * @param aEntry
    ///    *    The entry bound to the originally requested URI.  May be null when
    ///    *    loading from a particular application cache and the URI has not
    ///    *    been found in that application cache.
    ///    * @param aNew
    ///    *    Whether no data so far has been stored for this entry, i.e. reading
    ///    *    it will just fail.  When aNew is true, a server request should be
    ///    *    made and data stored to this new entry.
    ///    * @param aApplicationCache
    ///    *    When an entry had been found in an application cache, this is the
    ///    *    given application cache.  It should be associated with the loading
    ///    *    channel.
    ///    * @param aResult
    ///    *    Result of the request.  This may be a failure only when one of these
    ///    *    issues occur:
    ///    *    - the cache storage service could not be started due to some unexpected
    ///    *      faulure
    ///    *    - there is not enough disk space to create new entries
    ///    *    - cache entry was not found in a given application cache
    ///    */
    /// ```
    ///

    /// `void onCacheEntryAvailable (in nsICacheEntry aEntry, in boolean aNew, in nsIApplicationCache aApplicationCache, in nsresult aResult);`
    #[inline]
    pub unsafe fn OnCacheEntryAvailable(&self, aEntry: *const nsICacheEntry, aNew: bool, aApplicationCache: *const nsIApplicationCache, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheEntryAvailable)(self, aEntry, aNew, aApplicationCache, aResult)
    }


}


