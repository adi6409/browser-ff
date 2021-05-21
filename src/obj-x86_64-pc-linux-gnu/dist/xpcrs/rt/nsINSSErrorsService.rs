//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSErrorsService.idl
//


/// `interface nsINSSErrorsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINSSErrorsService {
    vtable: *const nsINSSErrorsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINSSErrorsService.
unsafe impl XpCom for nsINSSErrorsService {
    const IID: nsIID = nsID(0x12f60021, 0xe14b, 0x4020,
        [0x99, 0xd1, 0xed, 0x2c, 0x79, 0x5b, 0xe6, 0x6a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINSSErrorsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINSSErrorsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINSSErrorsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINSSErrorsService`.
    fn coerce_from(v: &nsINSSErrorsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINSSErrorsServiceCoerce for nsINSSErrorsService {
    #[inline]
    fn coerce_from(v: &nsINSSErrorsService) -> &Self {
        v
    }
}

impl nsINSSErrorsService {
    /// Cast this `nsINSSErrorsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINSSErrorsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINSSErrorsService {
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
impl<T: nsISupportsCoerce> nsINSSErrorsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINSSErrorsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINSSErrorsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINSSErrorsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] boolean isNSSErrorCode (in int32_t aNSPRCode); */
    pub IsNSSErrorCode: unsafe extern "system" fn (this: *const nsINSSErrorsService, aNSPRCode: int32_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] nsresult getXPCOMFromNSSError (in int32_t aNSPRCode); */
    pub GetXPCOMFromNSSError: unsafe extern "system" fn (this: *const nsINSSErrorsService, aNSPRCode: int32_t, _retval: *mut ::nserror::nsresult) -> ::nserror::nsresult,

    /* AString getErrorMessage (in nsresult aXPCOMErrorCode); */
    pub GetErrorMessage: unsafe extern "system" fn (this: *const nsINSSErrorsService, aXPCOMErrorCode: ::nserror::nsresult, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] uint32_t getErrorClass (in nsresult aXPCOMErrorCode); */
    pub GetErrorClass: unsafe extern "system" fn (this: *const nsINSSErrorsService, aXPCOMErrorCode: ::nserror::nsresult, _retval: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINSSErrorsService {

    pub const ERROR_CLASS_SSL_PROTOCOL: i64 = 1;


    pub const ERROR_CLASS_BAD_CERT: i64 = 2;

    /// ```text
    /// /**
    ///      *  The following values define the range of NSPR error codes used by NSS.
    ///      *  NSS remains the authorative source for these numbers, as a result,
    ///      *  the values might change in the future.
    ///      *  The security module will perform a runtime check and assertion
    ///      *  to ensure the values are in synch with NSS.
    ///      */
    /// ```
    ///

    pub const NSS_SEC_ERROR_BASE: i64 = -8192;


    pub const NSS_SEC_ERROR_LIMIT: i64 = -7192;


    pub const NSS_SSL_ERROR_BASE: i64 = -12288;


    pub const NSS_SSL_ERROR_LIMIT: i64 = -11288;

    /// ```text
    /// /**
    ///      * The error codes within each module must fit in 16 bits. We want these
    ///      * errors to fit in the same module as the NSS errors but not overlap with
    ///      * any of them. Converting an NSS SEC, NSS SSL, or mozilla::pkix error to
    ///      * an NS error involves negating the value of the error and then
    ///      * synthesizing an error in the NS_ERROR_MODULE_SECURITY module. Hence,
    ///      * mozilla::pkix errors will start at a negative value that both doesn't
    ///      * overlap with the current value ranges for NSS errors and that will fit
    ///      * in 16 bits when negated.
    ///      *
    ///      * Keep these in sync with pkixnss.h.
    ///      */
    /// ```
    ///

    pub const MOZILLA_PKIX_ERROR_BASE: i64 = -16384;


    pub const MOZILLA_PKIX_ERROR_LIMIT: i64 = -15384;

    /// ```text
    /// /**
    ///      *  @param aNSPRCode An error code obtained using PR_GetError()
    ///      *  @return True if it is error code defined by the NSS library
    ///      */
    /// ```
    ///

    /// `[must_use] boolean isNSSErrorCode (in int32_t aNSPRCode);`
    #[inline]
    pub unsafe fn IsNSSErrorCode(&self, aNSPRCode: int32_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsNSSErrorCode)(self, aNSPRCode, _retval)
    }


    /// ```text
    /// /**
    ///      *  Function will fail if aNSPRCode is not an NSS error code.
    ///      *  @param aNSPRCode An error code obtained using PR_GetError()
    ///      *  @return The result of the conversion, an XPCOM error code
    ///      */
    /// ```
    ///

    /// `[must_use] nsresult getXPCOMFromNSSError (in int32_t aNSPRCode);`
    #[inline]
    pub unsafe fn GetXPCOMFromNSSError(&self, aNSPRCode: int32_t, _retval: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetXPCOMFromNSSError)(self, aNSPRCode, _retval)
    }


    /// ```text
    /// /**
    ///      *  Function will fail if aXPCOMErrorCode is not an NSS error code.
    ///      *  @param aXPCOMErrorCode An error code obtained using getXPCOMFromNSSError
    ///      *  return A localized human readable error explanation.
    ///      */
    /// ```
    ///

    /// `AString getErrorMessage (in nsresult aXPCOMErrorCode);`
    #[inline]
    pub unsafe fn GetErrorMessage(&self, aXPCOMErrorCode: ::nserror::nsresult, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorMessage)(self, aXPCOMErrorCode, _retval)
    }


    /// ```text
    /// /**
    ///      *  Function will fail if aXPCOMErrorCode is not an NSS error code.
    ///      *  @param aXPCOMErrorCode An error code obtained using getXPCOMFromNSSError
    ///      *  return the error class of the code, either ERROR_CLASS_BAD_CERT
    ///      *         or ERROR_CLASS_SSL_PROTOCOL
    ///      */
    /// ```
    ///

    /// `[must_use] uint32_t getErrorClass (in nsresult aXPCOMErrorCode);`
    #[inline]
    pub unsafe fn GetErrorClass(&self, aXPCOMErrorCode: ::nserror::nsresult, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorClass)(self, aXPCOMErrorCode, _retval)
    }


}


