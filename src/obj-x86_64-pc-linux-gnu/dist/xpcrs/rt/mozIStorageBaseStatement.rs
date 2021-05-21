//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBaseStatement.idl
//


/// `interface mozIStorageBaseStatement : mozIStorageBindingParams`
///

/// ```text
/// /**
///  * The base interface for both pure asynchronous storage statements
///  * (mozIStorageAsyncStatement) and 'classic' storage statements
///  * (mozIStorageStatement) that can be used for both synchronous and asynchronous
///  * purposes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageBaseStatement {
    vtable: *const mozIStorageBaseStatementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageBaseStatement.
unsafe impl XpCom for mozIStorageBaseStatement {
    const IID: nsIID = nsID(0x16ca67aa, 0x1325, 0x43e2,
        [0xaa, 0xc7, 0x85, 0x9a, 0xfd, 0x15, 0x90, 0xb2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageBaseStatement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageBaseStatement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageBaseStatementCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageBaseStatement`.
    fn coerce_from(v: &mozIStorageBaseStatement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageBaseStatementCoerce for mozIStorageBaseStatement {
    #[inline]
    fn coerce_from(v: &mozIStorageBaseStatement) -> &Self {
        v
    }
}

impl mozIStorageBaseStatement {
    /// Cast this `mozIStorageBaseStatement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageBaseStatementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageBaseStatement {
    type Target = mozIStorageBindingParams;
    #[inline]
    fn deref(&self) -> &mozIStorageBindingParams {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: mozIStorageBindingParamsCoerce> mozIStorageBaseStatementCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageBaseStatement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageBaseStatement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageBaseStatementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: mozIStorageBindingParamsVTable,

    /* void finalize (); */
    pub Finalize: unsafe extern "system" fn (this: *const mozIStorageBaseStatement) -> ::nserror::nsresult,

    /* void bindParameters (in mozIStorageBindingParamsArray aParameters); */
    pub BindParameters: unsafe extern "system" fn (this: *const mozIStorageBaseStatement, aParameters: *const mozIStorageBindingParamsArray) -> ::nserror::nsresult,

    /* mozIStorageBindingParamsArray newBindingParamsArray (); */
    pub NewBindingParamsArray: unsafe extern "system" fn (this: *const mozIStorageBaseStatement, _retval: *mut*const mozIStorageBindingParamsArray) -> ::nserror::nsresult,

    /* mozIStoragePendingStatement executeAsync ([optional] in mozIStorageStatementCallback aCallback); */
    pub ExecuteAsync: unsafe extern "system" fn (this: *const mozIStorageBaseStatement, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult,

    /* readonly attribute long state; */
    pub GetState: unsafe extern "system" fn (this: *const mozIStorageBaseStatement, aState: *mut i32) -> ::nserror::nsresult,

    /* AString escapeStringForLIKE (in AString aValue, in wchar aEscapeChar); */
    pub EscapeStringForLIKE: unsafe extern "system" fn (this: *const mozIStorageBaseStatement, aValue: *const ::nsstring::nsAString, aEscapeChar: i16, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AUTF8String escapeUTF8StringForLIKE (in AUTF8String aValue, in char aEscapeChar); */
    pub EscapeUTF8StringForLIKE: unsafe extern "system" fn (this: *const mozIStorageBaseStatement, aValue: *const ::nsstring::nsACString, aEscapeChar: libc::c_char, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageBaseStatement {
    /// ```text
    /// /**
    ///    * The statement is not usable, either because it failed to initialize or
    ///    * was explicitly finalized.
    ///    */
    /// ```
    ///

    pub const MOZ_STORAGE_STATEMENT_INVALID: i64 = 0;

    /// ```text
    /// /**
    ///    * The statement is usable.
    ///    */
    /// ```
    ///

    pub const MOZ_STORAGE_STATEMENT_READY: i64 = 1;

    /// ```text
    /// /**
    ///    * Indicates that the statement is executing and the row getters may be used.
    ///    *
    ///    * @note This is only relevant for mozIStorageStatement instances being used
    ///    *       in a synchronous fashion.
    ///    */
    /// ```
    ///

    pub const MOZ_STORAGE_STATEMENT_EXECUTING: i64 = 2;

    /// ```text
    /// /**
    ///    * Finalizes a statement so you can successfully close a database connection.
    ///    * Once a statement has been finalized it can no longer be used for any
    ///    * purpose.
    ///    *
    ///    * Statements are implicitly finalized when their reference counts hits zero.
    ///    * If you are a native (C++) caller this is accomplished by setting all of
    ///    * your nsCOMPtr instances to be NULL.  If you are operating from JavaScript
    ///    * code then you cannot rely on this behavior because of the involvement of
    ///    * garbage collection.
    ///    *
    ///    * When finalizing an asynchronous statement you do not need to worry about
    ///    * whether the statement has actually been executed by the asynchronous
    ///    * thread; you just need to call finalize after your last call to executeAsync
    ///    * involving the statement.  However, you do need to use asyncClose instead of
    ///    * close on the connection if any statements have been used asynchronously.
    ///    */
    /// ```
    ///

    /// `void finalize ();`
    #[inline]
    pub unsafe fn Finalize(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Finalize)(self, )
    }


    /// ```text
    /// /**
    ///    * Binds the array of parameters to the statement.  When executeAsync is
    ///    * called, all the parameters in aParameters are bound and then executed.
    ///    *
    ///    * @param aParameters
    ///    *        The array of parameters to bind to the statement upon execution.
    ///    *
    ///    * @note This is only works on statements being used asynchronously.
    ///    */
    /// ```
    ///

    /// `void bindParameters (in mozIStorageBindingParamsArray aParameters);`
    #[inline]
    pub unsafe fn BindParameters(&self, aParameters: *const mozIStorageBindingParamsArray) -> ::nserror::nsresult {
        ((*self.vtable).BindParameters)(self, aParameters)
    }


    /// ```text
    /// /**
    ///    * Creates a new mozIStorageBindingParamsArray that can be used to bind
    ///    * multiple sets of data to a statement with bindParameters.
    ///    *
    ///    * @return a mozIStorageBindingParamsArray that multiple sets of parameters
    ///    *         can be bound to.
    ///    *
    ///    * @note This is only useful for statements being used asynchronously.
    ///    */
    /// ```
    ///

    /// `mozIStorageBindingParamsArray newBindingParamsArray ();`
    #[inline]
    pub unsafe fn NewBindingParamsArray(&self, _retval: *mut*const mozIStorageBindingParamsArray) -> ::nserror::nsresult {
        ((*self.vtable).NewBindingParamsArray)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Execute a query asynchronously using any currently bound parameters.  This
    ///    * statement can be reused immediately, and reset does not need to be called.
    ///    *
    ///    * @note If you have any custom defined functions, they must be re-entrant
    ///    *       since they can be called on multiple threads.
    ///    *
    ///    * @param aCallback [optional]
    ///    *        The callback object that will be notified of progress, errors, and
    ///    *        completion.
    ///    * @return an object that can be used to cancel the statements execution.
    ///    */
    /// ```
    ///

    /// `mozIStoragePendingStatement executeAsync ([optional] in mozIStorageStatementCallback aCallback);`
    #[inline]
    pub unsafe fn ExecuteAsync(&self, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult {
        ((*self.vtable).ExecuteAsync)(self, aCallback, _retval)
    }


    /// ```text
    /// /**
    ///    * Find out whether the statement is usable (has not been finalized).
    ///    */
    /// ```
    ///

    /// `readonly attribute long state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * Escape a string for SQL LIKE search.
    ///    *
    ///    * @note Consumers will have to use same escape char when doing statements
    ///    *       such as:   ...LIKE '?1' ESCAPE '/'...
    ///    *
    ///    * @param aValue
    ///    *        The string to escape for SQL LIKE.
    ///    * @param aEscapeChar
    ///    *        The escape character.
    ///    * @return an AString of an escaped version of aValue
    ///    *         (%, _ and the escape char are escaped with the escape char)
    ///    *         For example, we will convert "foo/bar_baz%20cheese"
    ///    *         into "foo//bar/_baz/%20cheese" (if the escape char is '/').
    ///    */
    /// ```
    ///

    /// `AString escapeStringForLIKE (in AString aValue, in wchar aEscapeChar);`
    #[inline]
    pub unsafe fn EscapeStringForLIKE(&self, aValue: *const ::nsstring::nsAString, aEscapeChar: i16, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).EscapeStringForLIKE)(self, aValue, aEscapeChar, _retval)
    }


    /// ```text
    /// /**
    ///    * The same as above, but for UTF8 strings.
    ///    */
    /// ```
    ///

    /// `AUTF8String escapeUTF8StringForLIKE (in AUTF8String aValue, in char aEscapeChar);`
    #[inline]
    pub unsafe fn EscapeUTF8StringForLIKE(&self, aValue: *const ::nsstring::nsACString, aEscapeChar: libc::c_char, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EscapeUTF8StringForLIKE)(self, aValue, aEscapeChar, _retval)
    }


}


