//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/mozIDOMWindow.idl
//


/// `interface mozIDOMWindow : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIDOMWindow {
    vtable: *const mozIDOMWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIDOMWindow.
unsafe impl XpCom for mozIDOMWindow {
    const IID: nsIID = nsID(0x75fbabd6, 0x7a2e, 0x4787,
        [0xaa, 0x33, 0x44, 0x9a, 0x33, 0x51, 0x21, 0x35]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIDOMWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIDOMWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIDOMWindowCoerce {
    /// Cheaply cast a value of this type from a `mozIDOMWindow`.
    fn coerce_from(v: &mozIDOMWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIDOMWindowCoerce for mozIDOMWindow {
    #[inline]
    fn coerce_from(v: &mozIDOMWindow) -> &Self {
        v
    }
}

impl mozIDOMWindow {
    /// Cast this `mozIDOMWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIDOMWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIDOMWindow {
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
impl<T: nsISupportsCoerce> mozIDOMWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIDOMWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIDOMWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIDOMWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIDOMWindow {


}


/// `interface mozIDOMWindowProxy : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIDOMWindowProxy {
    vtable: *const mozIDOMWindowProxyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIDOMWindowProxy.
unsafe impl XpCom for mozIDOMWindowProxy {
    const IID: nsIID = nsID(0x53ca090c, 0xe739, 0x48b9,
        [0x89, 0x11, 0x20, 0x8c, 0x72, 0xf9, 0x19, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIDOMWindowProxy {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIDOMWindowProxy.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIDOMWindowProxyCoerce {
    /// Cheaply cast a value of this type from a `mozIDOMWindowProxy`.
    fn coerce_from(v: &mozIDOMWindowProxy) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIDOMWindowProxyCoerce for mozIDOMWindowProxy {
    #[inline]
    fn coerce_from(v: &mozIDOMWindowProxy) -> &Self {
        v
    }
}

impl mozIDOMWindowProxy {
    /// Cast this `mozIDOMWindowProxy` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIDOMWindowProxyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIDOMWindowProxy {
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
impl<T: nsISupportsCoerce> mozIDOMWindowProxyCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIDOMWindowProxy) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIDOMWindowProxy
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIDOMWindowProxyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIDOMWindowProxy {


}


