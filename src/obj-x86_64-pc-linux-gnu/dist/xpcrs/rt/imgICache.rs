//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgICache.idl
//


/// `interface imgICache : nsISupports`
///

/// ```text
/// /**
///  * imgICache interface
///  *
///  * @author Stuart Parmenter <pavlov@netscape.com>
///  * @version 0.1
///  * @see imagelib2
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgICache {
    vtable: *const imgICacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgICache.
unsafe impl XpCom for imgICache {
    const IID: nsIID = nsID(0xbfdf23ff, 0x378e, 0x402e,
        [0x8a, 0x6c, 0x84, 0x0f, 0x0c, 0x82, 0xb6, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgICache {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgICache.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgICacheCoerce {
    /// Cheaply cast a value of this type from a `imgICache`.
    fn coerce_from(v: &imgICache) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgICacheCoerce for imgICache {
    #[inline]
    fn coerce_from(v: &imgICache) -> &Self {
        v
    }
}

impl imgICache {
    /// Cast this `imgICache` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgICacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgICache {
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
impl<T: nsISupportsCoerce> imgICacheCoerce for T {
    #[inline]
    fn coerce_from(v: &imgICache) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgICache
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgICacheVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void clearCache (in boolean chrome); */
    pub ClearCache: unsafe extern "system" fn (this: *const imgICache, chrome: bool) -> ::nserror::nsresult,

    /* [noscript] void removeEntry (in nsIURI uri, [optional] in Document doc); */
    pub RemoveEntry: unsafe extern "system" fn (this: *const imgICache, uri: *const nsIURI, doc: *const libc::c_void) -> ::nserror::nsresult,

    /* void removeEntriesFromPrincipal (in nsIPrincipal aPrincipal); */
    pub RemoveEntriesFromPrincipal: unsafe extern "system" fn (this: *const imgICache, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in Document doc); */
    pub FindEntryProperties: unsafe extern "system" fn (this: *const imgICache, uri: *const nsIURI, doc: *const libc::c_void, _retval: *mut*const nsIProperties) -> ::nserror::nsresult,

    /* void respectPrivacyNotifications (); */
    pub RespectPrivacyNotifications: unsafe extern "system" fn (this: *const imgICache) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void clearCacheForControlledDocument (in Document doc); */
    pub ClearCacheForControlledDocument: unsafe extern "system" fn (this: *const imgICache, doc: *const libc::c_void) -> libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgICache {

    /// ```text
    /// /**
    ///    * Evict images from the cache.
    ///    *
    ///    * @param chrome If TRUE,  evict only chrome images.
    ///    *               If FALSE, evict everything except chrome images.
    ///    */
    /// ```
    ///

    /// `void clearCache (in boolean chrome);`
    #[inline]
    pub unsafe fn ClearCache(&self, chrome: bool) -> ::nserror::nsresult {
        ((*self.vtable).ClearCache)(self, chrome)
    }


    /// ```text
    /// /**
    ///    * Evict images from the cache.
    ///    *
    ///    * @param uri The URI to remove.
    ///    * @param doc The document to remove the cache entry for.
    ///    * @throws NS_ERROR_NOT_AVAILABLE if \a uri was unable to be removed from
    ///    * the cache.
    ///    */
    /// ```
    ///

    /// `[noscript] void removeEntry (in nsIURI uri, [optional] in Document doc);`
    #[inline]
    pub unsafe fn RemoveEntry(&self, uri: *const nsIURI, doc: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).RemoveEntry)(self, uri, doc)
    }


    /// ```text
    /// /**
    ///    * Evict images from the cache with the same origin and the same
    ///    * originAttributes of the passed principal.
    ///    *
    ///    * @param aPrincipal The principal
    ///    */
    /// ```
    ///

    /// `void removeEntriesFromPrincipal (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn RemoveEntriesFromPrincipal(&self, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).RemoveEntriesFromPrincipal)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///    * Find Properties
    ///    * Used to get properties such as 'type' and 'content-disposition'
    ///    * 'type' is a nsISupportsCString containing the images' mime type such as
    ///    * 'image/png'
    ///    * 'content-disposition' will be a nsISupportsCString containing the header
    ///    * If you call this before any data has been loaded from a URI, it will
    ///    * succeed, but come back empty.
    ///    *
    ///    * Hopefully this will be removed with bug 805119
    ///    *
    ///    * @param uri The URI to look up.
    ///    * @param doc Optional pointer to the document that the cache entry belongs to.
    ///    * @returns NULL if the URL was not found in the cache
    ///    */
    /// ```
    ///

    /// `[must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in Document doc);`
    #[inline]
    pub unsafe fn FindEntryProperties(&self, uri: *const nsIURI, doc: *const libc::c_void, _retval: *mut*const nsIProperties) -> ::nserror::nsresult {
        ((*self.vtable).FindEntryProperties)(self, uri, doc, _retval)
    }


    /// ```text
    /// /**
    ///    * Make this cache instance respect private browsing notifications. This
    ///    * entails clearing the chrome and content caches whenever the
    ///    * last-pb-context-exited notification is observed.
    ///    */
    /// ```
    ///

    /// `void respectPrivacyNotifications ();`
    #[inline]
    pub unsafe fn RespectPrivacyNotifications(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RespectPrivacyNotifications)(self, )
    }


    /// ```text
    /// /**
    ///    * Clear the image cache for a document.  Controlled documents are responsible
    ///    * for doing this manually when they get destroyed.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void clearCacheForControlledDocument (in Document doc);`
    #[inline]
    pub unsafe fn ClearCacheForControlledDocument(&self, doc: *const libc::c_void) -> libc::c_void {
        ((*self.vtable).ClearCacheForControlledDocument)(self, doc)
    }


}


