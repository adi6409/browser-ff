//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageStatementCallback.idl
//


/// `interface mozIStorageStatementCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageStatementCallback {
    vtable: *const mozIStorageStatementCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageStatementCallback.
unsafe impl XpCom for mozIStorageStatementCallback {
    const IID: nsIID = nsID(0x29383d00, 0xd8c4, 0x4ddd,
        [0x9f, 0x8b, 0xc2, 0xfe, 0xb0, 0xf2, 0xfc, 0xfa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageStatementCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageStatementCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageStatementCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageStatementCallback`.
    fn coerce_from(v: &mozIStorageStatementCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageStatementCallbackCoerce for mozIStorageStatementCallback {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementCallback) -> &Self {
        v
    }
}

impl mozIStorageStatementCallback {
    /// Cast this `mozIStorageStatementCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageStatementCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageStatementCallback {
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
impl<T: nsISupportsCoerce> mozIStorageStatementCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageStatementCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageStatementCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleResult (in mozIStorageResultSet aResultSet); */
    pub HandleResult: unsafe extern "system" fn (this: *const mozIStorageStatementCallback, aResultSet: *const mozIStorageResultSet) -> ::nserror::nsresult,

    /* void handleError (in mozIStorageError aError); */
    pub HandleError: unsafe extern "system" fn (this: *const mozIStorageStatementCallback, aError: *const mozIStorageError) -> ::nserror::nsresult,

    /* void handleCompletion (in unsigned short aReason); */
    pub HandleCompletion: unsafe extern "system" fn (this: *const mozIStorageStatementCallback, aReason: u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageStatementCallback {
    /// ```text
    /// /**
    ///    * Called when the statement has finished executing.  This function will only
    ///    * be called once for any given asynchronous statement.
    ///    *
    ///    * @param aReason
    ///    *        Indicates if the statement is no longer executing because it either
    ///    *        finished (REASON_FINISHED), was canceled (REASON_CANCELED), or
    ///    *        a fatal error occurred (REASON_ERROR).
    ///    */
    /// ```
    ///

    pub const REASON_FINISHED: i64 = 0;


    pub const REASON_CANCELED: i64 = 1;


    pub const REASON_ERROR: i64 = 2;

    /// ```text
    /// /**
    ///    * Called when some result is obtained from the database.  This function can
    ///    * be called more than once with a different storageIResultSet each time for
    ///    * any given asynchronous statement.
    ///    *
    ///    * @param aResultSet
    ///    *        The result set containing the data from the database.
    ///    */
    /// ```
    ///

    /// `void handleResult (in mozIStorageResultSet aResultSet);`
    #[inline]
    pub unsafe fn HandleResult(&self, aResultSet: *const mozIStorageResultSet) -> ::nserror::nsresult {
        ((*self.vtable).HandleResult)(self, aResultSet)
    }


    /// ```text
    /// /**
    ///    * Called when some error occurs while executing the statement.  This function
    ///    * may be called more than once with a different storageIError each time for
    ///    * any given asynchronous statement.
    ///    *
    ///    * @param aError
    ///    *        An object containing information about the error.
    ///    */
    /// ```
    ///

    /// `void handleError (in mozIStorageError aError);`
    #[inline]
    pub unsafe fn HandleError(&self, aError: *const mozIStorageError) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, aError)
    }



    /// `void handleCompletion (in unsigned short aReason);`
    #[inline]
    pub unsafe fn HandleCompletion(&self, aReason: u16) -> ::nserror::nsresult {
        ((*self.vtable).HandleCompletion)(self, aReason)
    }


}


