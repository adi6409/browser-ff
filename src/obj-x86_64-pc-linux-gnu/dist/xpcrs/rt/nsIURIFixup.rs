//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIURIFixup.idl
//


/// `interface nsIURIFixupInfo : nsISupports`
///

/// ```text
/// /**
///  * Interface indicating what we found/corrected when fixing up a URI
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURIFixupInfo {
    vtable: *const nsIURIFixupInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURIFixupInfo.
unsafe impl XpCom for nsIURIFixupInfo {
    const IID: nsIID = nsID(0x4819f183, 0xb532, 0x4932,
        [0xac, 0x09, 0xb3, 0x09, 0xcd, 0x85, 0x3b, 0xe7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURIFixupInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURIFixupInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURIFixupInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIURIFixupInfo`.
    fn coerce_from(v: &nsIURIFixupInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURIFixupInfoCoerce for nsIURIFixupInfo {
    #[inline]
    fn coerce_from(v: &nsIURIFixupInfo) -> &Self {
        v
    }
}

impl nsIURIFixupInfo {
    /// Cast this `nsIURIFixupInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURIFixupInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURIFixupInfo {
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
impl<T: nsISupportsCoerce> nsIURIFixupInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIFixupInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURIFixupInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIFixupInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute BrowsingContext consumer; */
    pub GetConsumer: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aConsumer: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute BrowsingContext consumer; */
    pub SetConsumer: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aConsumer: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute nsIURI preferredURI; */
    pub GetPreferredURI: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aPreferredURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI preferredURI; */
    pub SetPreferredURI: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aPreferredURI: *const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI fixedURI; */
    pub GetFixedURI: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aFixedURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI fixedURI; */
    pub SetFixedURI: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aFixedURI: *const nsIURI) -> ::nserror::nsresult,

    /* attribute AString keywordProviderName; */
    pub GetKeywordProviderName: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aKeywordProviderName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString keywordProviderName; */
    pub SetKeywordProviderName: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aKeywordProviderName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString keywordAsSent; */
    pub GetKeywordAsSent: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aKeywordAsSent: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString keywordAsSent; */
    pub SetKeywordAsSent: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aKeywordAsSent: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean fixupChangedProtocol; */
    pub GetFixupChangedProtocol: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aFixupChangedProtocol: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean fixupChangedProtocol; */
    pub SetFixupChangedProtocol: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aFixupChangedProtocol: bool) -> ::nserror::nsresult,

    /* attribute boolean fixupCreatedAlternateURI; */
    pub GetFixupCreatedAlternateURI: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aFixupCreatedAlternateURI: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean fixupCreatedAlternateURI; */
    pub SetFixupCreatedAlternateURI: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aFixupCreatedAlternateURI: bool) -> ::nserror::nsresult,

    /* attribute AUTF8String originalInput; */
    pub GetOriginalInput: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aOriginalInput: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String originalInput; */
    pub SetOriginalInput: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aOriginalInput: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute nsIInputStream postData; */
    pub GetPostData: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* attribute nsIInputStream postData; */
    pub SetPostData: unsafe extern "system" fn (this: *const nsIURIFixupInfo, aPostData: *const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURIFixupInfo {

    /// ```text
    /// /**
    ///    * Consumer that asked for fixed up URI.
    ///    */
    /// ```
    ///

    /// `attribute BrowsingContext consumer;`
    #[inline]
    pub unsafe fn GetConsumer(&self, aConsumer: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetConsumer)(self, aConsumer)
    }


    /// ```text
    /// /**
    ///    * Consumer that asked for fixed up URI.
    ///    */
    /// ```
    ///

    /// `attribute BrowsingContext consumer;`
    #[inline]
    pub unsafe fn SetConsumer(&self, aConsumer: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetConsumer)(self, aConsumer)
    }


    /// ```text
    /// /**
    ///    * Our best guess as to what URI the consumer will want. Might
    ///    * be null if we couldn't salvage anything (for instance, because
        ///    * the input was invalid as a URI and FIXUP_FLAG_ALLOW_KEYWORD_LOOKUP
        ///    * was not passed)
    ///    */
    /// ```
    ///

    /// `attribute nsIURI preferredURI;`
    #[inline]
    pub unsafe fn GetPreferredURI(&self, aPreferredURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetPreferredURI)(self, aPreferredURI)
    }


    /// ```text
    /// /**
    ///    * Our best guess as to what URI the consumer will want. Might
    ///    * be null if we couldn't salvage anything (for instance, because
        ///    * the input was invalid as a URI and FIXUP_FLAG_ALLOW_KEYWORD_LOOKUP
        ///    * was not passed)
    ///    */
    /// ```
    ///

    /// `attribute nsIURI preferredURI;`
    #[inline]
    pub unsafe fn SetPreferredURI(&self, aPreferredURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetPreferredURI)(self, aPreferredURI)
    }


    /// ```text
    /// /**
    ///    * The fixed-up original input, *never* using a keyword search.
    ///    * (might be null if the original input was not recoverable as
        ///    * a URL, e.g. "foo bar"!)
    ///    */
    /// ```
    ///

    /// `attribute nsIURI fixedURI;`
    #[inline]
    pub unsafe fn GetFixedURI(&self, aFixedURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetFixedURI)(self, aFixedURI)
    }


    /// ```text
    /// /**
    ///    * The fixed-up original input, *never* using a keyword search.
    ///    * (might be null if the original input was not recoverable as
        ///    * a URL, e.g. "foo bar"!)
    ///    */
    /// ```
    ///

    /// `attribute nsIURI fixedURI;`
    #[inline]
    pub unsafe fn SetFixedURI(&self, aFixedURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetFixedURI)(self, aFixedURI)
    }


    /// ```text
    /// /**
    ///    * The name of the keyword search provider used to provide a keyword search;
    ///    * empty string if no keyword search was done.
    ///    */
    /// ```
    ///

    /// `attribute AString keywordProviderName;`
    #[inline]
    pub unsafe fn GetKeywordProviderName(&self, aKeywordProviderName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeywordProviderName)(self, aKeywordProviderName)
    }


    /// ```text
    /// /**
    ///    * The name of the keyword search provider used to provide a keyword search;
    ///    * empty string if no keyword search was done.
    ///    */
    /// ```
    ///

    /// `attribute AString keywordProviderName;`
    #[inline]
    pub unsafe fn SetKeywordProviderName(&self, aKeywordProviderName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetKeywordProviderName)(self, aKeywordProviderName)
    }


    /// ```text
    /// /**
    ///    * The keyword as used for the search (post trimming etc.)
    ///    * empty string if no keyword search was done.
    ///    */
    /// ```
    ///

    /// `attribute AString keywordAsSent;`
    #[inline]
    pub unsafe fn GetKeywordAsSent(&self, aKeywordAsSent: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeywordAsSent)(self, aKeywordAsSent)
    }


    /// ```text
    /// /**
    ///    * The keyword as used for the search (post trimming etc.)
    ///    * empty string if no keyword search was done.
    ///    */
    /// ```
    ///

    /// `attribute AString keywordAsSent;`
    #[inline]
    pub unsafe fn SetKeywordAsSent(&self, aKeywordAsSent: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetKeywordAsSent)(self, aKeywordAsSent)
    }


    /// ```text
    /// /**
    ///    * Whether we changed the protocol instead of using one from the input as-is.
    ///    */
    /// ```
    ///

    /// `attribute boolean fixupChangedProtocol;`
    #[inline]
    pub unsafe fn GetFixupChangedProtocol(&self, aFixupChangedProtocol: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFixupChangedProtocol)(self, aFixupChangedProtocol)
    }


    /// ```text
    /// /**
    ///    * Whether we changed the protocol instead of using one from the input as-is.
    ///    */
    /// ```
    ///

    /// `attribute boolean fixupChangedProtocol;`
    #[inline]
    pub unsafe fn SetFixupChangedProtocol(&self, aFixupChangedProtocol: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetFixupChangedProtocol)(self, aFixupChangedProtocol)
    }


    /// ```text
    /// /**
    ///    * Whether we created an alternative URI. We might have added a prefix and/or
    ///    * suffix, the contents of which are controlled by the
    ///    * browser.fixup.alternate.prefix and .suffix prefs, with the defaults being
    ///    * "www." and ".com", respectively.
    ///    */
    /// ```
    ///

    /// `attribute boolean fixupCreatedAlternateURI;`
    #[inline]
    pub unsafe fn GetFixupCreatedAlternateURI(&self, aFixupCreatedAlternateURI: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFixupCreatedAlternateURI)(self, aFixupCreatedAlternateURI)
    }


    /// ```text
    /// /**
    ///    * Whether we created an alternative URI. We might have added a prefix and/or
    ///    * suffix, the contents of which are controlled by the
    ///    * browser.fixup.alternate.prefix and .suffix prefs, with the defaults being
    ///    * "www." and ".com", respectively.
    ///    */
    /// ```
    ///

    /// `attribute boolean fixupCreatedAlternateURI;`
    #[inline]
    pub unsafe fn SetFixupCreatedAlternateURI(&self, aFixupCreatedAlternateURI: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetFixupCreatedAlternateURI)(self, aFixupCreatedAlternateURI)
    }


    /// ```text
    /// /**
    ///    * The original input
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String originalInput;`
    #[inline]
    pub unsafe fn GetOriginalInput(&self, aOriginalInput: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalInput)(self, aOriginalInput)
    }


    /// ```text
    /// /**
    ///    * The original input
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String originalInput;`
    #[inline]
    pub unsafe fn SetOriginalInput(&self, aOriginalInput: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetOriginalInput)(self, aOriginalInput)
    }


    /// ```text
    /// /**
    ///    * The POST data to submit with the returned URI (see nsISearchSubmission).
    ///    */
    /// ```
    ///

    /// `attribute nsIInputStream postData;`
    #[inline]
    pub unsafe fn GetPostData(&self, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetPostData)(self, aPostData)
    }


    /// ```text
    /// /**
    ///    * The POST data to submit with the returned URI (see nsISearchSubmission).
    ///    */
    /// ```
    ///

    /// `attribute nsIInputStream postData;`
    #[inline]
    pub unsafe fn SetPostData(&self, aPostData: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetPostData)(self, aPostData)
    }


}


/// `interface nsIURIFixup : nsISupports`
///

/// ```text
/// /**
///  * Interface implemented by objects capable of fixing up strings into URIs
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURIFixup {
    vtable: *const nsIURIFixupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURIFixup.
unsafe impl XpCom for nsIURIFixup {
    const IID: nsIID = nsID(0x1da7e9d4, 0x620b, 0x4949,
        [0x84, 0x9a, 0x1c, 0xd6, 0x07, 0x7b, 0x1b, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURIFixup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURIFixup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURIFixupCoerce {
    /// Cheaply cast a value of this type from a `nsIURIFixup`.
    fn coerce_from(v: &nsIURIFixup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURIFixupCoerce for nsIURIFixup {
    #[inline]
    fn coerce_from(v: &nsIURIFixup) -> &Self {
        v
    }
}

impl nsIURIFixup {
    /// Cast this `nsIURIFixup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURIFixupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURIFixup {
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
impl<T: nsISupportsCoerce> nsIURIFixupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIFixup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURIFixup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIFixupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, [optional] in unsigned long aFixupFlags); */
    pub GetFixupURIInfo: unsafe extern "system" fn (this: *const nsIURIFixup, aURIText: *const ::nsstring::nsACString, aFixupFlags: u32, _retval: *mut *const nsIURIFixupInfo) -> ::nserror::nsresult,

    /* unsigned long webNavigationFlagsToFixupFlags (in AUTF8String aURIText, in unsigned long aDocShellFlags); */
    pub WebNavigationFlagsToFixupFlags: unsafe extern "system" fn (this: *const nsIURIFixup, aURIText: *const ::nsstring::nsACString, aDocShellFlags: u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] in boolean aIsPrivateContext); */
    pub KeywordToURI: unsafe extern "system" fn (this: *const nsIURIFixup, aKeyword: *const ::nsstring::nsACString, aIsPrivateContext: bool, _retval: *mut *const nsIURIFixupInfo) -> ::nserror::nsresult,

    /* bool isDomainKnown (in AUTF8String aDomain); */
    pub IsDomainKnown: unsafe extern "system" fn (this: *const nsIURIFixup, aDomain: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURIFixup {
    /// ```text
    /// /** No fixup flags. */
    /// ```
    ///

    pub const FIXUP_FLAG_NONE: i64 = 0;

    /// ```text
    /// /**
    ///      * Allow the fixup to use a keyword lookup service to complete the URI.
    ///      * The fixup object implementer should honour this flag and only perform
    ///      * any lengthy keyword (or search) operation if it is set.
    ///      */
    /// ```
    ///

    pub const FIXUP_FLAG_ALLOW_KEYWORD_LOOKUP: i64 = 1;

    /// ```text
    /// /**
    ///      * Tell the fixup to make an alternate URI from the input URI, for example
    ///      * to turn foo into www.foo.com.
    ///      */
    /// ```
    ///

    pub const FIXUP_FLAGS_MAKE_ALTERNATE_URI: i64 = 2;


    pub const FIXUP_FLAG_PRIVATE_CONTEXT: i64 = 4;


    pub const FIXUP_FLAG_FIX_SCHEME_TYPOS: i64 = 8;

    /// ```text
    /// /**
    ///      * Tries to converts the specified string into a URI, first attempting
    ///      * to correct any errors in the syntax or other vagaries.
    ///      * It returns information about what it corrected
    ///      * (e.g. whether we could rescue the URI or "just" generated a keyword
        ///      * search URI instead).
    ///      *
    ///      * @param aURIText    Candidate URI.
    ///      * @param aFixupFlags Flags that govern ways the URI may be fixed up.
    ///      *                    Defaults to FIXUP_FLAG_NONE.
    ///      */
    /// ```
    ///

    /// `nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, [optional] in unsigned long aFixupFlags);`
    #[inline]
    pub unsafe fn GetFixupURIInfo(&self, aURIText: *const ::nsstring::nsACString, aFixupFlags: u32, _retval: *mut *const nsIURIFixupInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetFixupURIInfo)(self, aURIText, aFixupFlags, _retval)
    }


    /// ```text
    /// /**
    ///      * Convert load flags from nsIWebNavigation to URI fixup flags for use in
    ///      * getFixupURIInfo.
    ///      *
    ///      * @param aURIText       Candidate URI; used for determining whether to
    ///      *                       allow keyword lookups.
    ///      * @param aDocShellFlags Load flags from nsIDocShell to convert.
    ///      */
    /// ```
    ///

    /// `unsigned long webNavigationFlagsToFixupFlags (in AUTF8String aURIText, in unsigned long aDocShellFlags);`
    #[inline]
    pub unsafe fn WebNavigationFlagsToFixupFlags(&self, aURIText: *const ::nsstring::nsACString, aDocShellFlags: u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).WebNavigationFlagsToFixupFlags)(self, aURIText, aDocShellFlags, _retval)
    }


    /// ```text
    /// /**
    ///      * Converts the specified keyword string into a URI.  Note that it's the
    ///      * caller's responsibility to check whether keywords are enabled and
    ///      * whether aKeyword is a sensible keyword.
    ///      *
    ///      * @param aKeyword  The keyword string to convert into a URI
    ///      * @param aIsPrivateContext Whether this is invoked from a private context.
    ///      */
    /// ```
    ///

    /// `nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] in boolean aIsPrivateContext);`
    #[inline]
    pub unsafe fn KeywordToURI(&self, aKeyword: *const ::nsstring::nsACString, aIsPrivateContext: bool, _retval: *mut *const nsIURIFixupInfo) -> ::nserror::nsresult {
        ((*self.vtable).KeywordToURI)(self, aKeyword, aIsPrivateContext, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns true if the specified domain is known and false otherwise.
    ///      * A known domain is relevant when we have a single word and can't be
    ///      * sure whether to treat the word as a host name or should instead be
    ///      * treated as a search term.
    ///      *
    ///      * @param aDomain A domain name to query.
    ///      */
    /// ```
    ///

    /// `bool isDomainKnown (in AUTF8String aDomain);`
    #[inline]
    pub unsafe fn IsDomainKnown(&self, aDomain: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDomainKnown)(self, aDomain, _retval)
    }


}


