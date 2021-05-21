//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsITransfer.idl
//


/// `interface nsITransfer : nsIWebProgressListener2`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransfer {
    vtable: *const nsITransferVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransfer.
unsafe impl XpCom for nsITransfer {
    const IID: nsIID = nsID(0x37ec75d3, 0x97ad, 0x4da8,
        [0xaf, 0xaa, 0xea, 0xbe, 0x5b, 0x4a, 0xfd, 0x73]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransfer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransfer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransferCoerce {
    /// Cheaply cast a value of this type from a `nsITransfer`.
    fn coerce_from(v: &nsITransfer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransferCoerce for nsITransfer {
    #[inline]
    fn coerce_from(v: &nsITransfer) -> &Self {
        v
    }
}

impl nsITransfer {
    /// Cast this `nsITransfer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransferCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransfer {
    type Target = nsIWebProgressListener2;
    #[inline]
    fn deref(&self) -> &nsIWebProgressListener2 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIWebProgressListener2Coerce> nsITransferCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransfer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransfer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransferVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIWebProgressListener2VTable,

    /* void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification); */
    pub Init: unsafe extern "system" fn (this: *const nsITransfer, aSource: *const nsIURI, aTarget: *const nsIURI, aDisplayName: *const ::nsstring::nsAString, aMIMEInfo: *const nsIMIMEInfo, startTime: PRTime, aTempFile: *const nsIFile, aCancelable: *const nsICancelable, aIsPrivate: bool, aDownloadClassification: i32) -> ::nserror::nsresult,

    /* void initWithBrowsingContext (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification, in BrowsingContext aBrowsingContext, in boolean aHandleInternally); */
    pub InitWithBrowsingContext: unsafe extern "system" fn (this: *const nsITransfer, aSource: *const nsIURI, aTarget: *const nsIURI, aDisplayName: *const ::nsstring::nsAString, aMIMEInfo: *const nsIMIMEInfo, startTime: PRTime, aTempFile: *const nsIFile, aCancelable: *const nsICancelable, aIsPrivate: bool, aDownloadClassification: i32, aBrowsingContext: *const libc::c_void, aHandleInternally: bool) -> ::nserror::nsresult,

    /* void setSha256Hash (in ACString aHash); */
    pub SetSha256Hash: unsafe extern "system" fn (this: *const nsITransfer, aHash: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setSignatureInfo (in Array<Array<Array<uint8_t>>> aSignatureInfo); */
    pub SetSignatureInfo: unsafe extern "system" fn (this: *const nsITransfer, aSignatureInfo: *const thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>) -> ::nserror::nsresult,

    /* void setRedirects (in nsIArray aRedirects); */
    pub SetRedirects: unsafe extern "system" fn (this: *const nsITransfer, aRedirects: *const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransfer {

    pub const DOWNLOAD_ACCEPTABLE: i64 = 0;


    pub const DOWNLOAD_FORBIDDEN: i64 = 1;


    pub const DOWNLOAD_POTENTIALLY_UNSAFE: i64 = 2;

    /// ```text
    /// /**
    ///      * Initializes the transfer with certain properties.  This function must
    ///      * be called prior to accessing any properties on this interface.
    ///      *
    ///      * @param aSource The source URI of the transfer. Must not be null.
    ///      *
    ///      * @param aTarget The target URI of the transfer. Must not be null.
    ///      *
    ///      * @param aDisplayName The user-readable description of the transfer.
    ///      *                     Can be empty.
    ///      *
    ///      * @param aMIMEInfo The MIME info associated with the target,
    ///      *                  including MIME type and helper app when appropriate.
    ///      *                  This parameter is optional.
    ///      *
    ///      * @param startTime Time when the download started (ie, when the first
        ///      *                  response from the server was received)
    ///      *                  XXX presumably wbp and exthandler do this differently
    ///      *
    ///      * @param aTempFile The location of a temporary file; i.e. a file in which
    ///      *                  the received data will be stored, but which is not
    ///      *                  equal to the target file. (will be moved to the real
        ///      *                  target by the caller, when the download is finished)
    ///      *                  May be null.
    ///      *
    ///      * @param aCancelable An object that can be used to abort the download.
    ///      *                    Must not be null.
    ///      *                    Implementations are expected to hold a strong
    ///      *                    reference to this object until the download is
    ///      *                    finished, at which point they should release the
    ///      *                    reference.
    ///      *
    ///      * @param aIsPrivate Used to determine the privacy status of the new transfer.
    ///      *                   If true, indicates that the transfer was initiated from
    ///      *                   a source that desires privacy.
    ///      *
    ///      * @param aDownloadClassification Indicates wheter the dowload is unwanted,
    ///      *                                should be considered dangerous or insecure.
    ///      */
    /// ```
    ///

    /// `void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification);`
    #[inline]
    pub unsafe fn Init(&self, aSource: *const nsIURI, aTarget: *const nsIURI, aDisplayName: *const ::nsstring::nsAString, aMIMEInfo: *const nsIMIMEInfo, startTime: PRTime, aTempFile: *const nsIFile, aCancelable: *const nsICancelable, aIsPrivate: bool, aDownloadClassification: i32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aSource, aTarget, aDisplayName, aMIMEInfo, startTime, aTempFile, aCancelable, aIsPrivate, aDownloadClassification)
    }


    /// ```text
    /// /**
    ///      * Same as init, but allows for passing the browsingContext
    ///      * which will allow for opening the download with the same
    ///      * userContextId
    ///      *
    ///      * @param aBrowsingContext BrowsingContext of the initiating document.
    ///      *
    ///      * @param aHandleInternally Set to true if the download should be opened within
    ///      *                          the browser.
    ///      */
    /// ```
    ///

    /// `void initWithBrowsingContext (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification, in BrowsingContext aBrowsingContext, in boolean aHandleInternally);`
    #[inline]
    pub unsafe fn InitWithBrowsingContext(&self, aSource: *const nsIURI, aTarget: *const nsIURI, aDisplayName: *const ::nsstring::nsAString, aMIMEInfo: *const nsIMIMEInfo, startTime: PRTime, aTempFile: *const nsIFile, aCancelable: *const nsICancelable, aIsPrivate: bool, aDownloadClassification: i32, aBrowsingContext: *const libc::c_void, aHandleInternally: bool) -> ::nserror::nsresult {
        ((*self.vtable).InitWithBrowsingContext)(self, aSource, aTarget, aDisplayName, aMIMEInfo, startTime, aTempFile, aCancelable, aIsPrivate, aDownloadClassification, aBrowsingContext, aHandleInternally)
    }



    /// `void setSha256Hash (in ACString aHash);`
    #[inline]
    pub unsafe fn SetSha256Hash(&self, aHash: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetSha256Hash)(self, aHash)
    }



    /// `void setSignatureInfo (in Array<Array<Array<uint8_t>>> aSignatureInfo);`
    #[inline]
    pub unsafe fn SetSignatureInfo(&self, aSignatureInfo: *const thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>) -> ::nserror::nsresult {
        ((*self.vtable).SetSignatureInfo)(self, aSignatureInfo)
    }



    /// `void setRedirects (in nsIArray aRedirects);`
    #[inline]
    pub unsafe fn SetRedirects(&self, aRedirects: *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).SetRedirects)(self, aRedirects)
    }


}


