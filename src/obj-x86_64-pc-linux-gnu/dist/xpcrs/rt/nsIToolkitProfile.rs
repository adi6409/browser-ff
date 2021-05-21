//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIToolkitProfile.idl
//


/// `interface nsIProfileLock : nsISupports`
///

/// ```text
/// /**
///  * Hold on to a profile lock. Once you release the last reference to this
///  * interface, the profile lock is released.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProfileLock {
    vtable: *const nsIProfileLockVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProfileLock.
unsafe impl XpCom for nsIProfileLock {
    const IID: nsIID = nsID(0x7c58c703, 0xd245, 0x4864,
        [0x8d, 0x75, 0x96, 0x48, 0xca, 0x4a, 0x61, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProfileLock {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProfileLock.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProfileLockCoerce {
    /// Cheaply cast a value of this type from a `nsIProfileLock`.
    fn coerce_from(v: &nsIProfileLock) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProfileLockCoerce for nsIProfileLock {
    #[inline]
    fn coerce_from(v: &nsIProfileLock) -> &Self {
        v
    }
}

impl nsIProfileLock {
    /// Cast this `nsIProfileLock` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProfileLockCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProfileLock {
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
impl<T: nsISupportsCoerce> nsIProfileLockCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileLock) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProfileLock
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProfileLockVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile directory; */
    pub GetDirectory: unsafe extern "system" fn (this: *const nsIProfileLock, aDirectory: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute nsIFile localDirectory; */
    pub GetLocalDirectory: unsafe extern "system" fn (this: *const nsIProfileLock, aLocalDirectory: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute PRTime replacedLockTime; */
    pub GetReplacedLockTime: unsafe extern "system" fn (this: *const nsIProfileLock, aReplacedLockTime: *mut PRTime) -> ::nserror::nsresult,

    /* void unlock (); */
    pub Unlock: unsafe extern "system" fn (this: *const nsIProfileLock) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProfileLock {

    /// ```text
    /// /**
    ///      * The main profile directory.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile directory;`
    #[inline]
    pub unsafe fn GetDirectory(&self, aDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetDirectory)(self, aDirectory)
    }


    /// ```text
    /// /**
    ///      * A directory corresponding to the main profile directory that exists for
    ///      * the purpose of storing data on the local filesystem, including cache
    ///      * files or other data files that may not represent critical user data.
    ///      * (e.g., this directory may not be included as part of a backup scheme.)
    ///      *
    ///      * In some cases, this directory may just be the main profile directory.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile localDirectory;`
    #[inline]
    pub unsafe fn GetLocalDirectory(&self, aLocalDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetLocalDirectory)(self, aLocalDirectory)
    }


    /// ```text
    /// /**
    ///      * The timestamp of an existing profile lock at lock time.
    ///      */
    /// ```
    ///

    /// `readonly attribute PRTime replacedLockTime;`
    #[inline]
    pub unsafe fn GetReplacedLockTime(&self, aReplacedLockTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetReplacedLockTime)(self, aReplacedLockTime)
    }


    /// ```text
    /// /**
    ///      * Unlock the profile.
    ///      */
    /// ```
    ///

    /// `void unlock ();`
    #[inline]
    pub unsafe fn Unlock(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Unlock)(self, )
    }


}


/// `interface nsIToolkitProfile : nsISupports`
///

/// ```text
/// /**
///  * A interface representing a profile.
///  * @note THIS INTERFACE SHOULD BE IMPLEMENTED BY THE TOOLKIT CODE ONLY! DON'T
///  *       EVEN THINK ABOUT IMPLEMENTING THIS IN JAVASCRIPT!
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIToolkitProfile {
    vtable: *const nsIToolkitProfileVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIToolkitProfile.
unsafe impl XpCom for nsIToolkitProfile {
    const IID: nsIID = nsID(0x7422b090, 0x4a86, 0x4407,
        [0x97, 0x2e, 0x75, 0x46, 0x8a, 0x62, 0x53, 0x88]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIToolkitProfile {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIToolkitProfile.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIToolkitProfileCoerce {
    /// Cheaply cast a value of this type from a `nsIToolkitProfile`.
    fn coerce_from(v: &nsIToolkitProfile) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIToolkitProfileCoerce for nsIToolkitProfile {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfile) -> &Self {
        v
    }
}

impl nsIToolkitProfile {
    /// Cast this `nsIToolkitProfile` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIToolkitProfileCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIToolkitProfile {
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
impl<T: nsISupportsCoerce> nsIToolkitProfileCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfile) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIToolkitProfile
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIToolkitProfileVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile rootDir; */
    pub GetRootDir: unsafe extern "system" fn (this: *const nsIToolkitProfile, aRootDir: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute nsIFile localDir; */
    pub GetLocalDir: unsafe extern "system" fn (this: *const nsIToolkitProfile, aLocalDir: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute AUTF8String name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIToolkitProfile, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String name; */
    pub SetName: unsafe extern "system" fn (this: *const nsIToolkitProfile, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void remove (in boolean removeFiles); */
    pub Remove: unsafe extern "system" fn (this: *const nsIToolkitProfile, removeFiles: bool) -> ::nserror::nsresult,

    /* void removeInBackground (in boolean removeFiles); */
    pub RemoveInBackground: unsafe extern "system" fn (this: *const nsIToolkitProfile, removeFiles: bool) -> ::nserror::nsresult,

    /* nsIProfileLock lock (out nsIProfileUnlocker aUnlocker); */
    pub Lock: unsafe extern "system" fn (this: *const nsIToolkitProfile, aUnlocker: *mut*const nsIProfileUnlocker, _retval: *mut *const nsIProfileLock) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIToolkitProfile {

    /// ```text
    /// /**
    ///      * The location of the profile directory.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile rootDir;`
    #[inline]
    pub unsafe fn GetRootDir(&self, aRootDir: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetRootDir)(self, aRootDir)
    }


    /// ```text
    /// /**
    ///      * The location of the profile local directory, which may be the same as
    ///      * the root directory.  See nsIProfileLock::localDirectory.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile localDir;`
    #[inline]
    pub unsafe fn GetLocalDir(&self, aLocalDir: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetLocalDir)(self, aLocalDir)
    }


    /// ```text
    /// /**
    ///      * The name of the profile.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * The name of the profile.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String name;`
    #[inline]
    pub unsafe fn SetName(&self, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * Removes the profile from the registry of profiles.
    ///      *
    ///      * @param removeFiles
    ///      *        Indicates whether or not the profile directory should be
    ///      *        removed in addition.
    ///      */
    /// ```
    ///

    /// `void remove (in boolean removeFiles);`
    #[inline]
    pub unsafe fn Remove(&self, removeFiles: bool) -> ::nserror::nsresult {
        ((*self.vtable).Remove)(self, removeFiles)
    }


    /// ```text
    /// /**
    ///      * Removes the profile from the registry of profiles.
    ///      * The profile directory is removed in the stream transport thread.
    ///      *
    ///      * @param removeFiles
    ///      *        Indicates whether or not the profile directory should be
    ///      *        removed in addition.
    ///      */
    /// ```
    ///

    /// `void removeInBackground (in boolean removeFiles);`
    #[inline]
    pub unsafe fn RemoveInBackground(&self, removeFiles: bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoveInBackground)(self, removeFiles)
    }


    /// ```text
    /// /**
    ///      * Lock this profile using platform-specific locking methods.
    ///      *
    ///      * @param lockFile If locking fails, this may return a lockFile object
    ///      *                 which can be used in platform-specific ways to
    ///      *                 determine which process has the file locked. Null
    ///      *                 may be passed.
    ///      * @return An interface which holds a profile lock as long as you reference
    ///      *         it.
    ///      * @throws NS_ERROR_FILE_ACCESS_DENIED if the profile was already locked.
    ///      */
    /// ```
    ///

    /// `nsIProfileLock lock (out nsIProfileUnlocker aUnlocker);`
    #[inline]
    pub unsafe fn Lock(&self, aUnlocker: *mut*const nsIProfileUnlocker, _retval: *mut *const nsIProfileLock) -> ::nserror::nsresult {
        ((*self.vtable).Lock)(self, aUnlocker, _retval)
    }


}


