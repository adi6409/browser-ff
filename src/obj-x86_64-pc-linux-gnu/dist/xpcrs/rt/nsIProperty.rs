//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIProperty.idl
//


/// `interface nsIProperty : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProperty {
    vtable: *const nsIPropertyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProperty.
unsafe impl XpCom for nsIProperty {
    const IID: nsIID = nsID(0x6dcf9030, 0xa49f, 0x11d5,
        [0x91, 0x0d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProperty {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProperty.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPropertyCoerce {
    /// Cheaply cast a value of this type from a `nsIProperty`.
    fn coerce_from(v: &nsIProperty) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPropertyCoerce for nsIProperty {
    #[inline]
    fn coerce_from(v: &nsIProperty) -> &Self {
        v
    }
}

impl nsIProperty {
    /// Cast this `nsIProperty` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPropertyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProperty {
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
impl<T: nsISupportsCoerce> nsIPropertyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProperty) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProperty
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPropertyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIProperty, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIVariant value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIProperty, aValue: *mut*const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProperty {

    /// ```text
    /// /**
    ///      * Get the name of the property.
    ///      */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * Get the value of the property.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIVariant value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


}


