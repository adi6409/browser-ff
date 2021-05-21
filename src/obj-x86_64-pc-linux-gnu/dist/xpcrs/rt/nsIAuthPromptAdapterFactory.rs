//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptAdapterFactory.idl
//


/// `interface nsIAuthPromptAdapterFactory : nsISupports`
///

/// ```text
/// /**
///  * An interface for wrapping nsIAuthPrompt interfaces to make
///  * them usable via an nsIAuthPrompt2 interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthPromptAdapterFactory {
    vtable: *const nsIAuthPromptAdapterFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthPromptAdapterFactory.
unsafe impl XpCom for nsIAuthPromptAdapterFactory {
    const IID: nsIID = nsID(0x60e46383, 0xbb9a, 0x4860,
        [0x89, 0x62, 0x80, 0xd9, 0xc5, 0xc0, 0x5d, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthPromptAdapterFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthPromptAdapterFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthPromptAdapterFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsIAuthPromptAdapterFactory`.
    fn coerce_from(v: &nsIAuthPromptAdapterFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthPromptAdapterFactoryCoerce for nsIAuthPromptAdapterFactory {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptAdapterFactory) -> &Self {
        v
    }
}

impl nsIAuthPromptAdapterFactory {
    /// Cast this `nsIAuthPromptAdapterFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthPromptAdapterFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthPromptAdapterFactory {
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
impl<T: nsISupportsCoerce> nsIAuthPromptAdapterFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptAdapterFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthPromptAdapterFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthPromptAdapterFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIAuthPrompt2 createAdapter (in nsIAuthPrompt aPrompt); */
    pub CreateAdapter: unsafe extern "system" fn (this: *const nsIAuthPromptAdapterFactory, aPrompt: *const nsIAuthPrompt, _retval: *mut*const nsIAuthPrompt2) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthPromptAdapterFactory {

    /// ```text
    /// /**
    ///    * Wrap an object implementing nsIAuthPrompt so that it's usable via
    ///    * nsIAuthPrompt2.
    ///    */
    /// ```
    ///

    /// `nsIAuthPrompt2 createAdapter (in nsIAuthPrompt aPrompt);`
    #[inline]
    pub unsafe fn CreateAdapter(&self, aPrompt: *const nsIAuthPrompt, _retval: *mut*const nsIAuthPrompt2) -> ::nserror::nsresult {
        ((*self.vtable).CreateAdapter)(self, aPrompt, _retval)
    }


}


