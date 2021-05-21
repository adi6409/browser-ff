//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleListener.idl
//


/// `interface nsIConsoleListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIConsoleListener {
    vtable: *const nsIConsoleListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIConsoleListener.
unsafe impl XpCom for nsIConsoleListener {
    const IID: nsIID = nsID(0x35c400a4, 0x5792, 0x438c,
        [0xb9, 0x15, 0x65, 0xe3, 0x0d, 0x58, 0xd5, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIConsoleListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIConsoleListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIConsoleListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIConsoleListener`.
    fn coerce_from(v: &nsIConsoleListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIConsoleListenerCoerce for nsIConsoleListener {
    #[inline]
    fn coerce_from(v: &nsIConsoleListener) -> &Self {
        v
    }
}

impl nsIConsoleListener {
    /// Cast this `nsIConsoleListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIConsoleListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIConsoleListener {
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
impl<T: nsISupportsCoerce> nsIConsoleListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIConsoleListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIConsoleListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void observe (in nsIConsoleMessage aMessage); */
    pub Observe: unsafe extern "system" fn (this: *const nsIConsoleListener, aMessage: *const nsIConsoleMessage) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIConsoleListener {


    /// `void observe (in nsIConsoleMessage aMessage);`
    #[inline]
    pub unsafe fn Observe(&self, aMessage: *const nsIConsoleMessage) -> ::nserror::nsresult {
        ((*self.vtable).Observe)(self, aMessage)
    }


}


