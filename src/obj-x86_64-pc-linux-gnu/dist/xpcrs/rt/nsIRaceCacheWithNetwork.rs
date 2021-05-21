//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIRaceCacheWithNetwork.idl
//


/// `interface nsIRaceCacheWithNetwork : nsISupports`
///

/// ```text
/// /**
///  * This holds methods used to race the cache with the network for a specific
///  * channel. This interface is was designed with nsHttpChannel in mind, and it's
///  * expected this will be the only class implementing it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRaceCacheWithNetwork {
    vtable: *const nsIRaceCacheWithNetworkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRaceCacheWithNetwork.
unsafe impl XpCom for nsIRaceCacheWithNetwork {
    const IID: nsIID = nsID(0x4d963475, 0x8b16, 0x4c58,
        [0xb8, 0x04, 0x8a, 0x23, 0xd4, 0x94, 0x36, 0xc5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRaceCacheWithNetwork {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRaceCacheWithNetwork.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRaceCacheWithNetworkCoerce {
    /// Cheaply cast a value of this type from a `nsIRaceCacheWithNetwork`.
    fn coerce_from(v: &nsIRaceCacheWithNetwork) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRaceCacheWithNetworkCoerce for nsIRaceCacheWithNetwork {
    #[inline]
    fn coerce_from(v: &nsIRaceCacheWithNetwork) -> &Self {
        v
    }
}

impl nsIRaceCacheWithNetwork {
    /// Cast this `nsIRaceCacheWithNetwork` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRaceCacheWithNetworkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRaceCacheWithNetwork {
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
impl<T: nsISupportsCoerce> nsIRaceCacheWithNetworkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRaceCacheWithNetwork) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRaceCacheWithNetwork
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRaceCacheWithNetworkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void test_triggerNetwork (in long timeout); */
    pub Test_triggerNetwork: unsafe extern "system" fn (this: *const nsIRaceCacheWithNetwork, timeout: i32) -> ::nserror::nsresult,

    /* void test_delayCacheEntryOpeningBy (in long timeout); */
    pub Test_delayCacheEntryOpeningBy: unsafe extern "system" fn (this: *const nsIRaceCacheWithNetwork, timeout: i32) -> ::nserror::nsresult,

    /* void test_triggerDelayedOpenCacheEntry (); */
    pub Test_triggerDelayedOpenCacheEntry: unsafe extern "system" fn (this: *const nsIRaceCacheWithNetwork) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRaceCacheWithNetwork {

    /// ```text
    /// /****************************************************************************
    ///    * TEST ONLY: The following methods are for testing purposes only. Do not use
    ///    * them to do anything important in your code.
    ///    ****************************************************************************
    ///
    ///   /**
    ///    * Triggers network activity after given timeout. If timeout is 0, network
    ///    * activity is triggered immediately. If the cache.asyncOpenURI callbacks
    ///    * have already been called, the network activity may have already been triggered
    ///    * or the content may have already been delivered from the cache, so this
    ///    * operation will have no effect.
    ///    *
    ///    * @param timeout
    ///    *        - the delay in milliseconds until the network will be triggered.
    ///    */
    /// ```
    ///

    /// `void test_triggerNetwork (in long timeout);`
    #[inline]
    pub unsafe fn Test_triggerNetwork(&self, timeout: i32) -> ::nserror::nsresult {
        ((*self.vtable).Test_triggerNetwork)(self, timeout)
    }


    /// ```text
    /// /**
    ///    * Normally a HTTP channel would immediately call AsyncOpenURI leading to the
    ///    * cache storage to lookup the cache entry and return it. In order to
    ///    * simmulate real life conditions where fetching a cache entry takes a long
    ///    * time, we set a timer to delay the operation.
    ///    * Can only be called on the main thread.
    ///    *
    ///    * @param timeout
    ///    *        - the delay in milliseconds until the cache open will be triggered.
    ///    */
    /// ```
    ///

    /// `void test_delayCacheEntryOpeningBy (in long timeout);`
    #[inline]
    pub unsafe fn Test_delayCacheEntryOpeningBy(&self, timeout: i32) -> ::nserror::nsresult {
        ((*self.vtable).Test_delayCacheEntryOpeningBy)(self, timeout)
    }


    /// ```text
    /// /**
    ///    * Immediatelly triggers AsyncOpenURI if the timer hasn't fired.
    ///    * Can only be called on the main thread.
    ///    * This is only called in tests to reliably trigger the opening of the cache
    ///    * entry.
    ///    * @throws NS_ERROR_NOT_AVAILABLE if opening the cache wasn't delayed.
    ///    */
    /// ```
    ///

    /// `void test_triggerDelayedOpenCacheEntry ();`
    #[inline]
    pub unsafe fn Test_triggerDelayedOpenCacheEntry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Test_triggerDelayedOpenCacheEntry)(self, )
    }


}


