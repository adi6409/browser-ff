//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISecretDecoderRing.idl
//


/// `interface nsISecretDecoderRing : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISecretDecoderRing {
    vtable: *const nsISecretDecoderRingVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISecretDecoderRing.
unsafe impl XpCom for nsISecretDecoderRing {
    const IID: nsIID = nsID(0x0ec80360, 0x075c, 0x11d4,
        [0x9f, 0xd4, 0x00, 0xc0, 0x4f, 0x1b, 0x83, 0xd8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISecretDecoderRing {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISecretDecoderRing.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISecretDecoderRingCoerce {
    /// Cheaply cast a value of this type from a `nsISecretDecoderRing`.
    fn coerce_from(v: &nsISecretDecoderRing) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISecretDecoderRingCoerce for nsISecretDecoderRing {
    #[inline]
    fn coerce_from(v: &nsISecretDecoderRing) -> &Self {
        v
    }
}

impl nsISecretDecoderRing {
    /// Cast this `nsISecretDecoderRing` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISecretDecoderRingCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISecretDecoderRing {
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
impl<T: nsISupportsCoerce> nsISecretDecoderRingCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecretDecoderRing) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISecretDecoderRing
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISecretDecoderRingVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] ACString encryptString (in ACString text); */
    pub EncryptString: unsafe extern "system" fn (this: *const nsISecretDecoderRing, text: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [implicit_jscontext,must_use] Promise asyncEncryptStrings (in Array<AUTF8String> plaintexts); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncEncryptStrings: *const ::libc::c_void,

    /* [must_use] ACString decryptString (in ACString encryptedBase64Text); */
    pub DecryptString: unsafe extern "system" fn (this: *const nsISecretDecoderRing, encryptedBase64Text: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [implicit_jscontext,must_use] Promise asyncDecryptStrings (in Array<ACString> encryptedStrings); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncDecryptStrings: *const ::libc::c_void,

    /* [must_use] void changePassword (); */
    pub ChangePassword: unsafe extern "system" fn (this: *const nsISecretDecoderRing) -> ::nserror::nsresult,

    /* [must_use] void logout (); */
    pub Logout: unsafe extern "system" fn (this: *const nsISecretDecoderRing) -> ::nserror::nsresult,

    /* [must_use] void logoutAndTeardown (); */
    pub LogoutAndTeardown: unsafe extern "system" fn (this: *const nsISecretDecoderRing) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISecretDecoderRing {

    /// ```text
    /// /**
    ///    * Encrypt to Base64 output.
    ///    * Note that the input must basically be a byte array (i.e. the code points
        ///    * must be within the range [0, 255]). Hence, using this method directly to
    ///    * encrypt passwords (or any text, really) won't work as expected.
    ///    * Instead, use something like nsIScriptableUnicodeConverter to first convert
    ///    * the desired password or text to UTF-8, then encrypt that. Remember to
    ///    * convert back when calling decryptString().
    ///    *
    ///    * @param text The text to encrypt.
    ///    * @return The encrypted text, encoded as Base64.
    ///    */
    /// ```
    ///

    /// `[must_use] ACString encryptString (in ACString text);`
    #[inline]
    pub unsafe fn EncryptString(&self, text: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EncryptString)(self, text, _retval)
    }


    /// ```text
    /// /**
    ///    * Run encryptString on multiple strings, asynchronously. This will allow you
    ///    * to not jank the browser if you need to encrypt a large number of strings
    ///    * all at once. This method accepts an array of wstrings which it will convert
    ///    * to UTF-8 internally before encrypting.
    ///    *
    ///    * @param plaintexts the strings to encrypt.
    ///    * @return A promise for the list of encrypted strings, encoded as Base64.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncEncryptStrings (in Array<AUTF8String> plaintexts);`
    const _AsyncEncryptStrings: () = ();

    /// ```text
    /// /**
    ///    * Decrypt Base64 input.
    ///    * See the encryptString() documentation - this method has basically the same
    ///    * limitations.
    ///    *
    ///    * @param encryptedBase64Text Encrypted input text, encoded as Base64.
    ///    * @return The decoded text.
    ///    */
    /// ```
    ///

    /// `[must_use] ACString decryptString (in ACString encryptedBase64Text);`
    #[inline]
    pub unsafe fn DecryptString(&self, encryptedBase64Text: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).DecryptString)(self, encryptedBase64Text, _retval)
    }


    /// ```text
    /// /**
    ///    * Run decryptString on multiple strings, asynchronously. This will allow you
    ///    * to not jank the browser if you need to decrypt a large number of strings
    ///    * all at once.
    ///    *
    ///    * @param encryptedStrings the strings to decrypt, encoded as Base64.
    ///    * @return A promise that resolves with the list of decrypted strings in Unicode.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncDecryptStrings (in Array<ACString> encryptedStrings);`
    const _AsyncDecryptStrings: () = ();

    /// ```text
    /// /**
    ///    * Prompt the user to change the password on the SDR key.
    ///    */
    /// ```
    ///

    /// `[must_use] void changePassword ();`
    #[inline]
    pub unsafe fn ChangePassword(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ChangePassword)(self, )
    }


    /// ```text
    /// /**
    ///    * Logout of the security device that protects the SDR key.
    ///    */
    /// ```
    ///

    /// `[must_use] void logout ();`
    #[inline]
    pub unsafe fn Logout(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Logout)(self, )
    }


    /// ```text
    /// /**
    ///    * Logout of the security device that protects the SDR key and tear
    ///    * down authenticated objects.
    ///    */
    /// ```
    ///

    /// `[must_use] void logoutAndTeardown ();`
    #[inline]
    pub unsafe fn LogoutAndTeardown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LogoutAndTeardown)(self, )
    }


}


