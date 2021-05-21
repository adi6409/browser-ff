//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsIBFCacheEntry.idl
//


/// `interface nsIBFCacheEntry : nsISupports`
///

/// ```text
/// /**
///  * This interface lets you evict a document from the back/forward cache.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBFCacheEntry {
    vtable: *const nsIBFCacheEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBFCacheEntry.
unsafe impl XpCom for nsIBFCacheEntry {
    const IID: nsIID = nsID(0xa576060e, 0xc7df, 0x4d81,
        [0xaa, 0x8c, 0x5b, 0x52, 0xbd, 0x6a, 0xd2, 0x5d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBFCacheEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBFCacheEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBFCacheEntryCoerce {
    /// Cheaply cast a value of this type from a `nsIBFCacheEntry`.
    fn coerce_from(v: &nsIBFCacheEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBFCacheEntryCoerce for nsIBFCacheEntry {
    #[inline]
    fn coerce_from(v: &nsIBFCacheEntry) -> &Self {
        v
    }
}

impl nsIBFCacheEntry {
    /// Cast this `nsIBFCacheEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBFCacheEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBFCacheEntry {
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
impl<T: nsISupportsCoerce> nsIBFCacheEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBFCacheEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBFCacheEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBFCacheEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void RemoveFromBFCacheSync (); */
    pub RemoveFromBFCacheSync: unsafe extern "system" fn (this: *const nsIBFCacheEntry) -> ::nserror::nsresult,

    /* void RemoveFromBFCacheAsync (); */
    pub RemoveFromBFCacheAsync: unsafe extern "system" fn (this: *const nsIBFCacheEntry) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBFCacheEntry {


    /// `void RemoveFromBFCacheSync ();`
    #[inline]
    pub unsafe fn RemoveFromBFCacheSync(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveFromBFCacheSync)(self, )
    }



    /// `void RemoveFromBFCacheAsync ();`
    #[inline]
    pub unsafe fn RemoveFromBFCacheAsync(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveFromBFCacheAsync)(self, )
    }


}


