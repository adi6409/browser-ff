//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULContainerElement.idl
//


/// `interface nsIDOMXULContainerItemElement : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULContainerItemElement {
    vtable: *const nsIDOMXULContainerItemElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULContainerItemElement.
unsafe impl XpCom for nsIDOMXULContainerItemElement {
    const IID: nsIID = nsID(0x800a68c7, 0xb854, 0x4597,
        [0xa4, 0x36, 0x30, 0x55, 0xce, 0x5c, 0x5c, 0x96]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULContainerItemElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULContainerItemElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULContainerItemElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULContainerItemElement`.
    fn coerce_from(v: &nsIDOMXULContainerItemElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULContainerItemElementCoerce for nsIDOMXULContainerItemElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerItemElement) -> &Self {
        v
    }
}

impl nsIDOMXULContainerItemElement {
    /// Cast this `nsIDOMXULContainerItemElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULContainerItemElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULContainerItemElement {
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
impl<T: nsISupportsCoerce> nsIDOMXULContainerItemElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerItemElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULContainerItemElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULContainerItemElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Element parentContainer; */
    pub GetParentContainer: unsafe extern "system" fn (this: *const nsIDOMXULContainerItemElement, aParentContainer: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULContainerItemElement {

    /// ```text
    /// /**
    ///    * Returns the parent container if any.
    ///    */
    /// ```
    ///

    /// `readonly attribute Element parentContainer;`
    #[inline]
    pub unsafe fn GetParentContainer(&self, aParentContainer: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetParentContainer)(self, aParentContainer)
    }


}


/// `interface nsIDOMXULContainerElement : nsIDOMXULContainerItemElement`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULContainerElement {
    vtable: *const nsIDOMXULContainerElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULContainerElement.
unsafe impl XpCom for nsIDOMXULContainerElement {
    const IID: nsIID = nsID(0xb2bc96b8, 0x31fc, 0x42f4,
        [0x93, 0x7a, 0xbd, 0x27, 0x29, 0x1a, 0xf4, 0x0b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULContainerElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULContainerElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULContainerElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULContainerElement`.
    fn coerce_from(v: &nsIDOMXULContainerElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULContainerElementCoerce for nsIDOMXULContainerElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerElement) -> &Self {
        v
    }
}

impl nsIDOMXULContainerElement {
    /// Cast this `nsIDOMXULContainerElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULContainerElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULContainerElement {
    type Target = nsIDOMXULContainerItemElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULContainerItemElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDOMXULContainerItemElementCoerce> nsIDOMXULContainerElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULContainerElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULContainerElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULContainerElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMXULContainerItemElementVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULContainerElement {


}


