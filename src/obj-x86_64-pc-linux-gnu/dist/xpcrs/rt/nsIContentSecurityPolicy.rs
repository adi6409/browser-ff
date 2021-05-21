//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIContentSecurityPolicy.idl
//


/// `interface nsIContentSecurityPolicy : nsISerializable`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentSecurityPolicy {
    vtable: *const nsIContentSecurityPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentSecurityPolicy.
unsafe impl XpCom for nsIContentSecurityPolicy {
    const IID: nsIID = nsID(0x1d632008, 0x6c97, 0x48ae,
        [0xa5, 0x1c, 0x16, 0xe2, 0xda, 0xa0, 0xf4, 0xf6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentSecurityPolicy {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentSecurityPolicy.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentSecurityPolicyCoerce {
    /// Cheaply cast a value of this type from a `nsIContentSecurityPolicy`.
    fn coerce_from(v: &nsIContentSecurityPolicy) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentSecurityPolicyCoerce for nsIContentSecurityPolicy {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityPolicy) -> &Self {
        v
    }
}

impl nsIContentSecurityPolicy {
    /// Cast this `nsIContentSecurityPolicy` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentSecurityPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentSecurityPolicy {
    type Target = nsISerializable;
    #[inline]
    fn deref(&self) -> &nsISerializable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISerializableCoerce> nsIContentSecurityPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityPolicy) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentSecurityPolicy
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentSecurityPolicyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISerializableVTable,

    /* [binaryname(GetPolicyString)] AString getPolicy (in unsigned long index); */
    pub GetPolicyString: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, index: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] CSPPolicyPtr GetPolicy (in unsigned long index); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPolicy: *const ::libc::c_void,

    /* readonly attribute unsigned long policyCount; */
    pub GetPolicyCount: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aPolicyCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute bool upgradeInsecureRequests; */
    pub GetUpgradeInsecureRequests: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aUpgradeInsecureRequests: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool blockAllMixedContent; */
    pub GetBlockAllMixedContent: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aBlockAllMixedContent: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool enforcesFrameAncestors; */
    pub GetEnforcesFrameAncestors: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aEnforcesFrameAncestors: *mut bool) -> ::nserror::nsresult,

    /* void appendPolicy (in AString policyString, in boolean reportOnly, in boolean deliveredViaMetaTag); */
    pub AppendPolicy: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, policyString: *const ::nsstring::nsAString, reportOnly: bool, deliveredViaMetaTag: bool) -> ::nserror::nsresult,

    /* boolean getAllowsInline (in nsIContentSecurityPolicy_CSPDirective aDirective, in AString aNonce, in boolean aParserCreated, in Element aTriggeringElement, in nsICSPEventListener aCSPEventListener, in AString aContentOfPseudoScript, in unsigned long aLineNumber, in unsigned long aColumnNumber); */
    pub GetAllowsInline: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aDirective:  u8, aNonce: *const ::nsstring::nsAString, aParserCreated: bool, aTriggeringElement: *const libc::c_void, aCSPEventListener: *const nsICSPEventListener, aContentOfPseudoScript: *const ::nsstring::nsAString, aLineNumber: u32, aColumnNumber: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean getAllowsNavigateTo (in nsIURI aURI, in boolean aIsFormSubmission, in boolean aWasRedirected, in boolean aEnforceWhitelist); */
    pub GetAllowsNavigateTo: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aURI: *const nsIURI, aIsFormSubmission: bool, aWasRedirected: bool, aEnforceWhitelist: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean getAllowsEval (out boolean shouldReportViolations); */
    pub GetAllowsEval: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, shouldReportViolations: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* uint32_t getCSPSandboxFlags (); */
    pub GetCSPSandboxFlags: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* void logViolationDetails (in unsigned short violationType, in Element triggeringElement, in nsICSPEventListener aCSPEventListener, in AString sourceFile, in AString scriptSample, in int32_t lineNum, in int32_t columnNum, [optional] in AString nonce, [optional] in AString content); */
    pub LogViolationDetails: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, violationType: u16, triggeringElement: *const libc::c_void, aCSPEventListener: *const nsICSPEventListener, sourceFile: *const ::nsstring::nsAString, scriptSample: *const ::nsstring::nsAString, lineNum: int32_t, columnNum: int32_t, nonce: *const ::nsstring::nsAString, content: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] void setRequestContextWithDocument (in Document aDocument); */
    pub SetRequestContextWithDocument: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aDocument: *const libc::c_void) -> ::nserror::nsresult,

    /* [must_use] void setRequestContextWithPrincipal (in nsIPrincipal aRequestPrincipal, in nsIURI aSelfURI, in AString aReferrer, in unsigned long long aInnerWindowId); */
    pub SetRequestContextWithPrincipal: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aRequestPrincipal: *const nsIPrincipal, aSelfURI: *const nsIURI, aReferrer: *const ::nsstring::nsAString, aInnerWindowId: u64) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] readonly attribute nsIPrincipal requestPrincipal; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetRequestPrincipal: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] readonly attribute nsIURI selfURI; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetSelfURI: *const ::libc::c_void,

    /* [noscript] readonly attribute AString referrer; */
    pub GetReferrer: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aReferrer: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] readonly attribute unsigned long long innerWindowID; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetInnerWindowID: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] attribute boolean skipAllowInlineStyleCheck; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetSkipAllowInlineStyleCheck: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] attribute boolean skipAllowInlineStyleCheck; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetSkipAllowInlineStyleCheck: *const ::libc::c_void,

    /* [noscript] void ensureEventTarget (in nsIEventTarget aEventTarget); */
    pub EnsureEventTarget: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,

    /* boolean permitsAncestry (in nsILoadInfo aLoadInfo); */
    pub PermitsAncestry: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aLoadInfo: *const nsILoadInfo, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean permits (in Element aTriggeringElement, in nsICSPEventListener aCSPEventListener, in nsIURI aURI, in nsIContentSecurityPolicy_CSPDirective aDir, in boolean aSpecific); */
    pub Permits: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aTriggeringElement: *const libc::c_void, aCSPEventListener: *const nsICSPEventListener, aURI: *const nsIURI, aDir:  u8, aSpecific: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* short shouldLoad (in nsContentPolicyType aContentType, in nsICSPEventListener aCSPEventListener, in nsIURI aContentLocation, in nsIURI aOriginalURIIfRedirect, in bool aSendViolationReports, in AString aNonce, in boolean aParserCreated); */
    pub ShouldLoad: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, aContentType: nsContentPolicyType, aCSPEventListener: *const nsICSPEventListener, aContentLocation: *const nsIURI, aOriginalURIIfRedirect: *const nsIURI, aSendViolationReports: bool, aNonce: *const ::nsstring::nsAString, aParserCreated: bool, _retval: *mut i16) -> ::nserror::nsresult,

    /* AString toJSON (); */
    pub ToJSON: unsafe extern "system" fn (this: *const nsIContentSecurityPolicy, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentSecurityPolicy {

    pub const VIOLATION_TYPE_INLINE_SCRIPT: i64 = 1;


    pub const VIOLATION_TYPE_EVAL: i64 = 2;


    pub const VIOLATION_TYPE_INLINE_STYLE: i64 = 3;


    pub const VIOLATION_TYPE_NONCE_SCRIPT: i64 = 4;


    pub const VIOLATION_TYPE_NONCE_STYLE: i64 = 5;


    pub const VIOLATION_TYPE_HASH_SCRIPT: i64 = 6;


    pub const VIOLATION_TYPE_HASH_STYLE: i64 = 7;


    pub const VIOLATION_TYPE_REQUIRE_SRI_FOR_STYLE: i64 = 8;


    pub const VIOLATION_TYPE_REQUIRE_SRI_FOR_SCRIPT: i64 = 9;

    /// ```text
    /// /**
    ///    * Accessor method for a read-only string version of the policy at a given
    ///    * index.
    ///    */
    /// ```
    ///

    /// `[binaryname(GetPolicyString)] AString getPolicy (in unsigned long index);`
    #[inline]
    pub unsafe fn GetPolicyString(&self, index: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPolicyString)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Accessor method for a read-only pointer the policy object at a given
    ///    * index. Returns a null pointer if the index is larger than the current
    ///    * policy count.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] CSPPolicyPtr GetPolicy (in unsigned long index);`
    const _GetPolicy: () = ();

    /// ```text
    /// /**
    ///    * Returns the number of policies attached to this CSP instance.  Useful with
    ///    * getPolicy().
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long policyCount;`
    #[inline]
    pub unsafe fn GetPolicyCount(&self, aPolicyCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPolicyCount)(self, aPolicyCount)
    }


    /// ```text
    /// /**
    ///    * Returns whether this policy uses the directive upgrade-insecure-requests.
    ///    * Please note that upgrade-insecure-reqeusts also applies if the parent or
    ///    * including document (context) makes use of the directive.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool upgradeInsecureRequests;`
    #[inline]
    pub unsafe fn GetUpgradeInsecureRequests(&self, aUpgradeInsecureRequests: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUpgradeInsecureRequests)(self, aUpgradeInsecureRequests)
    }


    /// ```text
    /// /**
    ///    * Returns whether this policy uses the directive block-all-mixed-content.
    ///    * Please note that block-all-mixed-content takes presedence in case the
    ///    * directive upgrade-insecure-requests is defined in the same policy and
    ///    * will therefore block all mixed content without even trying to perform
    ///    * an upgrade.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool blockAllMixedContent;`
    #[inline]
    pub unsafe fn GetBlockAllMixedContent(&self, aBlockAllMixedContent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBlockAllMixedContent)(self, aBlockAllMixedContent)
    }


    /// ```text
    /// /**
    ///    * Returns whether this policy enforces the frame-ancestors directive.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool enforcesFrameAncestors;`
    #[inline]
    pub unsafe fn GetEnforcesFrameAncestors(&self, aEnforcesFrameAncestors: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEnforcesFrameAncestors)(self, aEnforcesFrameAncestors)
    }


    /// ```text
    /// /**
    ///    * Parse and install a CSP policy.
    ///    * @param aPolicy
    ///    *        String representation of the policy
    ///    *        (e.g., header value, meta content)
    ///    * @param reportOnly
    ///    *        Should this policy affect content, script and style processing or
    ///    *        just send reports if it is violated?
    ///    * @param deliveredViaMetaTag
    ///    *        Indicates whether the policy was delivered via the meta tag.
    ///    */
    /// ```
    ///

    /// `void appendPolicy (in AString policyString, in boolean reportOnly, in boolean deliveredViaMetaTag);`
    #[inline]
    pub unsafe fn AppendPolicy(&self, policyString: *const ::nsstring::nsAString, reportOnly: bool, deliveredViaMetaTag: bool) -> ::nserror::nsresult {
        ((*self.vtable).AppendPolicy)(self, policyString, reportOnly, deliveredViaMetaTag)
    }



    /// `boolean getAllowsInline (in nsIContentSecurityPolicy_CSPDirective aDirective, in AString aNonce, in boolean aParserCreated, in Element aTriggeringElement, in nsICSPEventListener aCSPEventListener, in AString aContentOfPseudoScript, in unsigned long aLineNumber, in unsigned long aColumnNumber);`
    #[inline]
    pub unsafe fn GetAllowsInline(&self, aDirective:  u8, aNonce: *const ::nsstring::nsAString, aParserCreated: bool, aTriggeringElement: *const libc::c_void, aCSPEventListener: *const nsICSPEventListener, aContentOfPseudoScript: *const ::nsstring::nsAString, aLineNumber: u32, aColumnNumber: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowsInline)(self, aDirective, aNonce, aParserCreated, aTriggeringElement, aCSPEventListener, aContentOfPseudoScript, aLineNumber, aColumnNumber, _retval)
    }



    /// `boolean getAllowsNavigateTo (in nsIURI aURI, in boolean aIsFormSubmission, in boolean aWasRedirected, in boolean aEnforceWhitelist);`
    #[inline]
    pub unsafe fn GetAllowsNavigateTo(&self, aURI: *const nsIURI, aIsFormSubmission: bool, aWasRedirected: bool, aEnforceWhitelist: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowsNavigateTo)(self, aURI, aIsFormSubmission, aWasRedirected, aEnforceWhitelist, _retval)
    }


    /// ```text
    /// /**
    ///    * whether this policy allows eval and eval-like functions
    ///    * such as setTimeout("code string", time).
    ///    * @param shouldReportViolations
    ///    *     Whether or not the use of eval should be reported.
    ///    *     This function returns "true" when violating report-only policies, but
    ///    *     when any policy (report-only or otherwise) is violated,
    ///    *     shouldReportViolations is true as well.
    ///    * @return
    ///    *     Whether or not the effects of the eval call should be allowed
    ///    *     (block the call if false).
    ///    */
    /// ```
    ///

    /// `boolean getAllowsEval (out boolean shouldReportViolations);`
    #[inline]
    pub unsafe fn GetAllowsEval(&self, shouldReportViolations: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowsEval)(self, shouldReportViolations, _retval)
    }


    /// ```text
    /// /**
    ///    * Delegate method called by the service when the protected document is loaded.
    ///    * Returns the union of all the sandbox flags contained in CSP policies. This is the most
    ///    * restrictive interpretation of flags set in multiple policies.
    ///    * See nsSandboxFlags.h for the possible flags.
    ///    *
    ///    * @return
    ///    *    sandbox flags or SANDBOXED_NONE if no sandbox directive exists
    ///    */
    /// ```
    ///

    /// `uint32_t getCSPSandboxFlags ();`
    #[inline]
    pub unsafe fn GetCSPSandboxFlags(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCSPSandboxFlags)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * For each violated policy (of type violationType), log policy violation on
    ///    * the Error Console and send a report to report-uris present in the violated
    ///    * policies.
    ///    *
    ///    * @param violationType
    ///    *     one of the VIOLATION_TYPE_* constants, e.g. inline-script or eval
    ///    * @param triggeringElement
    ///    *     the element that triggers this CSP violation. It can be null.
    ///    * @param sourceFile
    ///    *     name of the source file containing the violation (if available)
    ///    * @param contentSample
    ///    *     sample of the violating content (to aid debugging)
    ///    * @param lineNum
    ///    *     source line number of the violation (if available)
    ///    * @param columnNum
    ///    *     source column number of the violation (if available)
    ///    * @param aNonce
    ///    *     (optional) If this is a nonce violation, include the nonce so we can
    ///    *     recheck to determine which policies were violated and send the
    ///    *     appropriate reports.
    ///    * @param aContent
    ///    *     (optional) If this is a hash violation, include contents of the inline
    ///    *     resource in the question so we can recheck the hash in order to
    ///    *     determine which policies were violated and send the appropriate
    ///    *     reports.
    ///    */
    /// ```
    ///

    /// `void logViolationDetails (in unsigned short violationType, in Element triggeringElement, in nsICSPEventListener aCSPEventListener, in AString sourceFile, in AString scriptSample, in int32_t lineNum, in int32_t columnNum, [optional] in AString nonce, [optional] in AString content);`
    #[inline]
    pub unsafe fn LogViolationDetails(&self, violationType: u16, triggeringElement: *const libc::c_void, aCSPEventListener: *const nsICSPEventListener, sourceFile: *const ::nsstring::nsAString, scriptSample: *const ::nsstring::nsAString, lineNum: int32_t, columnNum: int32_t, nonce: *const ::nsstring::nsAString, content: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).LogViolationDetails)(self, violationType, triggeringElement, aCSPEventListener, sourceFile, scriptSample, lineNum, columnNum, nonce, content)
    }


    /// ```text
    /// /**
    ///    * Called after the CSP object is created to fill in appropriate request
    ///    * context. Either use
    ///    *  * aDocument (preferred), or if no document is available, then provide
    ///    *  * aPrincipal, aSelfURI, aReferrer, aInnerWindowId explicitly.
    ///    */
    /// ```
    ///

    /// `[must_use] void setRequestContextWithDocument (in Document aDocument);`
    #[inline]
    pub unsafe fn SetRequestContextWithDocument(&self, aDocument: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestContextWithDocument)(self, aDocument)
    }



    /// `[must_use] void setRequestContextWithPrincipal (in nsIPrincipal aRequestPrincipal, in nsIURI aSelfURI, in AString aReferrer, in unsigned long long aInnerWindowId);`
    #[inline]
    pub unsafe fn SetRequestContextWithPrincipal(&self, aRequestPrincipal: *const nsIPrincipal, aSelfURI: *const nsIURI, aReferrer: *const ::nsstring::nsAString, aInnerWindowId: u64) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestContextWithPrincipal)(self, aRequestPrincipal, aSelfURI, aReferrer, aInnerWindowId)
    }


    /// ```text
    /// /**
    ///    * Get the various arguments needed to create a new request context for a CSP.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] readonly attribute nsIPrincipal requestPrincipal;`
    const _GetRequestPrincipal: () = ();


    /// `[noscript,nostdcall,notxpcom] readonly attribute nsIURI selfURI;`
    const _GetSelfURI: () = ();


    /// `[noscript] readonly attribute AString referrer;`
    #[inline]
    pub unsafe fn GetReferrer(&self, aReferrer: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrer)(self, aReferrer)
    }



    /// `[noscript,nostdcall,notxpcom] readonly attribute unsigned long long innerWindowID;`
    const _GetInnerWindowID: () = ();

    /// ```text
    /// /**
    ///    * Warning: Do not set that attribute unless you know exactly what you are doing!
    ///    *
    ///    * Primarily used to allow Devtools to edit inline styles!
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] attribute boolean skipAllowInlineStyleCheck;`
    const _GetSkipAllowInlineStyleCheck: () = ();

    /// ```text
    /// /**
    ///    * Warning: Do not set that attribute unless you know exactly what you are doing!
    ///    *
    ///    * Primarily used to allow Devtools to edit inline styles!
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] attribute boolean skipAllowInlineStyleCheck;`
    const _SetSkipAllowInlineStyleCheck: () = ();

    /// ```text
    /// /**
    ///    *  Ensure we have a nsIEventTarget to use to label CSPReportSenderRunnable
    ///    */
    /// ```
    ///

    /// `[noscript] void ensureEventTarget (in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn EnsureEventTarget(&self, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).EnsureEventTarget)(self, aEventTarget)
    }


    /// ```text
    /// /**
    ///    * Verifies ancestry as permitted by the policy.
    ///    *
    ///    * NOTE: Calls to this may trigger violation reports when queried, so this
    ///    * value should not be cached.
    ///    *
    ///    * @param aLoadInfo
    ///    *    The loadinfo of the channel containing the protected resource
    ///    * @return
    ///    *    true if the frame's ancestors are all allowed by policy (except for
        ///    *    report-only policies, which will send reports and then return true
        ///    *    here when violated).
    ///    */
    /// ```
    ///

    /// `boolean permitsAncestry (in nsILoadInfo aLoadInfo);`
    #[inline]
    pub unsafe fn PermitsAncestry(&self, aLoadInfo: *const nsILoadInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PermitsAncestry)(self, aLoadInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Checks if a specific directive permits loading of a URI.
    ///    *
    ///    * NOTE: Calls to this may trigger violation reports when queried, so the
    ///    * return value should not be cached.
    ///    *
    ///    * @param aTriggeringElement
    ///    *        The element that triggers this CSP check. It can be null.
    ///    * @param aURI
    ///    *    The URI about to be loaded or used.
    ///    * @param aDir
    ///    *    The CSPDirective to query (see above constants *_DIRECTIVE).
    ///    * @param aSpecific
    ///    *    If "true" and the directive is specified to fall back to "default-src"
    ///    *    when it's not explicitly provided, directivePermits will NOT try
    ///    *    default-src when the specific directive is not used.  Setting this to
    ///    *    "false" allows CSP to fall back to default-src.  This function
    ///    *    behaves the same for both values of canUseDefault when querying
    ///    *    directives that don't fall-back.
    ///    * @return
    ///    *    Whether or not the provided URI is allowed by CSP under the given
    ///    *    directive. (block the pending operation if false).
    ///    */
    /// ```
    ///

    /// `boolean permits (in Element aTriggeringElement, in nsICSPEventListener aCSPEventListener, in nsIURI aURI, in nsIContentSecurityPolicy_CSPDirective aDir, in boolean aSpecific);`
    #[inline]
    pub unsafe fn Permits(&self, aTriggeringElement: *const libc::c_void, aCSPEventListener: *const nsICSPEventListener, aURI: *const nsIURI, aDir:  u8, aSpecific: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Permits)(self, aTriggeringElement, aCSPEventListener, aURI, aDir, aSpecific, _retval)
    }


    /// ```text
    /// /**
    ///    * Delegate method called by the service when sub-elements of the protected
    ///    * document are being loaded.  Given a bit of information about the request,
    ///    * decides whether or not the policy is satisfied.
    ///    *
    ///    * Calls to this may trigger violation reports when queried, so
    ///    * this value should not be cached.
    ///    *
    ///    * aOriginalURIIfRedirect must be passed only if this loading is the result
    ///    * of a redirect. In this case, aOriginalURIIfRedirect must be the original
    ///    * URL.
    ///    */
    /// ```
    ///

    /// `short shouldLoad (in nsContentPolicyType aContentType, in nsICSPEventListener aCSPEventListener, in nsIURI aContentLocation, in nsIURI aOriginalURIIfRedirect, in bool aSendViolationReports, in AString aNonce, in boolean aParserCreated);`
    #[inline]
    pub unsafe fn ShouldLoad(&self, aContentType: nsContentPolicyType, aCSPEventListener: *const nsICSPEventListener, aContentLocation: *const nsIURI, aOriginalURIIfRedirect: *const nsIURI, aSendViolationReports: bool, aNonce: *const ::nsstring::nsAString, aParserCreated: bool, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).ShouldLoad)(self, aContentType, aCSPEventListener, aContentLocation, aOriginalURIIfRedirect, aSendViolationReports, aNonce, aParserCreated, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the CSP in JSON notation.
    ///    */
    /// ```
    ///

    /// `AString toJSON ();`
    #[inline]
    pub unsafe fn ToJSON(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ToJSON)(self, _retval)
    }


}


/// `typedef nsIContentSecurityPolicy::CSPDirective  CSPDirective;`
///


pub type CSPDirective =  u8;


/// `interface nsICSPEventListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICSPEventListener {
    vtable: *const nsICSPEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICSPEventListener.
unsafe impl XpCom for nsICSPEventListener {
    const IID: nsIID = nsID(0xc3163b14, 0x3a8f, 0x46dd,
        [0xa4, 0xaf, 0xbd, 0x04, 0x68, 0x03, 0x64, 0xcd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICSPEventListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICSPEventListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICSPEventListenerCoerce {
    /// Cheaply cast a value of this type from a `nsICSPEventListener`.
    fn coerce_from(v: &nsICSPEventListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICSPEventListenerCoerce for nsICSPEventListener {
    #[inline]
    fn coerce_from(v: &nsICSPEventListener) -> &Self {
        v
    }
}

impl nsICSPEventListener {
    /// Cast this `nsICSPEventListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICSPEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICSPEventListener {
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
impl<T: nsISupportsCoerce> nsICSPEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICSPEventListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICSPEventListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICSPEventListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onCSPViolationEvent (in AString aJSON); */
    pub OnCSPViolationEvent: unsafe extern "system" fn (this: *const nsICSPEventListener, aJSON: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICSPEventListener {


    /// `void onCSPViolationEvent (in AString aJSON);`
    #[inline]
    pub unsafe fn OnCSPViolationEvent(&self, aJSON: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnCSPViolationEvent)(self, aJSON)
    }


}


