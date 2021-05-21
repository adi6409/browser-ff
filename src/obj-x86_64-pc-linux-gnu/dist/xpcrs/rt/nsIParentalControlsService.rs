//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/parentalcontrols/nsIParentalControlsService.idl
//


/// `interface nsIParentalControlsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIParentalControlsService {
    vtable: *const nsIParentalControlsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIParentalControlsService.
unsafe impl XpCom for nsIParentalControlsService {
    const IID: nsIID = nsID(0x2e97e5dd, 0x467b, 0x4aea,
        [0xa1, 0xbb, 0x67, 0x73, 0xc0, 0xf2, 0xbe, 0xb0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIParentalControlsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIParentalControlsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIParentalControlsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIParentalControlsService`.
    fn coerce_from(v: &nsIParentalControlsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIParentalControlsServiceCoerce for nsIParentalControlsService {
    #[inline]
    fn coerce_from(v: &nsIParentalControlsService) -> &Self {
        v
    }
}

impl nsIParentalControlsService {
    /// Cast this `nsIParentalControlsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIParentalControlsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIParentalControlsService {
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
impl<T: nsISupportsCoerce> nsIParentalControlsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParentalControlsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIParentalControlsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIParentalControlsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean parentalControlsEnabled; */
    pub GetParentalControlsEnabled: unsafe extern "system" fn (this: *const nsIParentalControlsService, aParentalControlsEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean blockFileDownloadsEnabled; */
    pub GetBlockFileDownloadsEnabled: unsafe extern "system" fn (this: *const nsIParentalControlsService, aBlockFileDownloadsEnabled: *mut bool) -> ::nserror::nsresult,

    /* boolean isAllowed (in short aAction, [optional] in nsIURI aUri); */
    pub IsAllowed: unsafe extern "system" fn (this: *const nsIParentalControlsService, aAction: i16, aUri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean loggingEnabled; */
    pub GetLoggingEnabled: unsafe extern "system" fn (this: *const nsIParentalControlsService, aLoggingEnabled: *mut bool) -> ::nserror::nsresult,

    /* void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget); */
    pub Log: unsafe extern "system" fn (this: *const nsIParentalControlsService, aEntryType: i16, aFlag: bool, aSource: *const nsIURI, aTarget: *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIParentalControlsService {
    /// ```text
    /// /**
    ///    * Action types that can be blocked for users.
    ///    */
    /// ```
    ///

    pub const DOWNLOAD: i64 = 1;


    pub const INSTALL_EXTENSION: i64 = 2;


    pub const INSTALL_APP: i64 = 3;


    pub const BROWSE: i64 = 4;


    pub const SHARE: i64 = 5;


    pub const BOOKMARK: i64 = 6;


    pub const ADD_CONTACT: i64 = 7;


    pub const SET_IMAGE: i64 = 8;


    pub const MODIFY_ACCOUNTS: i64 = 9;


    pub const REMOTE_DEBUGGING: i64 = 10;


    pub const IMPORT_SETTINGS: i64 = 11;


    pub const PRIVATE_BROWSING: i64 = 12;


    pub const DATA_CHOICES: i64 = 13;


    pub const CLEAR_HISTORY: i64 = 14;


    pub const MASTER_PASSWORD: i64 = 15;


    pub const GUEST_BROWSING: i64 = 16;


    pub const ADVANCED_SETTINGS: i64 = 17;


    pub const CAMERA_MICROPHONE: i64 = 18;


    pub const BLOCK_LIST: i64 = 19;


    pub const TELEMETRY: i64 = 20;


    pub const HEALTH_REPORT: i64 = 21;


    pub const DEFAULT_THEME: i64 = 22;

    /// ```text
    /// /**
    ///    * Log entry types. Additional types can be defined and implemented
    ///    * as needed. Other possible event types might include email events,
    ///    * media related events, and IM events.
    ///    */
    /// ```
    ///

    pub const ePCLog_URIVisit: i64 = 1;


    pub const ePCLog_FileDownload: i64 = 2;

    /// ```text
    /// /**
    ///    * @returns true if the current user account has parental controls
    ///    * restrictions enabled.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean parentalControlsEnabled;`
    #[inline]
    pub unsafe fn GetParentalControlsEnabled(&self, aParentalControlsEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetParentalControlsEnabled)(self, aParentalControlsEnabled)
    }


    /// ```text
    /// /**
    ///    * @returns true if the current user account parental controls
    ///    * restrictions include the blocking of all file downloads.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean blockFileDownloadsEnabled;`
    #[inline]
    pub unsafe fn GetBlockFileDownloadsEnabled(&self, aBlockFileDownloadsEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBlockFileDownloadsEnabled)(self, aBlockFileDownloadsEnabled)
    }


    /// ```text
    /// /**
    ///    * Check if the user can do the prescibed action for this uri.
    ///    *
    ///    * @param aAction             Action being performed
    ///    * @param aUri                The uri requesting this action
    ///    * @param aWindow             The window generating this event.
    ///    */
    /// ```
    ///

    /// `boolean isAllowed (in short aAction, [optional] in nsIURI aUri);`
    #[inline]
    pub unsafe fn IsAllowed(&self, aAction: i16, aUri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsAllowed)(self, aAction, aUri, _retval)
    }


    /// ```text
    /// /**
    ///    * @returns true if the current user account has parental controls
    ///    * logging enabled. If true, applications should log relevent events
    ///    * using 'log'.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean loggingEnabled;`
    #[inline]
    pub unsafe fn GetLoggingEnabled(&self, aLoggingEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLoggingEnabled)(self, aLoggingEnabled)
    }


    /// ```text
    /// /**
    ///    * Log an application specific parental controls
    ///    * event.
    ///    *
    ///    * @param aEntryType       Constant defining the type of event.
    ///    * @param aFlag            A flag indicating if the subject content
    ///    *                         was blocked.
    ///    * @param aSource          The URI source of the subject content.
    ///    * @param aTarget          The location the content was saved to if
    ///    *                         no blocking occurred.
    ///    */
    /// ```
    ///

    /// `void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget);`
    #[inline]
    pub unsafe fn Log(&self, aEntryType: i16, aFlag: bool, aSource: *const nsIURI, aTarget: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Log)(self, aEntryType, aFlag, aSource, aTarget)
    }


}


