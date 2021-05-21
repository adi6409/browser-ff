//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheEntryDescriptor.idl
//


/// `interface nsICacheEntryDescriptor : nsICacheEntryInfo`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheEntryDescriptor {
    vtable: *const nsICacheEntryDescriptorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheEntryDescriptor.
unsafe impl XpCom for nsICacheEntryDescriptor {
    const IID: nsIID = nsID(0x90b17d31, 0x46aa, 0x4fb1,
        [0xa2, 0x06, 0x47, 0x3c, 0x96, 0x6c, 0xbc, 0x18]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheEntryDescriptor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheEntryDescriptor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheEntryDescriptorCoerce {
    /// Cheaply cast a value of this type from a `nsICacheEntryDescriptor`.
    fn coerce_from(v: &nsICacheEntryDescriptor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheEntryDescriptorCoerce for nsICacheEntryDescriptor {
    #[inline]
    fn coerce_from(v: &nsICacheEntryDescriptor) -> &Self {
        v
    }
}

impl nsICacheEntryDescriptor {
    /// Cast this `nsICacheEntryDescriptor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheEntryDescriptorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheEntryDescriptor {
    type Target = nsICacheEntryInfo;
    #[inline]
    fn deref(&self) -> &nsICacheEntryInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICacheEntryInfoCoerce> nsICacheEntryDescriptorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryDescriptor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheEntryDescriptor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheEntryDescriptorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICacheEntryInfoVTable,

    /* void setExpirationTime (in uint32_t expirationTime); */
    pub SetExpirationTime: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, expirationTime: uint32_t) -> ::nserror::nsresult,

    /* void setDataSize (in unsigned long size); */
    pub SetDataSize: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, size: u32) -> ::nserror::nsresult,

    /* nsIInputStream openInputStream (in unsigned long offset); */
    pub OpenInputStream: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, offset: u32, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* nsIOutputStream openOutputStream (in unsigned long offset); */
    pub OpenOutputStream: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, offset: u32, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* attribute nsISupports cacheElement; */
    pub GetCacheElement: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aCacheElement: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsISupports cacheElement; */
    pub SetCacheElement: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aCacheElement: *const nsISupports) -> ::nserror::nsresult,

    /* attribute int64_t predictedDataSize; */
    pub GetPredictedDataSize: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aPredictedDataSize: *mut int64_t) -> ::nserror::nsresult,

    /* attribute int64_t predictedDataSize; */
    pub SetPredictedDataSize: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aPredictedDataSize: int64_t) -> ::nserror::nsresult,

    /* readonly attribute nsCacheAccessMode accessGranted; */
    pub GetAccessGranted: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aAccessGranted: *mut nsCacheAccessMode) -> ::nserror::nsresult,

    /* attribute nsCacheStoragePolicy storagePolicy; */
    pub GetStoragePolicy: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aStoragePolicy: *mut nsCacheStoragePolicy) -> ::nserror::nsresult,

    /* attribute nsCacheStoragePolicy storagePolicy; */
    pub SetStoragePolicy: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aStoragePolicy: nsCacheStoragePolicy) -> ::nserror::nsresult,

    /* readonly attribute nsIFile file; */
    pub GetFile: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsISupports securityInfo; */
    pub GetSecurityInfo: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsISupports securityInfo; */
    pub SetSecurityInfo: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aSecurityInfo: *const nsISupports) -> ::nserror::nsresult,

    /* readonly attribute unsigned long storageDataSize; */
    pub GetStorageDataSize: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, aStorageDataSize: *mut u32) -> ::nserror::nsresult,

    /* void doom (); */
    pub Doom: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor) -> ::nserror::nsresult,

    /* void doomAndFailPendingRequests (in nsresult status); */
    pub DoomAndFailPendingRequests: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, status: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void asyncDoom (in nsICacheListener listener); */
    pub AsyncDoom: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, listener: *const nsICacheListener) -> ::nserror::nsresult,

    /* void markValid (); */
    pub MarkValid: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor) -> ::nserror::nsresult,

    /* string getMetaDataElement (in string key); */
    pub GetMetaDataElement: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, key: *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* void setMetaDataElement (in string key, in string value); */
    pub SetMetaDataElement: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, key: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult,

    /* void visitMetaData (in nsICacheMetaDataVisitor visitor); */
    pub VisitMetaData: unsafe extern "system" fn (this: *const nsICacheEntryDescriptor, visitor: *const nsICacheMetaDataVisitor) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheEntryDescriptor {

    /// ```text
    /// /**
    ///      * Set the time at which the cache entry should be considered invalid (in
        ///      * seconds since the Epoch).
    ///      */
    /// ```
    ///

    /// `void setExpirationTime (in uint32_t expirationTime);`
    #[inline]
    pub unsafe fn SetExpirationTime(&self, expirationTime: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetExpirationTime)(self, expirationTime)
    }


    /// ```text
    /// /**
    ///      * Set the cache entry data size.  This will fail if the cache entry
    ///      * IS stream based.
    ///      */
    /// ```
    ///

    /// `void setDataSize (in unsigned long size);`
    #[inline]
    pub unsafe fn SetDataSize(&self, size: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetDataSize)(self, size)
    }


    /// ```text
    /// /**
    ///      * Open blocking input stream to cache data.  This will fail if the cache
    ///      * entry IS NOT stream based.  Use the stream transport service to
    ///      * asynchronously read this stream on a background thread.  The returned
    ///      * stream MAY implement nsISeekableStream.
    ///      *
    ///      * @param offset
    ///      *        read starting from this offset into the cached data.  an offset
    ///      *        beyond the end of the stream has undefined consequences.
    ///      *
    ///      * @return blocking, unbuffered input stream.
    ///      */
    /// ```
    ///

    /// `nsIInputStream openInputStream (in unsigned long offset);`
    #[inline]
    pub unsafe fn OpenInputStream(&self, offset: u32, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenInputStream)(self, offset, _retval)
    }


    /// ```text
    /// /**
    ///      * Open blocking output stream to cache data.  This will fail if the cache
    ///      * entry IS NOT stream based.  Use the stream transport service to
    ///      * asynchronously write to this stream on a background thread.  The returned
    ///      * stream MAY implement nsISeekableStream.
    ///      *
    ///      * If opening an output stream to existing cached data, the data will be
    ///      * truncated to the specified offset.
    ///      *
    ///      * @param offset
    ///      *        write starting from this offset into the cached data.  an offset
    ///      *        beyond the end of the stream has undefined consequences.
    ///      *
    ///      * @return blocking, unbuffered output stream.
    ///      */
    /// ```
    ///

    /// `nsIOutputStream openOutputStream (in unsigned long offset);`
    #[inline]
    pub unsafe fn OpenOutputStream(&self, offset: u32, _retval: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenOutputStream)(self, offset, _retval)
    }


    /// ```text
    /// /**
    ///      * Get/set the cache data element.  This will fail if the cache entry
    ///      * IS stream based.  The cache entry holds a strong reference to this
    ///      * object.  The object will be released when the cache entry is destroyed.
    ///      */
    /// ```
    ///

    /// `attribute nsISupports cacheElement;`
    #[inline]
    pub unsafe fn GetCacheElement(&self, aCacheElement: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheElement)(self, aCacheElement)
    }


    /// ```text
    /// /**
    ///      * Get/set the cache data element.  This will fail if the cache entry
    ///      * IS stream based.  The cache entry holds a strong reference to this
    ///      * object.  The object will be released when the cache entry is destroyed.
    ///      */
    /// ```
    ///

    /// `attribute nsISupports cacheElement;`
    #[inline]
    pub unsafe fn SetCacheElement(&self, aCacheElement: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetCacheElement)(self, aCacheElement)
    }


    /// ```text
    /// /**
    ///       * Stores the Content-Length specified in the HTTP header for this
    ///       * entry. Checked before we write to the cache entry, to prevent ever
    ///       * taking up space in the cache for an entry that we know up front
    ///       * is going to have to be evicted anyway. See bug 588507.
    ///       */
    /// ```
    ///

    /// `attribute int64_t predictedDataSize;`
    #[inline]
    pub unsafe fn GetPredictedDataSize(&self, aPredictedDataSize: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPredictedDataSize)(self, aPredictedDataSize)
    }


    /// ```text
    /// /**
    ///       * Stores the Content-Length specified in the HTTP header for this
    ///       * entry. Checked before we write to the cache entry, to prevent ever
    ///       * taking up space in the cache for an entry that we know up front
    ///       * is going to have to be evicted anyway. See bug 588507.
    ///       */
    /// ```
    ///

    /// `attribute int64_t predictedDataSize;`
    #[inline]
    pub unsafe fn SetPredictedDataSize(&self, aPredictedDataSize: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetPredictedDataSize)(self, aPredictedDataSize)
    }


    /// ```text
    /// /**
    ///      * Get the access granted to this descriptor.  See nsICache.idl for the
    ///      * definitions of the access modes and a thorough description of their
    ///      * corresponding meanings.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsCacheAccessMode accessGranted;`
    #[inline]
    pub unsafe fn GetAccessGranted(&self, aAccessGranted: *mut nsCacheAccessMode) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessGranted)(self, aAccessGranted)
    }


    /// ```text
    /// /**
    ///      * Get/set the storage policy of the cache entry.  See nsICache.idl for
    ///      * the definitions of the storage policies.
    ///      */
    /// ```
    ///

    /// `attribute nsCacheStoragePolicy storagePolicy;`
    #[inline]
    pub unsafe fn GetStoragePolicy(&self, aStoragePolicy: *mut nsCacheStoragePolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetStoragePolicy)(self, aStoragePolicy)
    }


    /// ```text
    /// /**
    ///      * Get/set the storage policy of the cache entry.  See nsICache.idl for
    ///      * the definitions of the storage policies.
    ///      */
    /// ```
    ///

    /// `attribute nsCacheStoragePolicy storagePolicy;`
    #[inline]
    pub unsafe fn SetStoragePolicy(&self, aStoragePolicy: nsCacheStoragePolicy) -> ::nserror::nsresult {
        ((*self.vtable).SetStoragePolicy)(self, aStoragePolicy)
    }


    /// ```text
    /// /**
    ///      * Get the disk file associated with the cache entry.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile file;`
    #[inline]
    pub unsafe fn GetFile(&self, aFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///      * Get/set security info on the cache entry for this descriptor.  This fails
    ///      * if the storage policy is not STORE_IN_MEMORY.
    ///      */
    /// ```
    ///

    /// `attribute nsISupports securityInfo;`
    #[inline]
    pub unsafe fn GetSecurityInfo(&self, aSecurityInfo: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetSecurityInfo)(self, aSecurityInfo)
    }


    /// ```text
    /// /**
    ///      * Get/set security info on the cache entry for this descriptor.  This fails
    ///      * if the storage policy is not STORE_IN_MEMORY.
    ///      */
    /// ```
    ///

    /// `attribute nsISupports securityInfo;`
    #[inline]
    pub unsafe fn SetSecurityInfo(&self, aSecurityInfo: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetSecurityInfo)(self, aSecurityInfo)
    }


    /// ```text
    /// /**
    ///      * Get the size of the cache entry data, as stored. This may differ
    ///      * from the entry's dataSize, if the entry is compressed.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long storageDataSize;`
    #[inline]
    pub unsafe fn GetStorageDataSize(&self, aStorageDataSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageDataSize)(self, aStorageDataSize)
    }


    /// ```text
    /// /**
    ///      * Doom the cache entry this descriptor references in order to slate it for
    ///      * removal.  Once doomed a cache entry cannot be undoomed.
    ///      *
    ///      * A descriptor with WRITE access can doom the cache entry and choose to
    ///      * fail pending requests.  This means that pending requests will not get
    ///      * a cache descriptor.  This is meant as a tool for clients that wish to
    ///      * instruct pending requests to skip the cache.
    ///      */
    /// ```
    ///

    /// `void doom ();`
    #[inline]
    pub unsafe fn Doom(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Doom)(self, )
    }



    /// `void doomAndFailPendingRequests (in nsresult status);`
    #[inline]
    pub unsafe fn DoomAndFailPendingRequests(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DoomAndFailPendingRequests)(self, status)
    }


    /// ```text
    /// /**
    ///      * Asynchronously doom an entry. Listener will be notified about the status
    ///      * of the operation. Null may be passed if caller doesn't care about the
    ///      * result.
    ///      */
    /// ```
    ///

    /// `void asyncDoom (in nsICacheListener listener);`
    #[inline]
    pub unsafe fn AsyncDoom(&self, listener: *const nsICacheListener) -> ::nserror::nsresult {
        ((*self.vtable).AsyncDoom)(self, listener)
    }


    /// ```text
    /// /**
    ///      * A writer must validate this cache object before any readers are given
    ///      * a descriptor to the object.
    ///      */
    /// ```
    ///

    /// `void markValid ();`
    #[inline]
    pub unsafe fn MarkValid(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MarkValid)(self, )
    }


    /// ```text
    /// /**
    ///      *  Explicitly close the descriptor (optional).
    ///      */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///      * Methods for accessing meta data.  Meta data is a table of key/value
    ///      * string pairs.  The strings do not have to conform to any particular
    ///      * charset, but they must be null terminated.
    ///      */
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
    ///      * Visitor will be called with key/value pair for each meta data element.
    ///      */
    /// ```
    ///

    /// `void visitMetaData (in nsICacheMetaDataVisitor visitor);`
    #[inline]
    pub unsafe fn VisitMetaData(&self, visitor: *const nsICacheMetaDataVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitMetaData)(self, visitor)
    }


}


/// `interface nsICacheMetaDataVisitor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheMetaDataVisitor {
    vtable: *const nsICacheMetaDataVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheMetaDataVisitor.
unsafe impl XpCom for nsICacheMetaDataVisitor {
    const IID: nsIID = nsID(0x22f9a49c, 0x3cf8, 0x4c23,
        [0x80, 0x06, 0x54, 0xef, 0xb1, 0x1a, 0xc5, 0x62]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheMetaDataVisitor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheMetaDataVisitor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheMetaDataVisitorCoerce {
    /// Cheaply cast a value of this type from a `nsICacheMetaDataVisitor`.
    fn coerce_from(v: &nsICacheMetaDataVisitor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheMetaDataVisitorCoerce for nsICacheMetaDataVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheMetaDataVisitor) -> &Self {
        v
    }
}

impl nsICacheMetaDataVisitor {
    /// Cast this `nsICacheMetaDataVisitor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheMetaDataVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheMetaDataVisitor {
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
impl<T: nsISupportsCoerce> nsICacheMetaDataVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheMetaDataVisitor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheMetaDataVisitor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheMetaDataVisitorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean visitMetaDataElement (in string key, in string value); */
    pub VisitMetaDataElement: unsafe extern "system" fn (this: *const nsICacheMetaDataVisitor, key: *const libc::c_char, value: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheMetaDataVisitor {

    /// ```text
    /// /**
    ///      * Called for each key/value pair in the meta data for a cache entry
    ///      */
    /// ```
    ///

    /// `boolean visitMetaDataElement (in string key, in string value);`
    #[inline]
    pub unsafe fn VisitMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).VisitMetaDataElement)(self, key, value, _retval)
    }


}


