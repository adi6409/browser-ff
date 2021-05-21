//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectResultListener.idl
//


/// `interface nsIRedirectResultListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRedirectResultListener {
    vtable: *const nsIRedirectResultListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRedirectResultListener.
unsafe impl XpCom for nsIRedirectResultListener {
    const IID: nsIID = nsID(0x85cd2640, 0xe91e, 0x41ac,
        [0xbd, 0xca, 0x1d, 0xbf, 0x10, 0xdc, 0x13, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRedirectResultListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRedirectResultListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRedirectResultListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIRedirectResultListener`.
    fn coerce_from(v: &nsIRedirectResultListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRedirectResultListenerCoerce for nsIRedirectResultListener {
    #[inline]
    fn coerce_from(v: &nsIRedirectResultListener) -> &Self {
        v
    }
}

impl nsIRedirectResultListener {
    /// Cast this `nsIRedirectResultListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRedirectResultListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRedirectResultListener {
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
impl<T: nsISupportsCoerce> nsIRedirectResultListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRedirectResultListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRedirectResultListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRedirectResultListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onRedirectResult (in boolean proceeding); */
    pub OnRedirectResult: unsafe extern "system" fn (this: *const nsIRedirectResultListener, proceeding: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRedirectResultListener {

    /// ```text
    /// /**
    ///    *  When an HTTP redirect has been processed (either successfully or not)
    ///    *  nsIHttpChannel will call this function if its callbacks implement this
    ///    *  interface.
    ///    *
    ///    *  @param proceeding
    ///    *         Indicated whether the redirect will be proceeding, or not (i.e.
        ///    *         has been canceled, or failed).
    ///    */
    /// ```
    ///

    /// `void onRedirectResult (in boolean proceeding);`
    #[inline]
    pub unsafe fn OnRedirectResult(&self, proceeding: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnRedirectResult)(self, proceeding)
    }


}


