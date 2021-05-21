//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageValueArray.idl
//


/// `interface mozIStorageValueArray : nsISupports`
///

/// ```text
/// /**
///  * mozIStorageValueArray wraps an array of SQL values, such as a single database
///  * row.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageValueArray {
    vtable: *const mozIStorageValueArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageValueArray.
unsafe impl XpCom for mozIStorageValueArray {
    const IID: nsIID = nsID(0x6e6306f4, 0xffa7, 0x40f5,
        [0x96, 0xca, 0x36, 0x15, 0x9c, 0xe8, 0xf4, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageValueArray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageValueArray.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageValueArrayCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageValueArray`.
    fn coerce_from(v: &mozIStorageValueArray) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageValueArrayCoerce for mozIStorageValueArray {
    #[inline]
    fn coerce_from(v: &mozIStorageValueArray) -> &Self {
        v
    }
}

impl mozIStorageValueArray {
    /// Cast this `mozIStorageValueArray` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageValueArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageValueArray {
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
impl<T: nsISupportsCoerce> mozIStorageValueArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageValueArray) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageValueArray
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageValueArrayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long numEntries; */
    pub GetNumEntries: unsafe extern "system" fn (this: *const mozIStorageValueArray, aNumEntries: *mut u32) -> ::nserror::nsresult,

    /* long getTypeOfIndex (in unsigned long aIndex); */
    pub GetTypeOfIndex: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut i32) -> ::nserror::nsresult,

    /* long getInt32 (in unsigned long aIndex); */
    pub GetInt32: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut i32) -> ::nserror::nsresult,

    /* long long getInt64 (in unsigned long aIndex); */
    pub GetInt64: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut i64) -> ::nserror::nsresult,

    /* double getDouble (in unsigned long aIndex); */
    pub GetDouble: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* AUTF8String getUTF8String (in unsigned long aIndex); */
    pub GetUTF8String: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AString getString (in unsigned long aIndex); */
    pub GetString: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData); */
    pub GetBlob: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, aDataSize: *mut u32, aData: *mut *mut u8) -> ::nserror::nsresult,

    /* AString getBlobAsString (in unsigned long aIndex); */
    pub GetBlobAsString: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AUTF8String getBlobAsUTF8String (in unsigned long aIndex); */
    pub GetBlobAsUTF8String: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean getIsNull (in unsigned long aIndex); */
    pub GetIsNull: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out string aResult); */
    pub GetSharedUTF8String: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, aByteLength: *mut u32, aResult: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript] void getSharedString (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out wstring aResult); */
    pub GetSharedString: unsafe extern "system" fn (this: *const mozIStorageValueArray, aIndex: u32, aByteLength: *mut u32, aResult: *mut *const i16) -> ::nserror::nsresult,

    /* [noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aByteLength, [shared, retval] out octetPtr aResult); */
    /// Unable to generate binding because `native type uint8_t unsupported`
    pub GetSharedBlob: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageValueArray {
    /// ```text
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
    ///    * numEntries
    ///    *
    ///    * number of entries in the array (each corresponding to a column
        ///    * in the database row)
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
    ///    * Returns the type of the value at the given column index;
    ///    * one of VALUE_TYPE_NULL, VALUE_TYPE_INTEGER, VALUE_TYPE_FLOAT,
    ///    * VALUE_TYPE_TEXT, VALUE_TYPE_BLOB.
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
    ///    * Obtain a value for the given entry (column) index.
    ///    * Due to SQLite's type conversion rules, any of these are valid
    ///    * for any column regardless of the column's data type.  However,
    ///    * if the specific type matters, getTypeOfIndex should be used
    ///    * first to identify the column type, and then the appropriate
    ///    * get method should be called.
    ///    *
    ///    * If you ask for a string value for a NULL column, you will get an empty
    ///    * string with IsVoid set to distinguish it from an explicitly set empty
    ///    * string.
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



    /// `double getDouble (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetDouble(&self, aIndex: u32, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDouble)(self, aIndex, _retval)
    }



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



    /// `void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData);`
    #[inline]
    pub unsafe fn GetBlob(&self, aIndex: u32, aDataSize: *mut u32, aData: *mut *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetBlob)(self, aIndex, aDataSize, aData)
    }



    /// `AString getBlobAsString (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetBlobAsString(&self, aIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetBlobAsString)(self, aIndex, _retval)
    }



    /// `AUTF8String getBlobAsUTF8String (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetBlobAsUTF8String(&self, aIndex: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBlobAsUTF8String)(self, aIndex, _retval)
    }



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


