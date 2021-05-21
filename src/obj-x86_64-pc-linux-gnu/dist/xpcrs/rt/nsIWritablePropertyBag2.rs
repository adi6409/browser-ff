//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIWritablePropertyBag2.idl
//


/// `interface nsIWritablePropertyBag2 : nsIPropertyBag2`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWritablePropertyBag2 {
    vtable: *const nsIWritablePropertyBag2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWritablePropertyBag2.
unsafe impl XpCom for nsIWritablePropertyBag2 {
    const IID: nsIID = nsID(0x9cfd1587, 0x360e, 0x4957,
        [0xa5, 0x8f, 0x4c, 0x2b, 0x1c, 0x5e, 0x7e, 0xd9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWritablePropertyBag2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWritablePropertyBag2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWritablePropertyBag2Coerce {
    /// Cheaply cast a value of this type from a `nsIWritablePropertyBag2`.
    fn coerce_from(v: &nsIWritablePropertyBag2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWritablePropertyBag2Coerce for nsIWritablePropertyBag2 {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag2) -> &Self {
        v
    }
}

impl nsIWritablePropertyBag2 {
    /// Cast this `nsIWritablePropertyBag2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWritablePropertyBag2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWritablePropertyBag2 {
    type Target = nsIPropertyBag2;
    #[inline]
    fn deref(&self) -> &nsIPropertyBag2 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPropertyBag2Coerce> nsIWritablePropertyBag2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWritablePropertyBag2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWritablePropertyBag2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPropertyBag2VTable,

    /* void setPropertyAsInt32 (in AString prop, in int32_t value); */
    pub SetPropertyAsInt32: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: int32_t) -> ::nserror::nsresult,

    /* void setPropertyAsUint32 (in AString prop, in uint32_t value); */
    pub SetPropertyAsUint32: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: uint32_t) -> ::nserror::nsresult,

    /* void setPropertyAsInt64 (in AString prop, in int64_t value); */
    pub SetPropertyAsInt64: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: int64_t) -> ::nserror::nsresult,

    /* void setPropertyAsUint64 (in AString prop, in uint64_t value); */
    pub SetPropertyAsUint64: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: uint64_t) -> ::nserror::nsresult,

    /* void setPropertyAsDouble (in AString prop, in double value); */
    pub SetPropertyAsDouble: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: libc::c_double) -> ::nserror::nsresult,

    /* void setPropertyAsAString (in AString prop, in AString value); */
    pub SetPropertyAsAString: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setPropertyAsACString (in AString prop, in ACString value); */
    pub SetPropertyAsACString: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setPropertyAsAUTF8String (in AString prop, in AUTF8String value); */
    pub SetPropertyAsAUTF8String: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setPropertyAsBool (in AString prop, in boolean value); */
    pub SetPropertyAsBool: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: bool) -> ::nserror::nsresult,

    /* void setPropertyAsInterface (in AString prop, in nsISupports value); */
    pub SetPropertyAsInterface: unsafe extern "system" fn (this: *const nsIWritablePropertyBag2, prop: *const ::nsstring::nsAString, value: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWritablePropertyBag2 {


    /// `void setPropertyAsInt32 (in AString prop, in int32_t value);`
    #[inline]
    pub unsafe fn SetPropertyAsInt32(&self, prop: *const ::nsstring::nsAString, value: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsInt32)(self, prop, value)
    }



    /// `void setPropertyAsUint32 (in AString prop, in uint32_t value);`
    #[inline]
    pub unsafe fn SetPropertyAsUint32(&self, prop: *const ::nsstring::nsAString, value: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsUint32)(self, prop, value)
    }



    /// `void setPropertyAsInt64 (in AString prop, in int64_t value);`
    #[inline]
    pub unsafe fn SetPropertyAsInt64(&self, prop: *const ::nsstring::nsAString, value: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsInt64)(self, prop, value)
    }



    /// `void setPropertyAsUint64 (in AString prop, in uint64_t value);`
    #[inline]
    pub unsafe fn SetPropertyAsUint64(&self, prop: *const ::nsstring::nsAString, value: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsUint64)(self, prop, value)
    }



    /// `void setPropertyAsDouble (in AString prop, in double value);`
    #[inline]
    pub unsafe fn SetPropertyAsDouble(&self, prop: *const ::nsstring::nsAString, value: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsDouble)(self, prop, value)
    }



    /// `void setPropertyAsAString (in AString prop, in AString value);`
    #[inline]
    pub unsafe fn SetPropertyAsAString(&self, prop: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsAString)(self, prop, value)
    }



    /// `void setPropertyAsACString (in AString prop, in ACString value);`
    #[inline]
    pub unsafe fn SetPropertyAsACString(&self, prop: *const ::nsstring::nsAString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsACString)(self, prop, value)
    }



    /// `void setPropertyAsAUTF8String (in AString prop, in AUTF8String value);`
    #[inline]
    pub unsafe fn SetPropertyAsAUTF8String(&self, prop: *const ::nsstring::nsAString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsAUTF8String)(self, prop, value)
    }



    /// `void setPropertyAsBool (in AString prop, in boolean value);`
    #[inline]
    pub unsafe fn SetPropertyAsBool(&self, prop: *const ::nsstring::nsAString, value: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsBool)(self, prop, value)
    }



    /// `void setPropertyAsInterface (in AString prop, in nsISupports value);`
    #[inline]
    pub unsafe fn SetPropertyAsInterface(&self, prop: *const ::nsstring::nsAString, value: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetPropertyAsInterface)(self, prop, value)
    }


}


