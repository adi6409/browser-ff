//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIPrefService.idl
//


/// `interface nsIPrefStatsCallback : nsISupports`
///

/// ```text
/// /**
///  * A helper function for reading access statistics for preferences.
///  * See nsIPrefService.readStats for more details.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrefStatsCallback {
    vtable: *const nsIPrefStatsCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrefStatsCallback.
unsafe impl XpCom for nsIPrefStatsCallback {
    const IID: nsIID = nsID(0xc3f0cedc, 0xe244, 0x4316,
        [0xb3, 0x3a, 0x80, 0x30, 0x6a, 0x1c, 0x35, 0xa1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrefStatsCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrefStatsCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrefStatsCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIPrefStatsCallback`.
    fn coerce_from(v: &nsIPrefStatsCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrefStatsCallbackCoerce for nsIPrefStatsCallback {
    #[inline]
    fn coerce_from(v: &nsIPrefStatsCallback) -> &Self {
        v
    }
}

impl nsIPrefStatsCallback {
    /// Cast this `nsIPrefStatsCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrefStatsCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrefStatsCallback {
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
impl<T: nsISupportsCoerce> nsIPrefStatsCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefStatsCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrefStatsCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrefStatsCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void visit (in ACString prefName, in unsigned long accessCount); */
    pub Visit: unsafe extern "system" fn (this: *const nsIPrefStatsCallback, prefName: *const ::nsstring::nsACString, accessCount: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrefStatsCallback {


    /// `void visit (in ACString prefName, in unsigned long accessCount);`
    #[inline]
    pub unsafe fn Visit(&self, prefName: *const ::nsstring::nsACString, accessCount: u32) -> ::nserror::nsresult {
        ((*self.vtable).Visit)(self, prefName, accessCount)
    }


}


/// `interface nsIPrefService : nsISupports`
///

/// ```text
/// /**
///  * The nsIPrefService interface is the main entry point into the back end
///  * preferences management library. The preference service is directly
///  * responsible for the management of the preferences files and also facilitates
///  * access to the preference branch object which allows the direct manipulation
///  * of the preferences themselves.
///  *
///  * @see nsIPrefBranch
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrefService {
    vtable: *const nsIPrefServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrefService.
unsafe impl XpCom for nsIPrefService {
    const IID: nsIID = nsID(0x1f84fd56, 0x3956, 0x40df,
        [0xb8, 0x6a, 0x1e, 0xa0, 0x14, 0x02, 0xee, 0x96]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrefService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrefService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrefServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPrefService`.
    fn coerce_from(v: &nsIPrefService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrefServiceCoerce for nsIPrefService {
    #[inline]
    fn coerce_from(v: &nsIPrefService) -> &Self {
        v
    }
}

impl nsIPrefService {
    /// Cast this `nsIPrefService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrefServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrefService {
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
impl<T: nsISupportsCoerce> nsIPrefServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrefService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrefServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resetPrefs (); */
    pub ResetPrefs: unsafe extern "system" fn (this: *const nsIPrefService) -> ::nserror::nsresult,

    /* void resetUserPrefs (); */
    pub ResetUserPrefs: unsafe extern "system" fn (this: *const nsIPrefService) -> ::nserror::nsresult,

    /* void savePrefFile (in nsIFile aFile); */
    pub SavePrefFile: unsafe extern "system" fn (this: *const nsIPrefService, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* nsIPrefBranch getBranch (in string aPrefRoot); */
    pub GetBranch: unsafe extern "system" fn (this: *const nsIPrefService, aPrefRoot: *const libc::c_char, _retval: *mut *const nsIPrefBranch) -> ::nserror::nsresult,

    /* nsIPrefBranch getDefaultBranch (in string aPrefRoot); */
    pub GetDefaultBranch: unsafe extern "system" fn (this: *const nsIPrefService, aPrefRoot: *const libc::c_char, _retval: *mut *const nsIPrefBranch) -> ::nserror::nsresult,

    /* readonly attribute boolean dirty; */
    pub GetDirty: unsafe extern "system" fn (this: *const nsIPrefService, aDirty: *mut bool) -> ::nserror::nsresult,

    /* void readDefaultPrefsFromFile (in nsIFile aFile); */
    pub ReadDefaultPrefsFromFile: unsafe extern "system" fn (this: *const nsIPrefService, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* void readUserPrefsFromFile (in nsIFile aFile); */
    pub ReadUserPrefsFromFile: unsafe extern "system" fn (this: *const nsIPrefService, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* void readStats (in nsIPrefStatsCallback callback); */
    pub ReadStats: unsafe extern "system" fn (this: *const nsIPrefService, callback: *const nsIPrefStatsCallback) -> ::nserror::nsresult,

    /* void resetStats (); */
    pub ResetStats: unsafe extern "system" fn (this: *const nsIPrefService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrefService {

    /// ```text
    /// /**
    ///    * Called to completely flush and re-initialize the preferences system.
    ///    *
    ///    * @throws Error The preference service failed to restart correctly.
    ///    */
    /// ```
    ///

    /// `void resetPrefs ();`
    #[inline]
    pub unsafe fn ResetPrefs(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetPrefs)(self, )
    }


    /// ```text
    /// /**
    ///    * Called to reset all preferences with user set values back to the
    ///    * application default values.
    ///    */
    /// ```
    ///

    /// `void resetUserPrefs ();`
    #[inline]
    pub unsafe fn ResetUserPrefs(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetUserPrefs)(self, )
    }


    /// ```text
    /// /**
    ///    * Called to write current preferences state to a file.
    ///    *
    ///    * @param aFile The file to be written.
    ///    *
    ///    * @note
    ///    * If nullptr is passed in for the aFile parameter the preference data is
    ///    * written out to the current preferences file (usually prefs.js.)
    ///    *
    ///    * @throws Error File failed to write.
    ///    *
    ///    * @see readUserPrefsFromFile
    ///    * @see nsIFile
    ///    */
    /// ```
    ///

    /// `void savePrefFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn SavePrefFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SavePrefFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * Call to get a Preferences "Branch" which accesses user preference data.
    ///    * Using a Set method on this object will always create or set a user
    ///    * preference value. When using a Get method a user set value will be
    ///    * returned if one exists, otherwise a default value will be returned.
    ///    *
    ///    * @param aPrefRoot The preference "root" on which to base this "branch".
    ///    *                  For example, if the root "browser.startup." is used, the
    ///    *                  branch will be able to easily access the preferences
    ///    *                  "browser.startup.page", "browser.startup.homepage", or
    ///    *                  "browser.startup.homepage_override" by simply requesting
    ///    *                  "page", "homepage", or "homepage_override". nullptr or ""
    ///    *                  may be used to access to the entire preference "tree".
    ///    *
    ///    * @return nsIPrefBranch The object representing the requested branch.
    ///    *
    ///    * @see getDefaultBranch
    ///    */
    /// ```
    ///

    /// `nsIPrefBranch getBranch (in string aPrefRoot);`
    #[inline]
    pub unsafe fn GetBranch(&self, aPrefRoot: *const libc::c_char, _retval: *mut *const nsIPrefBranch) -> ::nserror::nsresult {
        ((*self.vtable).GetBranch)(self, aPrefRoot, _retval)
    }


    /// ```text
    /// /**
    ///    * Call to get a Preferences "Branch" which accesses only the default
    ///    * preference data. Using a Set method on this object will always create or
    ///    * set a default preference value. When using a Get method a default value
    ///    * will always be returned.
    ///    *
    ///    * @param aPrefRoot The preference "root" on which to base this "branch".
    ///    *                  For example, if the root "browser.startup." is used, the
    ///    *                  branch will be able to easily access the preferences
    ///    *                  "browser.startup.page", "browser.startup.homepage", or
    ///    *                  "browser.startup.homepage_override" by simply requesting
    ///    *                  "page", "homepage", or "homepage_override". nullptr or ""
    ///    *                  may be used to access to the entire preference "tree".
    ///    *
    ///    * @note
    ///    * Few consumers will want to create default branch objects. Many of the
    ///    * branch methods do nothing on a default branch because the operations only
    ///    * make sense when applied to user set preferences.
    ///    *
    ///    * @return nsIPrefBranch The object representing the requested default branch.
    ///    *
    ///    * @see getBranch
    ///    */
    /// ```
    ///

    /// `nsIPrefBranch getDefaultBranch (in string aPrefRoot);`
    #[inline]
    pub unsafe fn GetDefaultBranch(&self, aPrefRoot: *const libc::c_char, _retval: *mut *const nsIPrefBranch) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultBranch)(self, aPrefRoot, _retval)
    }


    /// ```text
    /// /**
    ///    * The preference service is 'dirty' if there are changes to user preferences
    ///    * that have not been written to disk
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean dirty;`
    #[inline]
    pub unsafe fn GetDirty(&self, aDirty: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDirty)(self, aDirty)
    }


    /// ```text
    /// /**
    ///    * Read in the preferences specified in a default preference file. This
    ///    * method does not clear preferences that were already set, but it may
    ///    * overwrite existing preferences.
    ///    *
    ///    * @param aFile The file to be read.
    ///    *
    ///    * @throws Error File failed to read or contained invalid data.
    ///    * @note This method is intended for internal unit testing only!
    ///    */
    /// ```
    ///

    /// `void readDefaultPrefsFromFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn ReadDefaultPrefsFromFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).ReadDefaultPrefsFromFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * Like readDefaultPrefsFromFile, but for a user prefs file.
    ///    */
    /// ```
    ///

    /// `void readUserPrefsFromFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn ReadUserPrefsFromFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).ReadUserPrefsFromFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * Usage statistics for performance tests. This function takes a function
    ///    * that is passed (preferenceName, accessCount) as arguments for every
    ///    * recorded preference. You can use this function to build e.g. a JS object
    ///    * holding that data.
    ///    *
    ///    * This is not implemented in non-debug builds and will throw an error.
    ///    */
    /// ```
    ///

    /// `void readStats (in nsIPrefStatsCallback callback);`
    #[inline]
    pub unsafe fn ReadStats(&self, callback: *const nsIPrefStatsCallback) -> ::nserror::nsresult {
        ((*self.vtable).ReadStats)(self, callback)
    }


    /// ```text
    /// /**
    ///    * Reset usage statistics for performance tests.
    ///    *
    ///    * This is not implemented in non-debug builds and will throw an error.
    ///    */
    /// ```
    ///

    /// `void resetStats ();`
    #[inline]
    pub unsafe fn ResetStats(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetStats)(self, )
    }


}


