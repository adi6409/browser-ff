//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsICommandParams.idl
//


/// `interface nsICommandParams : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandParams {
    vtable: *const nsICommandParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandParams.
unsafe impl XpCom for nsICommandParams {
    const IID: nsIID = nsID(0xb1fdf3c4, 0x74e3, 0x4f7d,
        [0xa1, 0x4d, 0x2b, 0x76, 0xbc, 0xf5, 0x34, 0x82]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandParams {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandParams.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandParamsCoerce {
    /// Cheaply cast a value of this type from a `nsICommandParams`.
    fn coerce_from(v: &nsICommandParams) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandParamsCoerce for nsICommandParams {
    #[inline]
    fn coerce_from(v: &nsICommandParams) -> &Self {
        v
    }
}

impl nsICommandParams {
    /// Cast this `nsICommandParams` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandParams {
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
impl<T: nsISupportsCoerce> nsICommandParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandParams) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandParams
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandParamsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* short getValueType (in string name); */
    pub GetValueType: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut i16) -> ::nserror::nsresult,

    /* boolean getBooleanValue (in string name); */
    pub GetBooleanValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* long getLongValue (in string name); */
    pub GetLongValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut i32) -> ::nserror::nsresult,

    /* double getDoubleValue (in string name); */
    pub GetDoubleValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* AString getStringValue (in string name); */
    pub GetStringValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* ACString getCStringValue (in string name); */
    pub GetCStringValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsISupports getISupportsValue (in string name); */
    pub GetISupportsValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void setBooleanValue (in string name, in boolean value); */
    pub SetBooleanValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, value: bool) -> ::nserror::nsresult,

    /* void setLongValue (in string name, in long value); */
    pub SetLongValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, value: i32) -> ::nserror::nsresult,

    /* void setDoubleValue (in string name, in double value); */
    pub SetDoubleValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, value: libc::c_double) -> ::nserror::nsresult,

    /* void setStringValue (in string name, in AString value); */
    pub SetStringValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setCStringValue (in string name, in ACString value); */
    pub SetCStringValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setISupportsValue (in string name, in nsISupports value); */
    pub SetISupportsValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char, value: *const nsISupports) -> ::nserror::nsresult,

    /* void removeValue (in string name); */
    pub RemoveValue: unsafe extern "system" fn (this: *const nsICommandParams, name: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandParams {

    pub const eNoType: i64 = 0;


    pub const eBooleanType: i64 = 1;


    pub const eLongType: i64 = 2;


    pub const eDoubleType: i64 = 3;


    pub const eWStringType: i64 = 4;


    pub const eISupportsType: i64 = 5;


    pub const eStringType: i64 = 6;


    /// `short getValueType (in string name);`
    #[inline]
    pub unsafe fn GetValueType(&self, name: *const libc::c_char, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetValueType)(self, name, _retval)
    }



    /// `boolean getBooleanValue (in string name);`
    #[inline]
    pub unsafe fn GetBooleanValue(&self, name: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBooleanValue)(self, name, _retval)
    }



    /// `long getLongValue (in string name);`
    #[inline]
    pub unsafe fn GetLongValue(&self, name: *const libc::c_char, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLongValue)(self, name, _retval)
    }



    /// `double getDoubleValue (in string name);`
    #[inline]
    pub unsafe fn GetDoubleValue(&self, name: *const libc::c_char, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDoubleValue)(self, name, _retval)
    }



    /// `AString getStringValue (in string name);`
    #[inline]
    pub unsafe fn GetStringValue(&self, name: *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringValue)(self, name, _retval)
    }



    /// `ACString getCStringValue (in string name);`
    #[inline]
    pub unsafe fn GetCStringValue(&self, name: *const libc::c_char, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCStringValue)(self, name, _retval)
    }



    /// `nsISupports getISupportsValue (in string name);`
    #[inline]
    pub unsafe fn GetISupportsValue(&self, name: *const libc::c_char, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetISupportsValue)(self, name, _retval)
    }



    /// `void setBooleanValue (in string name, in boolean value);`
    #[inline]
    pub unsafe fn SetBooleanValue(&self, name: *const libc::c_char, value: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetBooleanValue)(self, name, value)
    }



    /// `void setLongValue (in string name, in long value);`
    #[inline]
    pub unsafe fn SetLongValue(&self, name: *const libc::c_char, value: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetLongValue)(self, name, value)
    }



    /// `void setDoubleValue (in string name, in double value);`
    #[inline]
    pub unsafe fn SetDoubleValue(&self, name: *const libc::c_char, value: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetDoubleValue)(self, name, value)
    }



    /// `void setStringValue (in string name, in AString value);`
    #[inline]
    pub unsafe fn SetStringValue(&self, name: *const libc::c_char, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetStringValue)(self, name, value)
    }



    /// `void setCStringValue (in string name, in ACString value);`
    #[inline]
    pub unsafe fn SetCStringValue(&self, name: *const libc::c_char, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCStringValue)(self, name, value)
    }



    /// `void setISupportsValue (in string name, in nsISupports value);`
    #[inline]
    pub unsafe fn SetISupportsValue(&self, name: *const libc::c_char, value: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetISupportsValue)(self, name, value)
    }



    /// `void removeValue (in string name);`
    #[inline]
    pub unsafe fn RemoveValue(&self, name: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RemoveValue)(self, name)
    }


}


