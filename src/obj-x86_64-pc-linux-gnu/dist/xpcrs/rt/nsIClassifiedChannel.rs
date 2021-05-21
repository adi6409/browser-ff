//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIClassifiedChannel.idl
//


/// `interface nsIClassifiedChannel : nsISupports`
///

/// ```text
/// /**
///  * nsIClassifiedChannel
///  *
///  * A channel may optionally implement this interface if it carries classified
///  * result information of channel classifier. The information contains, for
///  * example, the name of matched table and the name of matched provider.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClassifiedChannel {
    vtable: *const nsIClassifiedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClassifiedChannel.
unsafe impl XpCom for nsIClassifiedChannel {
    const IID: nsIID = nsID(0x70cf6091, 0xa1de, 0x4aa8,
        [0x82, 0x24, 0x05, 0x8f, 0x89, 0x64, 0xbe, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClassifiedChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClassifiedChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClassifiedChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIClassifiedChannel`.
    fn coerce_from(v: &nsIClassifiedChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClassifiedChannelCoerce for nsIClassifiedChannel {
    #[inline]
    fn coerce_from(v: &nsIClassifiedChannel) -> &Self {
        v
    }
}

impl nsIClassifiedChannel {
    /// Cast this `nsIClassifiedChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClassifiedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClassifiedChannel {
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
impl<T: nsISupportsCoerce> nsIClassifiedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClassifiedChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClassifiedChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClassifiedChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash); */
    pub SetMatchedInfo: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aList: *const ::nsstring::nsACString, aProvider: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString matchedList; */
    pub GetMatchedList: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aMatchedList: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString matchedProvider; */
    pub GetMatchedProvider: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aMatchedProvider: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString matchedFullHash; */
    pub GetMatchedFullHash: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aMatchedFullHash: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setMatchedTrackingInfo (in Array<ACString> aLists, in Array<ACString> aFullHashes); */
    pub SetMatchedTrackingInfo: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aLists: *const thin_vec::ThinVec<::nsstring::nsCString>, aFullHashes: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> matchedTrackingLists; */
    pub GetMatchedTrackingLists: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aMatchedTrackingLists: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> matchedTrackingFullHashes; */
    pub GetMatchedTrackingFullHashes: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aMatchedTrackingFullHashes: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long firstPartyClassificationFlags; */
    pub GetFirstPartyClassificationFlags: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aFirstPartyClassificationFlags: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long thirdPartyClassificationFlags; */
    pub GetThirdPartyClassificationFlags: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aThirdPartyClassificationFlags: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long classificationFlags; */
    pub GetClassificationFlags: unsafe extern "system" fn (this: *const nsIClassifiedChannel, aClassificationFlags: *mut u32) -> ::nserror::nsresult,

    /* boolean isThirdPartyTrackingResource (); */
    pub IsThirdPartyTrackingResource: unsafe extern "system" fn (this: *const nsIClassifiedChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isThirdPartySocialTrackingResource (); */
    pub IsThirdPartySocialTrackingResource: unsafe extern "system" fn (this: *const nsIClassifiedChannel, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClassifiedChannel {

    /// ```text
    /// /**
    ///    * Sets matched info of the classified channel.
    ///    *
    ///    * @param aList
    ///    *        Name of the Safe Browsing list that matched (e.g. goog-phish-shavar).
    ///    * @param aProvider
    ///    *        Name of the Safe Browsing provider that matched (e.g. google)
    ///    * @param aFullHash
    ///    *        Full hash of URL that matched Safe Browsing list.
    ///    */
    /// ```
    ///

    /// `void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash);`
    #[inline]
    pub unsafe fn SetMatchedInfo(&self, aList: *const ::nsstring::nsACString, aProvider: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetMatchedInfo)(self, aList, aProvider, aFullHash)
    }


    /// ```text
    /// /**
    ///    * Name of the list that matched
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString matchedList;`
    #[inline]
    pub unsafe fn GetMatchedList(&self, aMatchedList: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchedList)(self, aMatchedList)
    }


    /// ```text
    /// /**
    ///    * Name of provider that matched
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString matchedProvider;`
    #[inline]
    pub unsafe fn GetMatchedProvider(&self, aMatchedProvider: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchedProvider)(self, aMatchedProvider)
    }


    /// ```text
    /// /**
    ///    * Full hash of URL that matched
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString matchedFullHash;`
    #[inline]
    pub unsafe fn GetMatchedFullHash(&self, aMatchedFullHash: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchedFullHash)(self, aMatchedFullHash)
    }


    /// ```text
    /// /**
    ///    * Sets matched tracking info of the classified channel.
    ///    *
    ///    * @param aLists
    ///    *        Name of the Tracking Protection list that matched (e.g. content-track-digest256).
    ///    * @param aFullHash
    ///    *        Full hash of URLs that matched Tracking Protection list.
    ///    */
    /// ```
    ///

    /// `void setMatchedTrackingInfo (in Array<ACString> aLists, in Array<ACString> aFullHashes);`
    #[inline]
    pub unsafe fn SetMatchedTrackingInfo(&self, aLists: *const thin_vec::ThinVec<::nsstring::nsCString>, aFullHashes: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).SetMatchedTrackingInfo)(self, aLists, aFullHashes)
    }


    /// ```text
    /// /**
    ///    * Name of the lists that matched
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> matchedTrackingLists;`
    #[inline]
    pub unsafe fn GetMatchedTrackingLists(&self, aMatchedTrackingLists: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchedTrackingLists)(self, aMatchedTrackingLists)
    }


    /// ```text
    /// /**
    ///    * Full hash of URLs that matched
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> matchedTrackingFullHashes;`
    #[inline]
    pub unsafe fn GetMatchedTrackingFullHashes(&self, aMatchedTrackingFullHashes: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchedTrackingFullHashes)(self, aMatchedTrackingFullHashes)
    }


    /// ```text
    /// /**
    ///    * Returns the classification flags if the channel has been processed by
    ///    * URL-Classifier features and is considered first-party.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long firstPartyClassificationFlags;`
    #[inline]
    pub unsafe fn GetFirstPartyClassificationFlags(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetFirstPartyClassificationFlags)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Returns the classification flags if the channel has been processed by
    ///    * URL-Classifier features and is considered third-party with the top
    ///    * window URI.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long thirdPartyClassificationFlags;`
    #[inline]
    pub unsafe fn GetThirdPartyClassificationFlags(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetThirdPartyClassificationFlags)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute unsigned long classificationFlags;`
    #[inline]
    pub unsafe fn GetClassificationFlags(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetClassificationFlags)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Returns true  if the channel has been processed by URL-Classifier features
    ///    * and is considered third-party with the top window URI, and if it has loaded
    ///    * a resource that is classified as a tracker.
    ///    *
    ///    * This is a helper attribute which returns the same value of
    ///    * (thirdPartyClassificationFlags & CLASSIFIED_ANY_BASIC_TRACKING) or
    ///    * (thirdPartyClassificationFlags & CLASSIFIED_ANY_STRICT_TRACKING) or
    ///    * (thirdPartyClassificationFlags & CLASSIFIED_ANY_SOCIAL_TRACKING)
    ///    */
    /// ```
    ///

    /// `boolean isThirdPartyTrackingResource ();`
    #[inline]
    pub unsafe fn IsThirdPartyTrackingResource(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsThirdPartyTrackingResource)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the channel has loaded a 3rd party resource that is
    ///    * classified as a social tracker.
    ///    *
    ///    * This is a helper attribute which returns the same value of
    ///    * (classificationFlags & CLASSIFIED_ANY_SOCIAL_TRACKING)
    ///    *
    ///    * Note that top-level channels could be marked as tracking
    ///    * resources. In order to identify third-party social tracking resources
    ///    * specifically, check the flags manually or add a new helper here.
    ///    */
    /// ```
    ///

    /// `boolean isThirdPartySocialTrackingResource ();`
    #[inline]
    pub unsafe fn IsThirdPartySocialTrackingResource(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsThirdPartySocialTrackingResource)(self, _retval)
    }


}


