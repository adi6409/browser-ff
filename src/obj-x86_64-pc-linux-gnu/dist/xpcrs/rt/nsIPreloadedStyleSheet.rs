//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/base/nsIPreloadedStyleSheet.idl
//


/// `interface nsIPreloadedStyleSheet : nsISupports`
///

/// ```text
/// /**
///  * The nsIPreloadedStyleSheet interface is an opaque interface for
///  * style sheets returned by nsIStyleSheetService.preloadSheet, and
///  * which can be passed to nsIDOMWindowUtils.addSheet.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPreloadedStyleSheet {
    vtable: *const nsIPreloadedStyleSheetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPreloadedStyleSheet.
unsafe impl XpCom for nsIPreloadedStyleSheet {
    const IID: nsIID = nsID(0x2e2a84d0, 0x2102, 0x4b9e,
        [0x9b, 0x78, 0x16, 0x70, 0x62, 0x3a, 0x58, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPreloadedStyleSheet {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPreloadedStyleSheet.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPreloadedStyleSheetCoerce {
    /// Cheaply cast a value of this type from a `nsIPreloadedStyleSheet`.
    fn coerce_from(v: &nsIPreloadedStyleSheet) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPreloadedStyleSheetCoerce for nsIPreloadedStyleSheet {
    #[inline]
    fn coerce_from(v: &nsIPreloadedStyleSheet) -> &Self {
        v
    }
}

impl nsIPreloadedStyleSheet {
    /// Cast this `nsIPreloadedStyleSheet` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPreloadedStyleSheetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPreloadedStyleSheet {
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
impl<T: nsISupportsCoerce> nsIPreloadedStyleSheetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPreloadedStyleSheet) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPreloadedStyleSheet
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPreloadedStyleSheetVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPreloadedStyleSheet {


}


