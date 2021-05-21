//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIPrivacyTransitionObserver.idl
//


/// `interface nsIPrivacyTransitionObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrivacyTransitionObserver {
    vtable: *const nsIPrivacyTransitionObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrivacyTransitionObserver.
unsafe impl XpCom for nsIPrivacyTransitionObserver {
    const IID: nsIID = nsID(0xb4b1449d, 0x0ef0, 0x47f5,
        [0xb6, 0x2e, 0xad, 0xc5, 0x7f, 0xd4, 0x97, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrivacyTransitionObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrivacyTransitionObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrivacyTransitionObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIPrivacyTransitionObserver`.
    fn coerce_from(v: &nsIPrivacyTransitionObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrivacyTransitionObserverCoerce for nsIPrivacyTransitionObserver {
    #[inline]
    fn coerce_from(v: &nsIPrivacyTransitionObserver) -> &Self {
        v
    }
}

impl nsIPrivacyTransitionObserver {
    /// Cast this `nsIPrivacyTransitionObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrivacyTransitionObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrivacyTransitionObserver {
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
impl<T: nsISupportsCoerce> nsIPrivacyTransitionObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrivacyTransitionObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrivacyTransitionObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrivacyTransitionObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void privateModeChanged (in bool enabled); */
    pub PrivateModeChanged: unsafe extern "system" fn (this: *const nsIPrivacyTransitionObserver, enabled: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrivacyTransitionObserver {


    /// `void privateModeChanged (in bool enabled);`
    #[inline]
    pub unsafe fn PrivateModeChanged(&self, enabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).PrivateModeChanged)(self, enabled)
    }


}


