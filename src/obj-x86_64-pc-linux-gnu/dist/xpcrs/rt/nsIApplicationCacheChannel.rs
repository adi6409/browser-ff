//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheChannel.idl
//


/// `interface nsIApplicationCacheChannel : nsIApplicationCacheContainer`
///

/// ```text
/// /**
///  * Interface implemented by channels that support application caches.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationCacheChannel {
    vtable: *const nsIApplicationCacheChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationCacheChannel.
unsafe impl XpCom for nsIApplicationCacheChannel {
    const IID: nsIID = nsID(0x6fa816b1, 0x6d5f, 0x4380,
        [0x97, 0x04, 0x05, 0x4d, 0x09, 0x08, 0xcf, 0xa3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationCacheChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationCacheChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationCacheChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationCacheChannel`.
    fn coerce_from(v: &nsIApplicationCacheChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationCacheChannelCoerce for nsIApplicationCacheChannel {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheChannel) -> &Self {
        v
    }
}

impl nsIApplicationCacheChannel {
    /// Cast this `nsIApplicationCacheChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationCacheChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationCacheChannel {
    type Target = nsIApplicationCacheContainer;
    #[inline]
    fn deref(&self) -> &nsIApplicationCacheContainer {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIApplicationCacheContainerCoerce> nsIApplicationCacheChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationCacheChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationCacheChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIApplicationCacheContainerVTable,

    /* readonly attribute boolean loadedFromApplicationCache; */
    pub GetLoadedFromApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aLoadedFromApplicationCache: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean inheritApplicationCache; */
    pub GetInheritApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aInheritApplicationCache: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean inheritApplicationCache; */
    pub SetInheritApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aInheritApplicationCache: bool) -> ::nserror::nsresult,

    /* attribute boolean chooseApplicationCache; */
    pub GetChooseApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aChooseApplicationCache: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean chooseApplicationCache; */
    pub SetChooseApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aChooseApplicationCache: bool) -> ::nserror::nsresult,

    /* void markOfflineCacheEntryAsForeign (); */
    pub MarkOfflineCacheEntryAsForeign: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel) -> ::nserror::nsresult,

    /* attribute nsIApplicationCache applicationCacheForWrite; */
    pub GetApplicationCacheForWrite: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aApplicationCacheForWrite: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* attribute nsIApplicationCache applicationCacheForWrite; */
    pub SetApplicationCacheForWrite: unsafe extern "system" fn (this: *const nsIApplicationCacheChannel, aApplicationCacheForWrite: *const nsIApplicationCache) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationCacheChannel {

    /// ```text
    /// /**
    ///      * TRUE when the resource came from the application cache. This
    ///      * might be false even there is assigned an application cache
    ///      * e.g. in case of fallback of load of an entry matching bypass
    ///      * namespace.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean loadedFromApplicationCache;`
    #[inline]
    pub unsafe fn GetLoadedFromApplicationCache(&self, aLoadedFromApplicationCache: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadedFromApplicationCache)(self, aLoadedFromApplicationCache)
    }


    /// ```text
    /// /**
    ///      * When true, the channel will ask its notification callbacks for
    ///      * an application cache if one is not explicitly provided.  Default
    ///      * value is true.
    ///      *
    ///      * NS_ERROR_ALREADY_OPENED will be thrown if set after AsyncOpen()
    ///      * is called.
    ///      */
    /// ```
    ///

    /// `attribute boolean inheritApplicationCache;`
    #[inline]
    pub unsafe fn GetInheritApplicationCache(&self, aInheritApplicationCache: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInheritApplicationCache)(self, aInheritApplicationCache)
    }


    /// ```text
    /// /**
    ///      * When true, the channel will ask its notification callbacks for
    ///      * an application cache if one is not explicitly provided.  Default
    ///      * value is true.
    ///      *
    ///      * NS_ERROR_ALREADY_OPENED will be thrown if set after AsyncOpen()
    ///      * is called.
    ///      */
    /// ```
    ///

    /// `attribute boolean inheritApplicationCache;`
    #[inline]
    pub unsafe fn SetInheritApplicationCache(&self, aInheritApplicationCache: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetInheritApplicationCache)(self, aInheritApplicationCache)
    }


    /// ```text
    /// /**
    ///      * When true, the channel will choose an application cache if one
    ///      * was not explicitly provided and none is available from the
    ///      * notification callbacks.  Default value is false.
    ///      *
    ///      * This attribute will not be transferred through a redirect.
    ///      *
    ///      * NS_ERROR_ALREADY_OPENED will be thrown if set after AsyncOpen()
    ///      * is called.
    ///      */
    /// ```
    ///

    /// `attribute boolean chooseApplicationCache;`
    #[inline]
    pub unsafe fn GetChooseApplicationCache(&self, aChooseApplicationCache: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetChooseApplicationCache)(self, aChooseApplicationCache)
    }


    /// ```text
    /// /**
    ///      * When true, the channel will choose an application cache if one
    ///      * was not explicitly provided and none is available from the
    ///      * notification callbacks.  Default value is false.
    ///      *
    ///      * This attribute will not be transferred through a redirect.
    ///      *
    ///      * NS_ERROR_ALREADY_OPENED will be thrown if set after AsyncOpen()
    ///      * is called.
    ///      */
    /// ```
    ///

    /// `attribute boolean chooseApplicationCache;`
    #[inline]
    pub unsafe fn SetChooseApplicationCache(&self, aChooseApplicationCache: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetChooseApplicationCache)(self, aChooseApplicationCache)
    }


    /// ```text
    /// /**
    ///      * A shortcut method to mark the cache item of this channel as 'foreign'.
    ///      * See the 'cache selection algorithm' and CACHE_SELECTION_RELOAD
    ///      * action handling in nsContentSink.
    ///      */
    /// ```
    ///

    /// `void markOfflineCacheEntryAsForeign ();`
    #[inline]
    pub unsafe fn MarkOfflineCacheEntryAsForeign(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MarkOfflineCacheEntryAsForeign)(self, )
    }


    /// ```text
    /// /**
    ///      * Set offline application cache object to instruct the channel
    ///      * to cache for offline use using this application cache.
    ///      */
    /// ```
    ///

    /// `attribute nsIApplicationCache applicationCacheForWrite;`
    #[inline]
    pub unsafe fn GetApplicationCacheForWrite(&self, aApplicationCacheForWrite: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationCacheForWrite)(self, aApplicationCacheForWrite)
    }


    /// ```text
    /// /**
    ///      * Set offline application cache object to instruct the channel
    ///      * to cache for offline use using this application cache.
    ///      */
    /// ```
    ///

    /// `attribute nsIApplicationCache applicationCacheForWrite;`
    #[inline]
    pub unsafe fn SetApplicationCacheForWrite(&self, aApplicationCacheForWrite: *const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).SetApplicationCacheForWrite)(self, aApplicationCacheForWrite)
    }


}


