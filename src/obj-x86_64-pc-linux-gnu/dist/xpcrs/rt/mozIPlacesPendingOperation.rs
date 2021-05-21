//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIPlacesPendingOperation.idl
//


/// `interface mozIPlacesPendingOperation : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIPlacesPendingOperation {
    vtable: *const mozIPlacesPendingOperationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIPlacesPendingOperation.
unsafe impl XpCom for mozIPlacesPendingOperation {
    const IID: nsIID = nsID(0xebd31374, 0x3808, 0x40e4,
        [0x9e, 0x73, 0x30, 0x3b, 0xf7, 0x04, 0x67, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIPlacesPendingOperation {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIPlacesPendingOperation.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIPlacesPendingOperationCoerce {
    /// Cheaply cast a value of this type from a `mozIPlacesPendingOperation`.
    fn coerce_from(v: &mozIPlacesPendingOperation) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIPlacesPendingOperationCoerce for mozIPlacesPendingOperation {
    #[inline]
    fn coerce_from(v: &mozIPlacesPendingOperation) -> &Self {
        v
    }
}

impl mozIPlacesPendingOperation {
    /// Cast this `mozIPlacesPendingOperation` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIPlacesPendingOperationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIPlacesPendingOperation {
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
impl<T: nsISupportsCoerce> mozIPlacesPendingOperationCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPlacesPendingOperation) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIPlacesPendingOperation
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIPlacesPendingOperationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const mozIPlacesPendingOperation) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIPlacesPendingOperation {

    /// ```text
    /// /**
    ///    * Cancels a pending operation, if possible.  This will only fail if you try
    ///    * to cancel more than once.
    ///    */
    /// ```
    ///

    /// `void cancel ();`
    #[inline]
    pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, )
    }


}


