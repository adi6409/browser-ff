//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertificateDialogs.idl
//


/// `interface nsICertificateDialogs : nsISupports`
///

/// ```text
/// /**
///  * Functions that implement user interface dialogs to manage certificates.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertificateDialogs {
    vtable: *const nsICertificateDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertificateDialogs.
unsafe impl XpCom for nsICertificateDialogs {
    const IID: nsIID = nsID(0xda871dab, 0xf69e, 0x4173,
        [0xab, 0x26, 0x99, 0xfc, 0xd4, 0x7b, 0x0e, 0x85]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertificateDialogs {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertificateDialogs.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertificateDialogsCoerce {
    /// Cheaply cast a value of this type from a `nsICertificateDialogs`.
    fn coerce_from(v: &nsICertificateDialogs) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertificateDialogsCoerce for nsICertificateDialogs {
    #[inline]
    fn coerce_from(v: &nsICertificateDialogs) -> &Self {
        v
    }
}

impl nsICertificateDialogs {
    /// Cast this `nsICertificateDialogs` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertificateDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertificateDialogs {
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
impl<T: nsISupportsCoerce> nsICertificateDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertificateDialogs) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertificateDialogs
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertificateDialogsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust); */
    pub ConfirmDownloadCACert: unsafe extern "system" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, cert: *const nsIX509Cert, trust: *mut u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
    pub SetPKCS12FilePassword: unsafe extern "system" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, password: *mut ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
    pub GetPKCS12FilePassword: unsafe extern "system" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, password: *mut ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertificateDialogs {

    /// ```text
    /// /**
    ///    *  UI shown when a user is asked to download a new CA cert.
    ///    *  Provides user with ability to choose trust settings for the cert.
    ///    *  Asks the user to grant permission to import the certificate.
    ///    *
    ///    *  @param ctx A user interface context.
    ///    *  @param cert The certificate that is about to get installed.
    ///    *  @param trust A bit mask of trust flags.
    ///    *               See nsIX509CertDB for possible values.
    ///    *
    ///    *  @return true if the user allows to import the certificate.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust);`
    #[inline]
    pub unsafe fn ConfirmDownloadCACert(&self, ctx: *const nsIInterfaceRequestor, cert: *const nsIX509Cert, trust: *mut u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmDownloadCACert)(self, ctx, cert, trust, _retval)
    }


    /// ```text
    /// /**
    ///    *  UI shown when a user's personal certificate is going to be
    ///    *  exported to a backup file.
    ///    *  The implementation of this dialog should make sure to prompt the user to
    ///    *  type the password twice in order to confirm correct input.
    ///    *  The wording in the dialog should also motivate the user to enter a strong
    ///    *  password.
    ///    *
    ///    *  @param ctx A user interface context.
    ///    *  @param password The password provided by the user.
    ///    *
    ///    *  @return false if the user requests to cancel.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password);`
    #[inline]
    pub unsafe fn SetPKCS12FilePassword(&self, ctx: *const nsIInterfaceRequestor, password: *mut ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPKCS12FilePassword)(self, ctx, password, _retval)
    }


    /// ```text
    /// /**
    ///    *  UI shown when a user is about to restore a personal
    ///    *  certificate from a backup file.
    ///    *  The user is requested to enter the password
    ///    *  that was used in the past to protect that backup file.
    ///    *
    ///    *  @param ctx A user interface context.
    ///    *  @param password The password provided by the user.
    ///    *
    ///    *  @return false if the user requests to cancel.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password);`
    #[inline]
    pub unsafe fn GetPKCS12FilePassword(&self, ctx: *const nsIInterfaceRequestor, password: *mut ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPKCS12FilePassword)(self, ctx, password, _retval)
    }


}


