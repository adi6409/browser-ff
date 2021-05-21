//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserUsage.idl
//


/// `interface nsIBrowserUsage : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserUsage {
    vtable: *const nsIBrowserUsageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserUsage.
unsafe impl XpCom for nsIBrowserUsage {
    const IID: nsIID = nsID(0x2703b5ed, 0xa41f, 0x42be,
        [0x87, 0x64, 0xb7, 0x95, 0xeb, 0x67, 0xed, 0x25]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserUsage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserUsage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserUsageCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserUsage`.
    fn coerce_from(v: &nsIBrowserUsage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserUsageCoerce for nsIBrowserUsage {
    #[inline]
    fn coerce_from(v: &nsIBrowserUsage) -> &Self {
        v
    }
}

impl nsIBrowserUsage {
    /// Cast this `nsIBrowserUsage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserUsageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserUsage {
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
impl<T: nsISupportsCoerce> nsIBrowserUsageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserUsage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserUsage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserUsageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* uint32_t getUniqueDomainsVisitedInPast24Hours (); */
    pub GetUniqueDomainsVisitedInPast24Hours: unsafe extern "system" fn (this: *const nsIBrowserUsage, _retval: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserUsage {

    /// ```text
    /// /**
    ///    * Returns the number of unique domains (eTLD+1) visited in the past
    ///    * 24 hours by the user.
    ///    */
    /// ```
    ///

    /// `uint32_t getUniqueDomainsVisitedInPast24Hours ();`
    #[inline]
    pub unsafe fn GetUniqueDomainsVisitedInPast24Hours(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetUniqueDomainsVisitedInPast24Hours)(self, _retval)
    }


}


