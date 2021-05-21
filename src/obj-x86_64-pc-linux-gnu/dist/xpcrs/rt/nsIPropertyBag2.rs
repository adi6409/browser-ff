//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPropertyBag2.idl
//


/// `interface nsIPropertyBag2 : nsIPropertyBag`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPropertyBag2 {
    vtable: *const nsIPropertyBag2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPropertyBag2.
unsafe impl XpCom for nsIPropertyBag2 {
    const IID: nsIID = nsID(0x625cfd1e, 0xda1e, 0x4417,
        [0x9e, 0xe9, 0xdb, 0xc8, 0xe0, 0xb3, 0xfd, 0x79]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPropertyBag2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPropertyBag2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPropertyBag2Coerce {
    /// Cheaply cast a value of this type from a `nsIPropertyBag2`.
    fn coerce_from(v: &nsIPropertyBag2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPropertyBag2Coerce for nsIPropertyBag2 {
    #[inline]
    fn coerce_from(v: &nsIPropertyBag2) -> &Self {
        v
    }
}

impl nsIPropertyBag2 {
    /// Cast this `nsIPropertyBag2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPropertyBag2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPropertyBag2 {
    type Target = nsIPropertyBag;
    #[inline]
    fn deref(&self) -> &nsIPropertyBag {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPropertyBagCoerce> nsIPropertyBag2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIPropertyBag2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPropertyBag2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPropertyBag2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPropertyBagVTable,

    /* int32_t getPropertyAsInt32 (in AString prop); */
    pub GetPropertyAsInt32: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut int32_t) -> ::nserror::nsresult,

    /* uint32_t getPropertyAsUint32 (in AString prop); */
    pub GetPropertyAsUint32: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* int64_t getPropertyAsInt64 (in AString prop); */
    pub GetPropertyAsInt64: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut int64_t) -> ::nserror::nsresult,

    /* uint64_t getPropertyAsUint64 (in AString prop); */
    pub GetPropertyAsUint64: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut uint64_t) -> ::nserror::nsresult,

    /* double getPropertyAsDouble (in AString prop); */
    pub GetPropertyAsDouble: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* AString getPropertyAsAString (in AString prop); */
    pub GetPropertyAsAString: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* ACString getPropertyAsACString (in AString prop); */
    pub GetPropertyAsACString: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getPropertyAsAUTF8String (in AString prop); */
    pub GetPropertyAsAUTF8String: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean getPropertyAsBool (in AString prop); */
    pub GetPropertyAsBool: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub GetPropertyAsInterface: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* nsIVariant get (in AString prop); */
    pub Get: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* boolean hasKey (in AString prop); */
    pub HasKey: unsafe extern "system" fn (this: *const nsIPropertyBag2, prop: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPropertyBag2 {


    /// `int32_t getPropertyAsInt32 (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsInt32(&self, prop: *const ::nsstring::nsAString, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsInt32)(self, prop, _retval)
    }



    /// `uint32_t getPropertyAsUint32 (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsUint32(&self, prop: *const ::nsstring::nsAString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsUint32)(self, prop, _retval)
    }



    /// `int64_t getPropertyAsInt64 (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsInt64(&self, prop: *const ::nsstring::nsAString, _retval: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsInt64)(self, prop, _retval)
    }



    /// `uint64_t getPropertyAsUint64 (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsUint64(&self, prop: *const ::nsstring::nsAString, _retval: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsUint64)(self, prop, _retval)
    }



    /// `double getPropertyAsDouble (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsDouble(&self, prop: *const ::nsstring::nsAString, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsDouble)(self, prop, _retval)
    }



    /// `AString getPropertyAsAString (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsAString(&self, prop: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsAString)(self, prop, _retval)
    }



    /// `ACString getPropertyAsACString (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsACString(&self, prop: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsACString)(self, prop, _retval)
    }



    /// `AUTF8String getPropertyAsAUTF8String (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsAUTF8String(&self, prop: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsAUTF8String)(self, prop, _retval)
    }



    /// `boolean getPropertyAsBool (in AString prop);`
    #[inline]
    pub unsafe fn GetPropertyAsBool(&self, prop: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsBool)(self, prop, _retval)
    }


    /// ```text
    /// /**
    ///    * This method returns null if the value exists, but is null.
    ///    */
    /// ```
    ///

    /// `void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetPropertyAsInterface(&self, prop: *const ::nsstring::nsAString, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetPropertyAsInterface)(self, prop, iid, result)
    }


    /// ```text
    /// /**
    ///    * This method returns null if the value does not exist,
    ///    * or exists but is null.
    ///    */
    /// ```
    ///

    /// `nsIVariant get (in AString prop);`
    #[inline]
    pub unsafe fn Get(&self, prop: *const ::nsstring::nsAString, _retval: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, prop, _retval)
    }


    /// ```text
    /// /**
    ///    * Check for the existence of a key.
    ///    */
    /// ```
    ///

    /// `boolean hasKey (in AString prop);`
    #[inline]
    pub unsafe fn HasKey(&self, prop: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasKey)(self, prop, _retval)
    }


}


