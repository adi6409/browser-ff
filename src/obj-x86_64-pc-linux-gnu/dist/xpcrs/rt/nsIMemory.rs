//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemory.idl
//


/// `interface nsIMemory : nsISupports`
///

/// ```text
/// /**
///  *
///  * nsIMemory: interface to allocate and deallocate memory. Also provides
///  * for notifications in low-memory situations.
///  *
///  * The frozen exported symbols moz_xmalloc, moz_xrealloc, and free
///  * provide a more efficient way to access XPCOM memory allocation. Using
///  * those symbols is preferred to using the methods on this interface.
///  *
///  * A client that wishes to be notified of low memory situations (for
    ///  * example, because the client maintains a large memory cache that
    ///  * could be released when memory is tight) should register with the
///  * observer service (see nsIObserverService) using the topic
///  * "memory-pressure".  There are specific types of notications
///  * that can occur.  These types will be passed as the |aData|
///  * parameter of the of the "memory-pressure" notification:
///  *
///  * "low-memory"
///  * This will be passed as the extra data when the pressure
///  * observer is being asked to flush for low-memory conditions.
///  *
///  * "low-memory-ongoing"
///  * This will be passed when we continue to be in a low-memory
///  * condition and we want to flush caches and do other cheap
///  * forms of memory minimization, but heavy handed approaches like
///  * a GC are unlikely to succeed.
///  *
///  * "heap-minimize"
///  * This will be passed as the extra data when the pressure
///  * observer is being asked to flush because of a heap minimize
///  * call.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMemory {
    vtable: *const nsIMemoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMemory.
unsafe impl XpCom for nsIMemory {
    const IID: nsIID = nsID(0x1e004834, 0x6d8f, 0x425a,
        [0xbc, 0x9c, 0xa2, 0x81, 0x2e, 0xd4, 0x3b, 0xb7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMemory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMemory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMemoryCoerce {
    /// Cheaply cast a value of this type from a `nsIMemory`.
    fn coerce_from(v: &nsIMemory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMemoryCoerce for nsIMemory {
    #[inline]
    fn coerce_from(v: &nsIMemory) -> &Self {
        v
    }
}

impl nsIMemory {
    /// Cast this `nsIMemory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMemoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMemory {
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
impl<T: nsISupportsCoerce> nsIMemoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMemory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMemoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void heapMinimize (in boolean immediate); */
    pub HeapMinimize: unsafe extern "system" fn (this: *const nsIMemory, immediate: bool) -> ::nserror::nsresult,

    /* boolean isLowMemoryPlatform (); */
    pub IsLowMemoryPlatform: unsafe extern "system" fn (this: *const nsIMemory, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMemory {

    /// ```text
    /// /**
    ///      * Attempts to shrink the heap.
    ///      * @param immediate - if true, heap minimization will occur
    ///      *   immediately if the call was made on the main thread. If
    ///      *   false, the flush will be scheduled to happen when the app is
    ///      *   idle.
    ///      * @throws NS_ERROR_FAILURE if 'immediate' is set an the call
    ///      *   was not on the application's main thread.
    ///      */
    /// ```
    ///

    /// `void heapMinimize (in boolean immediate);`
    #[inline]
    pub unsafe fn HeapMinimize(&self, immediate: bool) -> ::nserror::nsresult {
        ((*self.vtable).HeapMinimize)(self, immediate)
    }


    /// ```text
    /// /**
    ///      * This predicate can be used to determine if the platform is a "low-memory"
    ///      * platform. Callers may use this to dynamically tune their behaviour
    ///      * to favour reduced memory usage at the expense of performance. The value
    ///      * returned by this function will not change over the lifetime of the process.
    ///      */
    /// ```
    ///

    /// `boolean isLowMemoryPlatform ();`
    #[inline]
    pub unsafe fn IsLowMemoryPlatform(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsLowMemoryPlatform)(self, _retval)
    }


}


