//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptProvider.idl
//


/// `interface nsIAuthPromptProvider : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthPromptProvider {
    vtable: *const nsIAuthPromptProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthPromptProvider.
unsafe impl XpCom for nsIAuthPromptProvider {
    const IID: nsIID = nsID(0xbd9dc0fa, 0x68ce, 0x47d0,
        [0x88, 0x59, 0x64, 0x18, 0xc2, 0xae, 0x85, 0x76]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthPromptProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthPromptProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthPromptProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIAuthPromptProvider`.
    fn coerce_from(v: &nsIAuthPromptProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthPromptProviderCoerce for nsIAuthPromptProvider {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptProvider) -> &Self {
        v
    }
}

impl nsIAuthPromptProvider {
    /// Cast this `nsIAuthPromptProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthPromptProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthPromptProvider {
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
impl<T: nsISupportsCoerce> nsIAuthPromptProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthPromptProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthPromptProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub GetAuthPrompt: unsafe extern "system" fn (this: *const nsIAuthPromptProvider, aPromptReason: uint32_t, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthPromptProvider {
    /// ```text
    /// /**
    ///      * Normal (non-proxy) prompt request.
    ///      */
    /// ```
    ///

    pub const PROMPT_NORMAL: i64 = 0;

    /// ```text
    /// /**
    ///      * Proxy auth request.
    ///      */
    /// ```
    ///

    pub const PROMPT_PROXY: i64 = 1;

    /// ```text
    /// /**
    ///      * Request a prompt interface for the given prompt reason;
    ///      * @throws NS_ERROR_NOT_AVAILABLE if no prompt is allowed or
    ///      * available for the given reason.
    ///      *
    ///      * @param aPromptReason   The reason for the auth prompt;
    ///      *                        one of #PROMPT_NORMAL or #PROMPT_PROXY
    ///      * @param iid             The desired interface, e.g.
    ///      *                        NS_GET_IID(nsIAuthPrompt2).
    ///      * @returns an nsIAuthPrompt2 interface, or throws NS_ERROR_NOT_AVAILABLE
    ///      */
    /// ```
    ///

    /// `void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetAuthPrompt(&self, aPromptReason: uint32_t, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetAuthPrompt)(self, aPromptReason, iid, result)
    }


}


