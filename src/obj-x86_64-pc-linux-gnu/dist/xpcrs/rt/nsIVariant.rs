//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIVariant.idl
//


/// `interface nsIVariant : nsISupports`
///

/// ```text
/// /**
///  * XPConnect has magic to transparently convert between nsIVariant and JS types.
///  * We mark the interface [scriptable] so that JS can use methods
///  * that refer to this interface. But we mark all the methods and attributes
///  * [noscript] since any nsIVariant object will be automatically converted to a
///  * JS type anyway.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIVariant {
    vtable: *const nsIVariantVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIVariant.
unsafe impl XpCom for nsIVariant {
    const IID: nsIID = nsID(0x81e4c2de, 0xacac, 0x4ad6,
        [0x90, 0x1a, 0xb5, 0xfb, 0x1b, 0x85, 0x1a, 0x0d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIVariant {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIVariant.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIVariantCoerce {
    /// Cheaply cast a value of this type from a `nsIVariant`.
    fn coerce_from(v: &nsIVariant) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIVariantCoerce for nsIVariant {
    #[inline]
    fn coerce_from(v: &nsIVariant) -> &Self {
        v
    }
}

impl nsIVariant {
    /// Cast this `nsIVariant` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIVariantCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIVariant {
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
impl<T: nsISupportsCoerce> nsIVariantCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIVariant) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIVariant
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIVariantVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [nostdcall,notxpcom] readonly attribute uint16_t dataType; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetDataType: *const ::libc::c_void,

    /* [noscript] uint8_t getAsInt8 (); */
    pub GetAsInt8: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut uint8_t) -> ::nserror::nsresult,

    /* [noscript] int16_t getAsInt16 (); */
    pub GetAsInt16: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut int16_t) -> ::nserror::nsresult,

    /* [noscript] int32_t getAsInt32 (); */
    pub GetAsInt32: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut int32_t) -> ::nserror::nsresult,

    /* [noscript] int64_t getAsInt64 (); */
    pub GetAsInt64: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut int64_t) -> ::nserror::nsresult,

    /* [noscript] uint8_t getAsUint8 (); */
    pub GetAsUint8: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut uint8_t) -> ::nserror::nsresult,

    /* [noscript] uint16_t getAsUint16 (); */
    pub GetAsUint16: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut uint16_t) -> ::nserror::nsresult,

    /* [noscript] uint32_t getAsUint32 (); */
    pub GetAsUint32: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [noscript] uint64_t getAsUint64 (); */
    pub GetAsUint64: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut uint64_t) -> ::nserror::nsresult,

    /* [noscript] float getAsFloat (); */
    pub GetAsFloat: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut libc::c_float) -> ::nserror::nsresult,

    /* [noscript] double getAsDouble (); */
    pub GetAsDouble: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* [noscript] boolean getAsBool (); */
    pub GetAsBool: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] char getAsChar (); */
    pub GetAsChar: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut libc::c_char) -> ::nserror::nsresult,

    /* [noscript] wchar getAsWChar (); */
    pub GetAsWChar: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut i16) -> ::nserror::nsresult,

    /* [notxpcom] nsresult getAsID (out nsID retval); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetAsID: *const ::libc::c_void,

    /* [noscript] AString getAsAString (); */
    pub GetAsAString: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] ACString getAsACString (); */
    pub GetAsACString: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] AUTF8String getAsAUTF8String (); */
    pub GetAsAUTF8String: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] string getAsString (); */
    pub GetAsString: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript] wstring getAsWString (); */
    pub GetAsWString: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut *const i16) -> ::nserror::nsresult,

    /* [noscript] nsISupports getAsISupports (); */
    pub GetAsISupports: unsafe extern "system" fn (this: *const nsIVariant, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [noscript] jsval getAsJSVal (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetAsJSVal: *const ::libc::c_void,

    /* [noscript] void getAsInterface (out nsIIDPtr iid, [iid_is (iid), retval] out nsQIResult iface); */
    pub GetAsInterface: unsafe extern "system" fn (this: *const nsIVariant, iid: *mut *mut nsIID, iface: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* [notxpcom] nsresult getAsArray (out uint16_t type, out nsIID iid, out uint32_t count, out voidPtr ptr); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetAsArray: *const ::libc::c_void,

    /* [noscript] void getAsStringWithSize (out uint32_t size, [size_is (size), retval] out string str); */
    pub GetAsStringWithSize: unsafe extern "system" fn (this: *const nsIVariant, size: *mut uint32_t, str: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript] void getAsWStringWithSize (out uint32_t size, [size_is (size), retval] out wstring str); */
    pub GetAsWStringWithSize: unsafe extern "system" fn (this: *const nsIVariant, size: *mut uint32_t, str: *mut *const i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIVariant {


    /// `[nostdcall,notxpcom] readonly attribute uint16_t dataType;`
    const _GetDataType: () = ();


    /// `[noscript] uint8_t getAsInt8 ();`
    #[inline]
    pub unsafe fn GetAsInt8(&self, _retval: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsInt8)(self, _retval)
    }



    /// `[noscript] int16_t getAsInt16 ();`
    #[inline]
    pub unsafe fn GetAsInt16(&self, _retval: *mut int16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsInt16)(self, _retval)
    }



    /// `[noscript] int32_t getAsInt32 ();`
    #[inline]
    pub unsafe fn GetAsInt32(&self, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsInt32)(self, _retval)
    }



    /// `[noscript] int64_t getAsInt64 ();`
    #[inline]
    pub unsafe fn GetAsInt64(&self, _retval: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsInt64)(self, _retval)
    }



    /// `[noscript] uint8_t getAsUint8 ();`
    #[inline]
    pub unsafe fn GetAsUint8(&self, _retval: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsUint8)(self, _retval)
    }



    /// `[noscript] uint16_t getAsUint16 ();`
    #[inline]
    pub unsafe fn GetAsUint16(&self, _retval: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsUint16)(self, _retval)
    }



    /// `[noscript] uint32_t getAsUint32 ();`
    #[inline]
    pub unsafe fn GetAsUint32(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsUint32)(self, _retval)
    }



    /// `[noscript] uint64_t getAsUint64 ();`
    #[inline]
    pub unsafe fn GetAsUint64(&self, _retval: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAsUint64)(self, _retval)
    }



    /// `[noscript] float getAsFloat ();`
    #[inline]
    pub unsafe fn GetAsFloat(&self, _retval: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetAsFloat)(self, _retval)
    }



    /// `[noscript] double getAsDouble ();`
    #[inline]
    pub unsafe fn GetAsDouble(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetAsDouble)(self, _retval)
    }



    /// `[noscript] boolean getAsBool ();`
    #[inline]
    pub unsafe fn GetAsBool(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAsBool)(self, _retval)
    }



    /// `[noscript] char getAsChar ();`
    #[inline]
    pub unsafe fn GetAsChar(&self, _retval: *mut libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetAsChar)(self, _retval)
    }



    /// `[noscript] wchar getAsWChar ();`
    #[inline]
    pub unsafe fn GetAsWChar(&self, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetAsWChar)(self, _retval)
    }



    /// `[notxpcom] nsresult getAsID (out nsID retval);`
    const _GetAsID: () = ();


    /// `[noscript] AString getAsAString ();`
    #[inline]
    pub unsafe fn GetAsAString(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsAString)(self, _retval)
    }



    /// `[noscript] ACString getAsACString ();`
    #[inline]
    pub unsafe fn GetAsACString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsACString)(self, _retval)
    }



    /// `[noscript] AUTF8String getAsAUTF8String ();`
    #[inline]
    pub unsafe fn GetAsAUTF8String(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsAUTF8String)(self, _retval)
    }



    /// `[noscript] string getAsString ();`
    #[inline]
    pub unsafe fn GetAsString(&self, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetAsString)(self, _retval)
    }



    /// `[noscript] wstring getAsWString ();`
    #[inline]
    pub unsafe fn GetAsWString(&self, _retval: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).GetAsWString)(self, _retval)
    }



    /// `[noscript] nsISupports getAsISupports ();`
    #[inline]
    pub unsafe fn GetAsISupports(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetAsISupports)(self, _retval)
    }



    /// `[noscript] jsval getAsJSVal ();`
    const _GetAsJSVal: () = ();


    /// `[noscript] void getAsInterface (out nsIIDPtr iid, [iid_is (iid), retval] out nsQIResult iface);`
    #[inline]
    pub unsafe fn GetAsInterface(&self, iid: *mut *mut nsIID, iface: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetAsInterface)(self, iid, iface)
    }



    /// `[notxpcom] nsresult getAsArray (out uint16_t type, out nsIID iid, out uint32_t count, out voidPtr ptr);`
    const _GetAsArray: () = ();


    /// `[noscript] void getAsStringWithSize (out uint32_t size, [size_is (size), retval] out string str);`
    #[inline]
    pub unsafe fn GetAsStringWithSize(&self, size: *mut uint32_t, str: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetAsStringWithSize)(self, size, str)
    }



    /// `[noscript] void getAsWStringWithSize (out uint32_t size, [size_is (size), retval] out wstring str);`
    #[inline]
    pub unsafe fn GetAsWStringWithSize(&self, size: *mut uint32_t, str: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).GetAsWStringWithSize)(self, size, str)
    }


}


/// `interface nsIWritableVariant : nsIVariant`
///

/// ```text
/// /**
///  * An object that implements nsIVariant may or may NOT also implement this
///  * nsIWritableVariant.
///  *
///  * If the 'writable' attribute is false then attempts to call any of the 'set'
///  * methods can be expected to fail. Setting the 'writable' attribute may or
///  * may not succeed.
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWritableVariant {
    vtable: *const nsIWritableVariantVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWritableVariant.
unsafe impl XpCom for nsIWritableVariant {
    const IID: nsIID = nsID(0x5586a590, 0x8c82, 0x11d5,
        [0x90, 0xf3, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWritableVariant {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWritableVariant.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWritableVariantCoerce {
    /// Cheaply cast a value of this type from a `nsIWritableVariant`.
    fn coerce_from(v: &nsIWritableVariant) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWritableVariantCoerce for nsIWritableVariant {
    #[inline]
    fn coerce_from(v: &nsIWritableVariant) -> &Self {
        v
    }
}

impl nsIWritableVariant {
    /// Cast this `nsIWritableVariant` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWritableVariantCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWritableVariant {
    type Target = nsIVariant;
    #[inline]
    fn deref(&self) -> &nsIVariant {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIVariantCoerce> nsIWritableVariantCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWritableVariant) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWritableVariant
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWritableVariantVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIVariantVTable,

    /* attribute boolean writable; */
    pub GetWritable: unsafe extern "system" fn (this: *const nsIWritableVariant, aWritable: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean writable; */
    pub SetWritable: unsafe extern "system" fn (this: *const nsIWritableVariant, aWritable: bool) -> ::nserror::nsresult,

    /* void setAsInt8 (in uint8_t aValue); */
    pub SetAsInt8: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: uint8_t) -> ::nserror::nsresult,

    /* void setAsInt16 (in int16_t aValue); */
    pub SetAsInt16: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: int16_t) -> ::nserror::nsresult,

    /* void setAsInt32 (in int32_t aValue); */
    pub SetAsInt32: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: int32_t) -> ::nserror::nsresult,

    /* void setAsInt64 (in int64_t aValue); */
    pub SetAsInt64: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: int64_t) -> ::nserror::nsresult,

    /* void setAsUint8 (in uint8_t aValue); */
    pub SetAsUint8: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: uint8_t) -> ::nserror::nsresult,

    /* void setAsUint16 (in uint16_t aValue); */
    pub SetAsUint16: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: uint16_t) -> ::nserror::nsresult,

    /* void setAsUint32 (in uint32_t aValue); */
    pub SetAsUint32: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: uint32_t) -> ::nserror::nsresult,

    /* void setAsUint64 (in uint64_t aValue); */
    pub SetAsUint64: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: uint64_t) -> ::nserror::nsresult,

    /* void setAsFloat (in float aValue); */
    pub SetAsFloat: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: libc::c_float) -> ::nserror::nsresult,

    /* void setAsDouble (in double aValue); */
    pub SetAsDouble: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: libc::c_double) -> ::nserror::nsresult,

    /* void setAsBool (in boolean aValue); */
    pub SetAsBool: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: bool) -> ::nserror::nsresult,

    /* void setAsChar (in char aValue); */
    pub SetAsChar: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: libc::c_char) -> ::nserror::nsresult,

    /* void setAsWChar (in wchar aValue); */
    pub SetAsWChar: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: i16) -> ::nserror::nsresult,

    /* void setAsID (in nsIDRef aValue); */
    pub SetAsID: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const nsID) -> ::nserror::nsresult,

    /* void setAsAString (in AString aValue); */
    pub SetAsAString: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setAsACString (in ACString aValue); */
    pub SetAsACString: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setAsAUTF8String (in AUTF8String aValue); */
    pub SetAsAUTF8String: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setAsString (in string aValue); */
    pub SetAsString: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const libc::c_char) -> ::nserror::nsresult,

    /* void setAsWString (in wstring aValue); */
    pub SetAsWString: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const i16) -> ::nserror::nsresult,

    /* void setAsISupports (in nsISupports aValue); */
    pub SetAsISupports: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const nsISupports) -> ::nserror::nsresult,

    /* void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface); */
    pub SetAsInterface: unsafe extern "system" fn (this: *const nsIWritableVariant, iid: *const nsIID, iface: *const libc::c_void) -> ::nserror::nsresult,

    /* [noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr); */
    pub SetAsArray: unsafe extern "system" fn (this: *const nsIWritableVariant, type_: uint16_t, iid: *const nsIID, count: uint32_t, ptr: *const libc::c_void) -> ::nserror::nsresult,

    /* void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str); */
    pub SetAsStringWithSize: unsafe extern "system" fn (this: *const nsIWritableVariant, size: uint32_t, str: *const libc::c_char) -> ::nserror::nsresult,

    /* void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str); */
    pub SetAsWStringWithSize: unsafe extern "system" fn (this: *const nsIWritableVariant, size: uint32_t, str: *const i16) -> ::nserror::nsresult,

    /* void setAsVoid (); */
    pub SetAsVoid: unsafe extern "system" fn (this: *const nsIWritableVariant) -> ::nserror::nsresult,

    /* void setAsEmpty (); */
    pub SetAsEmpty: unsafe extern "system" fn (this: *const nsIWritableVariant) -> ::nserror::nsresult,

    /* void setAsEmptyArray (); */
    pub SetAsEmptyArray: unsafe extern "system" fn (this: *const nsIWritableVariant) -> ::nserror::nsresult,

    /* void setFromVariant (in nsIVariant aValue); */
    pub SetFromVariant: unsafe extern "system" fn (this: *const nsIWritableVariant, aValue: *const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWritableVariant {


    /// `attribute boolean writable;`
    #[inline]
    pub unsafe fn GetWritable(&self, aWritable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWritable)(self, aWritable)
    }



    /// `attribute boolean writable;`
    #[inline]
    pub unsafe fn SetWritable(&self, aWritable: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetWritable)(self, aWritable)
    }



    /// `void setAsInt8 (in uint8_t aValue);`
    #[inline]
    pub unsafe fn SetAsInt8(&self, aValue: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsInt8)(self, aValue)
    }



    /// `void setAsInt16 (in int16_t aValue);`
    #[inline]
    pub unsafe fn SetAsInt16(&self, aValue: int16_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsInt16)(self, aValue)
    }



    /// `void setAsInt32 (in int32_t aValue);`
    #[inline]
    pub unsafe fn SetAsInt32(&self, aValue: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsInt32)(self, aValue)
    }



    /// `void setAsInt64 (in int64_t aValue);`
    #[inline]
    pub unsafe fn SetAsInt64(&self, aValue: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsInt64)(self, aValue)
    }



    /// `void setAsUint8 (in uint8_t aValue);`
    #[inline]
    pub unsafe fn SetAsUint8(&self, aValue: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsUint8)(self, aValue)
    }



    /// `void setAsUint16 (in uint16_t aValue);`
    #[inline]
    pub unsafe fn SetAsUint16(&self, aValue: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsUint16)(self, aValue)
    }



    /// `void setAsUint32 (in uint32_t aValue);`
    #[inline]
    pub unsafe fn SetAsUint32(&self, aValue: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsUint32)(self, aValue)
    }



    /// `void setAsUint64 (in uint64_t aValue);`
    #[inline]
    pub unsafe fn SetAsUint64(&self, aValue: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetAsUint64)(self, aValue)
    }



    /// `void setAsFloat (in float aValue);`
    #[inline]
    pub unsafe fn SetAsFloat(&self, aValue: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetAsFloat)(self, aValue)
    }



    /// `void setAsDouble (in double aValue);`
    #[inline]
    pub unsafe fn SetAsDouble(&self, aValue: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetAsDouble)(self, aValue)
    }



    /// `void setAsBool (in boolean aValue);`
    #[inline]
    pub unsafe fn SetAsBool(&self, aValue: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAsBool)(self, aValue)
    }



    /// `void setAsChar (in char aValue);`
    #[inline]
    pub unsafe fn SetAsChar(&self, aValue: libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetAsChar)(self, aValue)
    }



    /// `void setAsWChar (in wchar aValue);`
    #[inline]
    pub unsafe fn SetAsWChar(&self, aValue: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetAsWChar)(self, aValue)
    }



    /// `void setAsID (in nsIDRef aValue);`
    #[inline]
    pub unsafe fn SetAsID(&self, aValue: *const nsID) -> ::nserror::nsresult {
        ((*self.vtable).SetAsID)(self, aValue)
    }



    /// `void setAsAString (in AString aValue);`
    #[inline]
    pub unsafe fn SetAsAString(&self, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetAsAString)(self, aValue)
    }



    /// `void setAsACString (in ACString aValue);`
    #[inline]
    pub unsafe fn SetAsACString(&self, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAsACString)(self, aValue)
    }



    /// `void setAsAUTF8String (in AUTF8String aValue);`
    #[inline]
    pub unsafe fn SetAsAUTF8String(&self, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAsAUTF8String)(self, aValue)
    }



    /// `void setAsString (in string aValue);`
    #[inline]
    pub unsafe fn SetAsString(&self, aValue: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetAsString)(self, aValue)
    }



    /// `void setAsWString (in wstring aValue);`
    #[inline]
    pub unsafe fn SetAsWString(&self, aValue: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).SetAsWString)(self, aValue)
    }



    /// `void setAsISupports (in nsISupports aValue);`
    #[inline]
    pub unsafe fn SetAsISupports(&self, aValue: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetAsISupports)(self, aValue)
    }



    /// `void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface);`
    #[inline]
    pub unsafe fn SetAsInterface(&self, iid: *const nsIID, iface: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetAsInterface)(self, iid, iface)
    }



    /// `[noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr);`
    #[inline]
    pub unsafe fn SetAsArray(&self, type_: uint16_t, iid: *const nsIID, count: uint32_t, ptr: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetAsArray)(self, type_, iid, count, ptr)
    }



    /// `void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str);`
    #[inline]
    pub unsafe fn SetAsStringWithSize(&self, size: uint32_t, str: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetAsStringWithSize)(self, size, str)
    }



    /// `void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str);`
    #[inline]
    pub unsafe fn SetAsWStringWithSize(&self, size: uint32_t, str: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).SetAsWStringWithSize)(self, size, str)
    }



    /// `void setAsVoid ();`
    #[inline]
    pub unsafe fn SetAsVoid(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetAsVoid)(self, )
    }



    /// `void setAsEmpty ();`
    #[inline]
    pub unsafe fn SetAsEmpty(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetAsEmpty)(self, )
    }



    /// `void setAsEmptyArray ();`
    #[inline]
    pub unsafe fn SetAsEmptyArray(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetAsEmptyArray)(self, )
    }



    /// `void setFromVariant (in nsIVariant aValue);`
    #[inline]
    pub unsafe fn SetFromVariant(&self, aValue: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).SetFromVariant)(self, aValue)
    }


}


