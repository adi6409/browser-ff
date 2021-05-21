//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/html/nsIFormSubmitObserver.idl
//


/// `interface nsIFormSubmitObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFormSubmitObserver {
    vtable: *const nsIFormSubmitObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFormSubmitObserver.
unsafe impl XpCom for nsIFormSubmitObserver {
    const IID: nsIID = nsID(0x867cb7e7, 0x835d, 0x408b,
        [0x97, 0x88, 0xd2, 0x83, 0x4d, 0x28, 0x4e, 0x03]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFormSubmitObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFormSubmitObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFormSubmitObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIFormSubmitObserver`.
    fn coerce_from(v: &nsIFormSubmitObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFormSubmitObserverCoerce for nsIFormSubmitObserver {
    #[inline]
    fn coerce_from(v: &nsIFormSubmitObserver) -> &Self {
        v
    }
}

impl nsIFormSubmitObserver {
    /// Cast this `nsIFormSubmitObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFormSubmitObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFormSubmitObserver {
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
impl<T: nsISupportsCoerce> nsIFormSubmitObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormSubmitObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFormSubmitObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFormSubmitObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notifyInvalidSubmit (in HTMLFormElement formNode, in Array<Element> invalidElements); */
    pub NotifyInvalidSubmit: unsafe extern "system" fn (this: *const nsIFormSubmitObserver, formNode: *const libc::c_void, invalidElements: *const thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFormSubmitObserver {


    /// `void notifyInvalidSubmit (in HTMLFormElement formNode, in Array<Element> invalidElements);`
    #[inline]
    pub unsafe fn NotifyInvalidSubmit(&self, formNode: *const libc::c_void, invalidElements: *const thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult {
        ((*self.vtable).NotifyInvalidSubmit)(self, formNode, invalidElements)
    }


}


