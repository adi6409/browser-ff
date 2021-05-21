//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIPrefBranch.idl
//


/// `interface nsIPrefBranch : nsISupports`
///

/// ```text
/// /**
///  * The nsIPrefBranch interface is used to manipulate the preferences data. This
///  * object may be obtained from the preferences service (nsIPrefService) and
///  * used to get and set default and/or user preferences across the application.
///  *
///  * This object is created with a "root" value which describes the base point in
///  * the preferences "tree" from which this "branch" stems. Preferences are
///  * accessed off of this root by using just the final portion of the preference.
///  * For example, if this object is created with the root "browser.startup.",
///  * the preferences "browser.startup.page", "browser.startup.homepage",
///  * and "browser.startup.homepage_override" can be accessed by simply passing
///  * "page", "homepage", or "homepage_override" to the various Get/Set methods.
///  *
///  * @see nsIPrefService
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrefBranch {
    vtable: *const nsIPrefBranchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrefBranch.
unsafe impl XpCom for nsIPrefBranch {
    const IID: nsIID = nsID(0x55d25e49, 0x793f, 0x4727,
        [0xa6, 0x9f, 0xde, 0x8b, 0x15, 0xf4, 0xb9, 0x85]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrefBranch {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrefBranch.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrefBranchCoerce {
    /// Cheaply cast a value of this type from a `nsIPrefBranch`.
    fn coerce_from(v: &nsIPrefBranch) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrefBranchCoerce for nsIPrefBranch {
    #[inline]
    fn coerce_from(v: &nsIPrefBranch) -> &Self {
        v
    }
}

impl nsIPrefBranch {
    /// Cast this `nsIPrefBranch` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrefBranchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrefBranch {
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
impl<T: nsISupportsCoerce> nsIPrefBranchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefBranch) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrefBranch
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrefBranchVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString root; */
    pub GetRoot: unsafe extern "system" fn (this: *const nsIPrefBranch, aRoot: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* long getPrefType (in string aPrefName); */
    pub GetPrefType: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut i32) -> ::nserror::nsresult,

    /* [binaryname(GetBoolPrefWithDefault),optional_argc] boolean getBoolPref (in string aPrefName, [optional] in boolean aDefaultValue); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub GetBoolPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetBoolPref),noscript] boolean getBoolPrefXPCOM (in string aPrefName); */
    pub GetBoolPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* void setBoolPref (in string aPrefName, in boolean aValue); */
    pub SetBoolPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: bool) -> ::nserror::nsresult,

    /* [binaryname(GetFloatPrefWithDefault),optional_argc] float getFloatPref (in string aPrefName, [optional] in float aDefaultValue); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub GetFloatPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetFloatPref),noscript] float getFloatPrefXPCOM (in string aPrefName); */
    pub GetFloatPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut libc::c_float) -> ::nserror::nsresult,

    /* [binaryname(GetCharPrefWithDefault),optional_argc] ACString getCharPref (in string aPrefName, [optional] in ACString aDefaultValue); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub GetCharPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetCharPref),noscript] ACString getCharPrefXPCOM (in string aPrefName); */
    pub GetCharPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setCharPref (in string aPrefName, in ACString aValue); */
    pub SetCharPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [optional_argc] AUTF8String getStringPref (in string aPrefName, [optional] in AUTF8String aDefaultValue); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub GetStringPref: *const ::libc::c_void,

    /* void setStringPref (in string aPrefName, in AUTF8String aValue); */
    pub SetStringPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [binaryname(GetIntPrefWithDefault),optional_argc] long getIntPref (in string aPrefName, [optional] in long aDefaultValue); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub GetIntPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetIntPref),noscript] long getIntPrefXPCOM (in string aPrefName); */
    pub GetIntPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut i32) -> ::nserror::nsresult,

    /* void setIntPref (in string aPrefName, in long aValue); */
    pub SetIntPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: i32) -> ::nserror::nsresult,

    /* void getComplexValue (in string aPrefName, in nsIIDRef aType, [iid_is (aType), retval] out nsQIResult aValue); */
    pub GetComplexValue: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void setComplexValue (in string aPrefName, in nsIIDRef aType, in nsISupports aValue); */
    pub SetComplexValue: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: *const nsISupports) -> ::nserror::nsresult,

    /* void clearUserPref (in string aPrefName); */
    pub ClearUserPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char) -> ::nserror::nsresult,

    /* void lockPref (in string aPrefName); */
    pub LockPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char) -> ::nserror::nsresult,

    /* boolean prefHasUserValue (in string aPrefName); */
    pub PrefHasUserValue: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean prefIsLocked (in string aPrefName); */
    pub PrefIsLocked: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* void unlockPref (in string aPrefName); */
    pub UnlockPref: unsafe extern "system" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char) -> ::nserror::nsresult,

    /* void deleteBranch (in string aStartingAt); */
    pub DeleteBranch: unsafe extern "system" fn (this: *const nsIPrefBranch, aStartingAt: *const libc::c_char) -> ::nserror::nsresult,

    /* Array<ACString> getChildList (in string aStartingAt); */
    pub GetChildList: unsafe extern "system" fn (this: *const nsIPrefBranch, aStartingAt: *const libc::c_char, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void resetBranch (in string aStartingAt); */
    pub ResetBranch: unsafe extern "system" fn (this: *const nsIPrefBranch, aStartingAt: *const libc::c_char) -> ::nserror::nsresult,

    /* [binaryname(AddObserverImpl)] void addObserver (in ACString aDomain, in nsIObserver aObserver, [optional] in boolean aHoldWeak); */
    pub AddObserverImpl: unsafe extern "system" fn (this: *const nsIPrefBranch, aDomain: *const ::nsstring::nsACString, aObserver: *const nsIObserver, aHoldWeak: bool) -> ::nserror::nsresult,

    /* [binaryname(RemoveObserverImpl)] void removeObserver (in ACString aDomain, in nsIObserver aObserver); */
    pub RemoveObserverImpl: unsafe extern "system" fn (this: *const nsIPrefBranch, aDomain: *const ::nsstring::nsACString, aObserver: *const nsIObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrefBranch {
    /// ```text
    /// /**
    ///    * Values describing the basic preference types.
    ///    *
    ///    * @see getPrefType
    ///    */
    /// ```
    ///

    pub const PREF_INVALID: i64 = 0;


    pub const PREF_STRING: i64 = 32;


    pub const PREF_INT: i64 = 64;


    pub const PREF_BOOL: i64 = 128;

    /// ```text
    /// /**
    ///    * Called to get the root on which this branch is based, such as
    ///    * "browser.startup."
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString root;`
    #[inline]
    pub unsafe fn GetRoot(&self, aRoot: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRoot)(self, aRoot)
    }


    /// ```text
    /// /**
    ///    * Called to determine the type of a specific preference.
    ///    *
    ///    * @param aPrefName The preference to get the type of.
    ///    *
    ///    * @return long     A value representing the type of the preference. This
    ///    *                  value will be PREF_STRING, PREF_INT, or PREF_BOOL.
    ///    */
    /// ```
    ///

    /// `long getPrefType (in string aPrefName);`
    #[inline]
    pub unsafe fn GetPrefType(&self, aPrefName: *const libc::c_char, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrefType)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to get the state of an individual boolean preference.
    ///    *
    ///    * @param aPrefName The boolean preference to get the state of.
    ///    * @param aDefaultValue The value to return if the preference is not set.
    ///    *
    ///    * @return boolean  The value of the requested boolean preference.
    ///    *
    ///    * @see setBoolPref
    ///    */
    /// ```
    ///

    /// `[binaryname(GetBoolPrefWithDefault),optional_argc] boolean getBoolPref (in string aPrefName, [optional] in boolean aDefaultValue);`
    const _GetBoolPrefWithDefault: () = ();


    /// `[binaryname(GetBoolPref),noscript] boolean getBoolPrefXPCOM (in string aPrefName);`
    #[inline]
    pub unsafe fn GetBoolPref(&self, aPrefName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBoolPref)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to set the state of an individual boolean preference.
    ///    *
    ///    * @param aPrefName The boolean preference to set the state of.
    ///    * @param aValue    The boolean value to set the preference to.
    ///    *
    ///    * @throws Error if setting failed or the preference has a default
    ///              value of a type other than boolean.
    ///    *
    ///    * @see getBoolPref
    ///    */
    /// ```
    ///

    /// `void setBoolPref (in string aPrefName, in boolean aValue);`
    #[inline]
    pub unsafe fn SetBoolPref(&self, aPrefName: *const libc::c_char, aValue: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetBoolPref)(self, aPrefName, aValue)
    }


    /// ```text
    /// /**
    ///    * Called to get the state of an individual floating-point preference.
    ///    * "Floating point" preferences are really string preferences that
    ///    * are converted to floating point numbers.
    ///    *
    ///    * @param aPrefName The floating point preference to get the state of.
    ///    * @param aDefaultValue The value to return if the preference is not set.
    ///    *
    ///    * @return float  The value of the requested floating point preference.
    ///    *
    ///    * @see setCharPref
    ///    */
    /// ```
    ///

    /// `[binaryname(GetFloatPrefWithDefault),optional_argc] float getFloatPref (in string aPrefName, [optional] in float aDefaultValue);`
    const _GetFloatPrefWithDefault: () = ();


    /// `[binaryname(GetFloatPref),noscript] float getFloatPrefXPCOM (in string aPrefName);`
    #[inline]
    pub unsafe fn GetFloatPref(&self, aPrefName: *const libc::c_char, _retval: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetFloatPref)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to get the state of an individual ascii string preference.
    ///    *
    ///    * @param aPrefName The string preference to retrieve.
    ///    * @param aDefaultValue The string to return if the preference is not set.
    ///    *
    ///    * @return ACString   The value of the requested string preference.
    ///    *
    ///    * @see setCharPref
    ///    */
    /// ```
    ///

    /// `[binaryname(GetCharPrefWithDefault),optional_argc] ACString getCharPref (in string aPrefName, [optional] in ACString aDefaultValue);`
    const _GetCharPrefWithDefault: () = ();


    /// `[binaryname(GetCharPref),noscript] ACString getCharPrefXPCOM (in string aPrefName);`
    #[inline]
    pub unsafe fn GetCharPref(&self, aPrefName: *const libc::c_char, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCharPref)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to set the state of an individual ascii string preference.
    ///    *
    ///    * @param aPrefName The string preference to set.
    ///    * @param aValue    The string value to set the preference to.
    ///    *
    ///    * @throws Error if setting failed or the preference has a default
    ///              value of a type other than string.
    ///    *
    ///    * @see getCharPref
    ///    */
    /// ```
    ///

    /// `void setCharPref (in string aPrefName, in ACString aValue);`
    #[inline]
    pub unsafe fn SetCharPref(&self, aPrefName: *const libc::c_char, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCharPref)(self, aPrefName, aValue)
    }


    /// ```text
    /// /**
    ///    * Called to get the state of an individual unicode string preference.
    ///    *
    ///    * @param aPrefName The string preference to retrieve.
    ///    * @param aDefaultValue The string to return if the preference is not set.
    ///    *
    ///    * @return AUTF8String   The value of the requested string preference.
    ///    *
    ///    * @see setStringPref
    ///    */
    /// ```
    ///

    /// `[optional_argc] AUTF8String getStringPref (in string aPrefName, [optional] in AUTF8String aDefaultValue);`
    const _GetStringPref: () = ();

    /// ```text
    /// /**
    ///    * Called to set the state of an individual unicode string preference.
    ///    *
    ///    * @param aPrefName The string preference to set.
    ///    * @param aValue    The string value to set the preference to.
    ///    *
    ///    * @throws Error if setting failed or the preference has a default
    ///              value of a type other than string.
    ///    *
    ///    * @see getStringPref
    ///    */
    /// ```
    ///

    /// `void setStringPref (in string aPrefName, in AUTF8String aValue);`
    #[inline]
    pub unsafe fn SetStringPref(&self, aPrefName: *const libc::c_char, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetStringPref)(self, aPrefName, aValue)
    }


    /// ```text
    /// /**
    ///    * Called to get the state of an individual integer preference.
    ///    *
    ///    * @param aPrefName The integer preference to get the value of.
    ///    * @param aDefaultValue The value to return if the preference is not set.
    ///    *
    ///    * @return long     The value of the requested integer preference.
    ///    *
    ///    * @see setIntPref
    ///    */
    /// ```
    ///

    /// `[binaryname(GetIntPrefWithDefault),optional_argc] long getIntPref (in string aPrefName, [optional] in long aDefaultValue);`
    const _GetIntPrefWithDefault: () = ();


    /// `[binaryname(GetIntPref),noscript] long getIntPrefXPCOM (in string aPrefName);`
    #[inline]
    pub unsafe fn GetIntPref(&self, aPrefName: *const libc::c_char, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetIntPref)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to set the state of an individual integer preference.
    ///    *
    ///    * @param aPrefName The integer preference to set the value of.
    ///    * @param aValue    The integer value to set the preference to.
    ///    *
    ///    * @throws Error if setting failed or the preference has a default
    ///              value of a type other than integer.
    ///    *
    ///    * @see getIntPref
    ///    */
    /// ```
    ///

    /// `void setIntPref (in string aPrefName, in long aValue);`
    #[inline]
    pub unsafe fn SetIntPref(&self, aPrefName: *const libc::c_char, aValue: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetIntPref)(self, aPrefName, aValue)
    }


    /// ```text
    /// /**
    ///    * Called to get the state of an individual complex preference. A complex
    ///    * preference is a preference which represents an XPCOM object that can not
    ///    * be easily represented using a standard boolean, integer or string value.
    ///    *
    ///    * @param aPrefName The complex preference to get the value of.
    ///    * @param aType     The XPCOM interface that this complex preference
    ///    *                  represents. Interfaces currently supported are:
    ///    *                    - nsIFile
    ///    *                    - nsIPrefLocalizedString (Localized UniChar)
    ///    * @param aValue    The XPCOM object into which to the complex preference
    ///    *                  value should be retrieved.
    ///    *
    ///    * @throws Error The value does not exist or is the wrong type.
    ///    *
    ///    * @see setComplexValue
    ///    */
    /// ```
    ///

    /// `void getComplexValue (in string aPrefName, in nsIIDRef aType, [iid_is (aType), retval] out nsQIResult aValue);`
    #[inline]
    pub unsafe fn GetComplexValue(&self, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetComplexValue)(self, aPrefName, aType, aValue)
    }


    /// ```text
    /// /**
    ///    * Called to set the state of an individual complex preference. A complex
    ///    * preference is a preference which represents an XPCOM object that can not
    ///    * be easily represented using a standard boolean, integer or string value.
    ///    *
    ///    * @param aPrefName The complex preference to set the value of.
    ///    * @param aType     The XPCOM interface that this complex preference
    ///    *                  represents. Interfaces currently supported are:
    ///    *                    - nsIFile
    ///    *                    - nsISupportsString (UniChar)
    ///    *                      (deprecated; see setStringPref)
    ///    *                    - nsIPrefLocalizedString (Localized UniChar)
    ///    * @param aValue    The XPCOM object from which to set the complex preference
    ///    *                  value.
    ///    *
    ///    * @throws Error if setting failed or the value is the wrong type.
    ///    *
    ///    * @see getComplexValue
    ///    */
    /// ```
    ///

    /// `void setComplexValue (in string aPrefName, in nsIIDRef aType, in nsISupports aValue);`
    #[inline]
    pub unsafe fn SetComplexValue(&self, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetComplexValue)(self, aPrefName, aType, aValue)
    }


    /// ```text
    /// /**
    ///    * Called to clear a user set value from a specific preference. This will, in
    ///    * effect, reset the value to the default value. If no default value exists
    ///    * the preference will cease to exist.
    ///    *
    ///    * @param aPrefName The preference to be cleared.
    ///    *
    ///    * @note
    ///    * This method does nothing if this object is a default branch.
    ///    */
    /// ```
    ///

    /// `void clearUserPref (in string aPrefName);`
    #[inline]
    pub unsafe fn ClearUserPref(&self, aPrefName: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ClearUserPref)(self, aPrefName)
    }


    /// ```text
    /// /**
    ///    * Called to lock a specific preference. Locking a preference will cause the
    ///    * preference service to always return the default value regardless of
    ///    * whether there is a user set value or not.
    ///    *
    ///    * @param aPrefName The preference to be locked.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on the default branch.
    ///    *
    ///    * @throws Error The preference does not exist or an error occurred.
    ///    *
    ///    * @see unlockPref
    ///    */
    /// ```
    ///

    /// `void lockPref (in string aPrefName);`
    #[inline]
    pub unsafe fn LockPref(&self, aPrefName: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).LockPref)(self, aPrefName)
    }


    /// ```text
    /// /**
    ///    * Called to check if a specific preference has a user value associated to
    ///    * it.
    ///    *
    ///    * @param aPrefName The preference to be tested.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on the user branch.
    ///    *
    ///    * @note
    ///    * If a preference was manually set to a value that equals the default value,
    ///    * then the preference no longer has a user set value, i.e. it is
    ///    * considered reset to its default value.
    ///    * In particular, this method will return false for such a preference and
    ///    * the preference will not be saved to a file by nsIPrefService.savePrefFile.
    ///    *
    ///    * @return boolean  true  The preference has a user set value.
    ///    *                  false The preference only has a default value.
    ///    */
    /// ```
    ///

    /// `boolean prefHasUserValue (in string aPrefName);`
    #[inline]
    pub unsafe fn PrefHasUserValue(&self, aPrefName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PrefHasUserValue)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to check if a specific preference is locked. If a preference is
    ///    * locked calling its Get method will always return the default value.
    ///    *
    ///    * @param aPrefName The preference to be tested.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on the default branch.
    ///    *
    ///    * @return boolean  true  The preference is locked.
    ///    *                  false The preference is not locked.
    ///    *
    ///    * @see lockPref
    ///    * @see unlockPref
    ///    */
    /// ```
    ///

    /// `boolean prefIsLocked (in string aPrefName);`
    #[inline]
    pub unsafe fn PrefIsLocked(&self, aPrefName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PrefIsLocked)(self, aPrefName, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to unlock a specific preference. Unlocking a previously locked
    ///    * preference allows the preference service to once again return the user set
    ///    * value of the preference.
    ///    *
    ///    * @param aPrefName The preference to be unlocked.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on the default branch.
    ///    *
    ///    * @throws Error The preference does not exist or an error occurred.
    ///    *
    ///    * @see lockPref
    ///    */
    /// ```
    ///

    /// `void unlockPref (in string aPrefName);`
    #[inline]
    pub unsafe fn UnlockPref(&self, aPrefName: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).UnlockPref)(self, aPrefName)
    }


    /// ```text
    /// /**
    ///    * Called to remove all of the preferences referenced by this branch.
    ///    *
    ///    * @param aStartingAt The point on the branch at which to start the deleting
    ///    *                    preferences. Pass in "" to remove all preferences
    ///    *                    referenced by this branch.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on both.
    ///    *
    ///    * @throws Error The preference(s) do not exist or an error occurred.
    ///    */
    /// ```
    ///

    /// `void deleteBranch (in string aStartingAt);`
    #[inline]
    pub unsafe fn DeleteBranch(&self, aStartingAt: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).DeleteBranch)(self, aStartingAt)
    }


    /// ```text
    /// /**
    ///    * Returns an array of strings representing the child preferences of the
    ///    * root of this branch.
    ///    *
    ///    * @param aStartingAt The point on the branch at which to start enumerating
    ///    *                    the child preferences. Pass in "" to enumerate all
    ///    *                    preferences referenced by this branch.
    ///    * @return            The array of child preferences.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on both.
    ///    *
    ///    * @throws Error The preference(s) do not exist or an error occurred.
    ///    */
    /// ```
    ///

    /// `Array<ACString> getChildList (in string aStartingAt);`
    #[inline]
    pub unsafe fn GetChildList(&self, aStartingAt: *const libc::c_char, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetChildList)(self, aStartingAt, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to reset all of the preferences referenced by this branch to their
    ///    * default values.
    ///    *
    ///    * @param aStartingAt The point on the branch at which to start the resetting
    ///    *                    preferences to their default values. Pass in "" to
    ///    *                    reset all preferences referenced by this branch.
    ///    *
    ///    * @note
    ///    * This method can be called on either a default or user branch but, in
    ///    * effect, always operates on the user branch.
    ///    *
    ///    * @throws Error The preference(s) do not exist or an error occurred.
    ///    */
    /// ```
    ///

    /// `void resetBranch (in string aStartingAt);`
    #[inline]
    pub unsafe fn ResetBranch(&self, aStartingAt: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).ResetBranch)(self, aStartingAt)
    }


    /// ```text
    /// /**
    ///    * Add a preference change observer. On preference changes, the following
    ///    * arguments will be passed to the nsIObserver.observe() method:
    ///    *   aSubject - The nsIPrefBranch object (this)
    ///    *   aTopic   - The string defined by NS_PREFBRANCH_PREFCHANGE_TOPIC_ID
    ///    *   aData    - The name of the preference which has changed, relative to
    ///    *              the |root| of the aSubject branch.
    ///    *
    ///    * aSubject.get*Pref(aData) will get the new value of the modified
    ///    * preference. For example, if your observer is registered with
    ///    * addObserver("bar.", ...) on a branch with root "foo.", modifying
    ///    * the preference "foo.bar.baz" will trigger the observer, and aData
    ///    * parameter will be "bar.baz".
    ///    *
    ///    * @param aDomain   The preference on which to listen for changes. This can
    ///    *                  be the name of an entire branch to observe.
    ///    *                  e.g. Holding the "root" prefbranch and calling
    ///    *                  addObserver("foo.bar.", ...) will observe changes to
    ///    *                  foo.bar.baz and foo.bar.bzip
    ///    * @param aObserver The object to be notified if the preference changes.
    ///    * @param aHoldWeak true  Hold a weak reference to |aObserver|. The object
    ///    *                        must implement the nsISupportsWeakReference
    ///    *                        interface or this will fail.
    ///    *                  false Hold a strong reference to |aObserver|.
    ///    *
    ///    * @note
    ///    * Registering as a preference observer can open an object to potential
    ///    * cyclical references which will cause memory leaks. These cycles generally
    ///    * occur because an object both registers itself as an observer (causing the
        ///    * branch to hold a reference to the observer) and holds a reference to the
    ///    * branch object for the purpose of getting/setting preference values. There
    ///    * are 3 approaches which have been implemented in an attempt to avoid these
    ///    * situations.
    ///    * 1) The nsPrefBranch object supports nsISupportsWeakReference. Any consumer
///    *    may hold a weak reference to it instead of a strong one.
///    * 2) The nsPrefBranch object listens for xpcom-shutdown and frees all of the
///    *    objects currently in its observer list. This ensures that long lived
///    *    objects (services for example) will be freed correctly.
///    * 3) The observer can request to be held as a weak reference when it is
///    *    registered. This insures that shorter lived objects (say one tied to an
///    *    open window) will not fall into the cyclical reference trap.
///    *
///    * @note
///    * The list of registered observers may be changed during the dispatch of
///    * nsPref:changed notification. However, the observers are not guaranteed
///    * to be notified in any particular order, so you can't be sure whether the
///    * added/removed observer will be called during the notification when it
///    * is added/removed.
///    *
///    * @note
///    * It is possible to change preferences during the notification.
///    *
///    * @note
///    * It is not safe to change observers during this callback in Gecko
///    * releases before 1.9. If you want a safe way to remove a pref observer,
///    * please use an nsITimer.
///    *
///    * @see nsIObserver
///    * @see removeObserver
///    */
/// ```
///

/// `[binaryname(AddObserverImpl)] void addObserver (in ACString aDomain, in nsIObserver aObserver, [optional] in boolean aHoldWeak);`
#[inline]
pub unsafe fn AddObserverImpl(&self, aDomain: *const ::nsstring::nsACString, aObserver: *const nsIObserver, aHoldWeak: bool) -> ::nserror::nsresult {
((*self.vtable).AddObserverImpl)(self, aDomain, aObserver, aHoldWeak)
}


/// ```text
/// /**
///    * Remove a preference change observer.
///    *
///    * @param aDomain   The preference which is being observed for changes.
///    * @param aObserver An observer previously registered with addObserver().
///    *
///    * @note
///    * Note that you must call removeObserver() on the same nsIPrefBranch
///    * instance on which you called addObserver() in order to remove aObserver;
///    * otherwise, the observer will not be removed.
///    *
///    * @see nsIObserver
///    * @see addObserver
///    */
/// ```
///

/// `[binaryname(RemoveObserverImpl)] void removeObserver (in ACString aDomain, in nsIObserver aObserver);`
#[inline]
pub unsafe fn RemoveObserverImpl(&self, aDomain: *const ::nsstring::nsACString, aObserver: *const nsIObserver) -> ::nserror::nsresult {
((*self.vtable).RemoveObserverImpl)(self, aDomain, aObserver)
}


}


