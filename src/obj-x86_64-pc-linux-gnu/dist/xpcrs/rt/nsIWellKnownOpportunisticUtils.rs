//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIWellKnownOpportunisticUtils.idl
//


/// `interface nsIWellKnownOpportunisticUtils : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWellKnownOpportunisticUtils {
    vtable: *const nsIWellKnownOpportunisticUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWellKnownOpportunisticUtils.
unsafe impl XpCom for nsIWellKnownOpportunisticUtils {
    const IID: nsIID = nsID(0xb4f96c89, 0x5238, 0x450c,
        [0x8b, 0xda, 0xe1, 0x2c, 0x26, 0xf1, 0xd1, 0x50]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWellKnownOpportunisticUtils {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWellKnownOpportunisticUtils.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWellKnownOpportunisticUtilsCoerce {
    /// Cheaply cast a value of this type from a `nsIWellKnownOpportunisticUtils`.
    fn coerce_from(v: &nsIWellKnownOpportunisticUtils) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWellKnownOpportunisticUtilsCoerce for nsIWellKnownOpportunisticUtils {
    #[inline]
    fn coerce_from(v: &nsIWellKnownOpportunisticUtils) -> &Self {
        v
    }
}

impl nsIWellKnownOpportunisticUtils {
    /// Cast this `nsIWellKnownOpportunisticUtils` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWellKnownOpportunisticUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWellKnownOpportunisticUtils {
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
impl<T: nsISupportsCoerce> nsIWellKnownOpportunisticUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWellKnownOpportunisticUtils) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWellKnownOpportunisticUtils
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWellKnownOpportunisticUtilsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void verify (in ACString aJSON, in ACString aOrigin); */
    pub Verify: unsafe extern "system" fn (this: *const nsIWellKnownOpportunisticUtils, aJSON: *const ::nsstring::nsACString, aOrigin: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute bool valid; */
    pub GetValid: unsafe extern "system" fn (this: *const nsIWellKnownOpportunisticUtils, aValid: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWellKnownOpportunisticUtils {


    /// `[must_use] void verify (in ACString aJSON, in ACString aOrigin);`
    #[inline]
    pub unsafe fn Verify(&self, aJSON: *const ::nsstring::nsACString, aOrigin: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Verify)(self, aJSON, aOrigin)
    }



    /// `[must_use] readonly attribute bool valid;`
    #[inline]
    pub unsafe fn GetValid(&self, aValid: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetValid)(self, aValid)
    }


}


