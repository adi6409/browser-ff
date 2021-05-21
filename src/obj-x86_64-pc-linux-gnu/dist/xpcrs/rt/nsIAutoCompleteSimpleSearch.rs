//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSimpleSearch.idl
//


/// `interface nsIAutoCompleteSimpleSearch : nsIAutoCompleteSearch`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteSimpleSearch {
    vtable: *const nsIAutoCompleteSimpleSearchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteSimpleSearch.
unsafe impl XpCom for nsIAutoCompleteSimpleSearch {
    const IID: nsIID = nsID(0xdc185a77, 0xba88, 0x4caa,
        [0x8f, 0x16, 0x46, 0x52, 0x53, 0xf7, 0x59, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteSimpleSearch {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteSimpleSearch.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteSimpleSearchCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteSimpleSearch`.
    fn coerce_from(v: &nsIAutoCompleteSimpleSearch) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteSimpleSearchCoerce for nsIAutoCompleteSimpleSearch {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleSearch) -> &Self {
        v
    }
}

impl nsIAutoCompleteSimpleSearch {
    /// Cast this `nsIAutoCompleteSimpleSearch` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSimpleSearchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteSimpleSearch {
    type Target = nsIAutoCompleteSearch;
    #[inline]
    fn deref(&self) -> &nsIAutoCompleteSearch {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIAutoCompleteSearchCoerce> nsIAutoCompleteSimpleSearchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleSearch) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteSimpleSearch
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteSimpleSearchVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAutoCompleteSearchVTable,

    /* void overrideNextResult (in nsIAutoCompleteResult values); */
    pub OverrideNextResult: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleSearch, values: *const nsIAutoCompleteResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteSimpleSearch {

    /// ```text
    /// /**
    ///    * Sets the result that should be used next time `startSearch` is called.
    ///    */
    /// ```
    ///

    /// `void overrideNextResult (in nsIAutoCompleteResult values);`
    #[inline]
    pub unsafe fn OverrideNextResult(&self, values: *const nsIAutoCompleteResult) -> ::nserror::nsresult {
        ((*self.vtable).OverrideNextResult)(self, values)
    }


}


