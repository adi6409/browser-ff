//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/sessionstore/SessionStoreFunctions.idl
//


/// `interface nsISessionStoreFunctions : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISessionStoreFunctions {
    vtable: *const nsISessionStoreFunctionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISessionStoreFunctions.
unsafe impl XpCom for nsISessionStoreFunctions {
    const IID: nsIID = nsID(0x1a060fba, 0xa19d, 0x11e9,
        [0xb7, 0xeb, 0x58, 0x0d, 0x0e, 0xdd, 0x8e, 0x6f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISessionStoreFunctions {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISessionStoreFunctions.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISessionStoreFunctionsCoerce {
    /// Cheaply cast a value of this type from a `nsISessionStoreFunctions`.
    fn coerce_from(v: &nsISessionStoreFunctions) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISessionStoreFunctionsCoerce for nsISessionStoreFunctions {
    #[inline]
    fn coerce_from(v: &nsISessionStoreFunctions) -> &Self {
        v
    }
}

impl nsISessionStoreFunctions {
    /// Cast this `nsISessionStoreFunctions` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISessionStoreFunctionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISessionStoreFunctions {
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
impl<T: nsISupportsCoerce> nsISessionStoreFunctionsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISessionStoreFunctions) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISessionStoreFunctions
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISessionStoreFunctionsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void UpdateSessionStore (in Element aBrowser, in BrowsingContext aBrowsingContext, in uint32_t aFlushId, in boolean aIsFinal, in uint32_t aEpoch, in jsval aData, in boolean aCollectSHistory); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub UpdateSessionStore: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISessionStoreFunctions {


    /// `void UpdateSessionStore (in Element aBrowser, in BrowsingContext aBrowsingContext, in uint32_t aFlushId, in boolean aIsFinal, in uint32_t aEpoch, in jsval aData, in boolean aCollectSHistory);`
    const _UpdateSessionStore: () = ();

}


