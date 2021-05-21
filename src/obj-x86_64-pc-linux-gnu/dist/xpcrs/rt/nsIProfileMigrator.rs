//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIProfileMigrator.idl
//


/// `interface nsIProfileStartup : nsISupports`
///

/// ```text
/// /**
///  * Helper interface for nsIProfileMigrator.
///  *
///  * @provider Toolkit (Startup code)
///  * @client   Application (Profile-migration code)
///  * @obtainable nsIProfileMigrator.migrate
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProfileStartup {
    vtable: *const nsIProfileStartupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProfileStartup.
unsafe impl XpCom for nsIProfileStartup {
    const IID: nsIID = nsID(0x048e5ca1, 0x0eb7, 0x4bb1,
        [0xa9, 0xa2, 0xa3, 0x6f, 0x7d, 0x4e, 0x0e, 0x3c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProfileStartup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProfileStartup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProfileStartupCoerce {
    /// Cheaply cast a value of this type from a `nsIProfileStartup`.
    fn coerce_from(v: &nsIProfileStartup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProfileStartupCoerce for nsIProfileStartup {
    #[inline]
    fn coerce_from(v: &nsIProfileStartup) -> &Self {
        v
    }
}

impl nsIProfileStartup {
    /// Cast this `nsIProfileStartup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProfileStartupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProfileStartup {
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
impl<T: nsISupportsCoerce> nsIProfileStartupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileStartup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProfileStartup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProfileStartupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile directory; */
    pub GetDirectory: unsafe extern "system" fn (this: *const nsIProfileStartup, aDirectory: *mut*const nsIFile) -> ::nserror::nsresult,

    /* void doStartup (); */
    pub DoStartup: unsafe extern "system" fn (this: *const nsIProfileStartup) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProfileStartup {

    /// ```text
    /// /**
    ///    * The root directory of the semi-current profile, during profile migration.
    ///    * After nsIProfileMigrator.migrate has returned, this object will not be
    ///    * useful.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIFile directory;`
    #[inline]
    pub unsafe fn GetDirectory(&self, aDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetDirectory)(self, aDirectory)
    }


    /// ```text
    /// /**
    ///    * Do profile-startup by setting NS_APP_USER_PROFILE_50_DIR in the directory
    ///    * service and notifying the profile-startup observer topics.
    ///    */
    /// ```
    ///

    /// `void doStartup ();`
    #[inline]
    pub unsafe fn DoStartup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DoStartup)(self, )
    }


}


/// `interface nsIProfileMigrator : nsISupports`
///

/// ```text
/// /**
///  * Migrate application settings from an outside source.
///  *
///  * @provider Application (Profile-migration code)
///  * @client   Toolkit (Startup code)
///  * @obtainable service, contractid("@mozilla.org/toolkit/profile-migrator;1")
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProfileMigrator {
    vtable: *const nsIProfileMigratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProfileMigrator.
unsafe impl XpCom for nsIProfileMigrator {
    const IID: nsIID = nsID(0x3df284a5, 0x2258, 0x4d46,
        [0xa6, 0x64, 0x76, 0x1e, 0xcd, 0xc0, 0x4c, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProfileMigrator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProfileMigrator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProfileMigratorCoerce {
    /// Cheaply cast a value of this type from a `nsIProfileMigrator`.
    fn coerce_from(v: &nsIProfileMigrator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProfileMigratorCoerce for nsIProfileMigrator {
    #[inline]
    fn coerce_from(v: &nsIProfileMigrator) -> &Self {
        v
    }
}

impl nsIProfileMigrator {
    /// Cast this `nsIProfileMigrator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProfileMigratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProfileMigrator {
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
impl<T: nsISupportsCoerce> nsIProfileMigratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileMigrator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProfileMigrator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProfileMigratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in AUTF8String aProfileName); */
    pub Migrate: unsafe extern "system" fn (this: *const nsIProfileMigrator, aStartup: *const nsIProfileStartup, aKey: *const ::nsstring::nsACString, aProfileName: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProfileMigrator {

    /// ```text
    /// /**
    ///    * Migrate data from an outside source, if possible.  Does nothing
    ///    * otherwise.
    ///    *
    ///    * When this method is called, a default profile has been created;
    ///    * XPCOM has been initialized such that compreg.dat is in the
    ///    * profile; the directory service does *not* return a key for
    ///    * NS_APP_USER_PROFILE_50_DIR or any of the keys depending on an active
    ///    * profile. To figure out the directory of the "current" profile, use
    ///    * aStartup.directory.
    ///    *
    ///    * If your migrator needs to access services that use the profile (to
        ///    * set profile prefs or bookmarks, for example), use aStartup.doStartup.
    ///    *
    ///    * @param  aStartup nsIProfileStartup object to use during migration.
    ///    * @param  aKey     optional key of a migrator to use to skip source selection.
    ///    * @param  aProfileName optional name of the profile to use for migration.
    ///    *
    ///    * @note The startup code ignores COM exceptions thrown from this method.
    ///    */
    /// ```
    ///

    /// `void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in AUTF8String aProfileName);`
    #[inline]
    pub unsafe fn Migrate(&self, aStartup: *const nsIProfileStartup, aKey: *const ::nsstring::nsACString, aProfileName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Migrate)(self, aStartup, aKey, aProfileName)
    }


}


