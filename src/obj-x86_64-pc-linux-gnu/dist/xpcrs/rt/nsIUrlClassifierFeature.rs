//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIUrlClassifierFeature.idl
//


/// `interface nsIUrlClassifierFeature : nsISupports`
///

/// ```text
/// /**
///  * A single URLClassifier feature.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierFeature {
    vtable: *const nsIUrlClassifierFeatureVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierFeature.
unsafe impl XpCom for nsIUrlClassifierFeature {
    const IID: nsIID = nsID(0xa6c9b24e, 0xb4f1, 0x426e,
        [0xaf, 0x58, 0x2c, 0x97, 0x6c, 0x39, 0x43, 0xa8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierFeature {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierFeature.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierFeatureCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierFeature`.
    fn coerce_from(v: &nsIUrlClassifierFeature) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierFeatureCoerce for nsIUrlClassifierFeature {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierFeature) -> &Self {
        v
    }
}

impl nsIUrlClassifierFeature {
    /// Cast this `nsIUrlClassifierFeature` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierFeatureCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierFeature {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierFeatureCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierFeature) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierFeature
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierFeatureVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIUrlClassifierFeature, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] StringArrayRef getTables (in nsIUrlClassifierFeature_listType aListType); */
    /// Unable to generate binding because `native type nsTArray<nsCString> unsupported`
    pub GetTables: *const ::libc::c_void,

    /* [noscript] boolean hasTable (in ACString aTable, in nsIUrlClassifierFeature_listType aListType); */
    pub HasTable: unsafe extern "system" fn (this: *const nsIUrlClassifierFeature, aTable: *const ::nsstring::nsACString, aListType:  u8, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] boolean hasHostInPreferences (in ACString aHost, in nsIUrlClassifierFeature_listType aListType, out ACString aPrefTableName); */
    pub HasHostInPreferences: unsafe extern "system" fn (this: *const nsIUrlClassifierFeature, aHost: *const ::nsstring::nsACString, aListType:  u8, aPrefTableName: *mut ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute ACString exceptionHostList; */
    pub GetExceptionHostList: unsafe extern "system" fn (this: *const nsIUrlClassifierFeature, aExceptionHostList: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] boolean processChannel (in nsIChannel aChannel, in ConstStringArrayRef aList, in ConstStringArrayRef aHashes); */
    /// Unable to generate binding because `native type const nsTArray<nsCString> unsupported`
    pub ProcessChannel: *const ::libc::c_void,

    /* [noscript] nsIURI getURIByListType (in nsIChannel channel, in nsIUrlClassifierFeature_listType listType, out nsIUrlClassifierFeature_URIType URIType); */
    pub GetURIByListType: unsafe extern "system" fn (this: *const nsIUrlClassifierFeature, channel: *const nsIChannel, listType:  u8, URIType: *mut u8, _retval: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierFeature {

    /// ```text
    /// /**
    ///    * The feature name
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * Returns the tables for one of the possible lists.
    ///    */
    /// ```
    ///

    /// `[noscript] StringArrayRef getTables (in nsIUrlClassifierFeature_listType aListType);`
    const _GetTables: () = ();

    /// ```text
    /// /**
    ///    * Returns true if |aTable| is part of the tables of |aListType| type.
    ///    */
    /// ```
    ///

    /// `[noscript] boolean hasTable (in ACString aTable, in nsIUrlClassifierFeature_listType aListType);`
    #[inline]
    pub unsafe fn HasTable(&self, aTable: *const ::nsstring::nsACString, aListType:  u8, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasTable)(self, aTable, aListType, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if |aHost| is contained in the preference of |aListType| type.
    ///    * |aPrefTableName| will be set to the table name to use.
    ///    */
    /// ```
    ///

    /// `[noscript] boolean hasHostInPreferences (in ACString aHost, in nsIUrlClassifierFeature_listType aListType, out ACString aPrefTableName);`
    #[inline]
    pub unsafe fn HasHostInPreferences(&self, aHost: *const ::nsstring::nsACString, aListType:  u8, aPrefTableName: *mut ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasHostInPreferences)(self, aHost, aListType, aPrefTableName, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns a comma-separated list of hosts to be ignored.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString exceptionHostList;`
    #[inline]
    pub unsafe fn GetExceptionHostList(&self, aExceptionHostList: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetExceptionHostList)(self, aExceptionHostList)
    }


    /// ```text
    /// /**
    ///    * When this feature matches the channel, this method is executed to do
    ///    * 'something' on the channel. For instance, a tracking-annotation feature
    ///    * would mark the channel as tracker, a tracking-protection feature would
    ///    * cancel the channel.
    ///    * Returns if we should process other feature results or not. For instance,
    ///    * tracking-protection cancel the channel, and after that we should stop
    ///    * processing other features.
    ///    */
    /// ```
    ///

    /// `[noscript] boolean processChannel (in nsIChannel aChannel, in ConstStringArrayRef aList, in ConstStringArrayRef aHashes);`
    const _ProcessChannel: () = ();

    /// ```text
    /// /**
    ///    * Features can work with different URLs from a channel (channel url, or
        ///    * top-level, or something else). This method returns what we need to use for
    ///    * the current list.
    ///    * If the returned URI is created by CreatePairwiseEntityListURI(), the
    ///    * URIType is pairwiseEntitylistURI. Otherwise, it depends on the listType.
    ///    */
    /// ```
    ///

    /// `[noscript] nsIURI getURIByListType (in nsIChannel channel, in nsIUrlClassifierFeature_listType listType, out nsIUrlClassifierFeature_URIType URIType);`
    #[inline]
    pub unsafe fn GetURIByListType(&self, channel: *const nsIChannel, listType:  u8, URIType: *mut u8, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURIByListType)(self, channel, listType, URIType, _retval)
    }


}


/// `interface nsIUrlClassifierFeatureResult : nsISupports`
///

/// ```text
/// /**
///  * The result of the classifier operation is this interface.
///  * See asyncClassifyLocalWithFeatures() in nsIURIClassifier.idl.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierFeatureResult {
    vtable: *const nsIUrlClassifierFeatureResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierFeatureResult.
unsafe impl XpCom for nsIUrlClassifierFeatureResult {
    const IID: nsIID = nsID(0xccb88140, 0x5d66, 0x4873,
        [0x98, 0x15, 0xa1, 0xb9, 0x8d, 0x6c, 0xdc, 0x92]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierFeatureResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierFeatureResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierFeatureResultCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierFeatureResult`.
    fn coerce_from(v: &nsIUrlClassifierFeatureResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierFeatureResultCoerce for nsIUrlClassifierFeatureResult {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierFeatureResult) -> &Self {
        v
    }
}

impl nsIUrlClassifierFeatureResult {
    /// Cast this `nsIUrlClassifierFeatureResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierFeatureResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierFeatureResult {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierFeatureResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierFeatureResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierFeatureResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierFeatureResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI uri; */
    pub GetUri: unsafe extern "system" fn (this: *const nsIUrlClassifierFeatureResult, aUri: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIUrlClassifierFeature feature; */
    pub GetFeature: unsafe extern "system" fn (this: *const nsIUrlClassifierFeatureResult, aFeature: *mut *const nsIUrlClassifierFeature) -> ::nserror::nsresult,

    /* readonly attribute ACString list; */
    pub GetList: unsafe extern "system" fn (this: *const nsIUrlClassifierFeatureResult, aList: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierFeatureResult {


    /// `readonly attribute nsIURI uri;`
    #[inline]
    pub unsafe fn GetUri(&self, aUri: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetUri)(self, aUri)
    }



    /// `readonly attribute nsIUrlClassifierFeature feature;`
    #[inline]
    pub unsafe fn GetFeature(&self, aFeature: *mut *const nsIUrlClassifierFeature) -> ::nserror::nsresult {
        ((*self.vtable).GetFeature)(self, aFeature)
    }



    /// `readonly attribute ACString list;`
    #[inline]
    pub unsafe fn GetList(&self, aList: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetList)(self, aList)
    }


}


/// `interface nsIUrlClassifierFeatureCallback : nsISupports`
///

/// ```text
/// /**
///  * Callback function for nsIURIClassifier lookups.
///  * See asyncClassifyLocalWithFeatures() in nsIURIClassifier.idl.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierFeatureCallback {
    vtable: *const nsIUrlClassifierFeatureCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierFeatureCallback.
unsafe impl XpCom for nsIUrlClassifierFeatureCallback {
    const IID: nsIID = nsID(0x2ea83c26, 0xdfc9, 0x44ed,
        [0x9c, 0xfc, 0x17, 0x1d, 0x47, 0x53, 0xd7, 0x8e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierFeatureCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierFeatureCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierFeatureCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierFeatureCallback`.
    fn coerce_from(v: &nsIUrlClassifierFeatureCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierFeatureCallbackCoerce for nsIUrlClassifierFeatureCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierFeatureCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierFeatureCallback {
    /// Cast this `nsIUrlClassifierFeatureCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierFeatureCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierFeatureCallback {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierFeatureCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierFeatureCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierFeatureCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierFeatureCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onClassifyComplete (in Array<nsIUrlClassifierFeatureResult> aResults); */
    pub OnClassifyComplete: unsafe extern "system" fn (this: *const nsIUrlClassifierFeatureCallback, aResults: *const thin_vec::ThinVec<RefPtr<nsIUrlClassifierFeatureResult>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierFeatureCallback {

    /// ```text
    /// /**
    ///    * Called by the URI classifier service when it is done checking a URI.
    ///    *
    ///    * Clients are responsible for associating callback objects with classify()
    ///    * calls.
    ///    *
    ///    * @param aResults
    ///    *        List of nsIUrlClassifierFeatureResult objects.
    ///    */
    /// ```
    ///

    /// `void onClassifyComplete (in Array<nsIUrlClassifierFeatureResult> aResults);`
    #[inline]
    pub unsafe fn OnClassifyComplete(&self, aResults: *const thin_vec::ThinVec<RefPtr<nsIUrlClassifierFeatureResult>>) -> ::nserror::nsresult {
        ((*self.vtable).OnClassifyComplete)(self, aResults)
    }


}


