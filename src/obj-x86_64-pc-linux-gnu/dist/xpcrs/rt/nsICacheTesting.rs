//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheTesting.idl
//


/// `interface nsICacheTesting : nsISupports`
///

/// ```text
/// /**
///  * This is an internal interface used only for testing purposes.
///  *
///  * THIS IS NOT AN API TO BE USED BY EXTENSIONS! ONLY USED BY MOZILLA TESTS.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheTesting {
    vtable: *const nsICacheTestingVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheTesting.
unsafe impl XpCom for nsICacheTesting {
    const IID: nsIID = nsID(0x4e8ba935, 0x92e1, 0x4a74,
        [0x94, 0x4b, 0xb1, 0xa2, 0xf0, 0x2a, 0x74, 0x80]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheTesting {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheTesting.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheTestingCoerce {
    /// Cheaply cast a value of this type from a `nsICacheTesting`.
    fn coerce_from(v: &nsICacheTesting) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheTestingCoerce for nsICacheTesting {
    #[inline]
    fn coerce_from(v: &nsICacheTesting) -> &Self {
        v
    }
}

impl nsICacheTesting {
    /// Cast this `nsICacheTesting` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheTestingCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheTesting {
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
impl<T: nsISupportsCoerce> nsICacheTestingCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheTesting) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheTesting
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheTestingVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void suspendCacheIOThread (in uint32_t aLevel); */
    pub SuspendCacheIOThread: unsafe extern "system" fn (this: *const nsICacheTesting, aLevel: uint32_t) -> ::nserror::nsresult,

    /* void resumeCacheIOThread (); */
    pub ResumeCacheIOThread: unsafe extern "system" fn (this: *const nsICacheTesting) -> ::nserror::nsresult,

    /* void flush (in nsIObserver aObserver); */
    pub Flush: unsafe extern "system" fn (this: *const nsICacheTesting, aObserver: *const nsIObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheTesting {


    /// `void suspendCacheIOThread (in uint32_t aLevel);`
    #[inline]
    pub unsafe fn SuspendCacheIOThread(&self, aLevel: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SuspendCacheIOThread)(self, aLevel)
    }



    /// `void resumeCacheIOThread ();`
    #[inline]
    pub unsafe fn ResumeCacheIOThread(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResumeCacheIOThread)(self, )
    }



    /// `void flush (in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn Flush(&self, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).Flush)(self, aObserver)
    }


}


