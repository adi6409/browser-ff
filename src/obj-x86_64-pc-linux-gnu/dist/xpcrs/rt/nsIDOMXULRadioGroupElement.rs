//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULRadioGroupElement.idl
//


/// `interface nsIDOMXULRadioGroupElement : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULRadioGroupElement {
    vtable: *const nsIDOMXULRadioGroupElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULRadioGroupElement.
unsafe impl XpCom for nsIDOMXULRadioGroupElement {
    const IID: nsIID = nsID(0x2cc1d24b, 0xec9f, 0x4e18,
        [0xaa, 0x34, 0xa2, 0x98, 0xa9, 0x00, 0x7f, 0x23]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULRadioGroupElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULRadioGroupElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULRadioGroupElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULRadioGroupElement`.
    fn coerce_from(v: &nsIDOMXULRadioGroupElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULRadioGroupElementCoerce for nsIDOMXULRadioGroupElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULRadioGroupElement) -> &Self {
        v
    }
}

impl nsIDOMXULRadioGroupElement {
    /// Cast this `nsIDOMXULRadioGroupElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULRadioGroupElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULRadioGroupElement {
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
impl<T: nsISupportsCoerce> nsIDOMXULRadioGroupElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULRadioGroupElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULRadioGroupElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULRadioGroupElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute Element focusedItem; */
    pub GetFocusedItem: unsafe extern "system" fn (this: *const nsIDOMXULRadioGroupElement, aFocusedItem: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Element focusedItem; */
    pub SetFocusedItem: unsafe extern "system" fn (this: *const nsIDOMXULRadioGroupElement, aFocusedItem: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULRadioGroupElement {


    /// `attribute Element focusedItem;`
    #[inline]
    pub unsafe fn GetFocusedItem(&self, aFocusedItem: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedItem)(self, aFocusedItem)
    }



    /// `attribute Element focusedItem;`
    #[inline]
    pub unsafe fn SetFocusedItem(&self, aFocusedItem: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetFocusedItem)(self, aFocusedItem)
    }


}


