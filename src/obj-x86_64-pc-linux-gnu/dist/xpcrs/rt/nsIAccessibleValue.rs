//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleValue.idl
//


/// `interface nsIAccessibleValue : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleValue {
    vtable: *const nsIAccessibleValueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleValue.
unsafe impl XpCom for nsIAccessibleValue {
    const IID: nsIID = nsID(0x42a1e1dc, 0x58cf, 0x419d,
        [0xbf, 0xf0, 0xed, 0x33, 0x14, 0xc7, 0x00, 0x16]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleValue {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleValue.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleValueCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleValue`.
    fn coerce_from(v: &nsIAccessibleValue) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleValueCoerce for nsIAccessibleValue {
    #[inline]
    fn coerce_from(v: &nsIAccessibleValue) -> &Self {
        v
    }
}

impl nsIAccessibleValue {
    /// Cast this `nsIAccessibleValue` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleValueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleValue {
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
impl<T: nsISupportsCoerce> nsIAccessibleValueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleValue) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleValue
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleValueVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute double maximumValue; */
    pub GetMaximumValue: unsafe extern "system" fn (this: *const nsIAccessibleValue, aMaximumValue: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double minimumValue; */
    pub GetMinimumValue: unsafe extern "system" fn (this: *const nsIAccessibleValue, aMinimumValue: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double currentValue; */
    pub GetCurrentValue: unsafe extern "system" fn (this: *const nsIAccessibleValue, aCurrentValue: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double currentValue; */
    pub SetCurrentValue: unsafe extern "system" fn (this: *const nsIAccessibleValue, aCurrentValue: libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double minimumIncrement; */
    pub GetMinimumIncrement: unsafe extern "system" fn (this: *const nsIAccessibleValue, aMinimumIncrement: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleValue {


    /// `readonly attribute double maximumValue;`
    #[inline]
    pub unsafe fn GetMaximumValue(&self, aMaximumValue: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMaximumValue)(self, aMaximumValue)
    }



    /// `readonly attribute double minimumValue;`
    #[inline]
    pub unsafe fn GetMinimumValue(&self, aMinimumValue: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMinimumValue)(self, aMinimumValue)
    }



    /// `attribute double currentValue;`
    #[inline]
    pub unsafe fn GetCurrentValue(&self, aCurrentValue: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentValue)(self, aCurrentValue)
    }



    /// `attribute double currentValue;`
    #[inline]
    pub unsafe fn SetCurrentValue(&self, aCurrentValue: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentValue)(self, aCurrentValue)
    }



    /// `readonly attribute double minimumIncrement;`
    #[inline]
    pub unsafe fn GetMinimumIncrement(&self, aMinimumIncrement: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMinimumIncrement)(self, aMinimumIncrement)
    }


}


