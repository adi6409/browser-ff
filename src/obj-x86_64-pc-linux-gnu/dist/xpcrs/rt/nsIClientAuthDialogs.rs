//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIClientAuthDialogs.idl
//


/// `interface nsIClientAuthDialogs : nsISupports`
///

/// ```text
/// /**
///  * Provides UI for SSL client-auth dialogs.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClientAuthDialogs {
    vtable: *const nsIClientAuthDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClientAuthDialogs.
unsafe impl XpCom for nsIClientAuthDialogs {
    const IID: nsIID = nsID(0xfa4c7520, 0x1433, 0x11d5,
        [0xba, 0x24, 0x00, 0x10, 0x83, 0x03, 0xb1, 0x17]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClientAuthDialogs {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClientAuthDialogs.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClientAuthDialogsCoerce {
    /// Cheaply cast a value of this type from a `nsIClientAuthDialogs`.
    fn coerce_from(v: &nsIClientAuthDialogs) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClientAuthDialogsCoerce for nsIClientAuthDialogs {
    #[inline]
    fn coerce_from(v: &nsIClientAuthDialogs) -> &Self {
        v
    }
}

impl nsIClientAuthDialogs {
    /// Cast this `nsIClientAuthDialogs` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClientAuthDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClientAuthDialogs {
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
impl<T: nsISupportsCoerce> nsIClientAuthDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClientAuthDialogs) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClientAuthDialogs
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClientAuthDialogsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] boolean chooseCertificate (in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex, out boolean rememberClientAuthCertificate); */
    pub ChooseCertificate: unsafe extern "system" fn (this: *const nsIClientAuthDialogs, hostname: *const ::nsstring::nsACString, port: i32, organization: *const ::nsstring::nsACString, issuerOrg: *const ::nsstring::nsACString, certList: *const nsIArray, selectedIndex: *mut u32, rememberClientAuthCertificate: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClientAuthDialogs {

    /// ```text
    /// /**
    ///    * Called when a user is asked to choose a certificate for client auth.
    ///    *
    ///    * @param hostname Hostname of the server.
    ///    * @param port Port of the server.
    ///    * @param organization Organization field of the server cert.
    ///    * @param issuerOrg Organization field of the issuer cert of the server cert.
    ///    * @param certList List of certificates the user can choose from.
    ///    * @param selectedIndex Index of the cert in |certList| that the user chose.
    ///    *                      Ignored if the return value is false.
    ///    * @param rememberClientAuthCertificate Whether to remember selection.
    ///    * @return true if a certificate was chosen. false if the user canceled.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean chooseCertificate (in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex, out boolean rememberClientAuthCertificate);`
    #[inline]
    pub unsafe fn ChooseCertificate(&self, hostname: *const ::nsstring::nsACString, port: i32, organization: *const ::nsstring::nsACString, issuerOrg: *const ::nsstring::nsACString, certList: *const nsIArray, selectedIndex: *mut u32, rememberClientAuthCertificate: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ChooseCertificate)(self, hostname, port, organization, issuerOrg, certList, selectedIndex, rememberClientAuthCertificate, _retval)
    }


}


