//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageProgressHandler.idl
//


/// `interface mozIStorageProgressHandler : nsISupports`
///

/// ```text
/// /**
///  * mozIProgressHandler is to be implemented by storage consumers that
///  * wish to receive callbacks during the request execution.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageProgressHandler {
    vtable: *const mozIStorageProgressHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageProgressHandler.
unsafe impl XpCom for mozIStorageProgressHandler {
    const IID: nsIID = nsID(0xa3a6fcd4, 0xbf89, 0x4208,
        [0xa8, 0x37, 0xbf, 0x2a, 0x73, 0xaf, 0xd3, 0x0c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageProgressHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageProgressHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageProgressHandlerCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageProgressHandler`.
    fn coerce_from(v: &mozIStorageProgressHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageProgressHandlerCoerce for mozIStorageProgressHandler {
    #[inline]
    fn coerce_from(v: &mozIStorageProgressHandler) -> &Self {
        v
    }
}

impl mozIStorageProgressHandler {
    /// Cast this `mozIStorageProgressHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageProgressHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageProgressHandler {
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
impl<T: nsISupportsCoerce> mozIStorageProgressHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageProgressHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageProgressHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageProgressHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean onProgress (in mozIStorageConnection aConnection); */
    pub OnProgress: unsafe extern "system" fn (this: *const mozIStorageProgressHandler, aConnection: *const mozIStorageConnection, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageProgressHandler {

    /// ```text
    /// /**
    ///    * onProgress is invoked periodically during long running calls.
    ///    *
    ///    * @param aConnection    connection, for which progress handler is
    ///    *                       invoked.
    ///    *
    ///    * @return true to abort request, false to continue work.
    ///    */
    /// ```
    ///

    /// `boolean onProgress (in mozIStorageConnection aConnection);`
    #[inline]
    pub unsafe fn OnProgress(&self, aConnection: *const mozIStorageConnection, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnProgress)(self, aConnection, _retval)
    }


}


