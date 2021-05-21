//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEService.idl
//


/// `interface nsIMIMEService : nsISupports`
///

/// ```text
/// /**
///  * The MIME service is responsible for mapping file extensions to MIME-types
///  * (see RFC 2045). It also provides access to nsIMIMEInfo interfaces and
///  * acts as a general convenience wrapper of nsIMIMEInfo interfaces.
///  *
///  * The MIME service maintains a database with a <b>one</b> MIME type <b>to many</b>
///  * file extensions rule. Adding the same file extension to multiple MIME types
///  * is illegal and behavior is undefined.
///  *
///  * @see nsIMIMEInfo
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMIMEService {
    vtable: *const nsIMIMEServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMIMEService.
unsafe impl XpCom for nsIMIMEService {
    const IID: nsIID = nsID(0x5b3675a1, 0x02db, 0x4f8f,
        [0xa5, 0x60, 0xb3, 0x47, 0x36, 0x63, 0x5f, 0x47]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMIMEService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMIMEService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMIMEServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIMIMEService`.
    fn coerce_from(v: &nsIMIMEService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMIMEServiceCoerce for nsIMIMEService {
    #[inline]
    fn coerce_from(v: &nsIMIMEService) -> &Self {
        v
    }
}

impl nsIMIMEService {
    /// Cast this `nsIMIMEService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMIMEServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMIMEService {
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
impl<T: nsISupportsCoerce> nsIMIMEServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMIMEService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMIMEServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
    pub GetFromTypeAndExtension: unsafe extern "system" fn (this: *const nsIMIMEService, aMIMEType: *const ::nsstring::nsACString, aFileExt: *const ::nsstring::nsACString, _retval: *mut*const nsIMIMEInfo) -> ::nserror::nsresult,

    /* ACString getTypeFromExtension (in AUTF8String aFileExt); */
    pub GetTypeFromExtension: unsafe extern "system" fn (this: *const nsIMIMEService, aFileExt: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getTypeFromURI (in nsIURI aURI); */
    pub GetTypeFromURI: unsafe extern "system" fn (this: *const nsIMIMEService, aURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getTypeFromFile (in nsIFile aFile); */
    pub GetTypeFromFile: unsafe extern "system" fn (this: *const nsIMIMEService, aFile: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
    pub GetPrimaryExtension: unsafe extern "system" fn (this: *const nsIMIMEService, aMIMEType: *const ::nsstring::nsACString, aFileExt: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIMIMEInfo getMIMEInfoFromOS (in ACString aType, in ACString aFileExtension, out boolean aFound); */
    pub GetMIMEInfoFromOS: unsafe extern "system" fn (this: *const nsIMIMEService, aType: *const ::nsstring::nsACString, aFileExtension: *const ::nsstring::nsACString, aFound: *mut bool, _retval: *mut*const nsIMIMEInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMIMEService {

    /// ```text
    /// /**
    ///      * Retrieves an nsIMIMEInfo using both the extension
    ///      * and the type of a file. The type is given preference
    ///      * during the lookup. One of aMIMEType and aFileExt
    ///      * can be an empty string. At least one of aMIMEType and aFileExt
    ///      * must be nonempty.
    ///      */
    /// ```
    ///

    /// `nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt);`
    #[inline]
    pub unsafe fn GetFromTypeAndExtension(&self, aMIMEType: *const ::nsstring::nsACString, aFileExt: *const ::nsstring::nsACString, _retval: *mut*const nsIMIMEInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetFromTypeAndExtension)(self, aMIMEType, aFileExt, _retval)
    }


    /// ```text
    /// /**
    ///      * Retrieves a ACString representation of the MIME type
    ///      * associated with this file extension.
    ///      *
    ///      * @param  A file extension (excluding the dot ('.')).
    ///      * @return The MIME type, if any.
    ///      */
    /// ```
    ///

    /// `ACString getTypeFromExtension (in AUTF8String aFileExt);`
    #[inline]
    pub unsafe fn GetTypeFromExtension(&self, aFileExt: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTypeFromExtension)(self, aFileExt, _retval)
    }


    /// ```text
    /// /**
    ///      * Retrieves a ACString representation of the MIME type
    ///      * associated with this URI. The association is purely
    ///      * file extension to MIME type based. No attempt to determine
    ///      * the type via server headers or byte scanning is made.
    ///      *
    ///      * @param  The URI the user wants MIME info on.
    ///      * @return The MIME type, if any.
    ///      */
    /// ```
    ///

    /// `ACString getTypeFromURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetTypeFromURI(&self, aURI: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTypeFromURI)(self, aURI, _retval)
    }



    /// `ACString getTypeFromFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn GetTypeFromFile(&self, aFile: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTypeFromFile)(self, aFile, _retval)
    }


    /// ```text
    /// /**
    ///      * Given a Type/Extension combination, returns the default extension
    ///      * for this type. This may be identical to the passed-in extension.
    ///      *
    ///      * @param aMIMEType The Type to get information on. Must not be empty.
    ///      * @param aFileExt  File Extension. Can be empty.
    ///      */
    /// ```
    ///

    /// `AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt);`
    #[inline]
    pub unsafe fn GetPrimaryExtension(&self, aMIMEType: *const ::nsstring::nsACString, aFileExt: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryExtension)(self, aMIMEType, aFileExt, _retval)
    }



    /// `nsIMIMEInfo getMIMEInfoFromOS (in ACString aType, in ACString aFileExtension, out boolean aFound);`
    #[inline]
    pub unsafe fn GetMIMEInfoFromOS(&self, aType: *const ::nsstring::nsACString, aFileExtension: *const ::nsstring::nsACString, aFound: *mut bool, _retval: *mut*const nsIMIMEInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetMIMEInfoFromOS)(self, aType, aFileExtension, aFound, _retval)
    }


}


