//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICacheInfoChannel.idl
//


/// `interface nsIInputStreamReceiver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamReceiver {
    vtable: *const nsIInputStreamReceiverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamReceiver.
unsafe impl XpCom for nsIInputStreamReceiver {
    const IID: nsIID = nsID(0x1fb8ccf2, 0x5fa5, 0x45ec,
        [0xbc, 0x57, 0x8c, 0x80, 0x22, 0xa5, 0xd0, 0xd3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamReceiver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamReceiver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamReceiverCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamReceiver`.
    fn coerce_from(v: &nsIInputStreamReceiver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamReceiverCoerce for nsIInputStreamReceiver {
    #[inline]
    fn coerce_from(v: &nsIInputStreamReceiver) -> &Self {
        v
    }
}

impl nsIInputStreamReceiver {
    /// Cast this `nsIInputStreamReceiver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamReceiverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamReceiver {
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
impl<T: nsISupportsCoerce> nsIInputStreamReceiverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamReceiver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamReceiver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamReceiverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onInputStreamReady (in nsIInputStream aStream); */
    pub OnInputStreamReady: unsafe extern "system" fn (this: *const nsIInputStreamReceiver, aStream: *const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamReceiver {


    /// `void onInputStreamReady (in nsIInputStream aStream);`
    #[inline]
    pub unsafe fn OnInputStreamReady(&self, aStream: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).OnInputStreamReady)(self, aStream)
    }


}


/// `interface nsICacheInfoChannel : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheInfoChannel {
    vtable: *const nsICacheInfoChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheInfoChannel.
unsafe impl XpCom for nsICacheInfoChannel {
    const IID: nsIID = nsID(0x72c34415, 0xc6eb, 0x48af,
        [0x85, 0x1f, 0x77, 0x2f, 0xa9, 0xee, 0x59, 0x72]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheInfoChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheInfoChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheInfoChannelCoerce {
    /// Cheaply cast a value of this type from a `nsICacheInfoChannel`.
    fn coerce_from(v: &nsICacheInfoChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheInfoChannelCoerce for nsICacheInfoChannel {
    #[inline]
    fn coerce_from(v: &nsICacheInfoChannel) -> &Self {
        v
    }
}

impl nsICacheInfoChannel {
    /// Cast this `nsICacheInfoChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheInfoChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheInfoChannel {
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
impl<T: nsISupportsCoerce> nsICacheInfoChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheInfoChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheInfoChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheInfoChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute int32_t cacheTokenFetchCount; */
    pub GetCacheTokenFetchCount: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aCacheTokenFetchCount: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t cacheTokenExpirationTime; */
    pub GetCacheTokenExpirationTime: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aCacheTokenExpirationTime: *mut uint32_t) -> ::nserror::nsresult,

    /* boolean isFromCache (); */
    pub IsFromCache: unsafe extern "system" fn (this: *const nsICacheInfoChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isRacing (); */
    pub IsRacing: unsafe extern "system" fn (this: *const nsICacheInfoChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* uint64_t getCacheEntryId (); */
    pub GetCacheEntryId: unsafe extern "system" fn (this: *const nsICacheInfoChannel, _retval: *mut uint64_t) -> ::nserror::nsresult,

    /* attribute unsigned long cacheKey; */
    pub GetCacheKey: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aCacheKey: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long cacheKey; */
    pub SetCacheKey: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aCacheKey: u32) -> ::nserror::nsresult,

    /* attribute boolean allowStaleCacheContent; */
    pub GetAllowStaleCacheContent: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aAllowStaleCacheContent: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean allowStaleCacheContent; */
    pub SetAllowStaleCacheContent: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aAllowStaleCacheContent: bool) -> ::nserror::nsresult,

    /* attribute boolean preferCacheLoadOverBypass; */
    pub GetPreferCacheLoadOverBypass: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aPreferCacheLoadOverBypass: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean preferCacheLoadOverBypass; */
    pub SetPreferCacheLoadOverBypass: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aPreferCacheLoadOverBypass: bool) -> ::nserror::nsresult,

    /* void preferAlternativeDataType (in ACString type, in ACString contentType, in boolean deliverAltData); */
    pub PreferAlternativeDataType: unsafe extern "system" fn (this: *const nsICacheInfoChannel, type_: *const ::nsstring::nsACString, contentType: *const ::nsstring::nsACString, deliverAltData: bool) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] ConstPreferredAlternativeDataTypeArray preferredAlternativeDataTypes (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub PreferredAlternativeDataTypes: *const ::libc::c_void,

    /* readonly attribute ACString alternativeDataType; */
    pub GetAlternativeDataType: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aAlternativeDataType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void getAltDataInputStream (in ACString type, in nsIInputStreamReceiver aReceiver); */
    pub GetAltDataInputStream: unsafe extern "system" fn (this: *const nsICacheInfoChannel, type_: *const ::nsstring::nsACString, aReceiver: *const nsIInputStreamReceiver) -> ::nserror::nsresult,

    /* void getOriginalInputStream (in nsIInputStreamReceiver aReceiver); */
    pub GetOriginalInputStream: unsafe extern "system" fn (this: *const nsICacheInfoChannel, aReceiver: *const nsIInputStreamReceiver) -> ::nserror::nsresult,

    /* nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize); */
    pub OpenAlternativeOutputStream: unsafe extern "system" fn (this: *const nsICacheInfoChannel, type_: *const ::nsstring::nsACString, predictedSize: i64, _retval: *mut*const nsIAsyncOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheInfoChannel {

    /// ```text
    /// /**
    ///    * Get the number of times the cache entry has been opened. This attribute is
    ///    * equivalent to nsICachingChannel.cacheToken.fetchCount.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE if the cache entry or the alternate data
    ///    *         cache entry cannot be read.
    ///    */
    /// ```
    ///

    /// `readonly attribute int32_t cacheTokenFetchCount;`
    #[inline]
    pub unsafe fn GetCacheTokenFetchCount(&self, aCacheTokenFetchCount: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheTokenFetchCount)(self, aCacheTokenFetchCount)
    }


    /// ```text
    /// /**
    ///    * Get expiration time from cache token. This attribute is equivalent to
    ///    * nsICachingChannel.cacheToken.expirationTime.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t cacheTokenExpirationTime;`
    #[inline]
    pub unsafe fn GetCacheTokenExpirationTime(&self, aCacheTokenExpirationTime: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheTokenExpirationTime)(self, aCacheTokenExpirationTime)
    }


    /// ```text
    /// /**
    ///    * TRUE if this channel's data is being loaded from the cache.  This value
    ///    * is undefined before the channel fires its OnStartRequest notification
    ///    * and after the channel fires its OnStopRequest notification.
    ///    */
    /// ```
    ///

    /// `boolean isFromCache ();`
    #[inline]
    pub unsafe fn IsFromCache(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsFromCache)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the channel raced the cache and network requests.
    ///    * In order to determine if the response is coming from the cache or the
    ///    * network, the consumer can check isFromCache().
    ///    * The method can only be called after the channel fires its OnStartRequest
    ///    * notification.
    ///    */
    /// ```
    ///

    /// `boolean isRacing ();`
    #[inline]
    pub unsafe fn IsRacing(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsRacing)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * The unique ID of the corresponding nsICacheEntry from which the response is
    ///    * retrieved. By comparing the returned value, we can judge whether the data
    ///    * of two distinct nsICacheInfoChannels is from the same nsICacheEntry. This
    ///    * scenario could be useful when verifying whether the alternative data from
    ///    * one nsICacheInfochannel matches the main data from another one.
    ///    *
    ///    * Note: NS_ERROR_NOT_AVAILABLE is thrown when a nsICacheInfoChannel has no
    ///    * valid corresponding nsICacheEntry.
    ///    */
    /// ```
    ///

    /// `uint64_t getCacheEntryId ();`
    #[inline]
    pub unsafe fn GetCacheEntryId(&self, _retval: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheEntryId)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Set/get the cache key. This integer uniquely identifies the data in
    ///    * the cache for this channel.
    ///    *
    ///    * A cache key retrieved from a particular instance of nsICacheInfoChannel
    ///    * could be set on another instance of nsICacheInfoChannel provided the
    ///    * underlying implementations are compatible and provided the new
    ///    * channel instance was created with the same URI.  The implementation of
    ///    * nsICacheInfoChannel would be expected to use the cache entry identified
    ///    * by the cache token.  Depending on the value of nsIRequest::loadFlags,
    ///    * the cache entry may be validated, overwritten, or simply read.
    ///    *
    ///    * The cache key may be 0 indicating that the URI of the channel is
    ///    * sufficient to locate the same cache entry.  Setting a 0 cache key
    ///    * is likewise valid.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long cacheKey;`
    #[inline]
    pub unsafe fn GetCacheKey(&self, aCacheKey: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheKey)(self, aCacheKey)
    }


    /// ```text
    /// /**
    ///    * Set/get the cache key. This integer uniquely identifies the data in
    ///    * the cache for this channel.
    ///    *
    ///    * A cache key retrieved from a particular instance of nsICacheInfoChannel
    ///    * could be set on another instance of nsICacheInfoChannel provided the
    ///    * underlying implementations are compatible and provided the new
    ///    * channel instance was created with the same URI.  The implementation of
    ///    * nsICacheInfoChannel would be expected to use the cache entry identified
    ///    * by the cache token.  Depending on the value of nsIRequest::loadFlags,
    ///    * the cache entry may be validated, overwritten, or simply read.
    ///    *
    ///    * The cache key may be 0 indicating that the URI of the channel is
    ///    * sufficient to locate the same cache entry.  Setting a 0 cache key
    ///    * is likewise valid.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long cacheKey;`
    #[inline]
    pub unsafe fn SetCacheKey(&self, aCacheKey: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetCacheKey)(self, aCacheKey)
    }


    /// ```text
    /// /**
    ///    * Tells the channel to behave as if the LOAD_FROM_CACHE flag has been set,
    ///    * but without affecting the loads for the entire loadGroup in case of this
    ///    * channel being the default load group's channel.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowStaleCacheContent;`
    #[inline]
    pub unsafe fn GetAllowStaleCacheContent(&self, aAllowStaleCacheContent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowStaleCacheContent)(self, aAllowStaleCacheContent)
    }


    /// ```text
    /// /**
    ///    * Tells the channel to behave as if the LOAD_FROM_CACHE flag has been set,
    ///    * but without affecting the loads for the entire loadGroup in case of this
    ///    * channel being the default load group's channel.
    ///    */
    /// ```
    ///

    /// `attribute boolean allowStaleCacheContent;`
    #[inline]
    pub unsafe fn SetAllowStaleCacheContent(&self, aAllowStaleCacheContent: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllowStaleCacheContent)(self, aAllowStaleCacheContent)
    }


    /// ```text
    /// /**
    ///    * Tells the priority for LOAD_CACHE is raised over LOAD_BYPASS_CACHE or
    ///    * LOAD_BYPASS_LOCAL_CACHE in case those flags are set at the same time.
    ///    */
    /// ```
    ///

    /// `attribute boolean preferCacheLoadOverBypass;`
    #[inline]
    pub unsafe fn GetPreferCacheLoadOverBypass(&self, aPreferCacheLoadOverBypass: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPreferCacheLoadOverBypass)(self, aPreferCacheLoadOverBypass)
    }


    /// ```text
    /// /**
    ///    * Tells the priority for LOAD_CACHE is raised over LOAD_BYPASS_CACHE or
    ///    * LOAD_BYPASS_LOCAL_CACHE in case those flags are set at the same time.
    ///    */
    /// ```
    ///

    /// `attribute boolean preferCacheLoadOverBypass;`
    #[inline]
    pub unsafe fn SetPreferCacheLoadOverBypass(&self, aPreferCacheLoadOverBypass: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPreferCacheLoadOverBypass)(self, aPreferCacheLoadOverBypass)
    }


    /// ```text
    /// /**
    ///    * Calling this method instructs the channel to serve the alternative data
    ///    * if that was previously saved in the cache, otherwise it will serve the
    ///    * real data.
    ///    * @param type
    ///    *        a string identifying the alt-data format
    ///    * @param contentType
    ///    *        the contentType for which the preference applies.
    ///    *        an empty contentType means the preference applies for ANY contentType
    ///    * @param deliverAltData
    ///    *        if false, also if alt-data is available, the channel will deliver
    ///    *        the original data.
    ///    *
    ///    * The method may be called several times, with different type and contentType.
    ///    *
    ///    * Must be called before AsyncOpen.
    ///    */
    /// ```
    ///

    /// `void preferAlternativeDataType (in ACString type, in ACString contentType, in boolean deliverAltData);`
    #[inline]
    pub unsafe fn PreferAlternativeDataType(&self, type_: *const ::nsstring::nsACString, contentType: *const ::nsstring::nsACString, deliverAltData: bool) -> ::nserror::nsresult {
        ((*self.vtable).PreferAlternativeDataType)(self, type_, contentType, deliverAltData)
    }


    /// ```text
    /// /**
    ///    * Get the preferred alternative data type set by preferAlternativeDataType().
    ///    * The returned types stand for the desired data type instead of the type of the
    ///    * information retrieved from the network stack.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] ConstPreferredAlternativeDataTypeArray preferredAlternativeDataTypes ();`
    const _PreferredAlternativeDataTypes: () = ();

    /// ```text
    /// /**
    ///    * Holds the type of the alternative data representation that the channel
    ///    * is returning.
    ///    * Is empty string if no alternative data representation was requested, or
    ///    * if the requested representation wasn't found in the cache.
    ///    * Can only be called during or after OnStartRequest.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString alternativeDataType;`
    #[inline]
    pub unsafe fn GetAlternativeDataType(&self, aAlternativeDataType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAlternativeDataType)(self, aAlternativeDataType)
    }


    /// ```text
    /// /**
    ///    * If preferAlternativeDataType() has been called passing deliverAltData
    ///    * equal to false, this method will expose the alt-data inputStream if
    ///    * aviable.
    ///    */
    /// ```
    ///

    /// `void getAltDataInputStream (in ACString type, in nsIInputStreamReceiver aReceiver);`
    #[inline]
    pub unsafe fn GetAltDataInputStream(&self, type_: *const ::nsstring::nsACString, aReceiver: *const nsIInputStreamReceiver) -> ::nserror::nsresult {
        ((*self.vtable).GetAltDataInputStream)(self, type_, aReceiver)
    }


    /// ```text
    /// /**
    ///    * Sometimes when the channel is delivering alt-data, we may want to somehow
    ///    * access the original content too. This method asynchronously opens the
    ///    * input stream and delivers it to the receiver.
    ///    */
    /// ```
    ///

    /// `void getOriginalInputStream (in nsIInputStreamReceiver aReceiver);`
    #[inline]
    pub unsafe fn GetOriginalInputStream(&self, aReceiver: *const nsIInputStreamReceiver) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalInputStream)(self, aReceiver)
    }


    /// ```text
    /// /**
    ///    * Opens and returns an output stream that a consumer may use to save an
    ///    * alternate representation of the data.
    ///    * Must be called after the OnStopRequest that delivered the real data.
    ///    * The consumer may choose to replace the saved alt representation.
    ///    * Opening the output stream will fail if there are any open input streams
    ///    * reading the already saved alt representation. After successfully opening
    ///    * an output stream, if there is an error before the entire alt data can be
    ///    * written successfully, the client must signal failure by passing an error
    ///    * code to CloseWithStatus().
    ///    *
    ///    * @param type
    ///    *        type of the alternative data representation
    ///    * @param predictedSize
    ///    *        Predicted size of the data that will be written. It's used to decide
    ///    *        whether the resulting entry would exceed size limit, in which case
    ///    *        an error is thrown. If the size isn't known in advance, -1 should be
    ///    *        passed.
    ///    */
    /// ```
    ///

    /// `nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize);`
    #[inline]
    pub unsafe fn OpenAlternativeOutputStream(&self, type_: *const ::nsstring::nsACString, predictedSize: i64, _retval: *mut*const nsIAsyncOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).OpenAlternativeOutputStream)(self, type_, predictedSize, _retval)
    }


}


