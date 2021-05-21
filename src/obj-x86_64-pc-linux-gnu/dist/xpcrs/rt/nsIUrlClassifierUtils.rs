//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierUtils.idl
//


/// `interface nsIUrlClassifierParseFindFullHashCallback : nsISupports`
///

/// ```text
/// /**
///  * Interface for parseFindFullHashResponseV4 callback
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierParseFindFullHashCallback {
    vtable: *const nsIUrlClassifierParseFindFullHashCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierParseFindFullHashCallback.
unsafe impl XpCom for nsIUrlClassifierParseFindFullHashCallback {
    const IID: nsIID = nsID(0xfbb9684a, 0xa0aa, 0x11e6,
        [0x88, 0xb0, 0x08, 0x60, 0x6e, 0x45, 0x6b, 0x8a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierParseFindFullHashCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierParseFindFullHashCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierParseFindFullHashCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierParseFindFullHashCallback`.
    fn coerce_from(v: &nsIUrlClassifierParseFindFullHashCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierParseFindFullHashCallbackCoerce for nsIUrlClassifierParseFindFullHashCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierParseFindFullHashCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierParseFindFullHashCallback {
    /// Cast this `nsIUrlClassifierParseFindFullHashCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierParseFindFullHashCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierParseFindFullHashCallback {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierParseFindFullHashCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierParseFindFullHashCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierParseFindFullHashCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierParseFindFullHashCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration); */
    pub OnCompleteHashFound: unsafe extern "system" fn (this: *const nsIUrlClassifierParseFindFullHashCallback, aCompleteHash: *const ::nsstring::nsACString, aTableNames: *const ::nsstring::nsACString, aPerHashCacheDuration: u32) -> ::nserror::nsresult,

    /* void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration); */
    pub OnResponseParsed: unsafe extern "system" fn (this: *const nsIUrlClassifierParseFindFullHashCallback, aMinWaitDuration: u32, aNegCacheDuration: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierParseFindFullHashCallback {

    /// ```text
    /// /**
    ///    * Callback when a match is found in full hash response. This callback may be
    ///    * called multiple times when there are more than one matches in response.
    ///    *
    ///    * @param aCompleteHash A 32-byte complete hash string.
    ///    * @param aTableNames The table names that this complete hash is associated with.
    ///    *                    Since the server responded with a threat type, multiple
    ///    *                    list names can be returned. The caller is reponsible
    ///    *                    for filtering out the unrequested table names.
    ///    *                    See |convertThreatTypeToListNames| for the format.
    ///    * @param aPerHashCacheDuration See "FindFullHashesResponse" in safebrowsing.proto.
    ///    *
    ///    */
    /// ```
    ///

    /// `void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration);`
    #[inline]
    pub unsafe fn OnCompleteHashFound(&self, aCompleteHash: *const ::nsstring::nsACString, aTableNames: *const ::nsstring::nsACString, aPerHashCacheDuration: u32) -> ::nserror::nsresult {
        ((*self.vtable).OnCompleteHashFound)(self, aCompleteHash, aTableNames, aPerHashCacheDuration)
    }


    /// ```text
    /// /**
    ///    * Callback when full hash response is received.
    ///    *
    ///    * @param aMinWaitDuration See "FindFullHashesResponse" in safebrowsing.proto.
    ///    * @param aNegCacheDuration See "FindFullHashesResponse" in safebrowsing.proto.
    ///    *
    ///    */
    /// ```
    ///

    /// `void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration);`
    #[inline]
    pub unsafe fn OnResponseParsed(&self, aMinWaitDuration: u32, aNegCacheDuration: u32) -> ::nserror::nsresult {
        ((*self.vtable).OnResponseParsed)(self, aMinWaitDuration, aNegCacheDuration)
    }


}


/// `interface nsIUrlClassifierUtils : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierUtils {
    vtable: *const nsIUrlClassifierUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierUtils.
unsafe impl XpCom for nsIUrlClassifierUtils {
    const IID: nsIID = nsID(0xe4f0e59c, 0xb922, 0x48b0,
        [0xa7, 0xb6, 0x17, 0x35, 0xc1, 0xf9, 0x6f, 0xed]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierUtils {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierUtils.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierUtilsCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierUtils`.
    fn coerce_from(v: &nsIUrlClassifierUtils) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierUtilsCoerce for nsIUrlClassifierUtils {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierUtils) -> &Self {
        v
    }
}

impl nsIUrlClassifierUtils {
    /// Cast this `nsIUrlClassifierUtils` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierUtils {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierUtils) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierUtils
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierUtilsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getKeyForURI (in nsIURI uri); */
    pub GetKeyForURI: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, uri: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getProvider (in ACString tableName); */
    pub GetProvider: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getTelemetryProvider (in ACString tableName); */
    pub GetTelemetryProvider: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getProtocolVersion (in ACString provider); */
    pub GetProtocolVersion: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, provider: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString convertThreatTypeToListNames (in uint32_t threatType); */
    pub ConvertThreatTypeToListNames: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, threatType: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* uint32_t convertListNameToThreatType (in ACString listName); */
    pub ConvertListNameToThreatType: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, listName: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* ACString makeUpdateRequestV4 (in Array<ACString> aListNames, in Array<ACString> aStatesBase64); */
    pub MakeUpdateRequestV4: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, aListNames: *const thin_vec::ThinVec<::nsstring::nsCString>, aStatesBase64: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString makeFindFullHashRequestV4 (in Array<ACString> aListNames, in Array<ACString> aListStatesBase64, in Array<ACString> aPrefixes); */
    pub MakeFindFullHashRequestV4: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, aListNames: *const thin_vec::ThinVec<::nsstring::nsCString>, aListStatesBase64: *const thin_vec::ThinVec<::nsstring::nsCString>, aPrefixes: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString makeThreatHitReport (in nsIChannel aChannel, in ACString aListName, in ACString aHashBase64); */
    pub MakeThreatHitReport: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, aChannel: *const nsIChannel, aListName: *const ::nsstring::nsACString, aHashBase64: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void parseFindFullHashResponseV4 (in ACString aResponse, in nsIUrlClassifierParseFindFullHashCallback aCallback); */
    pub ParseFindFullHashResponseV4: unsafe extern "system" fn (this: *const nsIUrlClassifierUtils, aResponse: *const ::nsstring::nsACString, aCallback: *const nsIUrlClassifierParseFindFullHashCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierUtils {

    /// ```text
    /// /**
    ///    * Get the lookup string for a given URI.  This normalizes the hostname,
    ///    * url-decodes the string, and strips off the protocol.
    ///    *
    ///    * @param uri URI to get the lookup key for.
    ///    *
    ///    * @returns String containing the canonicalized URI.
    ///    */
    /// ```
    ///

    /// `ACString getKeyForURI (in nsIURI uri);`
    #[inline]
    pub unsafe fn GetKeyForURI(&self, uri: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeyForURI)(self, uri, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the provider by table name.
    ///    *
    ///    * @param tableName The table name that we want to lookup
    ///    *
    ///    * @returns the provider name that the given table belongs.
    ///    */
    /// ```
    ///

    /// `ACString getProvider (in ACString tableName);`
    #[inline]
    pub unsafe fn GetProvider(&self, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProvider)(self, tableName, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the provider used for Telemetry.
    ///    * Because recording Telemetry will leak user-controlled strings,
    ///    * only built-in providers should be recorded.
    ///    *
    ///    * @param tableName The table name that we want to lookup
    ///    *
    ///    * @returns the filtered provider for telemetry.
    ///    *
    ///    */
    /// ```
    ///

    /// `ACString getTelemetryProvider (in ACString tableName);`
    #[inline]
    pub unsafe fn GetTelemetryProvider(&self, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTelemetryProvider)(self, tableName, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the protocol version for the given provider.
    ///    *
    ///    * @param provider String the provider name. e.g. "google"
    ///    *
    ///    * @returns String to indicate the protocol version. e.g. "2.2"
    ///    */
    /// ```
    ///

    /// `ACString getProtocolVersion (in ACString provider);`
    #[inline]
    pub unsafe fn GetProtocolVersion(&self, provider: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolVersion)(self, provider, _retval)
    }


    /// ```text
    /// /**
    ///    * Convert threat type to list name.
    ///    *
    ///    * @param Integer to indicate threat type.
    ///    *
    ///    * @returns The list names separated by ','. For example,
    ///    *          'goog-phish-proto,test-phish-proto'.
    ///    */
    /// ```
    ///

    /// `ACString convertThreatTypeToListNames (in uint32_t threatType);`
    #[inline]
    pub unsafe fn ConvertThreatTypeToListNames(&self, threatType: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertThreatTypeToListNames)(self, threatType, _retval)
    }


    /// ```text
    /// /**
    ///    * Convert list name to threat type.
    ///    *
    ///    * @param The list name.
    ///    *
    ///    * @returns The threat type in integer.
    ///    */
    /// ```
    ///

    /// `uint32_t convertListNameToThreatType (in ACString listName);`
    #[inline]
    pub unsafe fn ConvertListNameToThreatType(&self, listName: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).ConvertListNameToThreatType)(self, listName, _retval)
    }


    /// ```text
    /// /**
    ///    * Make update request for given lists and their states.
    ///    *
    ///    * @param aListNames An array of list name represented in string.
    ///    * @param aState An array of states (encoded in base64 format) for each list.
    ///    *
    ///    * The two argument arrays must be the same length.
    ///    *
    ///    * @returns A base64url encoded string.
    ///    */
    /// ```
    ///

    /// `ACString makeUpdateRequestV4 (in Array<ACString> aListNames, in Array<ACString> aStatesBase64);`
    #[inline]
    pub unsafe fn MakeUpdateRequestV4(&self, aListNames: *const thin_vec::ThinVec<::nsstring::nsCString>, aStatesBase64: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).MakeUpdateRequestV4)(self, aListNames, aStatesBase64, _retval)
    }


    /// ```text
    /// /**
    ///    * Make "find full hash" request by for the given prefixes.
    ///    *
    ///    * @param aListNames An array of list names represented in string.
    ///    * @param aListStatesBase64 An array of list states represented in base64.
    ///    * @param aPrefixes An array of prefixes for which we'd like to find full hashes..
    ///    *
    ///    * The aListNames and aListStatesBase64 arrays must be the same length.
    ///    *
    ///    * @returns A base64url encoded string.
    ///    */
    /// ```
    ///

    /// `ACString makeFindFullHashRequestV4 (in Array<ACString> aListNames, in Array<ACString> aListStatesBase64, in Array<ACString> aPrefixes);`
    #[inline]
    pub unsafe fn MakeFindFullHashRequestV4(&self, aListNames: *const thin_vec::ThinVec<::nsstring::nsCString>, aListStatesBase64: *const thin_vec::ThinVec<::nsstring::nsCString>, aPrefixes: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).MakeFindFullHashRequestV4)(self, aListNames, aListStatesBase64, aPrefixes, _retval)
    }


    /// ```text
    /// /**
    ///    * Make ThreatHit report request body.
    ///    *
    ///    * @param aChannel channel which encountered the threat.
    ///    * @param aListName listname represented in string.
    ///    * @param aHashBase64 hash-based hit represented in base64.
    ///    *
    ///    * @returns A base64 encoded string.
    ///    */
    /// ```
    ///

    /// `ACString makeThreatHitReport (in nsIChannel aChannel, in ACString aListName, in ACString aHashBase64);`
    #[inline]
    pub unsafe fn MakeThreatHitReport(&self, aChannel: *const nsIChannel, aListName: *const ::nsstring::nsACString, aHashBase64: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).MakeThreatHitReport)(self, aChannel, aListName, aHashBase64, _retval)
    }


    /// ```text
    /// /**
    ///    * Parse V4 FindFullHash response.
    ///    *
    ///    * @param aResponse Byte stream from the server.
    ///    * @param aCallback The callback function on each complete hash parsed.
    ///    *                  Can be called multiple times in one parsing.
    ///    */
    /// ```
    ///

    /// `void parseFindFullHashResponseV4 (in ACString aResponse, in nsIUrlClassifierParseFindFullHashCallback aCallback);`
    #[inline]
    pub unsafe fn ParseFindFullHashResponseV4(&self, aResponse: *const ::nsstring::nsACString, aCallback: *const nsIUrlClassifierParseFindFullHashCallback) -> ::nserror::nsresult {
        ((*self.vtable).ParseFindFullHashResponseV4)(self, aResponse, aCallback)
    }


}


