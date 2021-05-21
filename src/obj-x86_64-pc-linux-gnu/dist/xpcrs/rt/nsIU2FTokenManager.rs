//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webauthn/nsIU2FTokenManager.idl
//


/// `interface nsIU2FTokenManager : nsISupports`
///

/// ```text
/// /**
///  * nsIU2FTokenManager
///  *
///  * An interface to the U2FTokenManager singleton.
///  *
///  * This should be used only by the WebAuthn browser UI prompts.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIU2FTokenManager {
    vtable: *const nsIU2FTokenManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIU2FTokenManager.
unsafe impl XpCom for nsIU2FTokenManager {
    const IID: nsIID = nsID(0x745e1eac, 0xe449, 0x4342,
        [0xbc, 0xa1, 0xee, 0x0e, 0x6e, 0xad, 0x09, 0xfc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIU2FTokenManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIU2FTokenManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIU2FTokenManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIU2FTokenManager`.
    fn coerce_from(v: &nsIU2FTokenManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIU2FTokenManagerCoerce for nsIU2FTokenManager {
    #[inline]
    fn coerce_from(v: &nsIU2FTokenManager) -> &Self {
        v
    }
}

impl nsIU2FTokenManager {
    /// Cast this `nsIU2FTokenManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIU2FTokenManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIU2FTokenManager {
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
impl<T: nsISupportsCoerce> nsIU2FTokenManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIU2FTokenManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIU2FTokenManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIU2FTokenManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resumeRegister (in uint64_t aTransactionID, in bool aForceNoneAttestation); */
    pub ResumeRegister: unsafe extern "system" fn (this: *const nsIU2FTokenManager, aTransactionID: uint64_t, aForceNoneAttestation: bool) -> ::nserror::nsresult,

    /* void cancel (in uint64_t aTransactionID); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIU2FTokenManager, aTransactionID: uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIU2FTokenManager {

    /// ```text
    /// /**
    ///      * Resumes the current WebAuthn/U2F transaction if that matches the given
    ///      * transaction ID. This is used only when direct attestation was requested
    ///      * and we have to wait for user input to proceed.
    ///      *
    ///      * @param aTransactionID : The ID of the transaction to resume.
    ///      * @param aForceNoneAttestation : The user might enforce none attestation.
    ///      */
    /// ```
    ///

    /// `void resumeRegister (in uint64_t aTransactionID, in bool aForceNoneAttestation);`
    #[inline]
    pub unsafe fn ResumeRegister(&self, aTransactionID: uint64_t, aForceNoneAttestation: bool) -> ::nserror::nsresult {
        ((*self.vtable).ResumeRegister)(self, aTransactionID, aForceNoneAttestation)
    }


    /// ```text
    /// /**
    ///      * Cancels the current WebAuthn/U2F transaction if that matches the given
    ///      * transaction ID.
    ///      *
    ///      * @param aTransactionID : The ID of the transaction to cancel.
    ///      */
    /// ```
    ///

    /// `void cancel (in uint64_t aTransactionID);`
    #[inline]
    pub unsafe fn Cancel(&self, aTransactionID: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, aTransactionID)
    }


}


