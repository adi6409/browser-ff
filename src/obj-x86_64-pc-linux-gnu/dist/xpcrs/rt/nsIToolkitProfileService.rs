//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIToolkitProfileService.idl
//


/// `interface nsIToolkitProfileService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIToolkitProfileService {
    vtable: *const nsIToolkitProfileServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIToolkitProfileService.
unsafe impl XpCom for nsIToolkitProfileService {
    const IID: nsIID = nsID(0x1947899b, 0xf369, 0x48fa,
        [0x89, 0xda, 0xf7, 0xc3, 0x7b, 0xb1, 0xe6, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIToolkitProfileService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIToolkitProfileService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIToolkitProfileServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIToolkitProfileService`.
    fn coerce_from(v: &nsIToolkitProfileService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIToolkitProfileServiceCoerce for nsIToolkitProfileService {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfileService) -> &Self {
        v
    }
}

impl nsIToolkitProfileService {
    /// Cast this `nsIToolkitProfileService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIToolkitProfileServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIToolkitProfileService {
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
impl<T: nsISupportsCoerce> nsIToolkitProfileServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfileService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIToolkitProfileService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIToolkitProfileServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute boolean isListOutdated; */
    pub GetIsListOutdated: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aIsListOutdated: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean startWithLastProfile; */
    pub GetStartWithLastProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aStartWithLastProfile: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean startWithLastProfile; */
    pub SetStartWithLastProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aStartWithLastProfile: bool) -> ::nserror::nsresult,

    /* readonly attribute nsISimpleEnumerator profiles; */
    pub GetProfiles: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aProfiles: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* readonly attribute nsIToolkitProfile currentProfile; */
    pub GetCurrentProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aCurrentProfile: *mut*const nsIToolkitProfile) -> ::nserror::nsresult,

    /* attribute nsIToolkitProfile defaultProfile; */
    pub GetDefaultProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aDefaultProfile: *mut*const nsIToolkitProfile) -> ::nserror::nsresult,

    /* attribute nsIToolkitProfile defaultProfile; */
    pub SetDefaultProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aDefaultProfile: *const nsIToolkitProfile) -> ::nserror::nsresult,

    /* readonly attribute boolean createdAlternateProfile; */
    pub GetCreatedAlternateProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aCreatedAlternateProfile: *mut bool) -> ::nserror::nsresult,

    /* bool selectStartupProfile (in Array<ACString> aArgv, in boolean aIsResetting, in AUTF8String aUpdateChannel, in AUTF8String aLegacyInstallHash, out nsIFile aRootDir, out nsIFile aLocalDir, out nsIToolkitProfile aProfile); */
    pub SelectStartupProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aArgv: *const thin_vec::ThinVec<::nsstring::nsCString>, aIsResetting: bool, aUpdateChannel: *const ::nsstring::nsACString, aLegacyInstallHash: *const ::nsstring::nsACString, aRootDir: *mut*const nsIFile, aLocalDir: *mut*const nsIFile, aProfile: *mut*const nsIToolkitProfile, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIToolkitProfile getProfileByName (in AUTF8String aName); */
    pub GetProfileByName: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aName: *const ::nsstring::nsACString, _retval: *mut*const nsIToolkitProfile) -> ::nserror::nsresult,

    /* nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName); */
    pub CreateProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aRootDir: *const nsIFile, aName: *const ::nsstring::nsACString, _retval: *mut*const nsIToolkitProfile) -> ::nserror::nsresult,

    /* nsIToolkitProfile createUniqueProfile (in nsIFile aRootDir, in AUTF8String aNamePrefix); */
    pub CreateUniqueProfile: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aRootDir: *const nsIFile, aNamePrefix: *const ::nsstring::nsACString, _retval: *mut*const nsIToolkitProfile) -> ::nserror::nsresult,

    /* readonly attribute unsigned long profileCount; */
    pub GetProfileCount: unsafe extern "system" fn (this: *const nsIToolkitProfileService, aProfileCount: *mut u32) -> ::nserror::nsresult,

    /* void flush (); */
    pub Flush: unsafe extern "system" fn (this: *const nsIToolkitProfileService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIToolkitProfileService {

    /// ```text
    /// /**
    ///      * Tests whether the profile lists on disk have changed since they were
    ///      * loaded. When this is true attempts to flush changes to disk will fail.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isListOutdated;`
    #[inline]
    pub unsafe fn GetIsListOutdated(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsListOutdated)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `attribute boolean startWithLastProfile;`
    #[inline]
    pub unsafe fn GetStartWithLastProfile(&self, aStartWithLastProfile: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetStartWithLastProfile)(self, aStartWithLastProfile)
    }



    /// `attribute boolean startWithLastProfile;`
    #[inline]
    pub unsafe fn SetStartWithLastProfile(&self, aStartWithLastProfile: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetStartWithLastProfile)(self, aStartWithLastProfile)
    }



    /// `readonly attribute nsISimpleEnumerator profiles;`
    #[inline]
    pub unsafe fn GetProfiles(&self, aProfiles: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetProfiles)(self, aProfiles)
    }


    /// ```text
    /// /**
    ///      * The profile currently in use if it is a named profile. This will return
    ///      * null if the current profile path doesn't match a profile in the database.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIToolkitProfile currentProfile;`
    #[inline]
    pub unsafe fn GetCurrentProfile(&self, aCurrentProfile: *mut*const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentProfile)(self, aCurrentProfile)
    }


    /// ```text
    /// /**
    ///      * The default profile for this build.
    ///      * On startup this is the profile selected unless overridden by command line
    ///      * arguments or environment variables. Setting this will change the profile
    ///      * used by default the next time the application is started.
    ///      * Attempting to change the default may throw an exception on builds that do
    ///      * not support changing the default profile, such as developer edition.
    ///      */
    /// ```
    ///

    /// `attribute nsIToolkitProfile defaultProfile;`
    #[inline]
    pub unsafe fn GetDefaultProfile(&self, aDefaultProfile: *mut*const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultProfile)(self, aDefaultProfile)
    }


    /// ```text
    /// /**
    ///      * The default profile for this build.
    ///      * On startup this is the profile selected unless overridden by command line
    ///      * arguments or environment variables. Setting this will change the profile
    ///      * used by default the next time the application is started.
    ///      * Attempting to change the default may throw an exception on builds that do
    ///      * not support changing the default profile, such as developer edition.
    ///      */
    /// ```
    ///

    /// `attribute nsIToolkitProfile defaultProfile;`
    #[inline]
    pub unsafe fn SetDefaultProfile(&self, aDefaultProfile: *const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultProfile)(self, aDefaultProfile)
    }


    /// ```text
    /// /**
    ///      * True if during startup a new profile was created for this install instead
    ///      * of using the profile that was the default for older versions.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean createdAlternateProfile;`
    #[inline]
    pub unsafe fn GetCreatedAlternateProfile(&self, aCreatedAlternateProfile: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCreatedAlternateProfile)(self, aCreatedAlternateProfile)
    }


    /// ```text
    /// /**
    ///      * Selects or creates a profile to use based on the profiles database, any
    ///      * environment variables and any command line arguments. Will not create
    ///      * a profile if aIsResetting is true. The profile is selected based on this
    ///      * order of preference:
    ///      * * Environment variables (set when restarting the application).
    ///      * * --profile command line argument.
    ///      * * --createprofile command line argument (this also causes the app to exit).
    ///      * * -p command line argument.
    ///      * * A new profile created if this is the first run of the application.
    ///      * * The default profile.
    ///      * aRootDir and aLocalDir are set to the data and local directories for the
    ///      * profile data. If a profile from the database was selected it will be
    ///      * returned in aProfile.
    ///      * This returns true if a new profile was created.
    ///      * This method is primarily for testing. It can be called only once.
    ///      */
    /// ```
    ///

    /// `bool selectStartupProfile (in Array<ACString> aArgv, in boolean aIsResetting, in AUTF8String aUpdateChannel, in AUTF8String aLegacyInstallHash, out nsIFile aRootDir, out nsIFile aLocalDir, out nsIToolkitProfile aProfile);`
    #[inline]
    pub unsafe fn SelectStartupProfile(&self, aArgv: *const thin_vec::ThinVec<::nsstring::nsCString>, aIsResetting: bool, aUpdateChannel: *const ::nsstring::nsACString, aLegacyInstallHash: *const ::nsstring::nsACString, aRootDir: *mut*const nsIFile, aLocalDir: *mut*const nsIFile, aProfile: *mut*const nsIToolkitProfile, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SelectStartupProfile)(self, aArgv, aIsResetting, aUpdateChannel, aLegacyInstallHash, aRootDir, aLocalDir, aProfile, _retval)
    }


    /// ```text
    /// /**
    ///      * Get a profile by name. This is mainly for use by the -P
    ///      * commandline flag.
    ///      *
    ///      * @param aName The profile name to find.
    ///      */
    /// ```
    ///

    /// `nsIToolkitProfile getProfileByName (in AUTF8String aName);`
    #[inline]
    pub unsafe fn GetProfileByName(&self, aName: *const ::nsstring::nsACString, _retval: *mut*const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).GetProfileByName)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a new profile.
    ///      *
    ///      * The profile temporary directory will be chosen based on where the
    ///      * profile directory is located.
    ///      *
    ///      * If a profile with the given name already exists it will be returned
    ///      * instead of creating a new profile.
    ///      *
    ///      * @param aRootDir
    ///      *        The profile directory. May be null, in which case a suitable
    ///      *        default will be chosen based on the profile name.
    ///      * @param aName
    ///      *        The profile name.
    ///      */
    /// ```
    ///

    /// `nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName);`
    #[inline]
    pub unsafe fn CreateProfile(&self, aRootDir: *const nsIFile, aName: *const ::nsstring::nsACString, _retval: *mut*const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).CreateProfile)(self, aRootDir, aName, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a new profile with a unique name.
    ///      *
    ///      * As above however the name given will be altered to make it a unique
    ///      * profile name.
    ///      *
    ///      * @param aRootDir
    ///      *        The profile directory. May be null, in which case a suitable
    ///      *        default will be chosen based on the profile name.
    ///      * @param aNamePrefix
    ///      *        The prefix to use for the profile name. If unused this will be
    ///      *        used as the profile name otherwise additional characters will be
    ///      *        added to make the name unique.
    ///      */
    /// ```
    ///

    /// `nsIToolkitProfile createUniqueProfile (in nsIFile aRootDir, in AUTF8String aNamePrefix);`
    #[inline]
    pub unsafe fn CreateUniqueProfile(&self, aRootDir: *const nsIFile, aNamePrefix: *const ::nsstring::nsACString, _retval: *mut*const nsIToolkitProfile) -> ::nserror::nsresult {
        ((*self.vtable).CreateUniqueProfile)(self, aRootDir, aNamePrefix, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns the number of profiles.
    ///      * @return the number of profiles.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long profileCount;`
    #[inline]
    pub unsafe fn GetProfileCount(&self, aProfileCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetProfileCount)(self, aProfileCount)
    }


    /// ```text
    /// /**
    ///      * Flush the profiles list file. This will fail with
    ///      * NS_ERROR_DATABASE_CHANGED if the files on disk have changed since the
    ///      * profiles were loaded.
    ///      */
    /// ```
    ///

    /// `void flush ();`
    #[inline]
    pub unsafe fn Flush(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Flush)(self, )
    }


}


