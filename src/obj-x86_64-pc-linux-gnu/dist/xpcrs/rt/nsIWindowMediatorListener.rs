//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowMediatorListener.idl
//


/// `interface nsIWindowMediatorListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWindowMediatorListener {
    vtable: *const nsIWindowMediatorListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWindowMediatorListener.
unsafe impl XpCom for nsIWindowMediatorListener {
    const IID: nsIID = nsID(0x2f276982, 0x0d60, 0x4377,
        [0xa5, 0x95, 0xd3, 0x50, 0xba, 0x51, 0x63, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWindowMediatorListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWindowMediatorListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWindowMediatorListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWindowMediatorListener`.
    fn coerce_from(v: &nsIWindowMediatorListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWindowMediatorListenerCoerce for nsIWindowMediatorListener {
    #[inline]
    fn coerce_from(v: &nsIWindowMediatorListener) -> &Self {
        v
    }
}

impl nsIWindowMediatorListener {
    /// Cast this `nsIWindowMediatorListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWindowMediatorListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWindowMediatorListener {
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
impl<T: nsISupportsCoerce> nsIWindowMediatorListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowMediatorListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWindowMediatorListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWindowMediatorListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onOpenWindow (in nsIAppWindow window); */
    pub OnOpenWindow: unsafe extern "system" fn (this: *const nsIWindowMediatorListener, window: *const nsIAppWindow) -> ::nserror::nsresult,

    /* void onCloseWindow (in nsIAppWindow window); */
    pub OnCloseWindow: unsafe extern "system" fn (this: *const nsIWindowMediatorListener, window: *const nsIAppWindow) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWindowMediatorListener {


    /// `void onOpenWindow (in nsIAppWindow window);`
    #[inline]
    pub unsafe fn OnOpenWindow(&self, window: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).OnOpenWindow)(self, window)
    }



    /// `void onCloseWindow (in nsIAppWindow window);`
    #[inline]
    pub unsafe fn OnCloseWindow(&self, window: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).OnCloseWindow)(self, window)
    }


}


