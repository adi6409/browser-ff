//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webbrowserpersist/nsIWebBrowserPersistDocument.idl
//


/// `interface nsIWebBrowserPersistURIMap : nsISupports`
///

/// ```text
/// /**
///  * Interface for the URI-mapping information that can be supplied when
///  * serializing the DOM of an nsIWebBrowserPersistDocument.
///  *
///  * @see nsIWebBrowserPersistDocument
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPersistURIMap {
    vtable: *const nsIWebBrowserPersistURIMapVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPersistURIMap.
unsafe impl XpCom for nsIWebBrowserPersistURIMap {
    const IID: nsIID = nsID(0xd52e8b93, 0x2771, 0x45e8,
        [0xa5, 0xb0, 0x6e, 0x12, 0xb6, 0x67, 0x04, 0x6b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPersistURIMap {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPersistURIMap.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPersistURIMapCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPersistURIMap`.
    fn coerce_from(v: &nsIWebBrowserPersistURIMap) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPersistURIMapCoerce for nsIWebBrowserPersistURIMap {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistURIMap) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistURIMap {
    /// Cast this `nsIWebBrowserPersistURIMap` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistURIMapCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPersistURIMap {
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
impl<T: nsISupportsCoerce> nsIWebBrowserPersistURIMapCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistURIMap) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPersistURIMap
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPersistURIMapVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long numMappedURIs; */
    pub GetNumMappedURIs: unsafe extern "system" fn (this: *const nsIWebBrowserPersistURIMap, aNumMappedURIs: *mut u32) -> ::nserror::nsresult,

    /* void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo); */
    pub GetURIMapping: unsafe extern "system" fn (this: *const nsIWebBrowserPersistURIMap, aIndex: u32, aMapFrom: *mut ::nsstring::nsACString, aMapTo: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String targetBaseURI; */
    pub GetTargetBaseURI: unsafe extern "system" fn (this: *const nsIWebBrowserPersistURIMap, aTargetBaseURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPersistURIMap {

    /// ```text
    /// /**
    ///    * The number of URI mappings.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long numMappedURIs;`
    #[inline]
    pub unsafe fn GetNumMappedURIs(&self, aNumMappedURIs: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumMappedURIs)(self, aNumMappedURIs)
    }


    /// ```text
    /// /**
    ///    * Obtain the URI mapping at the given index, which must be less than
    ///    * numMappedURIs, as a pair of URI spec strings.
    ///    */
    /// ```
    ///

    /// `void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo);`
    #[inline]
    pub unsafe fn GetURIMapping(&self, aIndex: u32, aMapFrom: *mut ::nsstring::nsACString, aMapTo: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetURIMapping)(self, aIndex, aMapFrom, aMapTo)
    }


    /// ```text
    /// /**
    ///    * The spec of the base URI that the document will have after it is
    ///    * serialized.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String targetBaseURI;`
    #[inline]
    pub unsafe fn GetTargetBaseURI(&self, aTargetBaseURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetBaseURI)(self, aTargetBaseURI)
    }


}


/// `interface nsIWebBrowserPersistDocument : nsISupports`
///

/// ```text
/// /**
///  * Interface representing a document that can be serialized with
///  * nsIWebBrowserPersist; it may or may not be in this process.  Some
///  * information is exposed as attributes, which may or may not reflect
///  * changes made to the underlying document; most of these are
///  * self-explanatory from their names and types.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPersistDocument {
    vtable: *const nsIWebBrowserPersistDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPersistDocument.
unsafe impl XpCom for nsIWebBrowserPersistDocument {
    const IID: nsIID = nsID(0x74aa4918, 0x5d15, 0x46b6,
        [0x9c, 0xcf, 0x74, 0xf9, 0x69, 0x6d, 0x72, 0x1d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPersistDocument {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPersistDocument.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPersistDocumentCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPersistDocument`.
    fn coerce_from(v: &nsIWebBrowserPersistDocument) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPersistDocumentCoerce for nsIWebBrowserPersistDocument {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocument) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistDocument {
    /// Cast this `nsIWebBrowserPersistDocument` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPersistDocument {
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
impl<T: nsISupportsCoerce> nsIWebBrowserPersistDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocument) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPersistDocument
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPersistDocumentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isClosed; */
    pub GetIsClosed: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aIsClosed: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isPrivate; */
    pub GetIsPrivate: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aIsPrivate: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String documentURI; */
    pub GetDocumentURI: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aDocumentURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String baseURI; */
    pub GetBaseURI: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aBaseURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString contentType; */
    pub GetContentType: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString characterSet; */
    pub GetCharacterSet: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aCharacterSet: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIReferrerInfo referrerInfo; */
    pub GetReferrerInfo: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult,

    /* readonly attribute nsICookieJarSettings cookieJarSettings; */
    pub GetCookieJarSettings: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aCookieJarSettings: *mut*const nsICookieJarSettings) -> ::nserror::nsresult,

    /* readonly attribute AString contentDisposition; */
    pub GetContentDisposition: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aContentDisposition: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIInputStream postData; */
    pub GetPostData: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long cacheKey; */
    pub GetCacheKey: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aCacheKey: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long persistFlags; */
    pub GetPersistFlags: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aPersistFlags: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long persistFlags; */
    pub SetPersistFlags: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aPersistFlags: u32) -> ::nserror::nsresult,

    /* void readResources (in nsIWebBrowserPersistResourceVisitor aVisitor); */
    pub ReadResources: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aVisitor: *const nsIWebBrowserPersistResourceVisitor) -> ::nserror::nsresult,

    /* void writeContent (in nsIOutputStream aStream, in nsIWebBrowserPersistURIMap aURIMap, in ACString aRequestedContentType, in unsigned long aEncoderFlags, in unsigned long aWrapColumn, in nsIWebBrowserPersistWriteCompletion aCompletion); */
    pub WriteContent: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocument, aStream: *const nsIOutputStream, aURIMap: *const nsIWebBrowserPersistURIMap, aRequestedContentType: *const ::nsstring::nsACString, aEncoderFlags: u32, aWrapColumn: u32, aCompletion: *const nsIWebBrowserPersistWriteCompletion) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] SHEntryRef GetHistory (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetHistory: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPersistDocument {


    /// `readonly attribute boolean isClosed;`
    #[inline]
    pub unsafe fn GetIsClosed(&self, aIsClosed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsClosed)(self, aIsClosed)
    }



    /// `readonly attribute boolean isPrivate;`
    #[inline]
    pub unsafe fn GetIsPrivate(&self, aIsPrivate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsPrivate)(self, aIsPrivate)
    }



    /// `readonly attribute AUTF8String documentURI;`
    #[inline]
    pub unsafe fn GetDocumentURI(&self, aDocumentURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentURI)(self, aDocumentURI)
    }



    /// `readonly attribute AUTF8String baseURI;`
    #[inline]
    pub unsafe fn GetBaseURI(&self, aBaseURI: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseURI)(self, aBaseURI)
    }



    /// `readonly attribute ACString contentType;`
    #[inline]
    pub unsafe fn GetContentType(&self, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentType)(self, aContentType)
    }



    /// `readonly attribute ACString characterSet;`
    #[inline]
    pub unsafe fn GetCharacterSet(&self, aCharacterSet: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCharacterSet)(self, aCharacterSet)
    }



    /// `readonly attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }



    /// `readonly attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn GetReferrerInfo(&self, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerInfo)(self, aReferrerInfo)
    }



    /// `readonly attribute nsICookieJarSettings cookieJarSettings;`
    #[inline]
    pub unsafe fn GetCookieJarSettings(&self, aCookieJarSettings: *mut*const nsICookieJarSettings) -> ::nserror::nsresult {
        ((*self.vtable).GetCookieJarSettings)(self, aCookieJarSettings)
    }



    /// `readonly attribute AString contentDisposition;`
    #[inline]
    pub unsafe fn GetContentDisposition(&self, aContentDisposition: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentDisposition)(self, aContentDisposition)
    }



    /// `readonly attribute nsIInputStream postData;`
    #[inline]
    pub unsafe fn GetPostData(&self, aPostData: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetPostData)(self, aPostData)
    }



    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///    * The cache key.  Unlike in nsISHEntry, where it's wrapped in an
    ///    * nsISupportsPRUint32, this is just the integer.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long cacheKey;`
    #[inline]
    pub unsafe fn GetCacheKey(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCacheKey)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * This attribute is set by nsIWebBrowserPersist implementations to
    ///    * propagate persist flags that apply to the DOM traversal and
    ///    * serialization (rather than to managing file I/O).
    ///    */
    /// ```
    ///

    /// `attribute unsigned long persistFlags;`
    #[inline]
    pub unsafe fn GetPersistFlags(&self, aPersistFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPersistFlags)(self, aPersistFlags)
    }


    /// ```text
    /// /**
    ///    * This attribute is set by nsIWebBrowserPersist implementations to
    ///    * propagate persist flags that apply to the DOM traversal and
    ///    * serialization (rather than to managing file I/O).
    ///    */
    /// ```
    ///

    /// `attribute unsigned long persistFlags;`
    #[inline]
    pub unsafe fn SetPersistFlags(&self, aPersistFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPersistFlags)(self, aPersistFlags)
    }


    /// ```text
    /// /**
    ///    * Walk the DOM searching for external resources needed to render it.
    ///    * The visitor callbacks may be called either before or after
    ///    * readResources returns.
    ///    *
    ///    * @see nsIWebBrowserPersistResourceVisitor
    ///    */
    /// ```
    ///

    /// `void readResources (in nsIWebBrowserPersistResourceVisitor aVisitor);`
    #[inline]
    pub unsafe fn ReadResources(&self, aVisitor: *const nsIWebBrowserPersistResourceVisitor) -> ::nserror::nsresult {
        ((*self.vtable).ReadResources)(self, aVisitor)
    }


    /// ```text
    /// /**
    ///    * Serialize the document's DOM.
    ///    *
    ///    * @param aStream       The output stream to write the document to.
    ///    *
    ///    * @param aURIMap       Optional; specifies URI rewriting to perform on
    ///    *                      external references (as read by readResources).
    ///    *                      If given, also causes relative hyperlinks to be
    ///    *                      converted to absolute in the written text.
    ///    *
    ///    * @param aRequestedContentType
    ///    *                      The desired MIME type to save the document as;
    ///    *                      optional and defaults to the document's type.
    ///    *                      (If no encoder exists for that type, "text/html"
        ///    *                      is used instead.)
    ///    *
    ///    * @param aEncoderFlags Flags to pass to the encoder.
    ///    *
    ///    * @param aWrapColumn   Desired text width, ignored if wrapping is not
    ///    *                      specified by the encoding flags, or if 0.
    ///    *
    ///    * @param aCompletion   Callback invoked when writing is complete.
    ///    *                      It may be called either before or after writeContent
    ///    *                      returns.
    ///    *
    ///    * @see nsIDocumentEncoder
    ///    */
    /// ```
    ///

    /// `void writeContent (in nsIOutputStream aStream, in nsIWebBrowserPersistURIMap aURIMap, in ACString aRequestedContentType, in unsigned long aEncoderFlags, in unsigned long aWrapColumn, in nsIWebBrowserPersistWriteCompletion aCompletion);`
    #[inline]
    pub unsafe fn WriteContent(&self, aStream: *const nsIOutputStream, aURIMap: *const nsIWebBrowserPersistURIMap, aRequestedContentType: *const ::nsstring::nsACString, aEncoderFlags: u32, aWrapColumn: u32, aCompletion: *const nsIWebBrowserPersistWriteCompletion) -> ::nserror::nsresult {
        ((*self.vtable).WriteContent)(self, aStream, aURIMap, aRequestedContentType, aEncoderFlags, aWrapColumn, aCompletion)
    }



    /// `[nostdcall,notxpcom] SHEntryRef GetHistory ();`
    const _GetHistory: () = ();

}


/// `interface nsIWebBrowserPersistResourceVisitor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPersistResourceVisitor {
    vtable: *const nsIWebBrowserPersistResourceVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPersistResourceVisitor.
unsafe impl XpCom for nsIWebBrowserPersistResourceVisitor {
    const IID: nsIID = nsID(0x8ce37706, 0xb7d3, 0x481a,
        [0xbe, 0x68, 0x54, 0xf1, 0x74, 0xfc, 0x0d, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPersistResourceVisitor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPersistResourceVisitor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPersistResourceVisitorCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPersistResourceVisitor`.
    fn coerce_from(v: &nsIWebBrowserPersistResourceVisitor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPersistResourceVisitorCoerce for nsIWebBrowserPersistResourceVisitor {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistResourceVisitor) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistResourceVisitor {
    /// Cast this `nsIWebBrowserPersistResourceVisitor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistResourceVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPersistResourceVisitor {
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
impl<T: nsISupportsCoerce> nsIWebBrowserPersistResourceVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistResourceVisitor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPersistResourceVisitor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPersistResourceVisitorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI, in nsContentPolicyType aContentPolicyType); */
    pub VisitResource: unsafe extern "system" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aURI: *const ::nsstring::nsACString, aContentPolicyType: nsContentPolicyType) -> ::nserror::nsresult,

    /* void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument); */
    pub VisitDocument: unsafe extern "system" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aSubDocument: *const nsIWebBrowserPersistDocument) -> ::nserror::nsresult,

    /* void visitBrowsingContext (in nsIWebBrowserPersistDocument aDocument, in BrowsingContext aContext); */
    pub VisitBrowsingContext: unsafe extern "system" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aContext: *const libc::c_void) -> ::nserror::nsresult,

    /* void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus); */
    pub EndVisit: unsafe extern "system" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPersistResourceVisitor {

    /// ```text
    /// /**
    ///  * Asynchronous visitor that receives external resources linked by an
    ///  * nsIWebBrowserPersistDocument and which are needed to render the
    ///  * document.
    ///  */
    /// /**
    ///    * Indicates a resource that is not a document; e.g., an image, script,
    ///    * or stylesheet.
    ///    *
    ///    * @param aDocument   The document containing the reference.
    ///    * @param aURI        The absolute URI spec for the referenced resource.
    ///    * @param aContentPolicyType The type of resource.
    ///    */
    /// ```
    ///

    /// `void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI, in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn VisitResource(&self, aDocument: *const nsIWebBrowserPersistDocument, aURI: *const ::nsstring::nsACString, aContentPolicyType: nsContentPolicyType) -> ::nserror::nsresult {
        ((*self.vtable).VisitResource)(self, aDocument, aURI, aContentPolicyType)
    }


    /// ```text
    /// /**
    ///    * Indicates a subdocument resource; e.g., a frame or iframe.
    ///    *
    ///    * @param aDocument     The document containing the reference.
    ///    * @param aSubDocument  The referenced document.
    ///    */
    /// ```
    ///

    /// `void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument);`
    #[inline]
    pub unsafe fn VisitDocument(&self, aDocument: *const nsIWebBrowserPersistDocument, aSubDocument: *const nsIWebBrowserPersistDocument) -> ::nserror::nsresult {
        ((*self.vtable).VisitDocument)(self, aDocument, aSubDocument)
    }


    /// ```text
    /// /**
    ///    * Indicates a cross origin subdocument resource; e.g., a frame
    ///    * or iframe loaded in another process.
    ///    *
    ///    * @param aDocument     The document containing the reference.
    ///    * @param aContext      The referenced document's browsing context.
    ///    */
    /// ```
    ///

    /// `void visitBrowsingContext (in nsIWebBrowserPersistDocument aDocument, in BrowsingContext aContext);`
    #[inline]
    pub unsafe fn VisitBrowsingContext(&self, aDocument: *const nsIWebBrowserPersistDocument, aContext: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).VisitBrowsingContext)(self, aDocument, aContext)
    }


    /// ```text
    /// /**
    ///    * Indicates that the document traversal is complete.
    ///    *
    ///    * @param aDocument   The document that was being traversed.
    ///    * @param aStatus     Indicates whether the traversal encountered an error.
    ///    */
    /// ```
    ///

    /// `void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus);`
    #[inline]
    pub unsafe fn EndVisit(&self, aDocument: *const nsIWebBrowserPersistDocument, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).EndVisit)(self, aDocument, aStatus)
    }


}


/// `interface nsIWebBrowserPersistWriteCompletion : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPersistWriteCompletion {
    vtable: *const nsIWebBrowserPersistWriteCompletionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPersistWriteCompletion.
unsafe impl XpCom for nsIWebBrowserPersistWriteCompletion {
    const IID: nsIID = nsID(0xa07e6892, 0x38ae, 0x4207,
        [0x83, 0x40, 0x7f, 0xa6, 0xec, 0x44, 0x6e, 0xd6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPersistWriteCompletion {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPersistWriteCompletion.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPersistWriteCompletionCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPersistWriteCompletion`.
    fn coerce_from(v: &nsIWebBrowserPersistWriteCompletion) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPersistWriteCompletionCoerce for nsIWebBrowserPersistWriteCompletion {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistWriteCompletion) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistWriteCompletion {
    /// Cast this `nsIWebBrowserPersistWriteCompletion` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistWriteCompletionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPersistWriteCompletion {
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
impl<T: nsISupportsCoerce> nsIWebBrowserPersistWriteCompletionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistWriteCompletion) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPersistWriteCompletion
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPersistWriteCompletionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus); */
    pub OnFinish: unsafe extern "system" fn (this: *const nsIWebBrowserPersistWriteCompletion, aDocument: *const nsIWebBrowserPersistDocument, aStream: *const nsIOutputStream, aContentType: *const ::nsstring::nsACString, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPersistWriteCompletion {

    /// ```text
    /// /**
    ///  * Asynchronous callback for when nsIWebBrowserPersistDocument is finished
    ///  * serializing the document's DOM.
    ///  */
    /// /**
    ///    * Indicates that serialization is finished.
    ///    *
    ///    * @param aDocument     The document that was being serialized.
    ///    *
    ///    * @param aStream       The stream that was being written to.  If it
    ///    *                      needs to be closed, the callback must do that;
    ///    *                      the serialization process leaves it open.
    ///    *
    ///    * @param aContentType  The content type with which the document was
    ///    *                      actually serialized; this may be useful to set
    ///    *                      metadata on the result, or if uploading it.
    ///    *
    ///    * @param aStatus       Indicates whether serialization encountered an error.
    ///    */
    /// ```
    ///

    /// `void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnFinish(&self, aDocument: *const nsIWebBrowserPersistDocument, aStream: *const nsIOutputStream, aContentType: *const ::nsstring::nsACString, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnFinish)(self, aDocument, aStream, aContentType, aStatus)
    }


}


/// `interface nsIWebBrowserPersistDocumentReceiver : nsISupports`
///

/// ```text
/// /**
///  * Asynchronous callback for creating a persistable document from some
///  * other object.
///  *
///  * XXXbz This should really be changed to just return a promise that
///  * then gets resolved or rejected...
///  *
///  * @see WebBrowserPersistable in FrameLoader.webidl.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPersistDocumentReceiver {
    vtable: *const nsIWebBrowserPersistDocumentReceiverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPersistDocumentReceiver.
unsafe impl XpCom for nsIWebBrowserPersistDocumentReceiver {
    const IID: nsIID = nsID(0x321e3174, 0x594f, 0x4036,
        [0xb7, 0xbe, 0x79, 0x1b, 0x82, 0x1b, 0xd3, 0x76]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPersistDocumentReceiver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPersistDocumentReceiver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPersistDocumentReceiverCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPersistDocumentReceiver`.
    fn coerce_from(v: &nsIWebBrowserPersistDocumentReceiver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPersistDocumentReceiverCoerce for nsIWebBrowserPersistDocumentReceiver {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocumentReceiver) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistDocumentReceiver {
    /// Cast this `nsIWebBrowserPersistDocumentReceiver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistDocumentReceiverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPersistDocumentReceiver {
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
impl<T: nsISupportsCoerce> nsIWebBrowserPersistDocumentReceiverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocumentReceiver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPersistDocumentReceiver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPersistDocumentReceiverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDocumentReady (in nsIWebBrowserPersistDocument aDocument); */
    pub OnDocumentReady: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocumentReceiver, aDocument: *const nsIWebBrowserPersistDocument) -> ::nserror::nsresult,

    /* void onError (in nsresult aFailure); */
    pub OnError: unsafe extern "system" fn (this: *const nsIWebBrowserPersistDocumentReceiver, aFailure: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPersistDocumentReceiver {


    /// `void onDocumentReady (in nsIWebBrowserPersistDocument aDocument);`
    #[inline]
    pub unsafe fn OnDocumentReady(&self, aDocument: *const nsIWebBrowserPersistDocument) -> ::nserror::nsresult {
        ((*self.vtable).OnDocumentReady)(self, aDocument)
    }



    /// `void onError (in nsresult aFailure);`
    #[inline]
    pub unsafe fn OnError(&self, aFailure: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnError)(self, aFailure)
    }


}


