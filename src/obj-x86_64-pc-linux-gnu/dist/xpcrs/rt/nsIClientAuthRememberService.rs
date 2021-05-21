//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIClientAuthRememberService.idl
//


/// `interface nsIClientAuthRememberRecord : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClientAuthRememberRecord {
    vtable: *const nsIClientAuthRememberRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClientAuthRememberRecord.
unsafe impl XpCom for nsIClientAuthRememberRecord {
    const IID: nsIID = nsID(0xe92825af, 0x7e81, 0x4b5c,
        [0xb4, 0x12, 0x8e, 0x1d, 0xd3, 0x6d, 0x14, 0xfe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClientAuthRememberRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClientAuthRememberRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClientAuthRememberRecordCoerce {
    /// Cheaply cast a value of this type from a `nsIClientAuthRememberRecord`.
    fn coerce_from(v: &nsIClientAuthRememberRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClientAuthRememberRecordCoerce for nsIClientAuthRememberRecord {
    #[inline]
    fn coerce_from(v: &nsIClientAuthRememberRecord) -> &Self {
        v
    }
}

impl nsIClientAuthRememberRecord {
    /// Cast this `nsIClientAuthRememberRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClientAuthRememberRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClientAuthRememberRecord {
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
impl<T: nsISupportsCoerce> nsIClientAuthRememberRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClientAuthRememberRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClientAuthRememberRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClientAuthRememberRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString asciiHost; */
    pub GetAsciiHost: unsafe extern "system" fn (this: *const nsIClientAuthRememberRecord, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString fingerprint; */
    pub GetFingerprint: unsafe extern "system" fn (this: *const nsIClientAuthRememberRecord, aFingerprint: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString dbKey; */
    pub GetDbKey: unsafe extern "system" fn (this: *const nsIClientAuthRememberRecord, aDbKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString entryKey; */
    pub GetEntryKey: unsafe extern "system" fn (this: *const nsIClientAuthRememberRecord, aEntryKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClientAuthRememberRecord {


    /// `readonly attribute ACString asciiHost;`
    #[inline]
    pub unsafe fn GetAsciiHost(&self, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiHost)(self, aAsciiHost)
    }



    /// `readonly attribute ACString fingerprint;`
    #[inline]
    pub unsafe fn GetFingerprint(&self, aFingerprint: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFingerprint)(self, aFingerprint)
    }



    /// `readonly attribute ACString dbKey;`
    #[inline]
    pub unsafe fn GetDbKey(&self, aDbKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDbKey)(self, aDbKey)
    }



    /// `readonly attribute ACString entryKey;`
    #[inline]
    pub unsafe fn GetEntryKey(&self, aEntryKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEntryKey)(self, aEntryKey)
    }


}


/// `interface nsIClientAuthRememberService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClientAuthRememberService {
    vtable: *const nsIClientAuthRememberServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClientAuthRememberService.
unsafe impl XpCom for nsIClientAuthRememberService {
    const IID: nsIID = nsID(0x1dbc6eb6, 0x0972, 0x4bdb,
        [0x9d, 0xc4, 0xac, 0xd0, 0xab, 0xf7, 0x23, 0x69]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClientAuthRememberService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClientAuthRememberService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClientAuthRememberServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIClientAuthRememberService`.
    fn coerce_from(v: &nsIClientAuthRememberService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClientAuthRememberServiceCoerce for nsIClientAuthRememberService {
    #[inline]
    fn coerce_from(v: &nsIClientAuthRememberService) -> &Self {
        v
    }
}

impl nsIClientAuthRememberService {
    /// Cast this `nsIClientAuthRememberService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClientAuthRememberServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClientAuthRememberService {
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
impl<T: nsISupportsCoerce> nsIClientAuthRememberServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClientAuthRememberService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClientAuthRememberService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClientAuthRememberServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void forgetRememberedDecision (in ACString key); */
    pub ForgetRememberedDecision: unsafe extern "system" fn (this: *const nsIClientAuthRememberService, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] Array<nsIClientAuthRememberRecord> getDecisions (); */
    pub GetDecisions: unsafe extern "system" fn (this: *const nsIClientAuthRememberService, _retval: *mut thin_vec::ThinVec<RefPtr<nsIClientAuthRememberRecord>>) -> ::nserror::nsresult,

    /* [must_use,noscript] void rememberDecision (in ACString aHostName, in const_OriginAttributesRef aOriginAttributes, in CERTCertificatePtr aServerCert, in CERTCertificatePtr aClientCert); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub RememberDecision: *const ::libc::c_void,

    /* [must_use,noscript] bool hasRememberedDecision (in ACString aHostName, in const_OriginAttributesRef aOriginAttributes, in CERTCertificatePtr aServerCert, out ACString aCertDBKey); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub HasRememberedDecision: *const ::libc::c_void,

    /* [must_use] void clearRememberedDecisions (); */
    pub ClearRememberedDecisions: unsafe extern "system" fn (this: *const nsIClientAuthRememberService) -> ::nserror::nsresult,

    /* [implicit_jscontext] void deleteDecisionsByHost (in ACString aHostName, in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub DeleteDecisionsByHost: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClientAuthRememberService {


    /// `[must_use] void forgetRememberedDecision (in ACString key);`
    #[inline]
    pub unsafe fn ForgetRememberedDecision(&self, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ForgetRememberedDecision)(self, key)
    }



    /// `[must_use] Array<nsIClientAuthRememberRecord> getDecisions ();`
    #[inline]
    pub unsafe fn GetDecisions(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsIClientAuthRememberRecord>>) -> ::nserror::nsresult {
        ((*self.vtable).GetDecisions)(self, _retval)
    }



    /// `[must_use,noscript] void rememberDecision (in ACString aHostName, in const_OriginAttributesRef aOriginAttributes, in CERTCertificatePtr aServerCert, in CERTCertificatePtr aClientCert);`
    const _RememberDecision: () = ();


    /// `[must_use,noscript] bool hasRememberedDecision (in ACString aHostName, in const_OriginAttributesRef aOriginAttributes, in CERTCertificatePtr aServerCert, out ACString aCertDBKey);`
    const _HasRememberedDecision: () = ();


    /// `[must_use] void clearRememberedDecisions ();`
    #[inline]
    pub unsafe fn ClearRememberedDecisions(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearRememberedDecisions)(self, )
    }



    /// `[implicit_jscontext] void deleteDecisionsByHost (in ACString aHostName, in jsval aOriginAttributes);`
    const _DeleteDecisionsByHost: () = ();

}


