//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheContainer.idl
//


/// `interface nsIApplicationCacheContainer : nsISupports`
///

/// ```text
/// /**
///  * Interface used by objects that can be associated with an
///  * application cache.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationCacheContainer {
    vtable: *const nsIApplicationCacheContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationCacheContainer.
unsafe impl XpCom for nsIApplicationCacheContainer {
    const IID: nsIID = nsID(0xbbb80700, 0x1f7f, 0x4258,
        [0xaf, 0xf4, 0x17, 0x43, 0xcc, 0x5a, 0x7d, 0x23]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationCacheContainer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationCacheContainer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationCacheContainerCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationCacheContainer`.
    fn coerce_from(v: &nsIApplicationCacheContainer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationCacheContainerCoerce for nsIApplicationCacheContainer {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheContainer) -> &Self {
        v
    }
}

impl nsIApplicationCacheContainer {
    /// Cast this `nsIApplicationCacheContainer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationCacheContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationCacheContainer {
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
impl<T: nsISupportsCoerce> nsIApplicationCacheContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheContainer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationCacheContainer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationCacheContainerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIApplicationCache applicationCache; */
    pub GetApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheContainer, aApplicationCache: *mut*const nsIApplicationCache) -> ::nserror::nsresult,

    /* attribute nsIApplicationCache applicationCache; */
    pub SetApplicationCache: unsafe extern "system" fn (this: *const nsIApplicationCacheContainer, aApplicationCache: *const nsIApplicationCache) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationCacheContainer {


    /// `attribute nsIApplicationCache applicationCache;`
    #[inline]
    pub unsafe fn GetApplicationCache(&self, aApplicationCache: *mut*const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationCache)(self, aApplicationCache)
    }



    /// `attribute nsIApplicationCache applicationCache;`
    #[inline]
    pub unsafe fn SetApplicationCache(&self, aApplicationCache: *const nsIApplicationCache) -> ::nserror::nsresult {
        ((*self.vtable).SetApplicationCache)(self, aApplicationCache)
    }


}


