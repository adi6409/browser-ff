//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIProperties.idl
//


/// `interface nsIProperties : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProperties {
    vtable: *const nsIPropertiesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProperties.
unsafe impl XpCom for nsIProperties {
    const IID: nsIID = nsID(0x78650582, 0x4e93, 0x4b60,
        [0x8e, 0x85, 0x26, 0xeb, 0xd3, 0xeb, 0x14, 0xca]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProperties {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProperties.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPropertiesCoerce {
    /// Cheaply cast a value of this type from a `nsIProperties`.
    fn coerce_from(v: &nsIProperties) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPropertiesCoerce for nsIProperties {
    #[inline]
    fn coerce_from(v: &nsIProperties) -> &Self {
        v
    }
}

impl nsIProperties {
    /// Cast this `nsIProperties` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPropertiesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProperties {
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
impl<T: nsISupportsCoerce> nsIPropertiesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProperties) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProperties
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPropertiesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void get (in string prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub Get: unsafe extern "system" fn (this: *const nsIProperties, prop: *const libc::c_char, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void set (in string prop, in nsISupports value); */
    pub Set: unsafe extern "system" fn (this: *const nsIProperties, prop: *const libc::c_char, value: *const nsISupports) -> ::nserror::nsresult,

    /* boolean has (in string prop); */
    pub Has: unsafe extern "system" fn (this: *const nsIProperties, prop: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* void undefine (in string prop); */
    pub Undefine: unsafe extern "system" fn (this: *const nsIProperties, prop: *const libc::c_char) -> ::nserror::nsresult,

    /* Array<ACString> getKeys (); */
    pub GetKeys: unsafe extern "system" fn (this: *const nsIProperties, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProperties {

    /// ```text
    /// /**
    ///      * Gets a property with a given name.
    ///      *
    ///      * @throws NS_ERROR_FAILURE if a property with that name doesn't exist.
    ///      * @throws NS_ERROR_NO_INTERFACE if the found property fails to QI to the
    ///      * given iid.
    ///      */
    /// ```
    ///

    /// `void get (in string prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn Get(&self, prop: *const libc::c_char, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, prop, iid, result)
    }


    /// ```text
    /// /**
    ///      * Sets a property with a given name to a given value.
    ///      */
    /// ```
    ///

    /// `void set (in string prop, in nsISupports value);`
    #[inline]
    pub unsafe fn Set(&self, prop: *const libc::c_char, value: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, prop, value)
    }


    /// ```text
    /// /**
    ///      * Returns true if the property with the given name exists.
    ///      */
    /// ```
    ///

    /// `boolean has (in string prop);`
    #[inline]
    pub unsafe fn Has(&self, prop: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Has)(self, prop, _retval)
    }


    /// ```text
    /// /**
    ///      * Undefines a property.
    ///      * @throws NS_ERROR_FAILURE if a property with that name doesn't
    ///      * already exist.
    ///      */
    /// ```
    ///

    /// `void undefine (in string prop);`
    #[inline]
    pub unsafe fn Undefine(&self, prop: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).Undefine)(self, prop)
    }


    /// ```text
    /// /**
    ///      *  Returns an array of the keys.
    ///      */
    /// ```
    ///

    /// `Array<ACString> getKeys ();`
    #[inline]
    pub unsafe fn GetKeys(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetKeys)(self, _retval)
    }


}


