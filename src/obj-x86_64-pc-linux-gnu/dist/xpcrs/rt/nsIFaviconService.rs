//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsIFaviconService.idl
//


/// `interface nsIFaviconService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFaviconService {
    vtable: *const nsIFaviconServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFaviconService.
unsafe impl XpCom for nsIFaviconService {
    const IID: nsIID = nsID(0xe81e0b0c, 0xb9f1, 0x4c2e,
        [0x8f, 0x3c, 0xb8, 0x09, 0x93, 0x3c, 0xf7, 0x3c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFaviconService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFaviconService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFaviconServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIFaviconService`.
    fn coerce_from(v: &nsIFaviconService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFaviconServiceCoerce for nsIFaviconService {
    #[inline]
    fn coerce_from(v: &nsIFaviconService) -> &Self {
        v
    }
}

impl nsIFaviconService {
    /// Cast this `nsIFaviconService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFaviconServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFaviconService {
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
impl<T: nsISupportsCoerce> nsIFaviconServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFaviconService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFaviconService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFaviconServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI); */
    pub GetFaviconLinkForIcon: unsafe extern "system" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* void expireAllFavicons (); */
    pub ExpireAllFavicons: unsafe extern "system" fn (this: *const nsIFaviconService) -> ::nserror::nsresult,

    /* void setDefaultIconURIPreferredSize (in unsigned short aDefaultSize); */
    pub SetDefaultIconURIPreferredSize: unsafe extern "system" fn (this: *const nsIFaviconService, aDefaultSize: u16) -> ::nserror::nsresult,

    /* unsigned short preferredSizeFromURI (in nsIURI aURI); */
    pub PreferredSizeFromURI: unsafe extern "system" fn (this: *const nsIFaviconService, aURI: *const nsIURI, _retval: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute nsIURI defaultFavicon; */
    pub GetDefaultFavicon: unsafe extern "system" fn (this: *const nsIFaviconService, aDefaultFavicon: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String defaultFaviconMimeType; */
    pub GetDefaultFaviconMimeType: unsafe extern "system" fn (this: *const nsIFaviconService, aDefaultFaviconMimeType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* mozIPlacesPendingOperation setAndFetchFaviconForPage (in nsIURI aPageURI, in nsIURI aFaviconURI, in boolean aForceReload, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback, [optional] in nsIPrincipal aLoadingPrincipal, [optional] in unsigned long long aRequestContextID); */
    pub SetAndFetchFaviconForPage: unsafe extern "system" fn (this: *const nsIFaviconService, aPageURI: *const nsIURI, aFaviconURI: *const nsIURI, aForceReload: bool, aFaviconLoadType: u32, aCallback: *const nsIFaviconDataCallback, aLoadingPrincipal: *const nsIPrincipal, aRequestContextID: u64, _retval: *mut*const mozIPlacesPendingOperation) -> ::nserror::nsresult,

    /* void replaceFaviconData (in nsIURI aFaviconURI, in Array<octet> aData, in AUTF8String aMimeType, [optional] in PRTime aExpiration); */
    pub ReplaceFaviconData: unsafe extern "system" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI, aData: *const thin_vec::ThinVec<u8>, aMimeType: *const ::nsstring::nsACString, aExpiration: PRTime) -> ::nserror::nsresult,

    /* void replaceFaviconDataFromDataURL (in nsIURI aFaviconURI, in AString aDataURL, [optional] in PRTime aExpiration, [optional] in nsIPrincipal aLoadingPrincipal); */
    pub ReplaceFaviconDataFromDataURL: unsafe extern "system" fn (this: *const nsIFaviconService, aFaviconURI: *const nsIURI, aDataURL: *const ::nsstring::nsAString, aExpiration: PRTime, aLoadingPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* void getFaviconURLForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
    pub GetFaviconURLForPage: unsafe extern "system" fn (this: *const nsIFaviconService, aPageURI: *const nsIURI, aCallback: *const nsIFaviconDataCallback, aPreferredWidth: u16) -> ::nserror::nsresult,

    /* void getFaviconDataForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
    pub GetFaviconDataForPage: unsafe extern "system" fn (this: *const nsIFaviconService, aPageURI: *const nsIURI, aCallback: *const nsIFaviconDataCallback, aPreferredWidth: u16) -> ::nserror::nsresult,

    /* void copyFavicons (in nsIURI aFromPageURI, in nsIURI aToPageURI, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback); */
    pub CopyFavicons: unsafe extern "system" fn (this: *const nsIFaviconService, aFromPageURI: *const nsIURI, aToPageURI: *const nsIURI, aFaviconLoadType: u32, aCallback: *const nsIFaviconDataCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFaviconService {

    pub const FAVICON_LOAD_PRIVATE: i64 = 1;


    pub const FAVICON_LOAD_NON_PRIVATE: i64 = 2;

    /// ```text
    /// /**
    ///    * The limit in bytes of the size of favicons in memory and passed via the
    ///    * favicon protocol.
    ///    */
    /// ```
    ///

    pub const MAX_FAVICON_BUFFER_SIZE: i64 = 65536;

    /// ```text
    /// /**
    ///    * For a given icon URI, this will return a URI that will result in the image.
    ///    * In most cases, this is an annotation URI.  For chrome URIs, this will do
    ///    * nothing but returning the input URI.
    ///    *
    ///    * No validity checking is done. If you pass an icon URI that we've never
    ///    * seen, you'll get back a URI that references an invalid icon. The moz-anno
    ///    * protocol handler's special case for "favicon" annotations will resolve
    ///    * invalid icons to the default icon, although without caching.
    ///    * For invalid chrome URIs, you'll get a broken image.
    ///    *
    ///    * @param aFaviconURI
    ///    *        The URI of an icon in the favicon service.
    ///    * @return A URI that will give you the icon image.  This is NOT the URI of
    ///    *         the icon as set on the page, but a URI that will give you the
    ///    *         data out of the favicon service.  For a normal page with a
    ///    *         favicon we've stored, this will be an annotation URI which will
    ///    *         then cause the corresponding favicon data to be loaded async from
    ///    *         this service.  For pages where we don't have a favicon, this will
    ///    *         be a chrome URI of the default icon. For chrome URIs, the
    ///    *         output will be the same as the input.
    ///    */
    /// ```
    ///

    /// `nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI);`
    #[inline]
    pub unsafe fn GetFaviconLinkForIcon(&self, aFaviconURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetFaviconLinkForIcon)(self, aFaviconURI, _retval)
    }


    /// ```text
    /// /**
    ///    * Expire all known favicons from the database.
    ///    *
    ///    * @note This is an async method.
    ///    *       On successful completion a "places-favicons-expired" notification is
    ///    *       dispatched through observer's service.
    ///    */
    /// ```
    ///

    /// `void expireAllFavicons ();`
    #[inline]
    pub unsafe fn ExpireAllFavicons(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ExpireAllFavicons)(self, )
    }


    /// ```text
    /// /**
    ///    * Sets the default size returned by preferredSizeFromURI when the uri doesn't
    ///    * specify a size ref. If this is not invoked first, or 0 is passed to it,
    ///    * preferredSizeFromURI() will return UINT16_MAX, that matches the biggest
    ///    * icon available.
    ///    */
    /// ```
    ///

    /// `void setDefaultIconURIPreferredSize (in unsigned short aDefaultSize);`
    #[inline]
    pub unsafe fn SetDefaultIconURIPreferredSize(&self, aDefaultSize: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultIconURIPreferredSize)(self, aDefaultSize)
    }


    /// ```text
    /// /**
    ///    * Tries to extract the preferred size from an icon uri ref fragment.
    ///    *
    ///    * @param aURI
    ///    *        The URI to parse.
    ///    * @return The preferred size, or a default size set through
    ///    *         setDefaultIconURIPreferredSize, or UINT16_MAX if neither are set.
    ///    */
    /// ```
    ///

    /// `unsigned short preferredSizeFromURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn PreferredSizeFromURI(&self, aURI: *const nsIURI, _retval: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).PreferredSizeFromURI)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///    * The default favicon URI
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI defaultFavicon;`
    #[inline]
    pub unsafe fn GetDefaultFavicon(&self, aDefaultFavicon: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultFavicon)(self, aDefaultFavicon)
    }


    /// ```text
    /// /**
    ///    * The default favicon mimeType
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String defaultFaviconMimeType;`
    #[inline]
    pub unsafe fn GetDefaultFaviconMimeType(&self, aDefaultFaviconMimeType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultFaviconMimeType)(self, aDefaultFaviconMimeType)
    }


    /// ```text
    /// /**
    ///    * Declares that a given page uses a favicon with the given URI and
    ///    * attempts to fetch and save the icon data by loading the favicon URI
    ///    * through an async network request.
    ///    *
    ///    * If the icon data already exists, we won't try to reload the icon unless
    ///    * aForceReload is true.  Similarly, if the icon is in the failed favicon
    ///    * cache we won't do anything unless aForceReload is true, in which case
    ///    * we'll try to reload the favicon.
    ///    *
    ///    * This function will only save favicons for pages that are already stored in
    ///    * the database, like visited pages or bookmarks.  For any other URIs, it
    ///    * will succeed but do nothing.  This function will also ignore the error
    ///    * page favicon URI (see FAVICON_ERRORPAGE_URL below).
    ///    *
    ///    * Icons that fail to load will automatically be added to the failed favicon
    ///    * cache, and this function will not save favicons for non-bookmarked URIs
    ///    * when history is disabled.
    ///    *
    ///    * @note This function is identical to
    ///    *       nsIFaviconService::setAndLoadFaviconForPage.
    ///    *
    ///    * @param aPageURI
    ///    *        URI of the page whose favicon is being set.
    ///    * @param aFaviconURI
    ///    *        URI of the favicon to associate with the page.
    ///    * @param aForceReload
    ///    *        If aForceReload is false, we try to reload the favicon only if we
    ///    *        don't have it or it has expired from the cache.  Setting
    ///    *        aForceReload to true causes us to reload the favicon even if we
    ///    *        have a usable copy.
    ///    * @param aFaviconLoadType
    ///    *        Set to FAVICON_LOAD_PRIVATE if the favicon is loaded from a private
    ///    *        browsing window.  Set to FAVICON_LOAD_NON_PRIVATE otherwise.
    ///    * @param [optional] aCallback
    ///    *        Once we're done setting and/or fetching the favicon, we invoke this
    ///    *        callback.
    ///    * @param [optional] aLoadingPrincipal
    ///    *        Principal of the page whose favicon is being set. If this argument
    ///    *        is omitted, the loadingPrincipal defaults to the nullPrincipal.
    ///    * @param [optional] aRequestContextID
    ///    *        used to inform Necko of how to link the
    ///    *        favicon request with other requests in the same tab.
    ///    *
    ///    * @see nsIFaviconDataCallback in nsIFaviconService.idl.
    ///    */
    /// ```
    ///

    /// `mozIPlacesPendingOperation setAndFetchFaviconForPage (in nsIURI aPageURI, in nsIURI aFaviconURI, in boolean aForceReload, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback, [optional] in nsIPrincipal aLoadingPrincipal, [optional] in unsigned long long aRequestContextID);`
    #[inline]
    pub unsafe fn SetAndFetchFaviconForPage(&self, aPageURI: *const nsIURI, aFaviconURI: *const nsIURI, aForceReload: bool, aFaviconLoadType: u32, aCallback: *const nsIFaviconDataCallback, aLoadingPrincipal: *const nsIPrincipal, aRequestContextID: u64, _retval: *mut*const mozIPlacesPendingOperation) -> ::nserror::nsresult {
        ((*self.vtable).SetAndFetchFaviconForPage)(self, aPageURI, aFaviconURI, aForceReload, aFaviconLoadType, aCallback, aLoadingPrincipal, aRequestContextID, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets the data for a given favicon URI either by replacing existing data in
    ///    * the database or taking the place of otherwise fetched icon data when
    ///    * calling setAndFetchFaviconForPage later.
    ///    *
    ///    * Favicon data for favicon URIs that are not associated with a page URI via
    ///    * setAndFetchFaviconForPage will be stored in memory, but may be expired at
    ///    * any time, so you should make an effort to associate favicon URIs with page
    ///    * URIs as soon as possible.
    ///    *
    ///    * It's better to not use this function for chrome: icon URIs since you can
    ///    * reference the chrome image yourself. getFaviconLinkForIcon/Page will ignore
    ///    * any associated data if the favicon URI is "chrome:" and just return the
    ///    * same chrome URI.
    ///    *
    ///    * This function does NOT send out notifications that the data has changed.
    ///    * Pages using this favicons that are visible in history or bookmarks views
    ///    * will keep the old icon until they have been refreshed by other means.
    ///    *
    ///    * This function tries to optimize the favicon size, if it is bigger
    ///    * than a defined limit we will try to convert it to a 16x16 png image.
    ///    * If the conversion fails and favicon is still bigger than our max accepted
    ///    * size it won't be saved.
    ///    *
    ///    * @param aFaviconURI
    ///    *        URI of the favicon whose data is being set.
    ///    * @param aData
    ///    *        Binary contents of the favicon to save
    ///    * @param aMimeType
    ///    *        MIME type of the data to store.  This is important so that we know
    ///    *        what to report when the favicon is used.  You should always set this
    ///    *        param unless you are clearing an icon.
    ///    * @param [optional] aExpiration
    ///    *        Time in microseconds since the epoch when this favicon expires.
    ///    *        Until this time, we won't try to load it again.
    ///    * @throws NS_ERROR_FAILURE
    ///    *         Thrown if the favicon is overbloated and won't be saved to the db.
    ///    */
    /// ```
    ///

    /// `void replaceFaviconData (in nsIURI aFaviconURI, in Array<octet> aData, in AUTF8String aMimeType, [optional] in PRTime aExpiration);`
    #[inline]
    pub unsafe fn ReplaceFaviconData(&self, aFaviconURI: *const nsIURI, aData: *const thin_vec::ThinVec<u8>, aMimeType: *const ::nsstring::nsACString, aExpiration: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceFaviconData)(self, aFaviconURI, aData, aMimeType, aExpiration)
    }


    /// ```text
    /// /**
    ///    * Same as replaceFaviconData but the data is provided by a string
    ///    * containing a data URL.
    ///    *
    ///    * @see replaceFaviconData
    ///    *
    ///    * @param aFaviconURI
    ///    *        URI of the favicon whose data is being set.
    ///    * @param aDataURL
    ///    *        string containing a data URL that represents the contents of
    ///    *        the favicon to save
    ///    * @param [optional] aExpiration
    ///    *        Time in microseconds since the epoch when this favicon expires.
    ///    *        Until this time, we won't try to load it again.
    ///    * @param [optional] aLoadingPrincipal
    ///    *        Principal of the page whose favicon is being set. If this argument
    ///    *        is omitted, the loadingPrincipal defaults to the nullPrincipal.
    ///    * @throws NS_ERROR_FAILURE
    ///    *         Thrown if the favicon is overbloated and won't be saved to the db.
    ///    */
    /// ```
    ///

    /// `void replaceFaviconDataFromDataURL (in nsIURI aFaviconURI, in AString aDataURL, [optional] in PRTime aExpiration, [optional] in nsIPrincipal aLoadingPrincipal);`
    #[inline]
    pub unsafe fn ReplaceFaviconDataFromDataURL(&self, aFaviconURI: *const nsIURI, aDataURL: *const ::nsstring::nsAString, aExpiration: PRTime, aLoadingPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceFaviconDataFromDataURL)(self, aFaviconURI, aDataURL, aExpiration, aLoadingPrincipal)
    }


    /// ```text
    /// /**
    ///    * Retrieves the favicon URI associated to the given page, if any.
    ///    *
    ///    * @param aPageURI
    ///    *        URI of the page whose favicon URI we're looking up.
    ///    * @param aCallback
    ///    *        This callback is always invoked to notify the result of the lookup.
    ///    *        The aURI parameter will be the favicon URI, or null when no favicon
    ///    *        is associated with the page or an error occurred while fetching it.
    ///    *        aDataLen will be always 0, aData will be an empty array, and
    ///    *        aMimeType will be an empty string, regardless of whether a favicon
    ///    *        was found.
    ///    * @param [optional] aPreferredWidth
    ///    *        The preferred icon width, 0 for the biggest available.
    ///    *
    ///    * @note If a favicon specific to this page cannot be found, this will try to
    ///    *       fallback to the /favicon.ico for the root domain.
    ///    *
    ///    * @see nsIFaviconDataCallback in nsIFaviconService.idl.
    ///    */
    /// ```
    ///

    /// `void getFaviconURLForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth);`
    #[inline]
    pub unsafe fn GetFaviconURLForPage(&self, aPageURI: *const nsIURI, aCallback: *const nsIFaviconDataCallback, aPreferredWidth: u16) -> ::nserror::nsresult {
        ((*self.vtable).GetFaviconURLForPage)(self, aPageURI, aCallback, aPreferredWidth)
    }


    /// ```text
    /// /**
    ///    * Retrieves the favicon URI and data associated to the given page, if any.
    ///    * If the page icon is not available, it will try to return the root domain
    ///    * icon data, when it's known.
    ///    *
    ///    * @param aPageURI
    ///    *        URI of the page whose favicon URI and data we're looking up.
    ///    * @param aCallback
    ///    *        This callback is always invoked to notify the result of the lookup.  The aURI
    ///    *        parameter will be the favicon URI, or null when no favicon is
    ///    *        associated with the page or an error occurred while fetching it.  If
    ///    *        aURI is not null, the other parameters may contain the favicon data.
    ///    *        However, if no favicon data is currently associated with the favicon
    ///    *        URI, aDataLen will be 0, aData will be an empty array, and aMimeType
    ///    *        will be an empty string.
    ///    * @param [optional] aPreferredWidth
    ///    *        The preferred icon width, 0 for the biggest available.
    ///    * @note If a favicon specific to this page cannot be found, this will try to
    ///    *       fallback to the /favicon.ico for the root domain.
    ///    *
    ///    * @see nsIFaviconDataCallback in nsIFaviconService.idl.
    ///    */
    /// ```
    ///

    /// `void getFaviconDataForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth);`
    #[inline]
    pub unsafe fn GetFaviconDataForPage(&self, aPageURI: *const nsIURI, aCallback: *const nsIFaviconDataCallback, aPreferredWidth: u16) -> ::nserror::nsresult {
        ((*self.vtable).GetFaviconDataForPage)(self, aPageURI, aCallback, aPreferredWidth)
    }


    /// ```text
    /// /**
    ///    * Copies cached favicons from a page to another one.
    ///    *
    ///    * @param aFromPageURI
    ///    *        URI of the originating page.
    ///    * @param aToPageURI
    ///    *        URI of the destination page.
    ///    * @param aFaviconLoadType
    ///    *        Set to FAVICON_LOAD_PRIVATE if the copy is started from a private
    ///    *        browsing window.  Set to FAVICON_LOAD_NON_PRIVATE otherwise.
    ///    * @param [optional] aCallback
    ///    *        Once we're done copying the favicon, we invoke this callback.
    ///    *        If a copy has been done, the callback will report one of the
    ///    *        favicons uri as aFaviconURI, otherwise all the params will be null.
    ///    */
    /// ```
    ///

    /// `void copyFavicons (in nsIURI aFromPageURI, in nsIURI aToPageURI, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback);`
    #[inline]
    pub unsafe fn CopyFavicons(&self, aFromPageURI: *const nsIURI, aToPageURI: *const nsIURI, aFaviconLoadType: u32, aCallback: *const nsIFaviconDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).CopyFavicons)(self, aFromPageURI, aToPageURI, aFaviconLoadType, aCallback)
    }


}


/// `interface nsIFaviconDataCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFaviconDataCallback {
    vtable: *const nsIFaviconDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFaviconDataCallback.
unsafe impl XpCom for nsIFaviconDataCallback {
    const IID: nsIID = nsID(0xc85e5c82, 0xb70f, 0x4621,
        [0x95, 0x28, 0xbe, 0xb2, 0xaa, 0x47, 0xfb, 0x44]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFaviconDataCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFaviconDataCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFaviconDataCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFaviconDataCallback`.
    fn coerce_from(v: &nsIFaviconDataCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFaviconDataCallbackCoerce for nsIFaviconDataCallback {
    #[inline]
    fn coerce_from(v: &nsIFaviconDataCallback) -> &Self {
        v
    }
}

impl nsIFaviconDataCallback {
    /// Cast this `nsIFaviconDataCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFaviconDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFaviconDataCallback {
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
impl<T: nsISupportsCoerce> nsIFaviconDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFaviconDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFaviconDataCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFaviconDataCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onComplete (in nsIURI aFaviconURI, in unsigned long aDataLen, [array, size_is (aDataLen), const] in octet aData, in AUTF8String aMimeType, in unsigned short aWidth); */
    pub OnComplete: unsafe extern "system" fn (this: *const nsIFaviconDataCallback, aFaviconURI: *const nsIURI, aDataLen: u32, aData: *const u8, aMimeType: *const ::nsstring::nsACString, aWidth: u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFaviconDataCallback {

    /// ```text
    /// /**
    ///    * Called when the required favicon's information is available.
    ///    *
    ///    * It's up to the invoking method to state if the callback is always invoked,
    ///    * or called on success only.  Check the method documentation to ensure that.
    ///    *
    ///    * The caller will receive the most information we can gather on the icon,
    ///    * but it's not guaranteed that all of them will be set.  For some method
    ///    * we could not know the favicon's data (it could just be too expensive to
        ///    * get it, or the method does not require we actually have any data).
    ///    * It's up to the caller to check aDataLen > 0 before using any data-related
    ///    * information like mime-type or data itself.
    ///    *
    ///    * @param aFaviconURI
    ///    *        Receives the "favicon URI" (not the "favicon link URI") associated
    ///    *        to the requested page.  This can be null if there is no associated
    ///    *        favicon URI, or the callback is notifying a failure.
    ///    * @param aDataLen
    ///    *        Size of the icon data in bytes.  Notice that a value of 0 does not
    ///    *        necessarily mean that we don't have an icon.
    ///    * @param aData
    ///    *        Icon data, or an empty array if aDataLen is 0.
    ///    * @param aMimeType
    ///    *        Mime type of the icon, or an empty string if aDataLen is 0.
    ///    * @param aWidth
    ///    *        Width of the icon. 0 if the width is unknown or if the icon is
    ///    *        vectorial.
    ///    *
    ///    * @note If you want to open a network channel to access the favicon, it's
    ///    *       recommended that you call the getFaviconLinkForIcon method to convert
    ///    *       the "favicon URI" into a "favicon link URI".
    ///    */
    /// ```
    ///

    /// `void onComplete (in nsIURI aFaviconURI, in unsigned long aDataLen, [array, size_is (aDataLen), const] in octet aData, in AUTF8String aMimeType, in unsigned short aWidth);`
    #[inline]
    pub unsafe fn OnComplete(&self, aFaviconURI: *const nsIURI, aDataLen: u32, aData: *const u8, aMimeType: *const ::nsstring::nsACString, aWidth: u16) -> ::nserror::nsresult {
        ((*self.vtable).OnComplete)(self, aFaviconURI, aDataLen, aData, aMimeType, aWidth)
    }


}


