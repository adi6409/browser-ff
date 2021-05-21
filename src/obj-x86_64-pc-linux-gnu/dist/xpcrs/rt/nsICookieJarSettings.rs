//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieJarSettings.idl
//


/// `interface nsICookieJarSettings : nsISerializable`
///

/// ```text
/// /**
///  * Cookie jar settings for top-level documents. Please see CookieJarSettings.h
///  * for more details.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICookieJarSettings {
    vtable: *const nsICookieJarSettingsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICookieJarSettings.
unsafe impl XpCom for nsICookieJarSettings {
    const IID: nsIID = nsID(0x3ec40331, 0x7cf0, 0x4b71,
        [0xba, 0x2a, 0x22, 0x65, 0xaa, 0xb8, 0xf6, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICookieJarSettings {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICookieJarSettings.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICookieJarSettingsCoerce {
    /// Cheaply cast a value of this type from a `nsICookieJarSettings`.
    fn coerce_from(v: &nsICookieJarSettings) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICookieJarSettingsCoerce for nsICookieJarSettings {
    #[inline]
    fn coerce_from(v: &nsICookieJarSettings) -> &Self {
        v
    }
}

impl nsICookieJarSettings {
    /// Cast this `nsICookieJarSettings` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICookieJarSettingsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICookieJarSettings {
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
impl<T: nsISerializableCoerce> nsICookieJarSettingsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieJarSettings) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICookieJarSettings
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICookieJarSettingsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISerializableVTable,

    /* [infallible] readonly attribute unsigned long cookieBehavior; */
    pub GetCookieBehavior: unsafe extern "system" fn (this: *const nsICookieJarSettings, aCookieBehavior: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isFirstPartyIsolated; */
    pub GetIsFirstPartyIsolated: unsafe extern "system" fn (this: *const nsICookieJarSettings, aIsFirstPartyIsolated: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean rejectThirdPartyContexts; */
    pub GetRejectThirdPartyContexts: unsafe extern "system" fn (this: *const nsICookieJarSettings, aRejectThirdPartyContexts: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean limitForeignContexts; */
    pub GetLimitForeignContexts: unsafe extern "system" fn (this: *const nsICookieJarSettings, aLimitForeignContexts: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean partitionForeign; */
    pub GetPartitionForeign: unsafe extern "system" fn (this: *const nsICookieJarSettings, aPartitionForeign: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean partitionForeign; */
    pub SetPartitionForeign: unsafe extern "system" fn (this: *const nsICookieJarSettings, aPartitionForeign: bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isOnContentBlockingAllowList; */
    pub GetIsOnContentBlockingAllowList: unsafe extern "system" fn (this: *const nsICookieJarSettings, aIsOnContentBlockingAllowList: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString partitionKey; */
    pub GetPartitionKey: unsafe extern "system" fn (this: *const nsICookieJarSettings, aPartitionKey: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* unsigned long cookiePermission (in nsIPrincipal aPrincipal); */
    pub CookiePermission: unsafe extern "system" fn (this: *const nsICookieJarSettings, aPrincipal: *const nsIPrincipal, _retval: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICookieJarSettings {

    /// ```text
    /// /**
    ///    * CookieBehavior at the loading of the document. Any other loadInfo
    ///    * inherits it from its document's loadInfo. If there is not a document
    ///    * involved, cookieBehavior is reject.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long cookieBehavior;`
    #[inline]
    pub unsafe fn GetCookieBehavior(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCookieBehavior)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * First-Party Isolation state at the loading of the document.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isFirstPartyIsolated;`
    #[inline]
    pub unsafe fn GetIsFirstPartyIsolated(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsFirstPartyIsolated)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Whether our cookie behavior mandates rejecting third-party contexts.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean rejectThirdPartyContexts;`
    #[inline]
    pub unsafe fn GetRejectThirdPartyContexts(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetRejectThirdPartyContexts)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean limitForeignContexts;`
    #[inline]
    pub unsafe fn GetLimitForeignContexts(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLimitForeignContexts)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Whether our cookie behavior mandates partitioning third-party content.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean partitionForeign;`
    #[inline]
    pub unsafe fn GetPartitionForeign(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPartitionForeign)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Whether our cookie behavior mandates partitioning third-party content.
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean partitionForeign;`
    #[inline]
    pub unsafe fn SetPartitionForeign(&self, aPartitionForeign: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPartitionForeign)(self, aPartitionForeign)
    }


    /// ```text
    /// /**
    ///    * Whether the top-level document is on the content blocking allow list.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isOnContentBlockingAllowList;`
    #[inline]
    pub unsafe fn GetIsOnContentBlockingAllowList(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsOnContentBlockingAllowList)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * The key used for partitioning.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString partitionKey;`
    #[inline]
    pub unsafe fn GetPartitionKey(&self, aPartitionKey: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPartitionKey)(self, aPartitionKey)
    }


    /// ```text
    /// /**
    ///    * CookiePermission at the loading of the document for a particular
    ///    * principal. It returns the same cookiePermission also in case it changes
    ///    * during the life-time of the top document.
    ///    */
    /// ```
    ///

    /// `unsigned long cookiePermission (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn CookiePermission(&self, aPrincipal: *const nsIPrincipal, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).CookiePermission)(self, aPrincipal, _retval)
    }


}


