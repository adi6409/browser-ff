//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIHandlerService.idl
//


/// `interface nsIHandlerService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHandlerService {
    vtable: *const nsIHandlerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHandlerService.
unsafe impl XpCom for nsIHandlerService {
    const IID: nsIID = nsID(0x53f0ad17, 0xec62, 0x46a1,
        [0xad, 0xbc, 0xef, 0xcc, 0xc0, 0x6b, 0xab, 0xcd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHandlerService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHandlerService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHandlerServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIHandlerService`.
    fn coerce_from(v: &nsIHandlerService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHandlerServiceCoerce for nsIHandlerService {
    #[inline]
    fn coerce_from(v: &nsIHandlerService) -> &Self {
        v
    }
}

impl nsIHandlerService {
    /// Cast this `nsIHandlerService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHandlerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHandlerService {
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
impl<T: nsISupportsCoerce> nsIHandlerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandlerService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHandlerService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHandlerServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void asyncInit (); */
    pub AsyncInit: unsafe extern "system" fn (this: *const nsIHandlerService) -> ::nserror::nsresult,

    /* nsISimpleEnumerator enumerate (); */
    pub Enumerate: unsafe extern "system" fn (this: *const nsIHandlerService, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType); */
    pub FillHandlerInfo: unsafe extern "system" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo, aOverrideType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void store (in nsIHandlerInfo aHandlerInfo); */
    pub Store: unsafe extern "system" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo) -> ::nserror::nsresult,

    /* boolean exists (in nsIHandlerInfo aHandlerInfo); */
    pub Exists: unsafe extern "system" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo, _retval: *mut bool) -> ::nserror::nsresult,

    /* void remove (in nsIHandlerInfo aHandlerInfo); */
    pub Remove: unsafe extern "system" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo) -> ::nserror::nsresult,

    /* ACString getTypeFromExtension (in ACString aFileExtension); */
    pub GetTypeFromExtension: unsafe extern "system" fn (this: *const nsIHandlerService, aFileExtension: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean existsForProtocolOS (in ACString aProtocolScheme); */
    pub ExistsForProtocolOS: unsafe extern "system" fn (this: *const nsIHandlerService, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean existsForProtocol (in ACString aProtocolScheme); */
    pub ExistsForProtocol: unsafe extern "system" fn (this: *const nsIHandlerService, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void getMIMEInfoFromOS (in nsIHandlerInfo aHandlerInfo, in ACString aMIMEType, in ACString aExtension, out bool aFound); */
    pub GetMIMEInfoFromOS: unsafe extern "system" fn (this: *const nsIHandlerService, aHandlerInfo: *const nsIHandlerInfo, aMIMEType: *const ::nsstring::nsACString, aExtension: *const ::nsstring::nsACString, aFound: *mut bool) -> ::nserror::nsresult,

    /* AString getApplicationDescription (in ACString aProtocolScheme); */
    pub GetApplicationDescription: unsafe extern "system" fn (this: *const nsIHandlerService, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHandlerService {

    /// ```text
    /// /**
    ///    * Asynchronously performs any IO that the nsIHandlerService needs to do
    ///    * before it can be of use.
    ///    */
    /// ```
    ///

    /// `void asyncInit ();`
    #[inline]
    pub unsafe fn AsyncInit(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AsyncInit)(self, )
    }


    /// ```text
    /// /**
    ///    * Retrieve a list of all handlers in the datastore.  This list is not
    ///    * guaranteed to be in any particular order, and callers should not assume
    ///    * it will remain in the same order in the future.
    ///    *
    ///    * @returns a list of all handlers in the datastore
    ///    */
    /// ```
    ///

    /// `nsISimpleEnumerator enumerate ();`
    #[inline]
    pub unsafe fn Enumerate(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Enumerate)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Fill a handler info object with information from the datastore.
    ///    *
    ///    * Note: because of the way the external helper app service currently mixes
    ///    * OS and user handler info in the same handler info object, this method
    ///    * takes an existing handler info object (probably retrieved from the OS)
    ///    * and "fills it in" with information from the datastore, overriding any
    ///    * existing properties on the object with properties from the datastore.
    ///    *
    ///    * Ultimately, however, we're going to separate OS and user handler info
    ///    * into separate objects, at which point this method should be renamed to
    ///    * something like "get" or "retrieve", take a class and type (or perhaps
        ///    * a type whose class can be determined by querying the type, for example
        ///    * an nsIContentType which is also an nsIMIMEType or an nsIProtocolScheme),
    ///    * and return a handler info object representing only the user info.
    ///    *
    ///    * Note: if you specify an override type, then the service will fill in
    ///    * the handler info object with information about that type instead of
    ///    * the type specified by the object's nsIHandlerInfo::type attribute.
    ///    *
    ///    * This is useful when you are trying to retrieve information about a MIME
    ///    * type that doesn't exist in the datastore, but you have a file extension
    ///    * for that type, and nsIHandlerService::getTypeFromExtension returns another
    ///    * MIME type that does exist in the datastore and can handle that extension.
    ///    *
    ///    * For example, the user clicks on a link, and the content has a MIME type
    ///    * that isn't in the datastore, but the link has a file extension, and that
    ///    * extension is associated with another MIME type in the datastore (perhaps
        ///    * an unofficial MIME type preceded an official one, like with image/x-png
        ///    * and image/png).
    ///    *
    ///    * In that situation, you can call this method to fill in the handler info
    ///    * object with information about that other type by passing the other type
    ///    * as the aOverrideType parameter.
    ///    *
    ///    * @param aHandlerInfo   the handler info object
    ///    * @param aOverrideType  a type to use instead of aHandlerInfo::type
    ///    *
    ///    * Note: if there is no information in the datastore about this type,
    ///    * this method throws NS_ERROR_NOT_AVAILABLE. Callers are encouraged to
    ///    * check exists() before calling fillHandlerInfo(), to prevent spamming the
    ///    * console with XPCOM exception errors.
    ///    */
    /// ```
    ///

    /// `void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType);`
    #[inline]
    pub unsafe fn FillHandlerInfo(&self, aHandlerInfo: *const nsIHandlerInfo, aOverrideType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).FillHandlerInfo)(self, aHandlerInfo, aOverrideType)
    }


    /// ```text
    /// /**
    ///    * Save the preferred action, preferred handler, possible handlers, and
    ///    * always ask properties of the given handler info object to the datastore.
    ///    * Updates an existing record or creates a new one if necessary.
    ///    *
    ///    * Note: if preferred action is undefined or invalid, then we assume
    ///    * the default value nsIHandlerInfo::useHelperApp.
    ///    *
    ///    * @param aHandlerInfo  the handler info object
    ///    */
    /// ```
    ///

    /// `void store (in nsIHandlerInfo aHandlerInfo);`
    #[inline]
    pub unsafe fn Store(&self, aHandlerInfo: *const nsIHandlerInfo) -> ::nserror::nsresult {
        ((*self.vtable).Store)(self, aHandlerInfo)
    }


    /// ```text
    /// /**
    ///    * Whether or not a record for the given handler info object exists
    ///    * in the datastore. If the datastore is corrupt (or some other error
        ///    * is caught in the implementation), false will be returned.
    ///    *
    ///    * @param aHandlerInfo  a handler info object
    ///    *
    ///    * @returns whether or not a record exists
    ///    */
    /// ```
    ///

    /// `boolean exists (in nsIHandlerInfo aHandlerInfo);`
    #[inline]
    pub unsafe fn Exists(&self, aHandlerInfo: *const nsIHandlerInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Exists)(self, aHandlerInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Remove the given handler info object from the datastore.  Deletes all
    ///    * records associated with the object, including the preferred handler, info,
    ///    * and type records plus the entry in the list of types, if they exist.
    ///    * Otherwise, it does nothing and does not return an error.
    ///    *
    ///    * @param aHandlerInfo  the handler info object
    ///    */
    /// ```
    ///

    /// `void remove (in nsIHandlerInfo aHandlerInfo);`
    #[inline]
    pub unsafe fn Remove(&self, aHandlerInfo: *const nsIHandlerInfo) -> ::nserror::nsresult {
        ((*self.vtable).Remove)(self, aHandlerInfo)
    }


    /// ```text
    /// /**
    ///    * Get the MIME type mapped to the given file extension in the datastore.
    ///    *
    ///    * XXX If we ever support extension -> protocol scheme mappings, then this
    ///    * method should work for those as well.
    ///    *
    ///    * Note: in general, you should use nsIMIMEService::getTypeFromExtension
    ///    * to get a MIME type from a file extension, as that method checks a variety
    ///    * of other sources besides just the datastore.  Use this only when you want
    ///    * to specifically get only the mapping available in the datastore.
    ///    *
    ///    * @param aFileExtension  the file extension
    ///    *
    ///    * @returns the MIME type, if any; otherwise returns an empty string ("").
    ///    */
    /// ```
    ///

    /// `ACString getTypeFromExtension (in ACString aFileExtension);`
    #[inline]
    pub unsafe fn GetTypeFromExtension(&self, aFileExtension: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTypeFromExtension)(self, aFileExtension, _retval)
    }


    /// ```text
    /// /**
    ///    * Whether or not there is a handler known to the OS for the
    ///    * specified protocol type.
    ///    *
    ///    * @param aProtocolScheme scheme to check for support
    ///    *
    ///    * @returns whether or not a handler exists
    ///    */
    /// ```
    ///

    /// `boolean existsForProtocolOS (in ACString aProtocolScheme);`
    #[inline]
    pub unsafe fn ExistsForProtocolOS(&self, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExistsForProtocolOS)(self, aProtocolScheme, _retval)
    }


    /// ```text
    /// /**
    ///    * Whether or not there is a handler in the datastore or OS for
    ///    * the specified protocol type. If there is no handler in the datastore,
    ///    * falls back to a check for an OS handler.
    ///    *
    ///    * @param aProtocolScheme scheme to check for support
    ///    *
    ///    * @returns whether or not a handler exists
    ///    */
    /// ```
    ///

    /// `boolean existsForProtocol (in ACString aProtocolScheme);`
    #[inline]
    pub unsafe fn ExistsForProtocol(&self, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExistsForProtocol)(self, aProtocolScheme, _retval)
    }



    /// `void getMIMEInfoFromOS (in nsIHandlerInfo aHandlerInfo, in ACString aMIMEType, in ACString aExtension, out bool aFound);`
    #[inline]
    pub unsafe fn GetMIMEInfoFromOS(&self, aHandlerInfo: *const nsIHandlerInfo, aMIMEType: *const ::nsstring::nsACString, aExtension: *const ::nsstring::nsACString, aFound: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMIMEInfoFromOS)(self, aHandlerInfo, aMIMEType, aExtension, aFound)
    }



    /// `AString getApplicationDescription (in ACString aProtocolScheme);`
    #[inline]
    pub unsafe fn GetApplicationDescription(&self, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationDescription)(self, aProtocolScheme, _retval)
    }


}


