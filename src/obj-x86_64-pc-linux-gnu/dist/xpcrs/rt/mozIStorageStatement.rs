//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageStatement.idl
//


/// `interface mozIStorageStatement : mozIStorageBaseStatement`
///

/// ```text
/// /**
///  * A SQL statement that can be used for both synchronous and asynchronous
///  * purposes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageStatement {
    vtable: *const mozIStorageStatementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageStatement.
unsafe impl XpCom for mozIStorageStatement {
    const IID: nsIID = nsID(0x5f567c35, 0x6c32, 0x4140,
        [0x82, 0x8c, 0x68, 0x3e, 0xa4, 0x9c, 0xfd, 0x3a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageStatement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageStatement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageStatementCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageStatement`.
    fn coerce_from(v: &mozIStorageStatement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageStatementCoerce for mozIStorageStatement {
    #[inline]
    fn coerce_from(v: &mozIStorageStatement) -> &Self {
        v
    }
}

impl mozIStorageStatement {
    /// Cast this `mozIStorageStatement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageStatementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageStatement {
    type Target = mozIStorageBaseStatement;
    #[inline]
    fn deref(&self) -> &mozIStorageBaseStatement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: mozIStorageBaseStatementCoerce> mozIStorageStatementCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageStatement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageStatement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageStatementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: mozIStorageBaseStatementVTable,

    /* mozIStorageStatement clone (); */
    pub Clone: unsafe extern "system" fn (this: *const mozIStorageStatement, _retval: *mut *const mozIStorageStatement) -> ::nserror::nsresult,

    /* readonly attribute unsigned long parameterCount; */
    pub GetParameterCount: unsafe extern "system" fn (this: *const mozIStorageStatement, aParameterCount: *mut u32) -> ::nserror::nsresult,

    /* AUTF8String getParameterName (in unsigned long aParamIndex); */
    pub GetParameterName: unsafe extern "system" fn (this: *const mozIStorageStatement, aParamIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* unsigned long getParameterIndex (in AUTF8String aName); */
    pub GetParameterIndex: unsafe extern "system" fn (this: *const mozIStorageStatement, aName: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long columnCount; */
    pub GetColumnCount: unsafe extern "system" fn (this: *const mozIStorageStatement, aColumnCount: *mut u32) -> ::nserror::nsresult,

    /* AUTF8String getColumnName (in unsigned long aColumnIndex); */
    pub GetColumnName: unsafe extern "system" fn (this: *const mozIStorageStatement, aColumnIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* unsigned long getColumnIndex (in AUTF8String aName); */
    pub GetColumnIndex: unsafe extern "system" fn (this: *const mozIStorageStatement, aName: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult,

    /* void reset (); */
    pub Reset: unsafe extern "system" fn (this: *const mozIStorageStatement) -> ::nserror::nsresult,

    /* void execute (); */
    pub Execute: unsafe extern "system" fn (this: *const mozIStorageStatement) -> ::nserror::nsresult,

    /* boolean executeStep (); */
    pub ExecuteStep: unsafe extern "system" fn (this: *const mozIStorageStatement, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long numEntries; */
    pub GetNumEntries: unsafe extern "system" fn (this: *const mozIStorageStatement, aNumEntries: *mut u32) -> ::nserror::nsresult,

    /* long getTypeOfIndex (in unsigned long aIndex); */
    pub GetTypeOfIndex: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut i32) -> ::nserror::nsresult,

    /* nsIVariant getVariant (in unsigned long aIndex); */
    pub GetVariant: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* long getInt32 (in unsigned long aIndex); */
    pub GetInt32: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut i32) -> ::nserror::nsresult,

    /* long long getInt64 (in unsigned long aIndex); */
    pub GetInt64: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut i64) -> ::nserror::nsresult,

    /* double getDouble (in unsigned long aIndex); */
    pub GetDouble: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* AUTF8String getUTF8String (in unsigned long aIndex); */
    pub GetUTF8String: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AString getString (in unsigned long aIndex); */
    pub GetString: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData); */
    pub GetBlob: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, aDataSize: *mut u32, aData: *mut *mut u8) -> ::nserror::nsresult,

    /* AString getBlobAsString (in unsigned long aIndex); */
    pub GetBlobAsString: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AUTF8String getBlobAsUTF8String (in unsigned long aIndex); */
    pub GetBlobAsUTF8String: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean getIsNull (in unsigned long aIndex); */
    pub GetIsNull: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out string aResult); */
    pub GetSharedUTF8String: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, aByteLength: *mut u32, aResult: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript] void getSharedString (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out wstring aResult); */
    pub GetSharedString: unsafe extern "system" fn (this: *const mozIStorageStatement, aIndex: u32, aByteLength: *mut u32, aResult: *mut *const i16) -> ::nserror::nsresult,

    /* [noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out octetPtr aResult); */
    /// Unable to generate binding because `native type uint8_t unsupported`
    pub GetSharedBlob: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageStatement {
    /// ```text
    /// /**
    ///    * Execute a query, using any currently-bound parameters.  Reset is called
    ///    * when no more data is returned.  This method is only available to JavaScript
    ///    * consumers.
    ///    *
    ///    * @deprecated As of Mozilla 1.9.2 in favor of executeStep().
    ///    *
    ///    * @return a boolean indicating whether there are more rows or not.
    ///    *
    ///    * [deprecated] boolean step();
    ///    */
    /// /**
    ///    * Obtains the current list of named parameters, which are settable.  This
    ///    * property is only available to JavaScript consumers.
    ///    *
    ///    * readonly attribute mozIStorageStatementParams params;
    ///    */
    /// /**
    ///    * Obtains the current row, with access to all the data members by name.  This
    ///    * property is only available to JavaScript consumers.
    ///    *
    ///    * readonly attribute mozIStorageStatementRow row;
    ///    */
    /// /**
    ///    * These type values are returned by getTypeOfIndex
    ///    * to indicate what type of value is present at
    ///    * a given column.
    ///    */
    /// ```
    ///

    pub const VALUE_TYPE_NULL: i64 = 0;


    pub const VALUE_TYPE_INTEGER: i64 = 1;


    pub const VALUE_TYPE_FLOAT: i64 = 2;


    pub const VALUE_TYPE_TEXT: i64 = 3;


    pub const VALUE_TYPE_BLOB: i64 = 4;

    /// ```text
    /// /**
    ///    * Create a clone of this statement, by initializing a new statement
    ///    * with the same connection and same SQL statement as this one.  It
    ///    * does not preserve statement state; that is, if a statement is
    ///    * being executed when it is cloned, the new statement will not be
    ///    * executing.
    ///    */
    /// ```
    ///

    /// `mozIStorageStatement clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const mozIStorageStatement) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }



    /// `readonly attribute unsigned long parameterCount;`
    #[inline]
    pub unsafe fn GetParameterCount(&self, aParameterCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetParameterCount)(self, aParameterCount)
    }


    /// ```text
    /// /**
    ///    * Name of nth parameter, if given
    ///    */
    /// ```
    ///

    /// `AUTF8String getParameterName (in unsigned long aParamIndex);`
    #[inline]
    pub unsafe fn GetParameterName(&self, aParamIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetParameterName)(self, aParamIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the index of the named parameter.
    ///    *
    ///    * @param aName
    ///    *        The name of the parameter you want the index for.  This does not
    ///    *        include the leading ':'.
    ///    * @return the index of the named parameter.
    ///    */
    /// ```
    ///

    /// `unsigned long getParameterIndex (in AUTF8String aName);`
    #[inline]
    pub unsafe fn GetParameterIndex(&self, aName: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetParameterIndex)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * Number of columns returned
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long columnCount;`
    #[inline]
    pub unsafe fn GetColumnCount(&self, aColumnCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnCount)(self, aColumnCount)
    }


    /// ```text
    /// /**
    ///    * Name of nth column
    ///    */
    /// ```
    ///

    /// `AUTF8String getColumnName (in unsigned long aColumnIndex);`
    #[inline]
    pub unsafe fn GetColumnName(&self, aColumnIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnName)(self, aColumnIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Obtains the index of the column with the specified name.
    ///    *
    ///    * @param aName
    ///    *        The name of the column.
    ///    * @return The index of the column with the specified name.
    ///    */
    /// ```
    ///

    /// `unsigned long getColumnIndex (in AUTF8String aName);`
    #[inline]
    pub unsafe fn GetColumnIndex(&self, aName: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnIndex)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * Reset parameters/statement execution
    ///    */
    /// ```
    ///

    /// `void reset ();`
    #[inline]
    pub unsafe fn Reset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, )
    }


    /// ```text
    /// /**
    ///    * Execute the query, ignoring any results.  This is accomplished by
    ///    * calling executeStep() once, and then calling reset().
    ///    *
    ///    * Error and last insert info, etc. are available from
    ///    * the mozStorageConnection.
    ///    */
    /// ```
    ///

    /// `void execute ();`
    #[inline]
    pub unsafe fn Execute(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Execute)(self, )
    }


    /// ```text
    /// /**
    ///    * Execute a query, using any currently-bound parameters.  Reset
    ///    * must be called on the statement after the last call of
    ///    * executeStep.
    ///    *
    ///    * @return a boolean indicating whether there are more rows or not;
    ///    *         row data may be accessed using mozIStorageValueArray methods on
    ///    *         the statement.
    ///    */
    /// ```
    ///

    /// `boolean executeStep ();`
    #[inline]
    pub unsafe fn ExecuteStep(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExecuteStep)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * The number of entries in the array (each corresponding to a column in the
        ///    * database row)
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long numEntries;`
    #[inline]
    pub unsafe fn GetNumEntries(&self, aNumEntries: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumEntries)(self, aNumEntries)
    }


    /// ```text
    /// /**
    ///    * Indicate the data type of the current result row for the the given column.
    ///    * SQLite will perform type conversion if you ask for a value as a different
    ///    * type than it is stored as.
    ///    *
    ///    * @param aIndex
    ///    *        0-based column index.
    ///    * @return The type of the value at the given column index; one of
    ///    *         VALUE_TYPE_NULL, VALUE_TYPE_INTEGER, VALUE_TYPE_FLOAT,
    ///    *         VALUE_TYPE_TEXT, VALUE_TYPE_BLOB.
    ///    */
    /// ```
    ///

    /// `long getTypeOfIndex (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetTypeOfIndex(&self, aIndex: u32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetTypeOfIndex)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieve the contents of a column from the current result row as a
    ///    * variant.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return A variant with the type of the column value.
    ///    */
    /// ```
    ///

    /// `nsIVariant getVariant (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetVariant(&self, aIndex: u32, _retval: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetVariant)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieve the contents of a column from the current result row as an
    ///    * integer.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return Column value interpreted as an integer per type conversion rules.
    ///    * @{
        ///    */
        /// ```
        ///

        /// `long getInt32 (in unsigned long aIndex);`
        #[inline]
        pub unsafe fn GetInt32(&self, aIndex: u32, _retval: *mut i32) -> ::nserror::nsresult {
            ((*self.vtable).GetInt32)(self, aIndex, _retval)
        }



        /// `long long getInt64 (in unsigned long aIndex);`
        #[inline]
        pub unsafe fn GetInt64(&self, aIndex: u32, _retval: *mut i64) -> ::nserror::nsresult {
            ((*self.vtable).GetInt64)(self, aIndex, _retval)
        }


        /// ```text
        /// /** @} */
    /// /**
    ///    * Retrieve the contents of a column from the current result row as a
    ///    * floating point double.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return Column value interpreted as a double per type conversion rules.
    ///    */
    /// ```
    ///

    /// `double getDouble (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetDouble(&self, aIndex: u32, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDouble)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieve the contents of a column from the current result row as a
    ///    * string.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return The value for the result column interpreted as a string.  If the
    ///    *         stored value was NULL, you will get an empty string with IsVoid set
    ///    *         to distinguish it from an explicitly set empty string.
    ///    * @{
        ///    */
        /// ```
        ///

        /// `AUTF8String getUTF8String (in unsigned long aIndex);`
        #[inline]
        pub unsafe fn GetUTF8String(&self, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).GetUTF8String)(self, aIndex, _retval)
        }



        /// `AString getString (in unsigned long aIndex);`
        #[inline]
        pub unsafe fn GetString(&self, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetString)(self, aIndex, _retval)
        }


        /// ```text
        /// /** @} */
    /// /**
    ///    * Retrieve the contents of a column from the current result row as a
    ///    * blob.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @param[out] aDataSize
    ///    *             The number of bytes in the blob.
    ///    * @param[out] aData
    ///    *             The contents of the BLOB.  This will be NULL if aDataSize == 0.
    ///    */
    /// ```
    ///

    /// `void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData);`
    #[inline]
    pub unsafe fn GetBlob(&self, aIndex: u32, aDataSize: *mut u32, aData: *mut *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetBlob)(self, aIndex, aDataSize, aData)
    }


    /// ```text
    /// /**
    ///    * Retrieve the contents of a Blob column from the current result row as a
    ///    * string.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return The value for the result Blob column interpreted as a String.
    ///    *         No encoding conversion is performed.
    ///    */
    /// ```
    ///

    /// `AString getBlobAsString (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetBlobAsString(&self, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetBlobAsString)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieve the contents of a Blob column from the current result row as a
    ///    * UTF8 string.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return The value for the result Blob column interpreted as a UTF8 String.
    ///    *         No encoding conversion is performed.
    ///    */
    /// ```
    ///

    /// `AUTF8String getBlobAsUTF8String (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetBlobAsUTF8String(&self, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBlobAsUTF8String)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Check whether the given column in the current result row is NULL.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @return true if the value for the result column is null.
    ///    */
    /// ```
    ///

    /// `boolean getIsNull (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetIsNull(&self, aIndex: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsNull)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns a shared string pointer.
    ///    *
    ///    * @param aIndex
    ///    *        0-based colummn index.
    ///    * @param aByteLength
    ///    *        The number of bytes in the string or blob. This is the same as the
    ///    *        number of characters for UTF-8 strings, and twice the number of
    ///    *        characters for UTF-16 strings.
    ///    * @param aResult
    ///    *        A pointer to the string or blob.
    ///    */
    /// ```
    ///

    /// `[noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out string aResult);`
    #[inline]
    pub unsafe fn GetSharedUTF8String(&self, aIndex: u32, aByteLength: *mut u32, aResult: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetSharedUTF8String)(self, aIndex, aByteLength, aResult)
    }



    /// `[noscript] void getSharedString (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out wstring aResult);`
    #[inline]
    pub unsafe fn GetSharedString(&self, aIndex: u32, aByteLength: *mut u32, aResult: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).GetSharedString)(self, aIndex, aByteLength, aResult)
    }



    /// `[noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out octetPtr aResult);`
    const _GetSharedBlob: () = ();

}


