//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/startupcache/nsIStartupCacheInfo.idl
//


/// `interface nsIStartupCacheInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStartupCacheInfo {
    vtable: *const nsIStartupCacheInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStartupCacheInfo.
unsafe impl XpCom for nsIStartupCacheInfo {
    const IID: nsIID = nsID(0xa6b2f8b0, 0x7438, 0x11ea,
        [0xbc, 0x55, 0x02, 0x42, 0xac, 0x13, 0x00, 0x03]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStartupCacheInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStartupCacheInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStartupCacheInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIStartupCacheInfo`.
    fn coerce_from(v: &nsIStartupCacheInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStartupCacheInfoCoerce for nsIStartupCacheInfo {
    #[inline]
    fn coerce_from(v: &nsIStartupCacheInfo) -> &Self {
        v
    }
}

impl nsIStartupCacheInfo {
    /// Cast this `nsIStartupCacheInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStartupCacheInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStartupCacheInfo {
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
impl<T: nsISupportsCoerce> nsIStartupCacheInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStartupCacheInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStartupCacheInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStartupCacheInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean IgnoreDiskCache; */
    pub GetIgnoreDiskCache: unsafe extern "system" fn (this: *const nsIStartupCacheInfo, aIgnoreDiskCache: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean FoundDiskCacheOnInit; */
    pub GetFoundDiskCacheOnInit: unsafe extern "system" fn (this: *const nsIStartupCacheInfo, aFoundDiskCacheOnInit: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean WroteToDiskCache; */
    pub GetWroteToDiskCache: unsafe extern "system" fn (this: *const nsIStartupCacheInfo, aWroteToDiskCache: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString DiskCachePath; */
    pub GetDiskCachePath: unsafe extern "system" fn (this: *const nsIStartupCacheInfo, aDiskCachePath: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStartupCacheInfo {

    /// ```text
    /// /**
    ///    * Returns true if the startup cache will not load from the cache from disk.
    ///    * This can happen if the cache file is corrupt or has been invalidated.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean IgnoreDiskCache;`
    #[inline]
    pub unsafe fn GetIgnoreDiskCache(&self, aIgnoreDiskCache: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIgnoreDiskCache)(self, aIgnoreDiskCache)
    }


    /// ```text
    /// /**
    ///    * Returns true if during initialization of the startup cache an existing
    ///    * cache file was found on disk. This does NOT indicate if the file loaded
    ///    * successfully.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean FoundDiskCacheOnInit;`
    #[inline]
    pub unsafe fn GetFoundDiskCacheOnInit(&self, aFoundDiskCacheOnInit: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFoundDiskCacheOnInit)(self, aFoundDiskCacheOnInit)
    }


    /// ```text
    /// /**
    ///    * Returns true once the current cache file as been written to disk at least
    ///    * once. If the cache was loaded from disk and never changed this may never
    ///    * be set to true.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean WroteToDiskCache;`
    #[inline]
    pub unsafe fn GetWroteToDiskCache(&self, aWroteToDiskCache: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWroteToDiskCache)(self, aWroteToDiskCache)
    }


    /// ```text
    /// /**
    ///    * The full path and filename of the startup cache file that will be stored on
    ///    * disk.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString DiskCachePath;`
    #[inline]
    pub unsafe fn GetDiskCachePath(&self, aDiskCachePath: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDiskCachePath)(self, aDiskCachePath)
    }


}


