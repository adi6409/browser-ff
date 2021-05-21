//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIIDNService.idl
//


/// `interface nsIIDNService : nsISupports`
///

/// ```text
/// /**
///  * nsIIDNService interface.
///  *
///  * IDN (Internationalized Domain Name) support. Provides facilities
///  * for manipulating IDN hostnames according to the specification set
///  * forth by the IETF.
///  *
///  * IDN effort:
///  * http://www.ietf.org/html.characters/idn-charter.html
///  * http://www.i-dns.net
///  *
///  * IDNA specification:
///  * http://search.ietf.org/internet-drafts/draft-ietf-idn-idna-06.txt
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIDNService {
    vtable: *const nsIIDNServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIDNService.
unsafe impl XpCom for nsIIDNService {
    const IID: nsIID = nsID(0xa592a60e, 0x3621, 0x4f19,
        [0xa3, 0x18, 0x2b, 0xf2, 0x33, 0xcf, 0xad, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIDNService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIDNService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIDNServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIIDNService`.
    fn coerce_from(v: &nsIIDNService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIDNServiceCoerce for nsIIDNService {
    #[inline]
    fn coerce_from(v: &nsIIDNService) -> &Self {
        v
    }
}

impl nsIIDNService {
    /// Cast this `nsIIDNService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIDNServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIDNService {
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
impl<T: nsISupportsCoerce> nsIIDNServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIDNService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIDNService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIDNServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString convertUTF8toACE (in AUTF8String input); */
    pub ConvertUTF8toACE: unsafe extern "system" fn (this: *const nsIIDNService, input: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String convertACEtoUTF8 (in ACString input); */
    pub ConvertACEtoUTF8: unsafe extern "system" fn (this: *const nsIIDNService, input: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean isACE (in ACString input); */
    pub IsACE: unsafe extern "system" fn (this: *const nsIIDNService, input: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* AUTF8String normalize (in AUTF8String input); */
    pub Normalize: unsafe extern "system" fn (this: *const nsIIDNService, input: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII); */
    pub ConvertToDisplayIDN: unsafe extern "system" fn (this: *const nsIIDNService, input: *const ::nsstring::nsACString, isASCII: *mut bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIDNService {

    /// ```text
    /// /**
    ///      * Prepares the input hostname according to IDNA ToASCII operation,
    ///      * the input hostname is assumed to be UTF8-encoded.
    ///      */
    /// ```
    ///

    /// `ACString convertUTF8toACE (in AUTF8String input);`
    #[inline]
    pub unsafe fn ConvertUTF8toACE(&self, input: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertUTF8toACE)(self, input, _retval)
    }


    /// ```text
    /// /**
    ///      * This is the ToUnicode operation as specified in the IDNA proposal,
    ///      * with an additional step to encode the result in UTF-8.
    ///      * It takes an ACE-encoded hostname and performs ToUnicode to it, then
    ///      * encodes the resulting string into UTF8.
    ///      */
    /// ```
    ///

    /// `AUTF8String convertACEtoUTF8 (in ACString input);`
    #[inline]
    pub unsafe fn ConvertACEtoUTF8(&self, input: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertACEtoUTF8)(self, input, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks if the input string is ACE encoded or not.
    ///      */
    /// ```
    ///

    /// `boolean isACE (in ACString input);`
    #[inline]
    pub unsafe fn IsACE(&self, input: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsACE)(self, input, _retval)
    }


    /// ```text
    /// /**
    ///      * Performs the unicode normalization needed for hostnames in IDN,
    ///      * for callers that want early normalization.
    ///      */
    /// ```
    ///

    /// `AUTF8String normalize (in AUTF8String input);`
    #[inline]
    pub unsafe fn Normalize(&self, input: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Normalize)(self, input, _retval)
    }


    /// ```text
    /// /**
    ///      * Normalizes and converts a host to UTF-8 if the host is in the IDN
    ///      * whitelist, otherwise converts it to ACE. This is useful for display
    ///      * purposes and to ensure an encoding consistent with nsIURI::GetHost().
    ///      * If the result is ASCII or ACE encoded, |isASCII| will be true.
    ///      */
    /// ```
    ///

    /// `AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII);`
    #[inline]
    pub unsafe fn ConvertToDisplayIDN(&self, input: *const ::nsstring::nsACString, isASCII: *mut bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertToDisplayIDN)(self, input, isASCII, _retval)
    }


}


