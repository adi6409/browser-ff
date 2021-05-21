//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBindingParams.idl
//


/// `interface mozIStorageBindingParams : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageBindingParams {
    vtable: *const mozIStorageBindingParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageBindingParams.
unsafe impl XpCom for mozIStorageBindingParams {
    const IID: nsIID = nsID(0x2d09f42f, 0x966e, 0x4663,
        [0xb4, 0xb3, 0xb0, 0xc8, 0x67, 0x6b, 0xf2, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageBindingParams {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageBindingParams.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageBindingParamsCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageBindingParams`.
    fn coerce_from(v: &mozIStorageBindingParams) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageBindingParamsCoerce for mozIStorageBindingParams {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParams) -> &Self {
        v
    }
}

impl mozIStorageBindingParams {
    /// Cast this `mozIStorageBindingParams` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageBindingParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageBindingParams {
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
impl<T: nsISupportsCoerce> mozIStorageBindingParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParams) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageBindingParams
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageBindingParamsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void bindByName (in AUTF8String aName, in nsIVariant aValue); */
    pub BindByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: *const nsIVariant) -> ::nserror::nsresult,

    /* [noscript] void bindUTF8StringByName (in AUTF8String aName, in AUTF8String aValue); */
    pub BindUTF8StringByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void bindStringByName (in AUTF8String aName, in AString aValue); */
    pub BindStringByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void bindDoubleByName (in AUTF8String aName, in double aValue); */
    pub BindDoubleByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: libc::c_double) -> ::nserror::nsresult,

    /* [noscript] void bindInt32ByName (in AUTF8String aName, in long aValue); */
    pub BindInt32ByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: i32) -> ::nserror::nsresult,

    /* [noscript] void bindInt64ByName (in AUTF8String aName, in long long aValue); */
    pub BindInt64ByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: i64) -> ::nserror::nsresult,

    /* [noscript] void bindNullByName (in AUTF8String aName); */
    pub BindNullByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [binaryname(BindBlobByName),noscript] void bindBlobByNameNoscript (in AUTF8String aName, [const] in octetPtr aValue, in unsigned long aValueSize); */
    /// Unable to generate binding because `native type uint8_t unsupported`
    pub BindBlobByName: *const ::libc::c_void,

    /* [binaryname(BindBlobArrayByName)] void bindBlobByName (in AUTF8String aName, in Array<octet> aValue); */
    pub BindBlobArrayByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: *const thin_vec::ThinVec<u8>) -> ::nserror::nsresult,

    /* void bindStringAsBlobByName (in AUTF8String aName, in AString aValue); */
    pub BindStringAsBlobByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void bindUTF8StringAsBlobByName (in AUTF8String aName, in AUTF8String aValue); */
    pub BindUTF8StringAsBlobByName: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void bindAdoptedBlobByName (in AUTF8String aName, in octetPtr aValue, in unsigned long aValueSize); */
    /// Unable to generate binding because `native type uint8_t unsupported`
    pub BindAdoptedBlobByName: *const ::libc::c_void,

    /* void bindByIndex (in unsigned long aIndex, in nsIVariant aValue); */
    pub BindByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: *const nsIVariant) -> ::nserror::nsresult,

    /* [noscript] void bindUTF8StringByIndex (in unsigned long aIndex, in AUTF8String aValue); */
    pub BindUTF8StringByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void bindStringByIndex (in unsigned long aIndex, in AString aValue); */
    pub BindStringByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void bindDoubleByIndex (in unsigned long aIndex, in double aValue); */
    pub BindDoubleByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: libc::c_double) -> ::nserror::nsresult,

    /* [noscript] void bindInt32ByIndex (in unsigned long aIndex, in long aValue); */
    pub BindInt32ByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: i32) -> ::nserror::nsresult,

    /* [noscript] void bindInt64ByIndex (in unsigned long aIndex, in long long aValue); */
    pub BindInt64ByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: i64) -> ::nserror::nsresult,

    /* [noscript] void bindNullByIndex (in unsigned long aIndex); */
    pub BindNullByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32) -> ::nserror::nsresult,

    /* [binaryname(BindBlobByIndex),noscript] void bindBlobByIndexNoscript (in unsigned long aIndex, [const] in octetPtr aValue, in unsigned long aValueSize); */
    /// Unable to generate binding because `native type uint8_t unsupported`
    pub BindBlobByIndex: *const ::libc::c_void,

    /* [binaryname(BindBlobArrayByIndex)] void bindBlobByIndex (in unsigned long aIndex, in Array<octet> aValue); */
    pub BindBlobArrayByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: *const thin_vec::ThinVec<u8>) -> ::nserror::nsresult,

    /* void bindStringAsBlobByIndex (in unsigned long aIndex, in AString aValue); */
    pub BindStringAsBlobByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void bindUTF8StringAsBlobByIndex (in unsigned long aIndex, in AUTF8String aValue); */
    pub BindUTF8StringAsBlobByIndex: unsafe extern "system" fn (this: *const mozIStorageBindingParams, aIndex: u32, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void bindAdoptedBlobByIndex (in unsigned long aIndex, in octetPtr aValue, in unsigned long aValueSize); */
    /// Unable to generate binding because `native type uint8_t unsupported`
    pub BindAdoptedBlobByIndex: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageBindingParams {

    /// ```text
    /// /**
    ///    * Binds aValue to the parameter with the name aName.
    ///    *
    ///    * @param aName
    ///    *        The name of the parameter to bind aValue to.
    ///    * @param aValue
    ///    *        The value to bind.
    ///    */
    /// ```
    ///

    /// `void bindByName (in AUTF8String aName, in nsIVariant aValue);`
    #[inline]
    pub unsafe fn BindByName(&self, aName: *const ::nsstring::nsACString, aValue: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).BindByName)(self, aName, aValue)
    }



    /// `[noscript] void bindUTF8StringByName (in AUTF8String aName, in AUTF8String aValue);`
    #[inline]
    pub unsafe fn BindUTF8StringByName(&self, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BindUTF8StringByName)(self, aName, aValue)
    }



    /// `[noscript] void bindStringByName (in AUTF8String aName, in AString aValue);`
    #[inline]
    pub unsafe fn BindStringByName(&self, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).BindStringByName)(self, aName, aValue)
    }



    /// `[noscript] void bindDoubleByName (in AUTF8String aName, in double aValue);`
    #[inline]
    pub unsafe fn BindDoubleByName(&self, aName: *const ::nsstring::nsACString, aValue: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).BindDoubleByName)(self, aName, aValue)
    }



    /// `[noscript] void bindInt32ByName (in AUTF8String aName, in long aValue);`
    #[inline]
    pub unsafe fn BindInt32ByName(&self, aName: *const ::nsstring::nsACString, aValue: i32) -> ::nserror::nsresult {
        ((*self.vtable).BindInt32ByName)(self, aName, aValue)
    }



    /// `[noscript] void bindInt64ByName (in AUTF8String aName, in long long aValue);`
    #[inline]
    pub unsafe fn BindInt64ByName(&self, aName: *const ::nsstring::nsACString, aValue: i64) -> ::nserror::nsresult {
        ((*self.vtable).BindInt64ByName)(self, aName, aValue)
    }



    /// `[noscript] void bindNullByName (in AUTF8String aName);`
    #[inline]
    pub unsafe fn BindNullByName(&self, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BindNullByName)(self, aName)
    }



    /// `[binaryname(BindBlobByName),noscript] void bindBlobByNameNoscript (in AUTF8String aName, [const] in octetPtr aValue, in unsigned long aValueSize);`
    const _BindBlobByName: () = ();


    /// `[binaryname(BindBlobArrayByName)] void bindBlobByName (in AUTF8String aName, in Array<octet> aValue);`
    #[inline]
    pub unsafe fn BindBlobArrayByName(&self, aName: *const ::nsstring::nsACString, aValue: *const thin_vec::ThinVec<u8>) -> ::nserror::nsresult {
        ((*self.vtable).BindBlobArrayByName)(self, aName, aValue)
    }



    /// `void bindStringAsBlobByName (in AUTF8String aName, in AString aValue);`
    #[inline]
    pub unsafe fn BindStringAsBlobByName(&self, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).BindStringAsBlobByName)(self, aName, aValue)
    }



    /// `void bindUTF8StringAsBlobByName (in AUTF8String aName, in AUTF8String aValue);`
    #[inline]
    pub unsafe fn BindUTF8StringAsBlobByName(&self, aName: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BindUTF8StringAsBlobByName)(self, aName, aValue)
    }



    /// `[noscript] void bindAdoptedBlobByName (in AUTF8String aName, in octetPtr aValue, in unsigned long aValueSize);`
    const _BindAdoptedBlobByName: () = ();

    /// ```text
    /// /**
    ///     * Binds aValue to the parameter with the index aIndex.
    ///     *
    ///     * @param aIndex
    ///     *        The zero-based index of the parameter to bind aValue to.
    ///     * @param aValue
    ///     *        The value to bind.
    ///     */
    /// ```
    ///

    /// `void bindByIndex (in unsigned long aIndex, in nsIVariant aValue);`
    #[inline]
    pub unsafe fn BindByIndex(&self, aIndex: u32, aValue: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).BindByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindUTF8StringByIndex (in unsigned long aIndex, in AUTF8String aValue);`
    #[inline]
    pub unsafe fn BindUTF8StringByIndex(&self, aIndex: u32, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BindUTF8StringByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindStringByIndex (in unsigned long aIndex, in AString aValue);`
    #[inline]
    pub unsafe fn BindStringByIndex(&self, aIndex: u32, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).BindStringByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindDoubleByIndex (in unsigned long aIndex, in double aValue);`
    #[inline]
    pub unsafe fn BindDoubleByIndex(&self, aIndex: u32, aValue: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).BindDoubleByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindInt32ByIndex (in unsigned long aIndex, in long aValue);`
    #[inline]
    pub unsafe fn BindInt32ByIndex(&self, aIndex: u32, aValue: i32) -> ::nserror::nsresult {
        ((*self.vtable).BindInt32ByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindInt64ByIndex (in unsigned long aIndex, in long long aValue);`
    #[inline]
    pub unsafe fn BindInt64ByIndex(&self, aIndex: u32, aValue: i64) -> ::nserror::nsresult {
        ((*self.vtable).BindInt64ByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindNullByIndex (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn BindNullByIndex(&self, aIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).BindNullByIndex)(self, aIndex)
    }



    /// `[binaryname(BindBlobByIndex),noscript] void bindBlobByIndexNoscript (in unsigned long aIndex, [const] in octetPtr aValue, in unsigned long aValueSize);`
    const _BindBlobByIndex: () = ();


    /// `[binaryname(BindBlobArrayByIndex)] void bindBlobByIndex (in unsigned long aIndex, in Array<octet> aValue);`
    #[inline]
    pub unsafe fn BindBlobArrayByIndex(&self, aIndex: u32, aValue: *const thin_vec::ThinVec<u8>) -> ::nserror::nsresult {
        ((*self.vtable).BindBlobArrayByIndex)(self, aIndex, aValue)
    }



    /// `void bindStringAsBlobByIndex (in unsigned long aIndex, in AString aValue);`
    #[inline]
    pub unsafe fn BindStringAsBlobByIndex(&self, aIndex: u32, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).BindStringAsBlobByIndex)(self, aIndex, aValue)
    }



    /// `void bindUTF8StringAsBlobByIndex (in unsigned long aIndex, in AUTF8String aValue);`
    #[inline]
    pub unsafe fn BindUTF8StringAsBlobByIndex(&self, aIndex: u32, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).BindUTF8StringAsBlobByIndex)(self, aIndex, aValue)
    }



    /// `[noscript] void bindAdoptedBlobByIndex (in unsigned long aIndex, in octetPtr aValue, in unsigned long aValueSize);`
    const _BindAdoptedBlobByIndex: () = ();

}


