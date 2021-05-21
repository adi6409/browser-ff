//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageCompletionCallback.idl
//


/// `interface mozIStorageCompletionCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageCompletionCallback {
    vtable: *const mozIStorageCompletionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageCompletionCallback.
unsafe impl XpCom for mozIStorageCompletionCallback {
    const IID: nsIID = nsID(0x8cbf2dc2, 0x91e0, 0x44bc,
        [0x98, 0x4f, 0x55, 0x36, 0x38, 0x41, 0x20, 0x71]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageCompletionCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageCompletionCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageCompletionCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageCompletionCallback`.
    fn coerce_from(v: &mozIStorageCompletionCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageCompletionCallbackCoerce for mozIStorageCompletionCallback {
    #[inline]
    fn coerce_from(v: &mozIStorageCompletionCallback) -> &Self {
        v
    }
}

impl mozIStorageCompletionCallback {
    /// Cast this `mozIStorageCompletionCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageCompletionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageCompletionCallback {
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
impl<T: nsISupportsCoerce> mozIStorageCompletionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageCompletionCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageCompletionCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageCompletionCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in nsresult status, [optional] in nsISupports value); */
    pub Complete: unsafe extern "system" fn (this: *const mozIStorageCompletionCallback, status: ::nserror::nsresult, value: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageCompletionCallback {

    /// ```text
    /// /**
    ///    * Indicates that the event this callback was passed in for has completed.
    ///    *
    ///    * @param status
    ///    *        The status of the call. Generally NS_OK if the operation
    ///    *        completed successfully.
    ///    * @param value
    ///    *        If the operation produces a result, the result. Otherwise,
    ///    *        |null|.
    ///    *
    ///    * @see The calling method for expected values.
    ///    */
    /// ```
    ///

    /// `void complete (in nsresult status, [optional] in nsISupports value);`
    #[inline]
    pub unsafe fn Complete(&self, status: ::nserror::nsresult, value: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, status, value)
    }


}


