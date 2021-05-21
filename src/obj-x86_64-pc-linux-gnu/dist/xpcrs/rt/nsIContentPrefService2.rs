//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentPrefService2.idl
//


/// `interface nsIContentPrefObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPrefObserver {
    vtable: *const nsIContentPrefObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPrefObserver.
unsafe impl XpCom for nsIContentPrefObserver {
    const IID: nsIID = nsID(0x43635c53, 0xb445, 0x4c4e,
        [0x8c, 0xc5, 0x56, 0x26, 0x97, 0x29, 0x9b, 0x55]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPrefObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPrefObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPrefObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIContentPrefObserver`.
    fn coerce_from(v: &nsIContentPrefObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPrefObserverCoerce for nsIContentPrefObserver {
    #[inline]
    fn coerce_from(v: &nsIContentPrefObserver) -> &Self {
        v
    }
}

impl nsIContentPrefObserver {
    /// Cast this `nsIContentPrefObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPrefObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPrefObserver {
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
impl<T: nsISupportsCoerce> nsIContentPrefObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPrefObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPrefObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate); */
    pub OnContentPrefSet: unsafe extern "system" fn (this: *const nsIContentPrefObserver, aGroup: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aValue: *const nsIVariant, aIsPrivate: bool) -> ::nserror::nsresult,

    /* void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate); */
    pub OnContentPrefRemoved: unsafe extern "system" fn (this: *const nsIContentPrefObserver, aGroup: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aIsPrivate: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPrefObserver {

    /// ```text
    /// /**
    ///    * Called when a content pref is set to a different value.
    ///    *
    ///    * @param    aGroup      the group to which the pref belongs, or null
    ///    *                       if it's a global pref (applies to all sites)
    ///    * @param    aName       the name of the pref that was set
    ///    * @param    aValue      the new value of the pref
    ///    * @param    aIsPrivate  an optional flag determining whether the
    ///    *                       original context is private or not
    ///    */
    /// ```
    ///

    /// `void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate);`
    #[inline]
    pub unsafe fn OnContentPrefSet(&self, aGroup: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aValue: *const nsIVariant, aIsPrivate: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnContentPrefSet)(self, aGroup, aName, aValue, aIsPrivate)
    }


    /// ```text
    /// /**
    ///    * Called when a content pref is removed.
    ///    *
    ///    * @param    aGroup      the group to which the pref belongs, or null
    ///    *                       if it's a global pref (applies to all sites)
    ///    * @param    aName       the name of the pref that was removed
    ///    * @param    aIsPrivate  an optional flag determining whether the
    ///    *                       original context is private or not
    ///    */
    /// ```
    ///

    /// `void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate);`
    #[inline]
    pub unsafe fn OnContentPrefRemoved(&self, aGroup: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aIsPrivate: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnContentPrefRemoved)(self, aGroup, aName, aIsPrivate)
    }


}


/// `interface nsIContentPrefService2 : nsISupports`
///

/// ```text
/// /**
///  * Content Preferences
///  *
///  * Content preferences allow the application to associate arbitrary data, or
///  * "preferences", with specific domains, or web "content".  Specifically, a
///  * content preference is a structure with three values: a domain with which the
///  * preference is associated, a name that identifies the preference within its
///  * domain, and a value.  (See nsIContentPref below.)
///  *
///  * For example, if you want to remember the user's preference for a certain zoom
///  * level on www.mozilla.org pages, you might store a preference whose domain is
///  * "www.mozilla.org", whose name is "zoomLevel", and whose value is the numeric
///  * zoom level.
///  *
///  * A preference need not have a domain, and in that case the preference is
///  * called a "global" preference.  This interface doesn't impart any special
///  * significance to global preferences; they're simply name-value pairs that
///  * aren't associated with any particular domain.  As a consumer of this
///  * interface, you might choose to let a global preference override all non-
///  * global preferences of the same name, for example, for whatever definition of
///  * "override" is appropriate for your use case.
///  *
///  *
///  * Domain Parameters
///  *
///  * Many methods of this interface accept a "domain" parameter.  Domains may be
///  * specified either exactly, like "example.com", or as full URLs, like
///  * "http://example.com/foo/bar".  In the latter case the API extracts the full
///  * domain from the URL, so if you specify "http://foo.bar.example.com/baz", the
///  * domain is taken to be "foo.bar.example.com", not "example.com".
///  *
///  *
///  * Private-Browsing Context Parameters
///  *
///  * Many methods also accept a "context" parameter.  This parameter relates to
///  * private browsing and determines the kind of storage that a method uses,
///  * either the usual permanent storage or temporary storage set aside for private
///  * browsing sessions.
///  *
///  * Pass null to unconditionally use permanent storage.  Pass an nsILoadContext
///  * to use storage appropriate to the context's usePrivateBrowsing attribute: if
///  * usePrivateBrowsing is true, temporary private-browsing storage is used, and
///  * otherwise permanent storage is used.  A context can be obtained from the
///  * window or channel whose content pertains to the preferences being modified or
///  * retrieved.
///  *
///  *
///  * Callbacks
///  *
///  * The methods of callback objects are always called asynchronously.
///  *
///  * Observers are called after callbacks are called, but they are called in the
///  * same turn of the event loop as callbacks.
///  *
///  * See nsIContentPrefCallback2 below for more information about callbacks.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPrefService2 {
    vtable: *const nsIContentPrefService2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPrefService2.
unsafe impl XpCom for nsIContentPrefService2 {
    const IID: nsIID = nsID(0xbed98666, 0xd995, 0x470f,
        [0xbe, 0xbd, 0x62, 0x47, 0x6d, 0x31, 0x85, 0x76]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPrefService2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPrefService2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPrefService2Coerce {
    /// Cheaply cast a value of this type from a `nsIContentPrefService2`.
    fn coerce_from(v: &nsIContentPrefService2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPrefService2Coerce for nsIContentPrefService2 {
    #[inline]
    fn coerce_from(v: &nsIContentPrefService2) -> &Self {
        v
    }
}

impl nsIContentPrefService2 {
    /// Cast this `nsIContentPrefService2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPrefService2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPrefService2 {
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
impl<T: nsISupportsCoerce> nsIContentPrefService2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefService2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPrefService2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPrefService2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getByName (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub GetByName: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void getByDomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub GetByDomainAndName: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void getBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub GetBySubdomainAndName: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void getGlobal (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub GetGlobal: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* nsIContentPref getCachedByDomainAndName (in AString domain, in AString name, in nsILoadContext context); */
    pub GetCachedByDomainAndName: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, _retval: *mut*const nsIContentPref) -> ::nserror::nsresult,

    /* Array<nsIContentPref> getCachedBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context); */
    pub GetCachedBySubdomainAndName: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, _retval: *mut thin_vec::ThinVec<RefPtr<nsIContentPref>>) -> ::nserror::nsresult,

    /* nsIContentPref getCachedGlobal (in AString name, in nsILoadContext context); */
    pub GetCachedGlobal: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, context: *const nsILoadContext, _retval: *mut*const nsIContentPref) -> ::nserror::nsresult,

    /* void set (in AString domain, in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub Set: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, value: *const nsIVariant, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void setGlobal (in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub SetGlobal: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, value: *const nsIVariant, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeByDomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveByDomainAndName: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveBySubdomainAndName: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeGlobal (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveGlobal: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeByDomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveByDomain: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeBySubdomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveBySubdomain: unsafe extern "system" fn (this: *const nsIContentPrefService2, domain: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeByName (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveByName: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeAllDomains (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveAllDomains: unsafe extern "system" fn (this: *const nsIContentPrefService2, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeAllDomainsSince (in unsigned long long since, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveAllDomainsSince: unsafe extern "system" fn (this: *const nsIContentPrefService2, since: u64, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void removeAllGlobals (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub RemoveAllGlobals: unsafe extern "system" fn (this: *const nsIContentPrefService2, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult,

    /* void addObserverForName (in AString name, in nsIContentPrefObserver observer); */
    pub AddObserverForName: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, observer: *const nsIContentPrefObserver) -> ::nserror::nsresult,

    /* void removeObserverForName (in AString name, in nsIContentPrefObserver observer); */
    pub RemoveObserverForName: unsafe extern "system" fn (this: *const nsIContentPrefService2, name: *const ::nsstring::nsAString, observer: *const nsIContentPrefObserver) -> ::nserror::nsresult,

    /* AString extractDomain (in AString str); */
    pub ExtractDomain: unsafe extern "system" fn (this: *const nsIContentPrefService2, str: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPrefService2 {

    /// ```text
    /// /**
    ///    * Gets all the preferences with the given name.
    ///    *
    ///    * @param name      The preferences' name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleResult is called once for each preference unless
    ///    *                  no such preferences exist, in which case handleResult
    ///    *                  is not called at all.
    ///    */
    /// ```
    ///

    /// `void getByName (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn GetByName(&self, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).GetByName)(self, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Gets the preference with the given domain and name.
    ///    *
    ///    * @param domain    The preference's domain.
    ///    * @param name      The preference's name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleResult is called once unless no such preference
    ///    *                  exists, in which case handleResult is not called at all.
    ///    */
    /// ```
    ///

    /// `void getByDomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn GetByDomainAndName(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).GetByDomainAndName)(self, domain, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Gets all preferences with the given name whose domains are either the same
    ///    * as or subdomains of the given domain.
    ///    *
    ///    * @param domain    The preferences' domain.
    ///    * @param name      The preferences' name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleResult is called once for each preference.  If no
    ///    *                  such preferences exist, handleResult is not called at all.
    ///    */
    /// ```
    ///

    /// `void getBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn GetBySubdomainAndName(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).GetBySubdomainAndName)(self, domain, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Gets the preference with no domain and the given name.
    ///    *
    ///    * @param name      The preference's name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleResult is called once unless no such preference
    ///    *                  exists, in which case handleResult is not called at all.
    ///    */
    /// ```
    ///

    /// `void getGlobal (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn GetGlobal(&self, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).GetGlobal)(self, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Synchronously retrieves from the in-memory cache the preference with the
    ///    * given domain and name.
    ///    *
    ///    * In addition to caching preference values, the cache also keeps track of
    ///    * preferences that are known not to exist.  If the preference is known not to
    ///    * exist, the value attribute of the returned object will be undefined
    ///    * (nsIDataType::VTYPE_VOID).
    ///    *
    ///    * If the preference is neither cached nor known not to exist, then null is
    ///    * returned, and get() must be called to determine whether the preference
    ///    * exists.
    ///    *
    ///    * @param domain   The preference's domain.
    ///    * @param name     The preference's name.
    ///    * @param context  The private-browsing context, if any.
    ///    * @return         The preference, or null if no such preference is known to
    ///    *                 exist.
    ///    */
    /// ```
    ///

    /// `nsIContentPref getCachedByDomainAndName (in AString domain, in AString name, in nsILoadContext context);`
    #[inline]
    pub unsafe fn GetCachedByDomainAndName(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, _retval: *mut*const nsIContentPref) -> ::nserror::nsresult {
        ((*self.vtable).GetCachedByDomainAndName)(self, domain, name, context, _retval)
    }


    /// ```text
    /// /**
    ///    * Synchronously retrieves from the in-memory cache all preferences with the
    ///    * given name whose domains are either the same as or subdomains of the given
    ///    * domain.
    ///    *
    ///    * The preferences are returned in an array through the out-parameter.  If a
    ///    * preference for a particular subdomain is known not to exist, then an object
    ///    * corresponding to that preference will be present in the array, and, as with
    ///    * getCachedByDomainAndName, its value attribute will be undefined.
    ///    *
    ///    * @param domain   The preferences' domain.
    ///    * @param name     The preferences' name.
    ///    * @param context  The private-browsing context, if any.
    ///    * @return         The array of preferences.
    ///    */
    /// ```
    ///

    /// `Array<nsIContentPref> getCachedBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context);`
    #[inline]
    pub unsafe fn GetCachedBySubdomainAndName(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, _retval: *mut thin_vec::ThinVec<RefPtr<nsIContentPref>>) -> ::nserror::nsresult {
        ((*self.vtable).GetCachedBySubdomainAndName)(self, domain, name, context, _retval)
    }


    /// ```text
    /// /**
    ///    * Synchronously retrieves from the in-memory cache the preference with no
    ///    * domain and the given name.
    ///    *
    ///    * As with getCachedByDomainAndName, if the preference is cached then it is
    ///    * returned; if the preference is known not to exist, then the value attribute
    ///    * of the returned object will be undefined; if the preference is neither
    ///    * cached nor known not to exist, then null is returned.
    ///    *
    ///    * @param name     The preference's name.
    ///    * @param context  The private-browsing context, if any.
    ///    * @return         The preference, or null if no such preference is known to
    ///    *                 exist.
    ///    */
    /// ```
    ///

    /// `nsIContentPref getCachedGlobal (in AString name, in nsILoadContext context);`
    #[inline]
    pub unsafe fn GetCachedGlobal(&self, name: *const ::nsstring::nsAString, context: *const nsILoadContext, _retval: *mut*const nsIContentPref) -> ::nserror::nsresult {
        ((*self.vtable).GetCachedGlobal)(self, name, context, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets a preference.
    ///    *
    ///    * @param domain    The preference's domain.
    ///    * @param name      The preference's name.
    ///    * @param value     The preference's value.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the preference has been
    ///    *                  stored.
    ///    */
    /// ```
    ///

    /// `void set (in AString domain, in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn Set(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, value: *const nsIVariant, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, domain, name, value, context, callback)
    }


    /// ```text
    /// /**
    ///    * Sets a preference with no domain.
    ///    *
    ///    * @param name      The preference's name.
    ///    * @param value     The preference's value.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the preference has been
    ///    *                  stored.
    ///    */
    /// ```
    ///

    /// `void setGlobal (in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn SetGlobal(&self, name: *const ::nsstring::nsAString, value: *const nsIVariant, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).SetGlobal)(self, name, value, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes the preference with the given domain and name.
    ///    *
    ///    * @param domain    The preference's domain.
    ///    * @param name      The preference's name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeByDomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveByDomainAndName(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveByDomainAndName)(self, domain, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all the preferences with the given name whose domains are either
    ///    * the same as or subdomains of the given domain.
    ///    *
    ///    * @param domain    The preferences' domain.
    ///    * @param name      The preferences' name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveBySubdomainAndName(&self, domain: *const ::nsstring::nsAString, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveBySubdomainAndName)(self, domain, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes the preference with no domain and the given name.
    ///    *
    ///    * @param name      The preference's name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeGlobal (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveGlobal(&self, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveGlobal)(self, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all preferences with the given domain.
    ///    *
    ///    * @param domain    The preferences' domain.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeByDomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveByDomain(&self, domain: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveByDomain)(self, domain, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all preferences whose domains are either the same as or subdomains
    ///    * of the given domain.
    ///    *
    ///    * @param domain    The preferences' domain.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeBySubdomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveBySubdomain(&self, domain: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveBySubdomain)(self, domain, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all preferences with the given name regardless of domain, including
    ///    * global preferences with the given name.
    ///    *
    ///    * @param name      The preferences' name.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeByName (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveByName(&self, name: *const ::nsstring::nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveByName)(self, name, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all non-global preferences -- in other words, all preferences that
    ///    * have a domain.
    ///    *
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeAllDomains (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveAllDomains(&self, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAllDomains)(self, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all non-global preferences created after and including |since|.
    ///    *
    ///    * @param since     Timestamp in milliseconds.
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeAllDomainsSince (in unsigned long long since, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveAllDomainsSince(&self, since: u64, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAllDomainsSince)(self, since, context, callback)
    }


    /// ```text
    /// /**
    ///    * Removes all global preferences -- in other words, all preferences that have
    ///    * no domain.
    ///    *
    ///    * @param context   The private-browsing context, if any.
    ///    * @param callback  handleCompletion is called when the operation completes.
    ///    */
    /// ```
    ///

    /// `void removeAllGlobals (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback);`
    #[inline]
    pub unsafe fn RemoveAllGlobals(&self, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAllGlobals)(self, context, callback)
    }


    /// ```text
    /// /**
    ///    * Registers an observer that will be notified whenever a preference with the
    ///    * given name is set or removed.
    ///    *
    ///    * When a set or remove method is called, observers are called after the set
    ///    * or removal completes and after the method's callback is called, and they
    ///    * are called in the same turn of the event loop as the callback.
    ///    *
    ///    * The service holds a strong reference to the observer, so the observer must
    ///    * be removed later to avoid leaking it.
    ///    *
    ///    * @param name      The name of the preferences to observe.  Pass null to
    ///    *                  observe all preference changes regardless of name.
    ///    * @param observer  The observer.
    ///    */
    /// ```
    ///

    /// `void addObserverForName (in AString name, in nsIContentPrefObserver observer);`
    #[inline]
    pub unsafe fn AddObserverForName(&self, name: *const ::nsstring::nsAString, observer: *const nsIContentPrefObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddObserverForName)(self, name, observer)
    }


    /// ```text
    /// /**
    ///    * Unregisters an observer for the given name.
    ///    *
    ///    * @param name      The name for which the observer was registered.  Pass null
    ///    *                  if the observer was added with a null name.
    ///    * @param observer  The observer.
    ///    */
    /// ```
    ///

    /// `void removeObserverForName (in AString name, in nsIContentPrefObserver observer);`
    #[inline]
    pub unsafe fn RemoveObserverForName(&self, name: *const ::nsstring::nsAString, observer: *const nsIContentPrefObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserverForName)(self, name, observer)
    }


    /// ```text
    /// /**
    ///    * Extracts and returns the domain from the given string representation of a
    ///    * URI.  This is how the API extracts domains from URIs passed to it.
    ///    *
    ///    * @param str  The string representation of a URI, like
    ///    *             "http://example.com/foo/bar".
    ///    * @return     If the given string is a valid URI, the domain of that URI is
    ///    *             returned.  Otherwise, the string itself is returned.
    ///    */
    /// ```
    ///

    /// `AString extractDomain (in AString str);`
    #[inline]
    pub unsafe fn ExtractDomain(&self, str: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ExtractDomain)(self, str, _retval)
    }


}


/// `interface nsIContentPrefCallback2 : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPrefCallback2 {
    vtable: *const nsIContentPrefCallback2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPrefCallback2.
unsafe impl XpCom for nsIContentPrefCallback2 {
    const IID: nsIID = nsID(0x1a12cf41, 0x79e8, 0x4d0f,
        [0x98, 0x99, 0x2f, 0x7b, 0x27, 0xc5, 0xd9, 0xa1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPrefCallback2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPrefCallback2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPrefCallback2Coerce {
    /// Cheaply cast a value of this type from a `nsIContentPrefCallback2`.
    fn coerce_from(v: &nsIContentPrefCallback2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPrefCallback2Coerce for nsIContentPrefCallback2 {
    #[inline]
    fn coerce_from(v: &nsIContentPrefCallback2) -> &Self {
        v
    }
}

impl nsIContentPrefCallback2 {
    /// Cast this `nsIContentPrefCallback2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPrefCallback2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPrefCallback2 {
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
impl<T: nsISupportsCoerce> nsIContentPrefCallback2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefCallback2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPrefCallback2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPrefCallback2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleResult (in nsIContentPref pref); */
    pub HandleResult: unsafe extern "system" fn (this: *const nsIContentPrefCallback2, pref: *const nsIContentPref) -> ::nserror::nsresult,

    /* void handleError (in nsresult error); */
    pub HandleError: unsafe extern "system" fn (this: *const nsIContentPrefCallback2, error: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void handleCompletion (in unsigned short reason); */
    pub HandleCompletion: unsafe extern "system" fn (this: *const nsIContentPrefCallback2, reason: u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPrefCallback2 {

    pub const COMPLETE_OK: i64 = 0;


    pub const COMPLETE_ERROR: i64 = 1;

    /// ```text
    /// /**
    ///  * The callback used by the above methods.
    ///  */
    /// /**
    ///    * For the retrieval methods, this is called once for each retrieved
    ///    * preference.  It is not called for other methods.
    ///    *
    ///    * @param pref  The retrieved preference.
    ///    */
    /// ```
    ///

    /// `void handleResult (in nsIContentPref pref);`
    #[inline]
    pub unsafe fn HandleResult(&self, pref: *const nsIContentPref) -> ::nserror::nsresult {
        ((*self.vtable).HandleResult)(self, pref)
    }


    /// ```text
    /// /**
    ///    * Called when an error occurs.  This may be called multiple times before
    ///    * handleCompletion is called.
    ///    *
    ///    * @param error  A number in Components.results describing the error.
    ///    */
    /// ```
    ///

    /// `void handleError (in nsresult error);`
    #[inline]
    pub unsafe fn HandleError(&self, error: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, error)
    }


    /// ```text
    /// /**
    ///    * Called when the method finishes.  This will be called exactly once for
    ///    * each method invocation, and afterward no other callback methods will be
    ///    * called.
    ///    *
    ///    * @param reason  One of the COMPLETE_* values indicating the manner in which
    ///    *                the method completed.
    ///    */
    /// ```
    ///

    /// `void handleCompletion (in unsigned short reason);`
    #[inline]
    pub unsafe fn HandleCompletion(&self, reason: u16) -> ::nserror::nsresult {
        ((*self.vtable).HandleCompletion)(self, reason)
    }


}


/// `interface nsIContentPref : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPref {
    vtable: *const nsIContentPrefVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPref.
unsafe impl XpCom for nsIContentPref {
    const IID: nsIID = nsID(0x9f24948d, 0x24b5, 0x4b1b,
        [0xb5, 0x54, 0x7d, 0xbd, 0x58, 0xc1, 0xd7, 0x92]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPref {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPref.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPrefCoerce {
    /// Cheaply cast a value of this type from a `nsIContentPref`.
    fn coerce_from(v: &nsIContentPref) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPrefCoerce for nsIContentPref {
    #[inline]
    fn coerce_from(v: &nsIContentPref) -> &Self {
        v
    }
}

impl nsIContentPref {
    /// Cast this `nsIContentPref` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPrefCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPref {
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
impl<T: nsISupportsCoerce> nsIContentPrefCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPref) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPref
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPrefVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString domain; */
    pub GetDomain: unsafe extern "system" fn (this: *const nsIContentPref, aDomain: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIContentPref, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIVariant value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIContentPref, aValue: *mut*const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPref {


    /// `readonly attribute AString domain;`
    #[inline]
    pub unsafe fn GetDomain(&self, aDomain: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDomain)(self, aDomain)
    }



    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `readonly attribute nsIVariant value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


}


