//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIOSKeyStore.idl
//


/// `interface nsIOSKeyStore : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOSKeyStore {
    vtable: *const nsIOSKeyStoreVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOSKeyStore.
unsafe impl XpCom for nsIOSKeyStore {
    const IID: nsIID = nsID(0x57972956, 0x5718, 0x42d2,
        [0x80, 0x70, 0xb3, 0xfc, 0x72, 0x21, 0x2e, 0xaf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOSKeyStore {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOSKeyStore.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOSKeyStoreCoerce {
    /// Cheaply cast a value of this type from a `nsIOSKeyStore`.
    fn coerce_from(v: &nsIOSKeyStore) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOSKeyStoreCoerce for nsIOSKeyStore {
    #[inline]
    fn coerce_from(v: &nsIOSKeyStore) -> &Self {
        v
    }
}

impl nsIOSKeyStore {
    /// Cast this `nsIOSKeyStore` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOSKeyStoreCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOSKeyStore {
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
impl<T: nsISupportsCoerce> nsIOSKeyStoreCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOSKeyStore) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOSKeyStore
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOSKeyStoreVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,must_use] Promise asyncGenerateSecret (in ACString label); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncGenerateSecret: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncSecretAvailable (in ACString label); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncSecretAvailable: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncRecoverSecret (in ACString label, in ACString recoveryPhrase); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncRecoverSecret: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncDeleteSecret (in ACString label); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncDeleteSecret: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncEncryptBytes (in ACString label, in Array<uint8_t> inBytes); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncEncryptBytes: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncDecryptBytes (in ACString label, in ACString encryptedBase64Text); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncDecryptBytes: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncLock (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncLock: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise asyncUnlock (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncUnlock: *const ::libc::c_void,

    /* readonly attribute bool isNSSKeyStore; */
    pub GetIsNSSKeyStore: unsafe extern "system" fn (this: *const nsIOSKeyStore, aIsNSSKeyStore: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOSKeyStore {

    /// ```text
    /// /**
    ///    * This interface provides encryption and decryption operations for data at
    ///    * rest. The key used to encrypt and decrypt the data is stored in the OS
    ///    * key store.
    ///    *
    ///    * Usage:
    ///    *
    ///    * // obtain the singleton OSKeyStore instance
    ///    * const oskeystore = Cc["@mozilla.org/security/oskeystore;1"].getService(Ci.nsIOSKeyStore);
    ///    *
    ///    * const PASSWORD_LABEL = "mylabel1";
    ///    * const COOKIE_LABEL = "mylabel2";
    ///    *
    ///    * // Unlock the key store.
    ///    * // Note that this is not necesssary. The key store will be unlocked
    ///    * // automatically when an operation is performed on it.
    ///    * await oskeystore.asyncUnlock();
    ///    *
    ///    * // Check if there's a secret for your label already.
    ///    * if (!await oskeystore.asyncSecretAvailable(PASSWORD_LABEL)) {
        ///    *   // Fail or generate a new secret for your label.
        ///    *   // If you want to generate a new secret, do.
        ///    *   // Hold onto `recoveryPhrase` to present to the user.
        ///    *   let recoveryPhrase = await oskeystore.asyncGenerateSecret(PASSWORD_LABEL);
        ///    * }
    ///    *
    ///    * // Assuming there's a secret with your label. Encrypt/Decrypt as follows.
    ///    * let encryptedPasswordBytes = await oskeystore.asyncEncryptBytes(PASSWORD_LABEL, passwordBytes);
    ///    * let newPasswordBytes = await oskeystore.asyncDecryptBytes(PASSWORD_LABEL, encryptedPasswordBytes);
    ///    *
    ///    * // Delete the secret from the key store.
    ///    * await oskeystore.asyncDeleteSecret(PASSWORD_LABEL);
    ///    *
    ///    * // Recover a secret from a recovery code.
    ///    * await oskeystore.asyncRecoverSecret(PASSWORD_LABEL, recoveryPhrase);
    ///    *
    ///    * // Lock the key store to prompt the user to log into her OS key store again.
    ///    * await oskeystore.asyncLock();
    ///    */
    /// /**
    ///    * Generate a new secret and store it in the OS key store with the given label.
    ///    * The caller should make sure that no other secrets with the same label are
    ///    * present before calling this function.
    ///    * This invalidates all previous ciphertexts created with the key
    ///    * corresponding to the given label.
    ///    *
    ///    * @param label The label to use for the secret.
    ///    * @return Promise that resolves to the recoveryPhrase string used to generate
    ///    *         the secret.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncGenerateSecret (in ACString label);`
    const _AsyncGenerateSecret: () = ();

    /// ```text
    /// /**
    ///    * Check whether a secret for a given label exists.
    ///    *
    ///    * @param label The label to lookup.
    ///    * @return Promise that resolves to a bool (whether a secret with label is
        ///    *         known or not) or an error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncSecretAvailable (in ACString label);`
    const _AsyncSecretAvailable: () = ();

    /// ```text
    /// /**
    ///    * Set a secret from a given recovery phrase.
    ///    * This might not be implemented on all platforms.
    ///    * This invalidates all previous ciphertexts.
    ///    *
    ///    * @param label The label to use for the secret.
    ///    * @param recoveryPhrase The recovery phrase that's used to generate the secret.
    ///    * @return Promise that resolves to undefined or an error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncRecoverSecret (in ACString label, in ACString recoveryPhrase);`
    const _AsyncRecoverSecret: () = ();

    /// ```text
    /// /**
    ///    * Delete secret with a given label. If there is no secret with the given
    ///    * label, no action is taken.
    ///    *
    ///    * @param label The label of the secret to delete.
    ///    * @return Promise that resolves to undefined or an error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncDeleteSecret (in ACString label);`
    const _AsyncDeleteSecret: () = ();

    /// ```text
    /// /**
    ///    * Encrypt the given data and then return the result as a base64-encoded
    ///    * string.
    ///    *
    ///    * @param label The label of the key to use to encrypt.
    ///    * @param inBytes The bytes to encrypt.
    ///    * @return Promise resolving to the encrypted text, encoded as Base64, or an
    ///    *         error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncEncryptBytes (in ACString label, in Array<uint8_t> inBytes);`
    const _AsyncEncryptBytes: () = ();

    /// ```text
    /// /**
    ///    * Decode and then decrypt the given base64-encoded string.
    ///    *
    ///    * @param label The label of the key to use to decrypt.
    ///    * @param encryptedBase64Text Encrypted input text, encoded as Base64.
    ///    * @return Promise resolving to the plaintext bytes or an error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncDecryptBytes (in ACString label, in ACString encryptedBase64Text);`
    const _AsyncDecryptBytes: () = ();

    /// ```text
    /// /**
    ///    * Lock the key store.
    ///    * The actual behaviour of this depends on the OS.
    ///    *
    ///    * @return Promise resolving to undefined or an error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncLock ();`
    const _AsyncLock: () = ();

    /// ```text
    /// /**
    ///    * Unlock the key store.
    ///    * The actual behaviour of this depends on the OS.
    ///    *
    ///    * @return Promise resolving to undefined or an error.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncUnlock ();`
    const _AsyncUnlock: () = ();

    /// ```text
    /// /**
    ///    * Check if the implementation is using the NSS key store.
    ///    * This is a special case because Firefox has to handle the locking and
    ///    * unlocking.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool isNSSKeyStore;`
    #[inline]
    pub unsafe fn GetIsNSSKeyStore(&self, aIsNSSKeyStore: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsNSSKeyStore)(self, aIsNSSKeyStore)
    }


}


