//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheService.idl
//


/// `interface nsICacheService : nsISupports`
///

/// ```text
/// /**
///  * @deprecated
///  *
///  * IMPORTANT NOTE: THIS INTERFACE IS NO LONGER SUPPORTED AND PLANNED TO BE
///  * REMOVED SOON. WE STRONGLY ENCORAGE TO MIGRATE THE EXISTING CODE AND FOR
///  * THE NEW CODE USE ONLY THE NEW HTTP CACHE API IN netwerk/cache2/.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheService {
    vtable: *const nsICacheServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheService.
unsafe impl XpCom for nsICacheService {
    const IID: nsIID = nsID(0x14dbe1e9, 0xf3bc, 0x45af,
        [0x92, 0xf4, 0x2c, 0x57, 0x4f, 0xcd, 0x4e, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheServiceCoerce {
    /// Cheaply cast a value of this type from a `nsICacheService`.
    fn coerce_from(v: &nsICacheService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheServiceCoerce for nsICacheService {
    #[inline]
    fn coerce_from(v: &nsICacheService) -> &Self {
        v
    }
}

impl nsICacheService {
    /// Cast this `nsICacheService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheService {
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
impl<T: nsISupportsCoerce> nsICacheServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased); */
    pub CreateSession: unsafe extern "system" fn (this: *const nsICacheService, clientID: *const libc::c_char, storagePolicy: nsCacheStoragePolicy, streamBased: bool, _retval: *mut*const nsICacheSession) -> ::nserror::nsresult,

    /* void visitEntries (in nsICacheVisitor visitor); */
    pub VisitEntries: unsafe extern "system" fn (this: *const nsICacheService, visitor: *const nsICacheVisitor) -> ::nserror::nsresult,

    /* void evictEntries (in nsCacheStoragePolicy storagePolicy); */
    pub EvictEntries: unsafe extern "system" fn (this: *const nsICacheService, storagePolicy: nsCacheStoragePolicy) -> ::nserror::nsresult,

    /* readonly attribute nsIEventTarget cacheIOTarget; */
    pub GetCacheIOTarget: unsafe extern "system" fn (this: *const nsICacheService, aCacheIOTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheService {

    /// ```text
    /// /**
    ///      * @throws NS_ERROR_NOT_IMPLEMENTED when the cache v2 is prefered to use.
    ///      *
    ///      * Create a cache session
    ///      *
    ///      * A cache session represents a client's access into the cache.  The cache
    ///      * session is not "owned" by the cache service.  Hence, it is possible to
    ///      * create duplicate cache sessions.  Entries created by a cache session
    ///      * are invisible to other cache sessions, unless the cache sessions are
    ///      * equivalent.
    ///      *
    ///      * @param clientID - Specifies the name of the client using the cache.
    ///      * @param storagePolicy - Limits the storage policy for all entries
    ///      *   accessed via the returned session.  As a result, devices excluded
    ///      *   by the storage policy will not be searched when opening entries
    ///      *   from the returned session.
    ///      * @param streamBased - Indicates whether or not the data being cached
    ///      *   can be represented as a stream.  The storagePolicy must be
    ///      *   consistent with the value of this field.  For example, a non-stream-
    ///      *   based cache entry can only have a storage policy of STORE_IN_MEMORY.
    ///      * @return new cache session.
    ///      */
    /// ```
    ///

    /// `nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased);`
    #[inline]
    pub unsafe fn CreateSession(&self, clientID: *const libc::c_char, storagePolicy: nsCacheStoragePolicy, streamBased: bool, _retval: *mut*const nsICacheSession) -> ::nserror::nsresult {
        ((*self.vtable).CreateSession)(self, clientID, storagePolicy, streamBased, _retval)
    }


    /// ```text
    /// /**
    ///      * @throws NS_ERROR_NOT_IMPLEMENTED when the cache v2 is prefered to use.
    ///      *
    ///      * Visit entries stored in the cache.  Used to implement about:cache.
    ///      */
    /// ```
    ///

    /// `void visitEntries (in nsICacheVisitor visitor);`
    #[inline]
    pub unsafe fn VisitEntries(&self, visitor: *const nsICacheVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitEntries)(self, visitor)
    }


    /// ```text
    /// /**
    ///      * @throws NS_ERROR_NOT_IMPLEMENTED when the cache v2 is prefered to use.
    ///      *
    ///      * Evicts all entries in all devices implied by the storage policy.
    ///      *
    ///      * @note This function may evict some items but will throw if it fails to evict
    ///      *       everything.
    ///      */
    /// ```
    ///

    /// `void evictEntries (in nsCacheStoragePolicy storagePolicy);`
    #[inline]
    pub unsafe fn EvictEntries(&self, storagePolicy: nsCacheStoragePolicy) -> ::nserror::nsresult {
        ((*self.vtable).EvictEntries)(self, storagePolicy)
    }


    /// ```text
    /// /**
    ///      * Event target which is used for I/O operations
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIEventTarget cacheIOTarget;`
    #[inline]
    pub unsafe fn GetCacheIOTarget(&self, aCacheIOTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheIOTarget)(self, aCacheIOTarget)
    }


}


/// `interface nsICacheServiceInternal : nsICacheService`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheServiceInternal {
    vtable: *const nsICacheServiceInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheServiceInternal.
unsafe impl XpCom for nsICacheServiceInternal {
    const IID: nsIID = nsID(0xd0fc8d38, 0xdb80, 0x4928,
        [0xbf, 0x1c, 0xb0, 0x08, 0x5d, 0xdf, 0xa9, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheServiceInternal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheServiceInternal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheServiceInternalCoerce {
    /// Cheaply cast a value of this type from a `nsICacheServiceInternal`.
    fn coerce_from(v: &nsICacheServiceInternal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheServiceInternalCoerce for nsICacheServiceInternal {
    #[inline]
    fn coerce_from(v: &nsICacheServiceInternal) -> &Self {
        v
    }
}

impl nsICacheServiceInternal {
    /// Cast this `nsICacheServiceInternal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheServiceInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheServiceInternal {
    type Target = nsICacheService;
    #[inline]
    fn deref(&self) -> &nsICacheService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICacheServiceCoerce> nsICacheServiceInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheServiceInternal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheServiceInternal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheServiceInternalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICacheServiceVTable,

    /* readonly attribute double lockHeldTime; */
    pub GetLockHeldTime: unsafe extern "system" fn (this: *const nsICacheServiceInternal, aLockHeldTime: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheServiceInternal {

    /// ```text
    /// /**
    ///      * This is an internal interface. It changes so frequently that it probably
    ///      * went away while you were reading this.
    ///      */
    /// /**
    ///      * Milliseconds for which the service lock has been held. 0 if unlocked.
    ///      */
    /// ```
    ///

    /// `readonly attribute double lockHeldTime;`
    #[inline]
    pub unsafe fn GetLockHeldTime(&self, aLockHeldTime: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetLockHeldTime)(self, aLockHeldTime)
    }


}


