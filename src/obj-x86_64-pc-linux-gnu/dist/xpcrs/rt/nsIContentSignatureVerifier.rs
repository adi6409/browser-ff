//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIContentSignatureVerifier.idl
//


/// `interface nsIContentSignatureVerifier : nsISupports`
///

/// ```text
/// /**
///  * An interface for verifying content-signatures, inspired by
///  * https://tools.ietf.org/html/draft-thomson-http-content-signature-00
///  * described here https://github.com/franziskuskiefer/content-signature/tree/pki
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentSignatureVerifier {
    vtable: *const nsIContentSignatureVerifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentSignatureVerifier.
unsafe impl XpCom for nsIContentSignatureVerifier {
    const IID: nsIID = nsID(0x45a5fe2f, 0xc350, 0x4b86,
        [0x96, 0x2d, 0x02, 0xd5, 0xaa, 0xaa, 0x95, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentSignatureVerifier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentSignatureVerifier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentSignatureVerifierCoerce {
    /// Cheaply cast a value of this type from a `nsIContentSignatureVerifier`.
    fn coerce_from(v: &nsIContentSignatureVerifier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentSignatureVerifierCoerce for nsIContentSignatureVerifier {
    #[inline]
    fn coerce_from(v: &nsIContentSignatureVerifier) -> &Self {
        v
    }
}

impl nsIContentSignatureVerifier {
    /// Cast this `nsIContentSignatureVerifier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentSignatureVerifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentSignatureVerifier {
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
impl<T: nsISupportsCoerce> nsIContentSignatureVerifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSignatureVerifier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentSignatureVerifier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentSignatureVerifierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,must_use] Promise asyncVerifyContentSignature (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aHostname); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncVerifyContentSignature: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentSignatureVerifier {

    /// ```text
    /// /**
    ///    * Verifies that the data matches the data that was used to generate the
    ///    * signature.
    ///    *
    ///    * @param aData                   The data to be tested.
    ///    * @param aContentSignatureHeader The content-signature header,
    ///    *                                url-safe base64 encoded.
    ///    * @param aCertificateChain       The certificate chain to use for verification.
    ///    *                                PEM encoded string.
    ///    * @param aHostname               The hostname for which the end entity must
    ///                                     be valid.
    ///    * @returns Promise that resolves with the value true if the signature
    ///    *          matches the data and aCertificateChain is valid within aContext,
    ///    *          and false if not. Rejects if another error occurred.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncVerifyContentSignature (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aHostname);`
    const _AsyncVerifyContentSignature: () = ();

}


