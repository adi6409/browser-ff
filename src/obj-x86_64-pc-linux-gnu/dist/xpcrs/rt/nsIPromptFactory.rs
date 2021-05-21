//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptFactory.idl
//


/// `interface nsIPromptFactory : nsISupports`
///

/// ```text
/// /**
///  * This interface allows creating various prompts that have a specific parent.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPromptFactory {
    vtable: *const nsIPromptFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPromptFactory.
unsafe impl XpCom for nsIPromptFactory {
    const IID: nsIID = nsID(0x2803541c, 0xc96a, 0x4ff1,
        [0xbd, 0x7c, 0x9c, 0xb5, 0x66, 0xd4, 0x6a, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPromptFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPromptFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPromptFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsIPromptFactory`.
    fn coerce_from(v: &nsIPromptFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPromptFactoryCoerce for nsIPromptFactory {
    #[inline]
    fn coerce_from(v: &nsIPromptFactory) -> &Self {
        v
    }
}

impl nsIPromptFactory {
    /// Cast this `nsIPromptFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPromptFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPromptFactory {
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
impl<T: nsISupportsCoerce> nsIPromptFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPromptFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPromptFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPromptFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub GetPrompt: unsafe extern "system" fn (this: *const nsIPromptFactory, aParent: *const mozIDOMWindowProxy, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPromptFactory {

    /// ```text
    /// /**
    ///    * Returns an object implementing the specified interface that creates
    ///    * prompts parented to aParent.
    ///    */
    /// ```
    ///

    /// `void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetPrompt(&self, aParent: *const mozIDOMWindowProxy, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetPrompt)(self, aParent, iid, result)
    }


}


