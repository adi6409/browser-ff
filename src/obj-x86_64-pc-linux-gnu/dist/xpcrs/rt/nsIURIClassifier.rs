//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIURIClassifier.idl
//


/// `interface nsIURIClassifierCallback : nsISupports`
///

/// ```text
/// /**
///  * Callback function for nsIURIClassifier lookups.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURIClassifierCallback {
    vtable: *const nsIURIClassifierCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURIClassifierCallback.
unsafe impl XpCom for nsIURIClassifierCallback {
    const IID: nsIID = nsID(0x8face46e, 0x0c96, 0x470f,
        [0xaf, 0x40, 0x00, 0x37, 0xdc, 0xd7, 0x97, 0xbd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURIClassifierCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURIClassifierCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURIClassifierCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIURIClassifierCallback`.
    fn coerce_from(v: &nsIURIClassifierCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURIClassifierCallbackCoerce for nsIURIClassifierCallback {
    #[inline]
    fn coerce_from(v: &nsIURIClassifierCallback) -> &Self {
        v
    }
}

impl nsIURIClassifierCallback {
    /// Cast this `nsIURIClassifierCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURIClassifierCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURIClassifierCallback {
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
impl<T: nsISupportsCoerce> nsIURIClassifierCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIClassifierCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURIClassifierCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIClassifierCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aFullHash); */
    pub OnClassifyComplete: unsafe extern "system" fn (this: *const nsIURIClassifierCallback, aErrorCode: ::nserror::nsresult, aList: *const ::nsstring::nsACString, aProvider: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURIClassifierCallback {

    /// ```text
    /// /**
    ///    * Called by the URI classifier service when it is done checking a URI.
    ///    *
    ///    * Clients are responsible for associating callback objects with classify()
    ///    * calls.
    ///    *
    ///    * @param aErrorCode
    ///    *        The error code with which the channel should be cancelled, or
    ///    *        NS_OK if the load should continue normally.
    ///    * @param aList
    ///    *        Name of the list that matched
    ///    * @param aProvider
    ///    *        Name of provider that matched
    ///    * @param aFullHash
    ///    *        Full hash of URL that matched
    ///    */
    /// ```
    ///

    /// `void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aFullHash);`
    #[inline]
    pub unsafe fn OnClassifyComplete(&self, aErrorCode: ::nserror::nsresult, aList: *const ::nsstring::nsACString, aProvider: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnClassifyComplete)(self, aErrorCode, aList, aProvider, aFullHash)
    }


}


/// `interface nsIURIClassifier : nsISupports`
///

/// ```text
/// /**
///  * The URI classifier service checks a URI against lists of phishing
///  * and malware sites.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURIClassifier {
    vtable: *const nsIURIClassifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURIClassifier.
unsafe impl XpCom for nsIURIClassifier {
    const IID: nsIID = nsID(0x596620cc, 0x76e3, 0x4133,
        [0x9d, 0x90, 0x36, 0x0e, 0x59, 0xa7, 0x94, 0xcf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURIClassifier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURIClassifier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURIClassifierCoerce {
    /// Cheaply cast a value of this type from a `nsIURIClassifier`.
    fn coerce_from(v: &nsIURIClassifier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURIClassifierCoerce for nsIURIClassifier {
    #[inline]
    fn coerce_from(v: &nsIURIClassifier) -> &Self {
        v
    }
}

impl nsIURIClassifier {
    /// Cast this `nsIURIClassifier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURIClassifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURIClassifier {
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
impl<T: nsISupportsCoerce> nsIURIClassifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIClassifier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURIClassifier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIClassifierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean classify (in nsIPrincipal aPrincipal, in nsISerialEventTarget aEventTarget, in nsIURIClassifierCallback aCallback); */
    pub Classify: unsafe extern "system" fn (this: *const nsIURIClassifier, aPrincipal: *const nsIPrincipal, aEventTarget: *const nsISerialEventTarget, aCallback: *const nsIURIClassifierCallback, _retval: *mut bool) -> ::nserror::nsresult,

    /* void asyncClassifyLocalWithFeatures (in nsIURI aURI, in Array<nsIUrlClassifierFeature> aFeatures, in nsIUrlClassifierFeature_listType aListType, in nsIUrlClassifierFeatureCallback aCallback); */
    pub AsyncClassifyLocalWithFeatures: unsafe extern "system" fn (this: *const nsIURIClassifier, aURI: *const nsIURI, aFeatures: *const thin_vec::ThinVec<RefPtr<nsIUrlClassifierFeature>>, aListType:  u8, aCallback: *const nsIUrlClassifierFeatureCallback) -> ::nserror::nsresult,

    /* nsIUrlClassifierFeature getFeatureByName (in ACString aFeatureName); */
    pub GetFeatureByName: unsafe extern "system" fn (this: *const nsIURIClassifier, aFeatureName: *const ::nsstring::nsACString, _retval: *mut *const nsIUrlClassifierFeature) -> ::nserror::nsresult,

    /* Array<ACString> getFeatureNames (); */
    pub GetFeatureNames: unsafe extern "system" fn (this: *const nsIURIClassifier, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* nsIUrlClassifierFeature createFeatureWithTables (in ACString aName, in Array<ACString> aBlocklistTables, in Array<ACString> aEntitylistTables); */
    pub CreateFeatureWithTables: unsafe extern "system" fn (this: *const nsIURIClassifier, aName: *const ::nsstring::nsACString, aBlocklistTables: *const thin_vec::ThinVec<::nsstring::nsCString>, aEntitylistTables: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut *const nsIUrlClassifierFeature) -> ::nserror::nsresult,

    /* void sendThreatHitReport (in nsIChannel aChannel, in ACString aProvider, in ACString aList, in ACString aFullHash); */
    pub SendThreatHitReport: unsafe extern "system" fn (this: *const nsIURIClassifier, aChannel: *const nsIChannel, aProvider: *const ::nsstring::nsACString, aList: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURIClassifier {

    /// ```text
    /// /**
    ///    * Classify a Principal using its URI.
    ///    *
    ///    * @param aPrincipal
    ///    *        The principal that should be checked by the URI classifier.
    ///    *
    ///    * @param nsISerialEventTarget
    ///    *        Event target for constructing actor in content process.
    ///    *        The event target should be tied to Docgroup/Tabgroup by
    ///    *        using EventTargetFor
    ///    *
    ///    * @param aCallback
    ///    *        The URI classifier will call this callback when the URI has been
    ///    *        classified.
    ///    *
    ///    * @return <code>false</code> if classification is not necessary.  The
    ///    *         callback will not be called.
    ///    *         <code>true</code> if classification will be performed.  The
    ///    *         callback will be called.
    ///    */
    /// ```
    ///

    /// `boolean classify (in nsIPrincipal aPrincipal, in nsISerialEventTarget aEventTarget, in nsIURIClassifierCallback aCallback);`
    #[inline]
    pub unsafe fn Classify(&self, aPrincipal: *const nsIPrincipal, aEventTarget: *const nsISerialEventTarget, aCallback: *const nsIURIClassifierCallback, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Classify)(self, aPrincipal, aEventTarget, aCallback, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchronously classify a URI with list of features. This does not make
    ///    * network requests.
    ///    */
    /// ```
    ///

    /// `void asyncClassifyLocalWithFeatures (in nsIURI aURI, in Array<nsIUrlClassifierFeature> aFeatures, in nsIUrlClassifierFeature_listType aListType, in nsIUrlClassifierFeatureCallback aCallback);`
    #[inline]
    pub unsafe fn AsyncClassifyLocalWithFeatures(&self, aURI: *const nsIURI, aFeatures: *const thin_vec::ThinVec<RefPtr<nsIUrlClassifierFeature>>, aListType:  u8, aCallback: *const nsIUrlClassifierFeatureCallback) -> ::nserror::nsresult {
        ((*self.vtable).AsyncClassifyLocalWithFeatures)(self, aURI, aFeatures, aListType, aCallback)
    }


    /// ```text
    /// /**
    ///    * Returns a feature named aFeatureName.
    ///    */
    /// ```
    ///

    /// `nsIUrlClassifierFeature getFeatureByName (in ACString aFeatureName);`
    #[inline]
    pub unsafe fn GetFeatureByName(&self, aFeatureName: *const ::nsstring::nsACString, _retval: *mut *const nsIUrlClassifierFeature) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatureByName)(self, aFeatureName, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns all the feature names.
    ///    */
    /// ```
    ///

    /// `Array<ACString> getFeatureNames ();`
    #[inline]
    pub unsafe fn GetFeatureNames(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatureNames)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Create a new feature with a list of tables. This method is just for
    ///    * testing! Don't use it elsewhere.
    ///    */
    /// ```
    ///

    /// `nsIUrlClassifierFeature createFeatureWithTables (in ACString aName, in Array<ACString> aBlocklistTables, in Array<ACString> aEntitylistTables);`
    #[inline]
    pub unsafe fn CreateFeatureWithTables(&self, aName: *const ::nsstring::nsACString, aBlocklistTables: *const thin_vec::ThinVec<::nsstring::nsCString>, aEntitylistTables: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut *const nsIUrlClassifierFeature) -> ::nserror::nsresult {
        ((*self.vtable).CreateFeatureWithTables)(self, aName, aBlocklistTables, aEntitylistTables, _retval)
    }


    /// ```text
    /// /**
    ///    * Report to the provider that a Safe Browsing warning was shown.
    ///    *
    ///    * @param aChannel
    ///    *        Channel for which the URL matched something on the threat list.
    ///    * @param aProvider
    ///    *        Provider to notify.
    ///    * @param aList
    ///    *        List where the full hash was found.
    ///    * @param aFullHash
    ///    *        Full URL hash that triggered the warning.
    ///    */
    /// ```
    ///

    /// `void sendThreatHitReport (in nsIChannel aChannel, in ACString aProvider, in ACString aList, in ACString aFullHash);`
    #[inline]
    pub unsafe fn SendThreatHitReport(&self, aChannel: *const nsIChannel, aProvider: *const ::nsstring::nsACString, aList: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendThreatHitReport)(self, aChannel, aProvider, aList, aFullHash)
    }


}


