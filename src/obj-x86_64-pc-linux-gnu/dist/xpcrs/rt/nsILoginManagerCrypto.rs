//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerCrypto.idl
//


/// `interface nsILoginManagerCrypto : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginManagerCrypto {
    vtable: *const nsILoginManagerCryptoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginManagerCrypto.
unsafe impl XpCom for nsILoginManagerCrypto {
    const IID: nsIID = nsID(0x2030770e, 0x542e, 0x40cd,
        [0x80, 0x61, 0xcd, 0x9d, 0x4a, 0xd4, 0x22, 0x7f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginManagerCrypto {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginManagerCrypto.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginManagerCryptoCoerce {
    /// Cheaply cast a value of this type from a `nsILoginManagerCrypto`.
    fn coerce_from(v: &nsILoginManagerCrypto) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginManagerCryptoCoerce for nsILoginManagerCrypto {
    #[inline]
    fn coerce_from(v: &nsILoginManagerCrypto) -> &Self {
        v
    }
}

impl nsILoginManagerCrypto {
    /// Cast this `nsILoginManagerCrypto` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginManagerCryptoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginManagerCrypto {
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
impl<T: nsISupportsCoerce> nsILoginManagerCryptoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManagerCrypto) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginManagerCrypto
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginManagerCryptoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString encrypt (in AString plainText); */
    pub Encrypt: unsafe extern "system" fn (this: *const nsILoginManagerCrypto, plainText: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* Promise encryptMany (in jsval plainTexts); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub EncryptMany: *const ::libc::c_void,

    /* AString decrypt (in AString cipherText); */
    pub Decrypt: unsafe extern "system" fn (this: *const nsILoginManagerCrypto, cipherText: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* Promise decryptMany (in jsval cipherTexts); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub DecryptMany: *const ::libc::c_void,

    /* readonly attribute boolean uiBusy; */
    pub GetUiBusy: unsafe extern "system" fn (this: *const nsILoginManagerCrypto, aUiBusy: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isLoggedIn; */
    pub GetIsLoggedIn: unsafe extern "system" fn (this: *const nsILoginManagerCrypto, aIsLoggedIn: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long defaultEncType; */
    pub GetDefaultEncType: unsafe extern "system" fn (this: *const nsILoginManagerCrypto, aDefaultEncType: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginManagerCrypto {

    pub const ENCTYPE_BASE64: i64 = 0;


    pub const ENCTYPE_SDR: i64 = 1;

    /// ```text
    /// /**
    ///    * encrypt
    ///    *
    ///    * @param plainText
    ///    *        The string to be encrypted.
    ///    *
    ///    * Encrypts the specified string, returning the ciphertext value.
    ///    *
    ///    * NOTE: The current implemention of this inferface simply uses NSS/PSM's
    ///    * "Secret Decoder Ring" service. It is not recommended for general
    ///    * purpose encryption/decryption.
    ///    *
    ///    * Can throw if the user cancels entry of their master password.
    ///    */
    /// ```
    ///

    /// `AString encrypt (in AString plainText);`
    #[inline]
    pub unsafe fn Encrypt(&self, plainText: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Encrypt)(self, plainText, _retval)
    }



    /// `Promise encryptMany (in jsval plainTexts);`
    const _EncryptMany: () = ();

    /// ```text
    /// /**
    ///    * decrypt
    ///    *
    ///    * @param cipherText
    ///    *        The string to be decrypted.
    ///    *
    ///    * Decrypts the specified string, returning the plaintext value.
    ///    *
    ///    * Can throw if the user cancels entry of their master password, or if the
    ///    * cipherText value can not be successfully decrypted (eg, if it was
        ///    * encrypted with some other key).
    ///    */
    /// ```
    ///

    /// `AString decrypt (in AString cipherText);`
    #[inline]
    pub unsafe fn Decrypt(&self, cipherText: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Decrypt)(self, cipherText, _retval)
    }


    /// ```text
    /// /**
    ///    * @param cipherTexts
    ///    *        The strings to be decrypted.
    ///    *
    ///    * Decrypts the specified strings, returning the plaintext values.
    ///    *
    ///    * Can throw if the user cancels entry of their master password, or if the
    ///    * cipherText value can not be successfully decrypted (eg, if it was
        ///    * encrypted with some other key).
    ///    */
    /// ```
    ///

    /// `Promise decryptMany (in jsval cipherTexts);`
    const _DecryptMany: () = ();

    /// ```text
    /// /**
    ///    * uiBusy
    ///    *
    ///    * True when a master password prompt is being displayed.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean uiBusy;`
    #[inline]
    pub unsafe fn GetUiBusy(&self, aUiBusy: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUiBusy)(self, aUiBusy)
    }


    /// ```text
    /// /**
    ///    * isLoggedIn
    ///    *
    ///    * Current login state of the token used for encryption. If the user is
    ///    * not logged in, performing a crypto operation will result in a master
    ///    * password prompt.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isLoggedIn;`
    #[inline]
    pub unsafe fn GetIsLoggedIn(&self, aIsLoggedIn: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsLoggedIn)(self, aIsLoggedIn)
    }


    /// ```text
    /// /**
    ///    * defaultEncType
    ///    *
    ///    * Default encryption type used by an implementation of this interface.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long defaultEncType;`
    #[inline]
    pub unsafe fn GetDefaultEncType(&self, aDefaultEncType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultEncType)(self, aDefaultEncType)
    }


}


