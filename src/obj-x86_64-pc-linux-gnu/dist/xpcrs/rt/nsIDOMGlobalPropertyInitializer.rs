//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMGlobalPropertyInitializer.idl
//


/// `interface nsIDOMGlobalPropertyInitializer : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMGlobalPropertyInitializer {
    vtable: *const nsIDOMGlobalPropertyInitializerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMGlobalPropertyInitializer.
unsafe impl XpCom for nsIDOMGlobalPropertyInitializer {
    const IID: nsIID = nsID(0x5842e275, 0x797f, 0x4afb,
        [0xb7, 0xe0, 0xe2, 0x9f, 0x0c, 0xb3, 0x12, 0xae]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMGlobalPropertyInitializer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMGlobalPropertyInitializer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMGlobalPropertyInitializerCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMGlobalPropertyInitializer`.
    fn coerce_from(v: &nsIDOMGlobalPropertyInitializer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMGlobalPropertyInitializerCoerce for nsIDOMGlobalPropertyInitializer {
    #[inline]
    fn coerce_from(v: &nsIDOMGlobalPropertyInitializer) -> &Self {
        v
    }
}

impl nsIDOMGlobalPropertyInitializer {
    /// Cast this `nsIDOMGlobalPropertyInitializer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMGlobalPropertyInitializerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMGlobalPropertyInitializer {
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
impl<T: nsISupportsCoerce> nsIDOMGlobalPropertyInitializerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGlobalPropertyInitializer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMGlobalPropertyInitializer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMGlobalPropertyInitializerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* jsval init (in mozIDOMWindow window); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Init: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMGlobalPropertyInitializer {


    /// `jsval init (in mozIDOMWindow window);`
    const _Init: () = ();

}


