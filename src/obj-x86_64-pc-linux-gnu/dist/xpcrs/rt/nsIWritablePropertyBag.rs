//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIWritablePropertyBag.idl
//


/// `interface nsIWritablePropertyBag : nsIPropertyBag`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWritablePropertyBag {
    vtable: *const nsIWritablePropertyBagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWritablePropertyBag.
unsafe impl XpCom for nsIWritablePropertyBag {
    const IID: nsIID = nsID(0x96fc4671, 0xeeb4, 0x4823,
        [0x94, 0x21, 0xe5, 0x0f, 0xb7, 0x0a, 0xd3, 0x53]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWritablePropertyBag {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWritablePropertyBag.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWritablePropertyBagCoerce {
    /// Cheaply cast a value of this type from a `nsIWritablePropertyBag`.
    fn coerce_from(v: &nsIWritablePropertyBag) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWritablePropertyBagCoerce for nsIWritablePropertyBag {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag) -> &Self {
        v
    }
}

impl nsIWritablePropertyBag {
    /// Cast this `nsIWritablePropertyBag` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWritablePropertyBagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWritablePropertyBag {
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
impl<T: nsIPropertyBagCoerce> nsIWritablePropertyBagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWritablePropertyBag
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWritablePropertyBagVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPropertyBagVTable,

    /* void setProperty (in AString name, in nsIVariant value); */
    pub SetProperty: unsafe extern "system" fn (this: *const nsIWritablePropertyBag, name: *const ::nsstring::nsAString, value: *const nsIVariant) -> ::nserror::nsresult,

    /* void deleteProperty (in AString name); */
    pub DeleteProperty: unsafe extern "system" fn (this: *const nsIWritablePropertyBag, name: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWritablePropertyBag {

    /// ```text
    /// /**
    ///      * Set a property with the given name to the given value.  If
    ///      * a property already exists with the given name, it is
    ///      * overwritten.
    ///      */
    /// ```
    ///

    /// `void setProperty (in AString name, in nsIVariant value);`
    #[inline]
    pub unsafe fn SetProperty(&self, name: *const ::nsstring::nsAString, value: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).SetProperty)(self, name, value)
    }


    /// ```text
    /// /**
    ///      * Delete a property with the given name.
    ///      * @throws NS_ERROR_FAILURE if a property with that name doesn't
    ///      * exist.
    ///      */
    /// ```
    ///

    /// `void deleteProperty (in AString name);`
    #[inline]
    pub unsafe fn DeleteProperty(&self, name: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).DeleteProperty)(self, name)
    }


}


