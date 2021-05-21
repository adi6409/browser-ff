//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/migration/nsIBrowserProfileMigrator.idl
//


/// `interface nsIBrowserProfileMigrator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserProfileMigrator {
    vtable: *const nsIBrowserProfileMigratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserProfileMigrator.
unsafe impl XpCom for nsIBrowserProfileMigrator {
    const IID: nsIID = nsID(0x22b56ffc, 0x3149, 0x43c5,
        [0xb5, 0xa9, 0xb3, 0xa6, 0xb6, 0x78, 0xde, 0x93]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserProfileMigrator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserProfileMigrator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserProfileMigratorCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserProfileMigrator`.
    fn coerce_from(v: &nsIBrowserProfileMigrator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserProfileMigratorCoerce for nsIBrowserProfileMigrator {
    #[inline]
    fn coerce_from(v: &nsIBrowserProfileMigrator) -> &Self {
        v
    }
}

impl nsIBrowserProfileMigrator {
    /// Cast this `nsIBrowserProfileMigrator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserProfileMigratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserProfileMigrator {
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
impl<T: nsISupportsCoerce> nsIBrowserProfileMigratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserProfileMigrator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserProfileMigrator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserProfileMigratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void migrate (in unsigned short aItems, in nsIProfileStartup aStartup, in jsval aProfile); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Migrate: *const ::libc::c_void,

    /* jsval getMigrateData (in jsval aProfile); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetMigrateData: *const ::libc::c_void,

    /* jsval getLastUsedDate (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetLastUsedDate: *const ::libc::c_void,

    /* jsval isSourceAvailable (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsSourceAvailable: *const ::libc::c_void,

    /* jsval getSourceProfiles (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetSourceProfiles: *const ::libc::c_void,

    /* readonly attribute boolean sourceLocked; */
    pub GetSourceLocked: unsafe extern "system" fn (this: *const nsIBrowserProfileMigrator, aSourceLocked: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserProfileMigrator {
    /// ```text
    /// /**
    ///    * profile items to migrate. use with migrate().
    ///    */
    /// ```
    ///

    pub const ALL: i64 = 0;


    pub const COOKIES: i64 = 2;


    pub const HISTORY: i64 = 4;


    pub const FORMDATA: i64 = 8;


    pub const PASSWORDS: i64 = 16;


    pub const BOOKMARKS: i64 = 32;


    pub const OTHERDATA: i64 = 64;


    pub const SESSION: i64 = 128;

    /// ```text
    /// /**
    ///    * Copy user profile information to the current active profile.
    ///    * @param aItems   list of data items to migrate. see above for values.
    ///    * @param aStartup helper interface which is non-null if called during startup.
    ///    * @param aProfile profile to migrate from, if there is more than one.
    ///    */
    /// ```
    ///

    /// `void migrate (in unsigned short aItems, in nsIProfileStartup aStartup, in jsval aProfile);`
    const _Migrate: () = ();

    /// ```text
    /// /**
    ///    * A bit field containing profile items that this migrator
    ///    * offers for import.
    ///    * @param   aProfile the profile that we are looking for available data
    ///    *          to import
    ///    * @return  Promise containing a bit field containing profile items (see above)
    ///    * @note    a return value of 0 represents no items rather than ALL.
    ///    */
    /// ```
    ///

    /// `jsval getMigrateData (in jsval aProfile);`
    const _GetMigrateData: () = ();

    /// ```text
    /// /**
    ///    * Get the last time data from this browser was modified
    ///    * @return a promise that resolves to a JS Date object
    ///    */
    /// ```
    ///

    /// `jsval getLastUsedDate ();`
    const _GetLastUsedDate: () = ();

    /// ```text
    /// /**
    ///    * Get whether or not there is any data that can be imported from this
    ///    * browser (i.e. whether or not it is installed, and there exists
        ///    * a user profile)
    ///    * @return a promise that resolves with a boolean.
    ///    */
    /// ```
    ///

    /// `jsval isSourceAvailable ();`
    const _IsSourceAvailable: () = ();

    /// ```text
    /// /**
    ///    * An enumeration of available profiles. If the import source does
    ///    * not support profiles, this attribute is null.
    ///    * @return a promise that resolves with an array of profiles or null.
    ///    */
    /// ```
    ///

    /// `jsval getSourceProfiles ();`
    const _GetSourceProfiles: () = ();

    /// ```text
    /// /**
    ///    * Whether the source browser data is locked/in-use meaning migration likely
    ///    * won't succeed and the user should be warned.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean sourceLocked;`
    #[inline]
    pub unsafe fn GetSourceLocked(&self, aSourceLocked: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceLocked)(self, aSourceLocked)
    }


}


