//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULRelatedElement.idl
//


/// `interface nsIDOMXULRelatedElement : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULRelatedElement {
    vtable: *const nsIDOMXULRelatedElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULRelatedElement.
unsafe impl XpCom for nsIDOMXULRelatedElement {
    const IID: nsIID = nsID(0x9fbac05a, 0xfb27, 0x470d,
        [0x8e, 0x5f, 0x02, 0x8b, 0x2d, 0xc5, 0x4a, 0xd0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULRelatedElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULRelatedElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULRelatedElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULRelatedElement`.
    fn coerce_from(v: &nsIDOMXULRelatedElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULRelatedElementCoerce for nsIDOMXULRelatedElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULRelatedElement) -> &Self {
        v
    }
}

impl nsIDOMXULRelatedElement {
    /// Cast this `nsIDOMXULRelatedElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULRelatedElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULRelatedElement {
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
impl<T: nsISupportsCoerce> nsIDOMXULRelatedElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULRelatedElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULRelatedElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULRelatedElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Element getRelatedElement (in Node aElement); */
    pub GetRelatedElement: unsafe extern "system" fn (this: *const nsIDOMXULRelatedElement, aElement: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULRelatedElement {

    /// ```text
    /// /**
    ///    * Retrun an element associated with the given element. It's implemented
    ///    * by container elements having relation between their items. For example,
    ///    * this interface is implemented by XUL tabs and XUL tabpanels elements
    ///    * and used to get XUL tab element by linked tab panel and vice versa.
    ///    */
    /// ```
    ///

    /// `Element getRelatedElement (in Node aElement);`
    #[inline]
    pub unsafe fn GetRelatedElement(&self, aElement: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetRelatedElement)(self, aElement, _retval)
    }


}


