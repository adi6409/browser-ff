//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/search/nsISearchService.idl
//


/// `interface nsISearchSubmission : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISearchSubmission {
    vtable: *const nsISearchSubmissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISearchSubmission.
unsafe impl XpCom for nsISearchSubmission {
    const IID: nsIID = nsID(0x5799251f, 0x5b55, 0x4df7,
        [0xa9, 0xe7, 0x0c, 0x27, 0x81, 0x2c, 0x46, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISearchSubmission {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISearchSubmission.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISearchSubmissionCoerce {
    /// Cheaply cast a value of this type from a `nsISearchSubmission`.
    fn coerce_from(v: &nsISearchSubmission) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISearchSubmissionCoerce for nsISearchSubmission {
    #[inline]
    fn coerce_from(v: &nsISearchSubmission) -> &Self {
        v
    }
}

impl nsISearchSubmission {
    /// Cast this `nsISearchSubmission` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISearchSubmissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISearchSubmission {
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
impl<T: nsISupportsCoerce> nsISearchSubmissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchSubmission) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISearchSubmission
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISearchSubmissionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIInputStream postData; */
    pub GetPostData: unsafe extern "system" fn (this: *const nsISearchSubmission, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* readonly attribute nsIURI uri; */
    pub GetUri: unsafe extern "system" fn (this: *const nsISearchSubmission, aUri: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISearchSubmission {

    /// ```text
    /// /**
    ///    * The POST data associated with a search submission, wrapped in a MIME
    ///    * input stream. May be null.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIInputStream postData;`
    #[inline]
    pub unsafe fn GetPostData(&self, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetPostData)(self, aPostData)
    }


    /// ```text
    /// /**
    ///    * The URI to submit a search to.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI uri;`
    #[inline]
    pub unsafe fn GetUri(&self, aUri: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetUri)(self, aUri)
    }


}


/// `interface nsISearchEngine : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISearchEngine {
    vtable: *const nsISearchEngineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISearchEngine.
unsafe impl XpCom for nsISearchEngine {
    const IID: nsIID = nsID(0x620bd920, 0x0491, 0x48c8,
        [0x99, 0xa8, 0xd6, 0x04, 0x7e, 0x64, 0x80, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISearchEngine {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISearchEngine.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISearchEngineCoerce {
    /// Cheaply cast a value of this type from a `nsISearchEngine`.
    fn coerce_from(v: &nsISearchEngine) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISearchEngineCoerce for nsISearchEngine {
    #[inline]
    fn coerce_from(v: &nsISearchEngine) -> &Self {
        v
    }
}

impl nsISearchEngine {
    /// Cast this `nsISearchEngine` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISearchEngineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISearchEngine {
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
impl<T: nsISupportsCoerce> nsISearchEngineCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchEngine) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISearchEngine
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISearchEngineVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISearchSubmission getSubmission (in AString data, [optional] in AString responseType, [optional] in AString purpose); */
    pub GetSubmission: unsafe extern "system" fn (this: *const nsISearchEngine, data: *const ::nsstring::nsAString, responseType: *const ::nsstring::nsAString, purpose: *const ::nsstring::nsAString, _retval: *mut *const nsISearchSubmission) -> ::nserror::nsresult,

    /* readonly attribute AString searchUrlQueryParamName; */
    pub GetSearchUrlQueryParamName: unsafe extern "system" fn (this: *const nsISearchEngine, aSearchUrlQueryParamName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString searchUrlPublicSuffix; */
    pub GetSearchUrlPublicSuffix: unsafe extern "system" fn (this: *const nsISearchEngine, aSearchUrlPublicSuffix: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean supportsResponseType (in AString responseType); */
    pub SupportsResponseType: unsafe extern "system" fn (this: *const nsISearchEngine, responseType: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getIconURLBySize (in long width, in long height); */
    pub GetIconURLBySize: unsafe extern "system" fn (this: *const nsISearchEngine, width: i32, height: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* jsval getIcons (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetIcons: *const ::libc::c_void,

    /* void speculativeConnect (in jsval options); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SpeculativeConnect: *const ::libc::c_void,

    /* attribute AString alias; */
    pub GetAlias: unsafe extern "system" fn (this: *const nsISearchEngine, aAlias: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString alias; */
    pub SetAlias: unsafe extern "system" fn (this: *const nsISearchEngine, aAlias: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute Array<AString> aliases; */
    pub GetAliases: unsafe extern "system" fn (this: *const nsISearchEngine, aAliases: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* readonly attribute AString description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsISearchEngine, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean hidden; */
    pub GetHidden: unsafe extern "system" fn (this: *const nsISearchEngine, aHidden: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean hidden; */
    pub SetHidden: unsafe extern "system" fn (this: *const nsISearchEngine, aHidden: bool) -> ::nserror::nsresult,

    /* readonly attribute nsIURI iconURI; */
    pub GetIconURI: unsafe extern "system" fn (this: *const nsISearchEngine, aIconURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsISearchEngine, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString searchForm; */
    pub GetSearchForm: unsafe extern "system" fn (this: *const nsISearchEngine, aSearchForm: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean sendAttributionRequest; */
    pub GetSendAttributionRequest: unsafe extern "system" fn (this: *const nsISearchEngine, aSendAttributionRequest: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString telemetryId; */
    pub GetTelemetryId: unsafe extern "system" fn (this: *const nsISearchEngine, aTelemetryId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString identifier; */
    pub GetIdentifier: unsafe extern "system" fn (this: *const nsISearchEngine, aIdentifier: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean isAppProvided; */
    pub GetIsAppProvided: unsafe extern "system" fn (this: *const nsISearchEngine, aIsAppProvided: *mut bool) -> ::nserror::nsresult,

    /* AString getResultDomain ([optional] in AString responseType); */
    pub GetResultDomain: unsafe extern "system" fn (this: *const nsISearchEngine, responseType: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISearchEngine {

    /// ```text
    /// /**
    ///    * Gets a nsISearchSubmission object that contains information about what to
    ///    * send to the search engine, including the URI and postData, if applicable.
    ///    *
    ///    * @param  data
    ///    *         Data to add to the submission object.
    ///    *         i.e. the search terms.
    ///    *
    ///    * @param  responseType [optional]
    ///    *         The MIME type that we'd like to receive in response
    ///    *         to this submission.  If null, will default to "text/html".
    ///    *
    ///    * @param purpose [optional]
    ///    *        A string meant to indicate the context of the search request. This
    ///    *        allows the search service to provide a different nsISearchSubmission
    ///    *        depending on e.g. where the search is triggered in the UI.
    ///    *
    ///    * @returns A nsISearchSubmission object that contains information about what
    ///    *          to send to the search engine.  If no submission can be
    ///    *          obtained for the given responseType, returns null.
    ///    */
    /// ```
    ///

    /// `nsISearchSubmission getSubmission (in AString data, [optional] in AString responseType, [optional] in AString purpose);`
    #[inline]
    pub unsafe fn GetSubmission(&self, data: *const ::nsstring::nsAString, responseType: *const ::nsstring::nsAString, purpose: *const ::nsstring::nsAString, _retval: *mut *const nsISearchSubmission) -> ::nserror::nsresult {
        ((*self.vtable).GetSubmission)(self, data, responseType, purpose, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the name of the parameter used for the search terms for a submission
    ///    * URL of type `SearchUtils.URL_TYPE.SEARCH`.
    ///    *
    ///    * @returns A string which is the name of the parameter, or empty string
    ///    *          if no parameter cannot be found or is not supported (e.g. POST).
    ///    */
    /// ```
    ///

    /// `readonly attribute AString searchUrlQueryParamName;`
    #[inline]
    pub unsafe fn GetSearchUrlQueryParamName(&self, aSearchUrlQueryParamName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchUrlQueryParamName)(self, aSearchUrlQueryParamName)
    }


    /// ```text
    /// /**
    ///    * Returns the public suffix for the submission URL of type
    ///    * `SearchUtils.URL_TYPE.SEARCH`.
    ///    *
    ///    * @returns A string which is a known public suffix, or empty string
    ///    *          if one cannot be found.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString searchUrlPublicSuffix;`
    #[inline]
    pub unsafe fn GetSearchUrlPublicSuffix(&self, aSearchUrlPublicSuffix: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchUrlPublicSuffix)(self, aSearchUrlPublicSuffix)
    }


    /// ```text
    /// /**
    ///    * Determines whether the engine can return responses in the given
    ///    * MIME type.  Returns true if the engine spec has a URL with the
    ///    * given responseType, false otherwise.
    ///    *
    ///    * @param responseType
    ///    *        The MIME type to check for
    ///    */
    /// ```
    ///

    /// `boolean supportsResponseType (in AString responseType);`
    #[inline]
    pub unsafe fn SupportsResponseType(&self, responseType: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SupportsResponseType)(self, responseType, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns a string with the URL to an engine's icon matching both width and
    ///    * height. Returns null if icon with specified dimensions is not found.
    ///    *
    ///    * @param width
    ///    *        Width of the requested icon.
    ///    * @param height
    ///    *        Height of the requested icon.
    ///    */
    /// ```
    ///

    /// `AString getIconURLBySize (in long width, in long height);`
    #[inline]
    pub unsafe fn GetIconURLBySize(&self, width: i32, height: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIconURLBySize)(self, width, height, _retval)
    }


    /// ```text
    /// /**
    ///    * Gets an array of all available icons. Each entry is an object with
    ///    * width, height and url properties. width and height are numeric and
    ///    * represent the icon's dimensions. url is a string with the URL for
    ///    * the icon.
    ///    */
    /// ```
    ///

    /// `jsval getIcons ();`
    const _GetIcons: () = ();

    /// ```text
    /// /**
    ///    * Opens a speculative connection to the engine's search URI
    ///    * (and suggest URI, if different) to reduce request latency
    ///    *
    ///    * @param  options
    ///    *         An object that must contain the following fields:
    ///    *         {window} the content window for the window performing the search
    ///    *         {originAttributes} the originAttributes for performing the search
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if options is omitted or lacks required
    ///    *         elemeents
    ///    */
    /// ```
    ///

    /// `void speculativeConnect (in jsval options);`
    const _SpeculativeConnect: () = ();

    /// ```text
    /// /**
    ///    * An optional shortcut alias for the engine.
    ///    * When non-null, this is a unique identifier.
    ///    */
    /// ```
    ///

    /// `attribute AString alias;`
    #[inline]
    pub unsafe fn GetAlias(&self, aAlias: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAlias)(self, aAlias)
    }


    /// ```text
    /// /**
    ///    * An optional shortcut alias for the engine.
    ///    * When non-null, this is a unique identifier.
    ///    */
    /// ```
    ///

    /// `attribute AString alias;`
    #[inline]
    pub unsafe fn SetAlias(&self, aAlias: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetAlias)(self, aAlias)
    }


    /// ```text
    /// /**
    ///    * An array of aliases including the user defined alias and
    ///    * ones specified by the webextensions keywords field.
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<AString> aliases;`
    #[inline]
    pub unsafe fn GetAliases(&self, aAliases: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAliases)(self, aAliases)
    }


    /// ```text
    /// /**
    ///    * A text description describing the engine.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///    * Whether the engine should be hidden from the user.
    ///    */
    /// ```
    ///

    /// `attribute boolean hidden;`
    #[inline]
    pub unsafe fn GetHidden(&self, aHidden: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHidden)(self, aHidden)
    }


    /// ```text
    /// /**
    ///    * Whether the engine should be hidden from the user.
    ///    */
    /// ```
    ///

    /// `attribute boolean hidden;`
    #[inline]
    pub unsafe fn SetHidden(&self, aHidden: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHidden)(self, aHidden)
    }


    /// ```text
    /// /**
    ///    * A nsIURI corresponding to the engine's icon, stored locally. May be null.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI iconURI;`
    #[inline]
    pub unsafe fn GetIconURI(&self, aIconURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetIconURI)(self, aIconURI)
    }


    /// ```text
    /// /**
    ///    * The display name of the search engine. This is a unique identifier.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * A URL string pointing to the engine's search form.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString searchForm;`
    #[inline]
    pub unsafe fn GetSearchForm(&self, aSearchForm: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchForm)(self, aSearchForm)
    }


    /// ```text
    /// /**
    ///    * A boolean to indicate if we should send an attribution request to Mozilla's
    ///    * server.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean sendAttributionRequest;`
    #[inline]
    pub unsafe fn GetSendAttributionRequest(&self, aSendAttributionRequest: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSendAttributionRequest)(self, aSendAttributionRequest)
    }


    /// ```text
    /// /**
    ///    * The identifier to use for this engine when submitting to telemetry.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString telemetryId;`
    #[inline]
    pub unsafe fn GetTelemetryId(&self, aTelemetryId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTelemetryId)(self, aTelemetryId)
    }


    /// ```text
    /// /**
    ///    * An optional unique identifier for this search engine within the context of
    ///    * the distribution, as provided by the distributing entity.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString identifier;`
    #[inline]
    pub unsafe fn GetIdentifier(&self, aIdentifier: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetIdentifier)(self, aIdentifier)
    }


    /// ```text
    /// /**
    ///    * Whether or not this extension is provided by the application, e.g. it is
    ///    * in the list of configured search engines, or provided by a distribution
    ///    * set-up.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isAppProvided;`
    #[inline]
    pub unsafe fn GetIsAppProvided(&self, aIsAppProvided: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAppProvided)(self, aIsAppProvided)
    }


    /// ```text
    /// /**
    ///    * Gets a string representing the hostname from which search results for a
    ///    * given responseType are returned.  This can be specified as an url attribute
    ///    * in the engine description file, but will default to host from the <Url>'s
    ///    * template otherwise.
    ///    *
    ///    * @param  responseType [optional]
    ///    *         The MIME type to get resultDomain for.  Defaults to "text/html".
    ///    *
    ///    * @return the resultDomain for the given responseType.
    ///    */
    /// ```
    ///

    /// `AString getResultDomain ([optional] in AString responseType);`
    #[inline]
    pub unsafe fn GetResultDomain(&self, responseType: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetResultDomain)(self, responseType, _retval)
    }


}


/// `interface nsISearchParseSubmissionResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISearchParseSubmissionResult {
    vtable: *const nsISearchParseSubmissionResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISearchParseSubmissionResult.
unsafe impl XpCom for nsISearchParseSubmissionResult {
    const IID: nsIID = nsID(0x0dc93e51, 0xa7bf, 0x4a16,
        [0x86, 0x2d, 0x4b, 0x34, 0x69, 0xff, 0x62, 0x06]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISearchParseSubmissionResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISearchParseSubmissionResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISearchParseSubmissionResultCoerce {
    /// Cheaply cast a value of this type from a `nsISearchParseSubmissionResult`.
    fn coerce_from(v: &nsISearchParseSubmissionResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISearchParseSubmissionResultCoerce for nsISearchParseSubmissionResult {
    #[inline]
    fn coerce_from(v: &nsISearchParseSubmissionResult) -> &Self {
        v
    }
}

impl nsISearchParseSubmissionResult {
    /// Cast this `nsISearchParseSubmissionResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISearchParseSubmissionResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISearchParseSubmissionResult {
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
impl<T: nsISupportsCoerce> nsISearchParseSubmissionResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchParseSubmissionResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISearchParseSubmissionResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISearchParseSubmissionResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISearchEngine engine; */
    pub GetEngine: unsafe extern "system" fn (this: *const nsISearchParseSubmissionResult, aEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult,

    /* readonly attribute AString terms; */
    pub GetTerms: unsafe extern "system" fn (this: *const nsISearchParseSubmissionResult, aTerms: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString termsParameterName; */
    pub GetTermsParameterName: unsafe extern "system" fn (this: *const nsISearchParseSubmissionResult, aTermsParameterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute long termsOffset; */
    pub GetTermsOffset: unsafe extern "system" fn (this: *const nsISearchParseSubmissionResult, aTermsOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long termsLength; */
    pub GetTermsLength: unsafe extern "system" fn (this: *const nsISearchParseSubmissionResult, aTermsLength: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISearchParseSubmissionResult {

    /// ```text
    /// /**
    ///    * The search engine associated with the URL passed in to
    ///    * nsISearchEngine::parseSubmissionURL, or null if the URL does not represent
    ///    * a search submission.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISearchEngine engine;`
    #[inline]
    pub unsafe fn GetEngine(&self, aEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).GetEngine)(self, aEngine)
    }


    /// ```text
    /// /**
    ///    * String containing the sought terms.  This can be an empty string in case no
    ///    * terms were specified or the URL does not represent a search submission.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString terms;`
    #[inline]
    pub unsafe fn GetTerms(&self, aTerms: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTerms)(self, aTerms)
    }


    /// ```text
    /// /**
    ///    * The name of the query parameter used by `engine` for queries. E.g. "q".
    ///    */
    /// ```
    ///

    /// `readonly attribute AString termsParameterName;`
    #[inline]
    pub unsafe fn GetTermsParameterName(&self, aTermsParameterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTermsParameterName)(self, aTermsParameterName)
    }


    /// ```text
    /// /**
    ///    * The offset of the string |terms| in the URL passed in to
    ///    * nsISearchEngine::parseSubmissionURL, or -1 if the URL does not represent
    ///    * a search submission.
    ///    */
    /// ```
    ///

    /// `readonly attribute long termsOffset;`
    #[inline]
    pub unsafe fn GetTermsOffset(&self, aTermsOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetTermsOffset)(self, aTermsOffset)
    }


    /// ```text
    /// /**
    ///    * The length of the |terms| in the original encoding of the URL passed in to
    ///    * nsISearchEngine::parseSubmissionURL. If the search term in the original
    ///    * URL is encoded then this will be bigger than |terms.length|.
    ///    */
    /// ```
    ///

    /// `readonly attribute long termsLength;`
    #[inline]
    pub unsafe fn GetTermsLength(&self, aTermsLength: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetTermsLength)(self, aTermsLength)
    }


}


/// `interface nsISearchService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISearchService {
    vtable: *const nsISearchServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISearchService.
unsafe impl XpCom for nsISearchService {
    const IID: nsIID = nsID(0x0301834b, 0x2630, 0x440e,
        [0x8b, 0x98, 0xdb, 0x8d, 0xc5, 0x5f, 0x34, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISearchService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISearchService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISearchServiceCoerce {
    /// Cheaply cast a value of this type from a `nsISearchService`.
    fn coerce_from(v: &nsISearchService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISearchServiceCoerce for nsISearchService {
    #[inline]
    fn coerce_from(v: &nsISearchService) -> &Self {
        v
    }
}

impl nsISearchService {
    /// Cast this `nsISearchService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISearchServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISearchService {
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
impl<T: nsISupportsCoerce> nsISearchServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISearchService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISearchServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Promise init (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub Init: *const ::libc::c_void,

    /* readonly attribute bool isInitialized; */
    pub GetIsInitialized: unsafe extern "system" fn (this: *const nsISearchService, aIsInitialized: *mut bool) -> ::nserror::nsresult,

    /* Promise runBackgroundChecks (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub RunBackgroundChecks: *const ::libc::c_void,

    /* Promise resetToOriginalDefaultEngine (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub ResetToOriginalDefaultEngine: *const ::libc::c_void,

    /* Promise addOpenSearchEngine (in AString engineURL, in AString iconURL); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AddOpenSearchEngine: *const ::libc::c_void,

    /* Promise addPolicyEngine (in jsval details); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddPolicyEngine: *const ::libc::c_void,

    /* Promise addUserEngine (in AString name, in AString url, [optional] in AString alias); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AddUserEngine: *const ::libc::c_void,

    /* Promise addEngineWithDetails (in AString name, in jsval details); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddEngineWithDetails: *const ::libc::c_void,

    /* Promise addEnginesFromExtension (in jsval extension); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddEnginesFromExtension: *const ::libc::c_void,

    /* void restoreDefaultEngines (); */
    pub RestoreDefaultEngines: unsafe extern "system" fn (this: *const nsISearchService) -> ::nserror::nsresult,

    /* Promise getEngineByAlias (in AString alias); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetEngineByAlias: *const ::libc::c_void,

    /* nsISearchEngine getEngineByName (in AString aEngineName); */
    pub GetEngineByName: unsafe extern "system" fn (this: *const nsISearchService, aEngineName: *const ::nsstring::nsAString, _retval: *mut *const nsISearchEngine) -> ::nserror::nsresult,

    /* Promise getEngines (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetEngines: *const ::libc::c_void,

    /* Promise getVisibleEngines (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetVisibleEngines: *const ::libc::c_void,

    /* Promise getAppProvidedEngines (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetAppProvidedEngines: *const ::libc::c_void,

    /* Promise getEnginesByExtensionID (in AString extensionID); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetEnginesByExtensionID: *const ::libc::c_void,

    /* Promise moveEngine (in nsISearchEngine engine, in long newIndex); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub MoveEngine: *const ::libc::c_void,

    /* Promise removeEngine (in nsISearchEngine engine); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub RemoveEngine: *const ::libc::c_void,

    /* Promise removeWebExtensionEngine (in AString id); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub RemoveWebExtensionEngine: *const ::libc::c_void,

    /* readonly attribute nsISearchEngine originalDefaultEngine; */
    pub GetOriginalDefaultEngine: unsafe extern "system" fn (this: *const nsISearchService, aOriginalDefaultEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult,

    /* readonly attribute nsISearchEngine originalPrivateDefaultEngine; */
    pub GetOriginalPrivateDefaultEngine: unsafe extern "system" fn (this: *const nsISearchService, aOriginalPrivateDefaultEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult,

    /* attribute nsISearchEngine defaultEngine; */
    pub GetDefaultEngine: unsafe extern "system" fn (this: *const nsISearchService, aDefaultEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult,

    /* attribute nsISearchEngine defaultEngine; */
    pub SetDefaultEngine: unsafe extern "system" fn (this: *const nsISearchService, aDefaultEngine: *const nsISearchEngine) -> ::nserror::nsresult,

    /* Promise getDefault (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetDefault: *const ::libc::c_void,

    /* Promise setDefault (in nsISearchEngine engine); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SetDefault: *const ::libc::c_void,

    /* attribute nsISearchEngine defaultPrivateEngine; */
    pub GetDefaultPrivateEngine: unsafe extern "system" fn (this: *const nsISearchService, aDefaultPrivateEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult,

    /* attribute nsISearchEngine defaultPrivateEngine; */
    pub SetDefaultPrivateEngine: unsafe extern "system" fn (this: *const nsISearchService, aDefaultPrivateEngine: *const nsISearchEngine) -> ::nserror::nsresult,

    /* Promise getDefaultPrivate (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetDefaultPrivate: *const ::libc::c_void,

    /* Promise setDefaultPrivate (in nsISearchEngine engine); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SetDefaultPrivate: *const ::libc::c_void,

    /* Promise maybeSetAndOverrideDefault (in jsval extension); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub MaybeSetAndOverrideDefault: *const ::libc::c_void,

    /* Promise getDefaultEngineInfo (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetDefaultEngineInfo: *const ::libc::c_void,

    /* nsISearchParseSubmissionResult parseSubmissionURL (in AString url); */
    pub ParseSubmissionURL: unsafe extern "system" fn (this: *const nsISearchService, url: *const ::nsstring::nsAString, _retval: *mut *const nsISearchParseSubmissionResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISearchService {

    pub const ERROR_DOWNLOAD_FAILURE: i64 = 1;


    pub const ERROR_DUPLICATE_ENGINE: i64 = 2;


    pub const ERROR_ENGINE_CORRUPTED: i64 = 3;

    /// ```text
    /// /**
    ///    * Start asynchronous initialization.
    ///    *
    ///    * The promise is resolved once initialization is complete, which may be
    ///    * immediately, if initialization has already been completed by some previous
    ///    * call to this method.
    ///    * This method should only be called when you need or want to wait for the
    ///    * full initialization of the search service.
    ///    */
    /// ```
    ///

    /// `Promise init ();`
    const _Init: () = ();

    /// ```text
    /// /**
    ///    * Determine whether initialization has been completed.
    ///    *
    ///    * Clients of the service can use this attribute to quickly determine whether
    ///    * initialization is complete, and decide to trigger some immediate treatment,
    ///    * to launch asynchronous initialization or to bailout.
    ///    *
    ///    * Note that this attribute does not indicate that initialization has succeeded.
    ///    *
    ///    * @return |true| if the search service is now initialized, |false| if
    ///    * initialization has not been triggered yet.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool isInitialized;`
    #[inline]
    pub unsafe fn GetIsInitialized(&self, aIsInitialized: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInitialized)(self, aIsInitialized)
    }


    /// ```text
    /// /**
    ///    * Runs background checks; Designed to be run on idle.
    ///    */
    /// ```
    ///

    /// `Promise runBackgroundChecks ();`
    const _RunBackgroundChecks: () = ();

    /// ```text
    /// /**
    ///    * Resets the default engine to its original value.
    ///    */
    /// ```
    ///

    /// `Promise resetToOriginalDefaultEngine ();`
    const _ResetToOriginalDefaultEngine: () = ();

    /// ```text
    /// /**
    ///    * Adds a new Open Search engine from the file at the supplied URI.
    ///    *
    ///    * @param engineURL
    ///    *        The URL to the search engine's description file.
    ///    *
    ///    * @param iconURL
    ///    *        A URL string to an icon file to be used as the search engine's
    ///    *        icon. This value may be overridden by an icon specified in the
    ///    *        engine description file.
    ///    *
    ///    * @throws NS_ERROR_FAILURE if the description file cannot be successfully
    ///    *         loaded.
    ///    */
    /// ```
    ///

    /// `Promise addOpenSearchEngine (in AString engineURL, in AString iconURL);`
    const _AddOpenSearchEngine: () = ();

    /// ```text
    /// /**
    ///    * Adds a new search engine for enterprises.
    ///    *
    ///    * @param details
    ///    *        An object that simulates the WebExtension manifest structure:
    ///    *
    ///    *        {iconURL} Optional
    ///    *        {description} Optional
    ///    *        {chrome_settings_overrides} This should contain a {search_provider}
    ///    *                                    structure that mimics the WebExtension
    ///    *                                    manifest structure.
    ///    */
    /// ```
    ///

    /// `Promise addPolicyEngine (in jsval details);`
    const _AddPolicyEngine: () = ();

    /// ```text
    /// /**
    ///    * Adds a new search engine defined by the user.
    ///    *
    ///    * @param name
    ///    *        The name of the engine.
    ///    * @param url
    ///    *        The url of the engine with %s denoting where to
    ///    *        replace the search term.
    ///    * @param alias [optional]
    ///    *        The alias to refer to the engine.
    ///    */
    /// ```
    ///

    /// `Promise addUserEngine (in AString name, in AString url, [optional] in AString alias);`
    const _AddUserEngine: () = ();

    /// ```text
    /// /**
    ///    * Test-only function for adding an engine. This should be considered obsolete,
    ///    * and will be replaced soon (bug 1649186).
    ///    */
    /// ```
    ///

    /// `Promise addEngineWithDetails (in AString name, in jsval details);`
    const _AddEngineWithDetails: () = ();

    /// ```text
    /// /**
    ///    * Adds search providers to the search service.  If the search
    ///    * service is configured to load multiple locales for the extension,
    ///    * it may load more than one search engine. If called directly
    ///    * ensure the extension has been initialised.
    ///    *
    ///    * @param extension
    ///    *        The extension to load from.
    ///    * @returns Promise that resolves when finished.
    ///    */
    /// ```
    ///

    /// `Promise addEnginesFromExtension (in jsval extension);`
    const _AddEnginesFromExtension: () = ();

    /// ```text
    /// /**
    ///    * Un-hides all engines in the set of engines returned by getAppProvidedEngines.
    ///    */
    /// ```
    ///

    /// `void restoreDefaultEngines ();`
    #[inline]
    pub unsafe fn RestoreDefaultEngines(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RestoreDefaultEngines)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns an engine with the specified alias.
    ///    *
    ///    * @param   alias
    ///    *          The search engine's alias.
    ///    * @returns The corresponding nsISearchEngine object, or null if it doesn't
    ///    *          exist.
    ///    */
    /// ```
    ///

    /// `Promise getEngineByAlias (in AString alias);`
    const _GetEngineByAlias: () = ();

    /// ```text
    /// /**
    ///    * Returns an engine with the specified name.
    ///    *
    ///    * @param   aEngineName
    ///    *          The name of the engine.
    ///    * @returns The corresponding nsISearchEngine object, or null if it doesn't
    ///    *          exist.
    ///    */
    /// ```
    ///

    /// `nsISearchEngine getEngineByName (in AString aEngineName);`
    #[inline]
    pub unsafe fn GetEngineByName(&self, aEngineName: *const ::nsstring::nsAString, _retval: *mut *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).GetEngineByName)(self, aEngineName, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns an array of all installed search engines.
    ///    * The array is sorted either to the user requirements or the default order.
    ///    *
    ///    * @returns an array of nsISearchEngine objects.
    ///    */
    /// ```
    ///

    /// `Promise getEngines ();`
    const _GetEngines: () = ();

    /// ```text
    /// /**
    ///    * Returns an array of all installed search engines whose hidden attribute is
    ///    * false.
    ///    * The array is sorted either to the user requirements or the default order.
    ///    *
    ///    * @returns an array of nsISearchEngine objects.
    ///    */
    /// ```
    ///

    /// `Promise getVisibleEngines ();`
    const _GetVisibleEngines: () = ();

    /// ```text
    /// /**
    ///    * Returns an array of all default search engines. This includes all loaded
    ///    * engines that aren't in the user's profile directory.
    ///    * The array is sorted to the default order.
    ///    *
    ///    * @returns an array of nsISearchEngine objects.
    ///    */
    /// ```
    ///

    /// `Promise getAppProvidedEngines ();`
    const _GetAppProvidedEngines: () = ();

    /// ```text
    /// /**
    ///    * Returns an array of search engines installed by a given extension.
    ///    *
    ///    * @returns an array of nsISearchEngine objects.
    ///    */
    /// ```
    ///

    /// `Promise getEnginesByExtensionID (in AString extensionID);`
    const _GetEnginesByExtensionID: () = ();

    /// ```text
    /// /**
    ///    * Moves a visible search engine.
    ///    *
    ///    * @param  engine
    ///    *         The engine to move.
    ///    * @param  newIndex
    ///    *         The engine's new index in the set of visible engines.
    ///    *
    ///    * @throws NS_ERROR_FAILURE if newIndex is out of bounds, or if engine is
    ///    *         hidden.
    ///    */
    /// ```
    ///

    /// `Promise moveEngine (in nsISearchEngine engine, in long newIndex);`
    const _MoveEngine: () = ();

    /// ```text
    /// /**
    ///    * Removes the search engine. If the search engine is installed in a global
    ///    * location, this will just hide the engine. If the engine is in the user's
    ///    * profile directory, it will be removed from disk.
    ///    *
    ///    * @param  engine
    ///    *         The engine to remove.
    ///    */
    /// ```
    ///

    /// `Promise removeEngine (in nsISearchEngine engine);`
    const _RemoveEngine: () = ();

    /// ```text
    /// /**
    ///    * Notify nsSearchService that an extension has been removed. Removes any
    ///    * engines that are associated with that extension.
    ///    *
    ///    * @param  id
    ///    *         The id of the extension.
    ///    */
    /// ```
    ///

    /// `Promise removeWebExtensionEngine (in AString id);`
    const _RemoveWebExtensionEngine: () = ();

    /// ```text
    /// /**
    ///    * The original Engine object that is the default for this region,
    ///    * ignoring changes the user may have subsequently made.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISearchEngine originalDefaultEngine;`
    #[inline]
    pub unsafe fn GetOriginalDefaultEngine(&self, aOriginalDefaultEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalDefaultEngine)(self, aOriginalDefaultEngine)
    }


    /// ```text
    /// /**
    ///    * The original Engine object that is the default for this region when in
    ///    * private browsing mode, ignoring changes the user may have subsequently made.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISearchEngine originalPrivateDefaultEngine;`
    #[inline]
    pub unsafe fn GetOriginalPrivateDefaultEngine(&self, aOriginalPrivateDefaultEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalPrivateDefaultEngine)(self, aOriginalPrivateDefaultEngine)
    }


    /// ```text
    /// /**
    ///    * The currently active search engine.
    ///    * Unless the application doesn't ship any search plugin, this should never
    ///    * be null. If the currently active engine is removed, this attribute will
    ///    * fallback first to the original default engine if it's not hidden, then to
    ///    * the first visible engine, and as a last resort it will unhide the original
    ///    * default engine.
    ///    */
    /// ```
    ///

    /// `attribute nsISearchEngine defaultEngine;`
    #[inline]
    pub unsafe fn GetDefaultEngine(&self, aDefaultEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultEngine)(self, aDefaultEngine)
    }


    /// ```text
    /// /**
    ///    * The currently active search engine.
    ///    * Unless the application doesn't ship any search plugin, this should never
    ///    * be null. If the currently active engine is removed, this attribute will
    ///    * fallback first to the original default engine if it's not hidden, then to
    ///    * the first visible engine, and as a last resort it will unhide the original
    ///    * default engine.
    ///    */
    /// ```
    ///

    /// `attribute nsISearchEngine defaultEngine;`
    #[inline]
    pub unsafe fn SetDefaultEngine(&self, aDefaultEngine: *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultEngine)(self, aDefaultEngine)
    }



    /// `Promise getDefault ();`
    const _GetDefault: () = ();


    /// `Promise setDefault (in nsISearchEngine engine);`
    const _SetDefault: () = ();

    /// ```text
    /// /**
    ///    * The currently active search engine for private browsing mode.
    ///    * @see defaultEngine.
    ///    */
    /// ```
    ///

    /// `attribute nsISearchEngine defaultPrivateEngine;`
    #[inline]
    pub unsafe fn GetDefaultPrivateEngine(&self, aDefaultPrivateEngine: *mut *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultPrivateEngine)(self, aDefaultPrivateEngine)
    }


    /// ```text
    /// /**
    ///    * The currently active search engine for private browsing mode.
    ///    * @see defaultEngine.
    ///    */
    /// ```
    ///

    /// `attribute nsISearchEngine defaultPrivateEngine;`
    #[inline]
    pub unsafe fn SetDefaultPrivateEngine(&self, aDefaultPrivateEngine: *const nsISearchEngine) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultPrivateEngine)(self, aDefaultPrivateEngine)
    }



    /// `Promise getDefaultPrivate ();`
    const _GetDefaultPrivate: () = ();


    /// `Promise setDefaultPrivate (in nsISearchEngine engine);`
    const _SetDefaultPrivate: () = ();

    /// ```text
    /// /**
    ///    * Allows the add-on manager to discover if a WebExtension based search engine
    ///    * may change the default to an application provided search engine.
    ///    * If that WebExtension is on the allow list, then it will override the
    ///    * built-in engine's urls and parameters.
    ///    *
    ///    *  @param extension
    ///    *        The extension to load from.
    ///    *  @returns An object with two booleans:
    ///    *        - canChangeToAppProvided: indicates if the WebExtension engine may
    ///    *            set the named engine as default e.g. it is application provided.
    ///    *        - canInstallEngine: indicates if the WebExtension engine may be
    ///    *            installed, e.g. it is not an app-provided engine.
    ///    */
    /// ```
    ///

    /// `Promise maybeSetAndOverrideDefault (in jsval extension);`
    const _MaybeSetAndOverrideDefault: () = ();

    /// ```text
    /// /**
    ///    * Gets a representation of the default engine in an anonymized JSON
    ///    * string suitable for recording in the Telemetry environment.
    ///    *
    ///    * @return {object} result
    ///    *   contains anonymized info about the default engine(s).
    ///    * @return {string} result.defaultSearchEngine
    ///    *   contains the telemetry id of the default engine.
    ///    * @return {object} result.defaultSearchEngineData
    ///    *   contains information about the default engine:
    ///    *     name, loadPath, original submissionURL
    ///    * @return {string} [result.defaultPrivateSearchEngine]
    ///    *   only returned if the preference for having a separate engine in private
    ///    *   mode is turned on.
    ///    *   contains the telemetry id of the default engine for private browsing mode.
    ///    * @return {object} [result.defaultPrivateSearchEngineData]
    ///    *   only returned if the preference for having a separate engine in private
    ///    *   mode is turned on.
    ///    *   contains information about the default engine for private browsing mode:
    ///    *     name, loadPath, original submissionURL
    ///    */
    /// ```
    ///

    /// `Promise getDefaultEngineInfo ();`
    const _GetDefaultEngineInfo: () = ();

    /// ```text
    /// /**
    ///    * Determines if the provided URL represents results from a search engine, and
    ///    * provides details about the match.
    ///    *
    ///    * The lookup mechanism checks whether the domain name and path of the
    ///    * provided HTTP or HTTPS URL matches one of the known values for the visible
    ///    * search engines.  The match does not depend on which of the schemes is used.
    ///    * The expected URI parameter for the search terms must exist in the query
    ///    * string, but other parameters are ignored.
    ///    *
    ///    * @param url
    ///    *        String containing the URL to parse, for example
    ///    *        "https://www.google.com/search?q=terms".
    ///    */
    /// ```
    ///

    /// `nsISearchParseSubmissionResult parseSubmissionURL (in AString url);`
    #[inline]
    pub unsafe fn ParseSubmissionURL(&self, url: *const ::nsstring::nsAString, _retval: *mut *const nsISearchParseSubmissionResult) -> ::nserror::nsresult {
        ((*self.vtable).ParseSubmissionURL)(self, url, _retval)
    }


}


