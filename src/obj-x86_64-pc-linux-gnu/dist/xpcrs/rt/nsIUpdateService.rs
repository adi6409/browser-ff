//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/update/nsIUpdateService.idl
//


/// `interface nsIUpdatePatch : nsISupports`
///

/// ```text
/// /**
///  * An interface that describes an object representing a patch file that can
///  * be downloaded and applied to a version of this application so that it
///  * can be updated.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdatePatch {
    vtable: *const nsIUpdatePatchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdatePatch.
unsafe impl XpCom for nsIUpdatePatch {
    const IID: nsIID = nsID(0xdc8fb8a9, 0x3a53, 0x4031,
        [0x94, 0x69, 0x2a, 0x51, 0x97, 0xea, 0x30, 0xe7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdatePatch {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdatePatch.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdatePatchCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdatePatch`.
    fn coerce_from(v: &nsIUpdatePatch) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdatePatchCoerce for nsIUpdatePatch {
    #[inline]
    fn coerce_from(v: &nsIUpdatePatch) -> &Self {
        v
    }
}

impl nsIUpdatePatch {
    /// Cast this `nsIUpdatePatch` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdatePatchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdatePatch {
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
impl<T: nsISupportsCoerce> nsIUpdatePatchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdatePatch) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdatePatch
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdatePatchVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIUpdatePatch, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString URL; */
    pub GetURL: unsafe extern "system" fn (this: *const nsIUpdatePatch, aURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString finalURL; */
    pub GetFinalURL: unsafe extern "system" fn (this: *const nsIUpdatePatch, aFinalURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString finalURL; */
    pub SetFinalURL: unsafe extern "system" fn (this: *const nsIUpdatePatch, aFinalURL: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long size; */
    pub GetSize: unsafe extern "system" fn (this: *const nsIUpdatePatch, aSize: *mut u32) -> ::nserror::nsresult,

    /* attribute AString state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIUpdatePatch, aState: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString state; */
    pub SetState: unsafe extern "system" fn (this: *const nsIUpdatePatch, aState: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute long errorCode; */
    pub GetErrorCode: unsafe extern "system" fn (this: *const nsIUpdatePatch, aErrorCode: *mut i32) -> ::nserror::nsresult,

    /* attribute long errorCode; */
    pub SetErrorCode: unsafe extern "system" fn (this: *const nsIUpdatePatch, aErrorCode: i32) -> ::nserror::nsresult,

    /* attribute boolean selected; */
    pub GetSelected: unsafe extern "system" fn (this: *const nsIUpdatePatch, aSelected: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean selected; */
    pub SetSelected: unsafe extern "system" fn (this: *const nsIUpdatePatch, aSelected: bool) -> ::nserror::nsresult,

    /* Element serialize (in Document updates); */
    pub Serialize: unsafe extern "system" fn (this: *const nsIUpdatePatch, updates: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdatePatch {

    /// ```text
    /// /**
    ///    * The type of this patch:
    ///    * "partial"      A binary difference between two application versions
    ///    * "complete"     A complete patch containing all of the replacement files
    ///    *                to update to the new version
    ///    */
    /// ```
    ///

    /// `readonly attribute AString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    * The URL this patch was being downloaded from
    ///    */
    /// ```
    ///

    /// `readonly attribute AString URL;`
    #[inline]
    pub unsafe fn GetURL(&self, aURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetURL)(self, aURL)
    }


    /// ```text
    /// /**
    ///    * The final URL this patch was being downloaded from
    ///    */
    /// ```
    ///

    /// `attribute AString finalURL;`
    #[inline]
    pub unsafe fn GetFinalURL(&self, aFinalURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFinalURL)(self, aFinalURL)
    }


    /// ```text
    /// /**
    ///    * The final URL this patch was being downloaded from
    ///    */
    /// ```
    ///

    /// `attribute AString finalURL;`
    #[inline]
    pub unsafe fn SetFinalURL(&self, aFinalURL: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFinalURL)(self, aFinalURL)
    }


    /// ```text
    /// /**
    ///    * The size of this file, in bytes.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long size;`
    #[inline]
    pub unsafe fn GetSize(&self, aSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSize)(self, aSize)
    }


    /// ```text
    /// /**
    ///    * The state of this patch
    ///    */
    /// ```
    ///

    /// `attribute AString state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * The state of this patch
    ///    */
    /// ```
    ///

    /// `attribute AString state;`
    #[inline]
    pub unsafe fn SetState(&self, aState: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * A numeric error code that conveys additional information about the state of
    ///    * a failed update. If the update is not in the "failed" state the value is
    ///    * zero. The possible values are located in common/updatererrors.h and values between
    ///    * 80 and 99 are in nsUpdateService.js.
    ///    */
    /// ```
    ///

    /// `attribute long errorCode;`
    #[inline]
    pub unsafe fn GetErrorCode(&self, aErrorCode: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorCode)(self, aErrorCode)
    }


    /// ```text
    /// /**
    ///    * A numeric error code that conveys additional information about the state of
    ///    * a failed update. If the update is not in the "failed" state the value is
    ///    * zero. The possible values are located in common/updatererrors.h and values between
    ///    * 80 and 99 are in nsUpdateService.js.
    ///    */
    /// ```
    ///

    /// `attribute long errorCode;`
    #[inline]
    pub unsafe fn SetErrorCode(&self, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetErrorCode)(self, aErrorCode)
    }


    /// ```text
    /// /**
    ///    * true if this patch is currently selected as the patch to be downloaded and
    ///    * installed for this update transaction, false if another patch from this
    ///    * update has been selected.
    ///    */
    /// ```
    ///

    /// `attribute boolean selected;`
    #[inline]
    pub unsafe fn GetSelected(&self, aSelected: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSelected)(self, aSelected)
    }


    /// ```text
    /// /**
    ///    * true if this patch is currently selected as the patch to be downloaded and
    ///    * installed for this update transaction, false if another patch from this
    ///    * update has been selected.
    ///    */
    /// ```
    ///

    /// `attribute boolean selected;`
    #[inline]
    pub unsafe fn SetSelected(&self, aSelected: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSelected)(self, aSelected)
    }


    /// ```text
    /// /**
    ///    * Serializes this patch object into a DOM Element
    ///    * @param   updates
    ///    *          The document to serialize into
    ///    * @returns The DOM Element created by the serialization process
    ///    */
    /// ```
    ///

    /// `Element serialize (in Document updates);`
    #[inline]
    pub unsafe fn Serialize(&self, updates: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).Serialize)(self, updates, _retval)
    }


}


/// `interface nsIUpdate : nsISupports`
///

/// ```text
/// /**
///  * An interface that describes an object representing an available update to
///  * the current application - this update may have several available patches
///  * from which one must be selected to download and install, for example we
///  * might select a binary difference patch first and attempt to apply that,
///  * then if the application process fails fall back to downloading a complete
///  * file-replace patch. This object also contains information about the update
///  * that the front end and other application services can use to learn more
///  * about what is going on.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdate {
    vtable: *const nsIUpdateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdate.
unsafe impl XpCom for nsIUpdate {
    const IID: nsIID = nsID(0xe094c045, 0xf4ff, 0x41fd,
        [0x92, 0xda, 0xcd, 0x2e, 0xff, 0xd2, 0xc7, 0xc9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdate {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdate.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdate`.
    fn coerce_from(v: &nsIUpdate) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateCoerce for nsIUpdate {
    #[inline]
    fn coerce_from(v: &nsIUpdate) -> &Self {
        v
    }
}

impl nsIUpdate {
    /// Cast this `nsIUpdate` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdate {
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
impl<T: nsISupportsCoerce> nsIUpdateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdate) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdate
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIUpdate, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIUpdate, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString displayVersion; */
    pub GetDisplayVersion: unsafe extern "system" fn (this: *const nsIUpdate, aDisplayVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString appVersion; */
    pub GetAppVersion: unsafe extern "system" fn (this: *const nsIUpdate, aAppVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString previousAppVersion; */
    pub GetPreviousAppVersion: unsafe extern "system" fn (this: *const nsIUpdate, aPreviousAppVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString buildID; */
    pub GetBuildID: unsafe extern "system" fn (this: *const nsIUpdate, aBuildID: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString detailsURL; */
    pub GetDetailsURL: unsafe extern "system" fn (this: *const nsIUpdate, aDetailsURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString serviceURL; */
    pub GetServiceURL: unsafe extern "system" fn (this: *const nsIUpdate, aServiceURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString channel; */
    pub GetChannel: unsafe extern "system" fn (this: *const nsIUpdate, aChannel: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean unsupported; */
    pub GetUnsupported: unsafe extern "system" fn (this: *const nsIUpdate, aUnsupported: *mut bool) -> ::nserror::nsresult,

    /* attribute long long promptWaitTime; */
    pub GetPromptWaitTime: unsafe extern "system" fn (this: *const nsIUpdate, aPromptWaitTime: *mut i64) -> ::nserror::nsresult,

    /* attribute long long promptWaitTime; */
    pub SetPromptWaitTime: unsafe extern "system" fn (this: *const nsIUpdate, aPromptWaitTime: i64) -> ::nserror::nsresult,

    /* attribute boolean isCompleteUpdate; */
    pub GetIsCompleteUpdate: unsafe extern "system" fn (this: *const nsIUpdate, aIsCompleteUpdate: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isCompleteUpdate; */
    pub SetIsCompleteUpdate: unsafe extern "system" fn (this: *const nsIUpdate, aIsCompleteUpdate: bool) -> ::nserror::nsresult,

    /* attribute long long installDate; */
    pub GetInstallDate: unsafe extern "system" fn (this: *const nsIUpdate, aInstallDate: *mut i64) -> ::nserror::nsresult,

    /* attribute long long installDate; */
    pub SetInstallDate: unsafe extern "system" fn (this: *const nsIUpdate, aInstallDate: i64) -> ::nserror::nsresult,

    /* attribute AString statusText; */
    pub GetStatusText: unsafe extern "system" fn (this: *const nsIUpdate, aStatusText: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString statusText; */
    pub SetStatusText: unsafe extern "system" fn (this: *const nsIUpdate, aStatusText: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIUpdatePatch selectedPatch; */
    pub GetSelectedPatch: unsafe extern "system" fn (this: *const nsIUpdate, aSelectedPatch: *mut *const nsIUpdatePatch) -> ::nserror::nsresult,

    /* attribute AString state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIUpdate, aState: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString state; */
    pub SetState: unsafe extern "system" fn (this: *const nsIUpdate, aState: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute long errorCode; */
    pub GetErrorCode: unsafe extern "system" fn (this: *const nsIUpdate, aErrorCode: *mut i32) -> ::nserror::nsresult,

    /* attribute long errorCode; */
    pub SetErrorCode: unsafe extern "system" fn (this: *const nsIUpdate, aErrorCode: i32) -> ::nserror::nsresult,

    /* attribute boolean elevationFailure; */
    pub GetElevationFailure: unsafe extern "system" fn (this: *const nsIUpdate, aElevationFailure: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean elevationFailure; */
    pub SetElevationFailure: unsafe extern "system" fn (this: *const nsIUpdate, aElevationFailure: bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long patchCount; */
    pub GetPatchCount: unsafe extern "system" fn (this: *const nsIUpdate, aPatchCount: *mut u32) -> ::nserror::nsresult,

    /* nsIUpdatePatch getPatchAt (in unsigned long index); */
    pub GetPatchAt: unsafe extern "system" fn (this: *const nsIUpdate, index: u32, _retval: *mut *const nsIUpdatePatch) -> ::nserror::nsresult,

    /* Element serialize (in Document updates); */
    pub Serialize: unsafe extern "system" fn (this: *const nsIUpdate, updates: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdate {

    /// ```text
    /// /**
    ///    * The type of update:
    ///    *   "major"  A major new version of the Application
    ///    *   "minor"  A minor update to the Application (e.g. security update)
    ///    */
    /// ```
    ///

    /// `readonly attribute AString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    * The name of the update, or "<Application Name> <Update Version>"
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * The string to display in the user interface for the version. If you want
    ///    * a real version number use appVersion.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString displayVersion;`
    #[inline]
    pub unsafe fn GetDisplayVersion(&self, aDisplayVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayVersion)(self, aDisplayVersion)
    }


    /// ```text
    /// /**
    ///    * The Application version of this update.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString appVersion;`
    #[inline]
    pub unsafe fn GetAppVersion(&self, aAppVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppVersion)(self, aAppVersion)
    }


    /// ```text
    /// /**
    ///    * The Application version prior to the application being updated.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString previousAppVersion;`
    #[inline]
    pub unsafe fn GetPreviousAppVersion(&self, aPreviousAppVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPreviousAppVersion)(self, aPreviousAppVersion)
    }


    /// ```text
    /// /**
    ///    * The Build ID of this update. Used to determine a particular build, down
    ///    * to the hour, minute and second of its creation. This allows the system
    ///    * to differentiate between several nightly builds with the same |version|
    ///    * for example.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString buildID;`
    #[inline]
    pub unsafe fn GetBuildID(&self, aBuildID: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetBuildID)(self, aBuildID)
    }


    /// ```text
    /// /**
    ///    * The URL to a page which offers details about the content of this
    ///    * update. Ideally, this page is not the release notes but some other page
    ///    * that summarizes the differences between this update and the previous,
    ///    * which also links to the release notes.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString detailsURL;`
    #[inline]
    pub unsafe fn GetDetailsURL(&self, aDetailsURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDetailsURL)(self, aDetailsURL)
    }


    /// ```text
    /// /**
    ///    * The URL to the Update Service that supplied this update.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString serviceURL;`
    #[inline]
    pub unsafe fn GetServiceURL(&self, aServiceURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetServiceURL)(self, aServiceURL)
    }


    /// ```text
    /// /**
    ///    * The channel used to retrieve this update from the Update Service.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString channel;`
    #[inline]
    pub unsafe fn GetChannel(&self, aChannel: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetChannel)(self, aChannel)
    }


    /// ```text
    /// /**
    ///    * Whether the update is no longer supported on this system.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean unsupported;`
    #[inline]
    pub unsafe fn GetUnsupported(&self, aUnsupported: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUnsupported)(self, aUnsupported)
    }


    /// ```text
    /// /**
    ///    * Allows overriding the default amount of time in seconds before prompting the
    ///    * user to apply an update. If not specified, the value of
    ///    * app.update.promptWaitTime will be used.
    ///    */
    /// ```
    ///

    /// `attribute long long promptWaitTime;`
    #[inline]
    pub unsafe fn GetPromptWaitTime(&self, aPromptWaitTime: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetPromptWaitTime)(self, aPromptWaitTime)
    }


    /// ```text
    /// /**
    ///    * Allows overriding the default amount of time in seconds before prompting the
    ///    * user to apply an update. If not specified, the value of
    ///    * app.update.promptWaitTime will be used.
    ///    */
    /// ```
    ///

    /// `attribute long long promptWaitTime;`
    #[inline]
    pub unsafe fn SetPromptWaitTime(&self, aPromptWaitTime: i64) -> ::nserror::nsresult {
        ((*self.vtable).SetPromptWaitTime)(self, aPromptWaitTime)
    }


    /// ```text
    /// /**
    ///    * Whether or not the update being downloaded is a complete replacement of
    ///    * the user's existing installation or a patch representing the difference
    ///    * between the new version and the previous version.
    ///    */
    /// ```
    ///

    /// `attribute boolean isCompleteUpdate;`
    #[inline]
    pub unsafe fn GetIsCompleteUpdate(&self, aIsCompleteUpdate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsCompleteUpdate)(self, aIsCompleteUpdate)
    }


    /// ```text
    /// /**
    ///    * Whether or not the update being downloaded is a complete replacement of
    ///    * the user's existing installation or a patch representing the difference
    ///    * between the new version and the previous version.
    ///    */
    /// ```
    ///

    /// `attribute boolean isCompleteUpdate;`
    #[inline]
    pub unsafe fn SetIsCompleteUpdate(&self, aIsCompleteUpdate: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsCompleteUpdate)(self, aIsCompleteUpdate)
    }


    /// ```text
    /// /**
    ///    * When the update was installed.
    ///    */
    /// ```
    ///

    /// `attribute long long installDate;`
    #[inline]
    pub unsafe fn GetInstallDate(&self, aInstallDate: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetInstallDate)(self, aInstallDate)
    }


    /// ```text
    /// /**
    ///    * When the update was installed.
    ///    */
    /// ```
    ///

    /// `attribute long long installDate;`
    #[inline]
    pub unsafe fn SetInstallDate(&self, aInstallDate: i64) -> ::nserror::nsresult {
        ((*self.vtable).SetInstallDate)(self, aInstallDate)
    }


    /// ```text
    /// /**
    ///    * A message associated with this update, if any.
    ///    */
    /// ```
    ///

    /// `attribute AString statusText;`
    #[inline]
    pub unsafe fn GetStatusText(&self, aStatusText: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStatusText)(self, aStatusText)
    }


    /// ```text
    /// /**
    ///    * A message associated with this update, if any.
    ///    */
    /// ```
    ///

    /// `attribute AString statusText;`
    #[inline]
    pub unsafe fn SetStatusText(&self, aStatusText: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetStatusText)(self, aStatusText)
    }


    /// ```text
    /// /**
    ///    * The currently selected patch for this update.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIUpdatePatch selectedPatch;`
    #[inline]
    pub unsafe fn GetSelectedPatch(&self, aSelectedPatch: *mut *const nsIUpdatePatch) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedPatch)(self, aSelectedPatch)
    }


    /// ```text
    /// /**
    ///    * The state of the selected patch:
    ///    *   "downloading"        The update is being downloaded.
    ///    *   "pending"            The update is ready to be applied.
    ///    *   "pending-service"    The update is ready to be applied with the service.
    ///    *   "pending-elevate"    The update is ready to be applied but requires elevation.
    ///    *   "applying"           The update is being applied.
    ///    *   "applied"            The update is ready to be switched to.
    ///    *   "applied-os"         The update is OS update and to be installed.
    ///    *   "applied-service"    The update is ready to be switched to with the service.
    ///    *   "succeeded"          The update was successfully applied.
    ///    *   "download-failed"    The update failed to be downloaded.
    ///    *   "failed"             The update failed to be applied.
    ///    */
    /// ```
    ///

    /// `attribute AString state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * The state of the selected patch:
    ///    *   "downloading"        The update is being downloaded.
    ///    *   "pending"            The update is ready to be applied.
    ///    *   "pending-service"    The update is ready to be applied with the service.
    ///    *   "pending-elevate"    The update is ready to be applied but requires elevation.
    ///    *   "applying"           The update is being applied.
    ///    *   "applied"            The update is ready to be switched to.
    ///    *   "applied-os"         The update is OS update and to be installed.
    ///    *   "applied-service"    The update is ready to be switched to with the service.
    ///    *   "succeeded"          The update was successfully applied.
    ///    *   "download-failed"    The update failed to be downloaded.
    ///    *   "failed"             The update failed to be applied.
    ///    */
    /// ```
    ///

    /// `attribute AString state;`
    #[inline]
    pub unsafe fn SetState(&self, aState: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * A numeric error code that conveys additional information about the state of
    ///    * a failed update. If the update is not in the "failed" state the value is
    ///    * zero. The possible values are located in common/updatererrors.h and values between
    ///    * 80 and 99 are in nsUpdateService.js.
    ///    */
    /// ```
    ///

    /// `attribute long errorCode;`
    #[inline]
    pub unsafe fn GetErrorCode(&self, aErrorCode: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorCode)(self, aErrorCode)
    }


    /// ```text
    /// /**
    ///    * A numeric error code that conveys additional information about the state of
    ///    * a failed update. If the update is not in the "failed" state the value is
    ///    * zero. The possible values are located in common/updatererrors.h and values between
    ///    * 80 and 99 are in nsUpdateService.js.
    ///    */
    /// ```
    ///

    /// `attribute long errorCode;`
    #[inline]
    pub unsafe fn SetErrorCode(&self, aErrorCode: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetErrorCode)(self, aErrorCode)
    }


    /// ```text
    /// /**
    ///    * Whether an elevation failure has been encountered for this update.
    ///    */
    /// ```
    ///

    /// `attribute boolean elevationFailure;`
    #[inline]
    pub unsafe fn GetElevationFailure(&self, aElevationFailure: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetElevationFailure)(self, aElevationFailure)
    }


    /// ```text
    /// /**
    ///    * Whether an elevation failure has been encountered for this update.
    ///    */
    /// ```
    ///

    /// `attribute boolean elevationFailure;`
    #[inline]
    pub unsafe fn SetElevationFailure(&self, aElevationFailure: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetElevationFailure)(self, aElevationFailure)
    }


    /// ```text
    /// /**
    ///    * The number of patches supplied by this update.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long patchCount;`
    #[inline]
    pub unsafe fn GetPatchCount(&self, aPatchCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPatchCount)(self, aPatchCount)
    }


    /// ```text
    /// /**
    ///    * Retrieves a patch.
    ///    * @param   index
    ///    *          The index of the patch to retrieve.
    ///    * @returns The nsIUpdatePatch at the specified index.
    ///    */
    /// ```
    ///

    /// `nsIUpdatePatch getPatchAt (in unsigned long index);`
    #[inline]
    pub unsafe fn GetPatchAt(&self, index: u32, _retval: *mut *const nsIUpdatePatch) -> ::nserror::nsresult {
        ((*self.vtable).GetPatchAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Serializes this update object into a DOM Element
    ///    * @param   updates
    ///    *          The document to serialize into
    ///    * @returns The DOM Element created by the serialization process
    ///    */
    /// ```
    ///

    /// `Element serialize (in Document updates);`
    #[inline]
    pub unsafe fn Serialize(&self, updates: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).Serialize)(self, updates, _retval)
    }


}


/// `interface nsIUpdateCheckListener : nsISupports`
///

/// ```text
/// /**
///  * An interface describing an object that listens to the progress of an update
///  * check operation. This object is notified as the check continues, finishes
///  * and if it has an error.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdateCheckListener {
    vtable: *const nsIUpdateCheckListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdateCheckListener.
unsafe impl XpCom for nsIUpdateCheckListener {
    const IID: nsIID = nsID(0x4aa2b4bb, 0x39ea, 0x407b,
        [0x98, 0xff, 0x89, 0xf1, 0x91, 0x34, 0xd4, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdateCheckListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdateCheckListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateCheckListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdateCheckListener`.
    fn coerce_from(v: &nsIUpdateCheckListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateCheckListenerCoerce for nsIUpdateCheckListener {
    #[inline]
    fn coerce_from(v: &nsIUpdateCheckListener) -> &Self {
        v
    }
}

impl nsIUpdateCheckListener {
    /// Cast this `nsIUpdateCheckListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateCheckListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdateCheckListener {
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
impl<T: nsISupportsCoerce> nsIUpdateCheckListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateCheckListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdateCheckListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateCheckListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onCheckComplete (in jsval request, in Array<nsIUpdate> updates); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub OnCheckComplete: *const ::libc::c_void,

    /* void onError (in jsval request, in nsIUpdate update); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub OnError: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdateCheckListener {

    /// ```text
    /// /**
    ///    * The update check was completed.
    ///    * @param   request
    ///    *          The XMLHttpRequest handling the update check.
    ///    * @param   updates
    ///    *          An array of nsIUpdate objects listing available updates.
    ///    */
    /// ```
    ///

    /// `void onCheckComplete (in jsval request, in Array<nsIUpdate> updates);`
    const _OnCheckComplete: () = ();

    /// ```text
    /// /**
    ///    * An error occurred while loading the remote update service file.
    ///    * @param   request
    ///    *          The XMLHttpRequest handling the update check.
    ///    * @param   update
    ///    *          A nsIUpdate object that contains details about the
    ///    *          error in its |statusText| property.
    ///    */
    /// ```
    ///

    /// `void onError (in jsval request, in nsIUpdate update);`
    const _OnError: () = ();

}


/// `interface nsIUpdateChecker : nsISupports`
///

/// ```text
/// /**
///  * An interface describing an object that knows how to check for updates.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdateChecker {
    vtable: *const nsIUpdateCheckerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdateChecker.
unsafe impl XpCom for nsIUpdateChecker {
    const IID: nsIID = nsID(0x877ace25, 0x8bc5, 0x452a,
        [0x85, 0x86, 0x9c, 0x1c, 0xf2, 0x87, 0x19, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdateChecker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdateChecker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateCheckerCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdateChecker`.
    fn coerce_from(v: &nsIUpdateChecker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateCheckerCoerce for nsIUpdateChecker {
    #[inline]
    fn coerce_from(v: &nsIUpdateChecker) -> &Self {
        v
    }
}

impl nsIUpdateChecker {
    /// Cast this `nsIUpdateChecker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateCheckerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdateChecker {
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
impl<T: nsISupportsCoerce> nsIUpdateCheckerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateChecker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdateChecker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateCheckerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force); */
    pub CheckForUpdates: unsafe extern "system" fn (this: *const nsIUpdateChecker, listener: *const nsIUpdateCheckListener, force: bool) -> ::nserror::nsresult,

    /* void stopCurrentCheck (); */
    pub StopCurrentCheck: unsafe extern "system" fn (this: *const nsIUpdateChecker) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdateChecker {

    /// ```text
    /// /**
    ///    * Checks for available updates, notifying a listener of the results.
    ///    * @param   listener
    ///    *          An object implementing nsIUpdateCheckListener which is notified
    ///    *          of the results of an update check.
    ///    * @param   force
    ///    *          Forces the checker to check for updates, regardless of the
    ///    *          current value of the user's update settings. This is used by
    ///    *          any piece of UI that offers the user the imperative option to
    ///    *          check for updates now, regardless of their update settings.
    ///    *          However, if updates are disabled by policy, setting force to true
    ///    *          will not override the the policy.
    ///    */
    /// ```
    ///

    /// `void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force);`
    #[inline]
    pub unsafe fn CheckForUpdates(&self, listener: *const nsIUpdateCheckListener, force: bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckForUpdates)(self, listener, force)
    }


    /// ```text
    /// /**
    ///    * Ends any pending update check.
    ///    */
    /// ```
    ///

    /// `void stopCurrentCheck ();`
    #[inline]
    pub unsafe fn StopCurrentCheck(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopCurrentCheck)(self, )
    }


}


/// `interface nsIApplicationUpdateService : nsISupports`
///

/// ```text
/// /**
///  * An interface describing a global application service that handles performing
///  * background update checks and provides utilities for selecting and
///  * downloading update patches.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationUpdateService {
    vtable: *const nsIApplicationUpdateServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationUpdateService.
unsafe impl XpCom for nsIApplicationUpdateService {
    const IID: nsIID = nsID(0x1107d207, 0xa263, 0x403a,
        [0xb2, 0x68, 0x05, 0x77, 0x2e, 0xc1, 0x07, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationUpdateService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationUpdateService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationUpdateServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationUpdateService`.
    fn coerce_from(v: &nsIApplicationUpdateService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationUpdateServiceCoerce for nsIApplicationUpdateService {
    #[inline]
    fn coerce_from(v: &nsIApplicationUpdateService) -> &Self {
        v
    }
}

impl nsIApplicationUpdateService {
    /// Cast this `nsIApplicationUpdateService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationUpdateServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationUpdateService {
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
impl<T: nsISupportsCoerce> nsIApplicationUpdateServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationUpdateService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationUpdateService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationUpdateServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool checkForBackgroundUpdates (); */
    pub CheckForBackgroundUpdates: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIUpdateChecker backgroundChecker; */
    pub GetBackgroundChecker: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aBackgroundChecker: *mut *const nsIUpdateChecker) -> ::nserror::nsresult,

    /* nsIUpdate selectUpdate (in Array<nsIUpdate> updates); */
    pub SelectUpdate: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, updates: *const thin_vec::ThinVec<RefPtr<nsIUpdate>>, _retval: *mut *const nsIUpdate) -> ::nserror::nsresult,

    /* void addDownloadListener (in nsIRequestObserver listener); */
    pub AddDownloadListener: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, listener: *const nsIRequestObserver) -> ::nserror::nsresult,

    /* void removeDownloadListener (in nsIRequestObserver listener); */
    pub RemoveDownloadListener: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, listener: *const nsIRequestObserver) -> ::nserror::nsresult,

    /* bool downloadUpdate (in nsIUpdate update, in boolean background); */
    pub DownloadUpdate: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, update: *const nsIUpdate, background: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* void stopDownload (); */
    pub StopDownload: unsafe extern "system" fn (this: *const nsIApplicationUpdateService) -> ::nserror::nsresult,

    /* readonly attribute boolean isDownloading; */
    pub GetIsDownloading: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aIsDownloading: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean canCheckForUpdates; */
    pub GetCanCheckForUpdates: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aCanCheckForUpdates: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean elevationRequired; */
    pub GetElevationRequired: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aElevationRequired: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean canApplyUpdates; */
    pub GetCanApplyUpdates: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aCanApplyUpdates: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isOtherInstanceHandlingUpdates; */
    pub GetIsOtherInstanceHandlingUpdates: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aIsOtherInstanceHandlingUpdates: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean canStageUpdates; */
    pub GetCanStageUpdates: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aCanStageUpdates: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean manualUpdateOnly; */
    pub GetManualUpdateOnly: unsafe extern "system" fn (this: *const nsIApplicationUpdateService, aManualUpdateOnly: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationUpdateService {

    /// ```text
    /// /**
    ///    * Checks for available updates in the background using the listener provided
    ///    * by the application update service for background checks.
    ///    * @returns true if the update check was started, false if not. Note that the
    ///    *          check starting does not necessarily mean that the check will
    ///    *          succeed or that an update will be downloaded.
    ///    */
    /// ```
    ///

    /// `bool checkForBackgroundUpdates ();`
    #[inline]
    pub unsafe fn CheckForBackgroundUpdates(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckForBackgroundUpdates)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * The Update Checker used for background update checking.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIUpdateChecker backgroundChecker;`
    #[inline]
    pub unsafe fn GetBackgroundChecker(&self, aBackgroundChecker: *mut *const nsIUpdateChecker) -> ::nserror::nsresult {
        ((*self.vtable).GetBackgroundChecker)(self, aBackgroundChecker)
    }


    /// ```text
    /// /**
    ///    * Selects the best update to install from a list of available updates.
    ///    * @param   updates
    ///    *          An array of updates that are available
    ///    */
    /// ```
    ///

    /// `nsIUpdate selectUpdate (in Array<nsIUpdate> updates);`
    #[inline]
    pub unsafe fn SelectUpdate(&self, updates: *const thin_vec::ThinVec<RefPtr<nsIUpdate>>, _retval: *mut *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).SelectUpdate)(self, updates, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds a listener that receives progress and state information about the
    ///    * update that is currently being downloaded, e.g. to update a user
    ///    * interface. Registered listeners will be called for all downloads and all
    ///    * updates during a browser session; they are not automatically removed
    ///    * following the first (successful or failed) download.
    ///    * @param   listener
    ///    *          An object implementing nsIRequestObserver and optionally
    ///    *          nsIProgressEventSink that is to be notified of state and
    ///    *          progress information as the update is downloaded.
    ///    */
    /// ```
    ///

    /// `void addDownloadListener (in nsIRequestObserver listener);`
    #[inline]
    pub unsafe fn AddDownloadListener(&self, listener: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddDownloadListener)(self, listener)
    }


    /// ```text
    /// /**
    ///    * Removes a listener that is receiving progress and state information
    ///    * about the update that is currently being downloaded.
    ///    * @param   listener
    ///    *          The listener object to remove.
    ///    */
    /// ```
    ///

    /// `void removeDownloadListener (in nsIRequestObserver listener);`
    #[inline]
    pub unsafe fn RemoveDownloadListener(&self, listener: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDownloadListener)(self, listener)
    }


    /// ```text
    /// /**
    ///    *
    ///    */
    /// ```
    ///

    /// `bool downloadUpdate (in nsIUpdate update, in boolean background);`
    #[inline]
    pub unsafe fn DownloadUpdate(&self, update: *const nsIUpdate, background: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).DownloadUpdate)(self, update, background, _retval)
    }


    /// ```text
    /// /**
    ///    * Stop the active update download process. This is the equivalent of
    ///    * calling nsIRequest::Cancel on the download's nsIRequest. When downloading
    ///    * with nsIIncrementalDownload, this will leave the partial download in place.
    ///    * When downloading with BITS, any partial download progress will be removed.
    ///    */
    /// ```
    ///

    /// `void stopDownload ();`
    #[inline]
    pub unsafe fn StopDownload(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopDownload)(self, )
    }


    /// ```text
    /// /**
    ///    * Whether or not there is an download happening at the moment.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isDownloading;`
    #[inline]
    pub unsafe fn GetIsDownloading(&self, aIsDownloading: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDownloading)(self, aIsDownloading)
    }


    /// ```text
    /// /**
    ///    * Whether or not the Update Service can check for updates. This is a function
    ///    * of whether or not application update is disabled by the application and the
    ///    * platform the application is running on.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canCheckForUpdates;`
    #[inline]
    pub unsafe fn GetCanCheckForUpdates(&self, aCanCheckForUpdates: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanCheckForUpdates)(self, aCanCheckForUpdates)
    }


    /// ```text
    /// /**
    ///    * Whether or not the installation requires elevation. Currently only
    ///    * implemented on OSX, returns false on other platforms.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean elevationRequired;`
    #[inline]
    pub unsafe fn GetElevationRequired(&self, aElevationRequired: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetElevationRequired)(self, aElevationRequired)
    }


    /// ```text
    /// /**
    ///    * Whether or not the Update Service can download and install updates.
    ///    * On Windows, this is a function of whether or not the maintenance service
    ///    * is installed and enabled. On other systems, and as a fallback on Windows,
    ///    * this depends on whether the current user has write access to the install
    ///    * directory.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canApplyUpdates;`
    #[inline]
    pub unsafe fn GetCanApplyUpdates(&self, aCanApplyUpdates: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanApplyUpdates)(self, aCanApplyUpdates)
    }


    /// ```text
    /// /**
    ///    * Whether or not a different instance is handling updates of this
    ///    * installation.  This currently only ever returns true on Windows
    ///    * when 2 instances of an application are open. Only one of the instances
    ///    * will actually handle updates for the installation.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isOtherInstanceHandlingUpdates;`
    #[inline]
    pub unsafe fn GetIsOtherInstanceHandlingUpdates(&self, aIsOtherInstanceHandlingUpdates: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsOtherInstanceHandlingUpdates)(self, aIsOtherInstanceHandlingUpdates)
    }


    /// ```text
    /// /**
    ///    * Whether the Update Service is able to stage updates.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canStageUpdates;`
    #[inline]
    pub unsafe fn GetCanStageUpdates(&self, aCanStageUpdates: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanStageUpdates)(self, aCanStageUpdates)
    }


    /// ```text
    /// /**
    ///    * Indicates whether or not the enterprise policy that allows only manual
    ///    * updating is active. One of the features of this policy is not being
    ///    * notified of updates; you are intended to need to manually tell Firefox
    ///    * that you want to update each time that you want to do so.
    ///    *
    ///    * This policy has some implications for the way that update checks work. We
    ///    * don't want to do background update checks. Without being able to notify
    ///    * the user, there's not really anything to do if we find one. However, we
    ///    * will allow "automatic" update checks when loading the update interfaces
    ///    * in about:preferences, the About Dialog, etc. When those interfaces are
    ///    * open, we do have a way of telling the user about an update without
    ///    * bothering them with a doorhanger.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean manualUpdateOnly;`
    #[inline]
    pub unsafe fn GetManualUpdateOnly(&self, aManualUpdateOnly: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetManualUpdateOnly)(self, aManualUpdateOnly)
    }


}


/// `interface nsIUpdateProcessor : nsISupports`
///

/// ```text
/// /**
///  * An interface describing a component which handles the job of processing
///  * an update after it's been downloaded.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdateProcessor {
    vtable: *const nsIUpdateProcessorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdateProcessor.
unsafe impl XpCom for nsIUpdateProcessor {
    const IID: nsIID = nsID(0x74439497, 0xd796, 0x4915,
        [0x8c, 0xef, 0x3d, 0xfe, 0x43, 0x02, 0x7e, 0x4d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdateProcessor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdateProcessor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateProcessorCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdateProcessor`.
    fn coerce_from(v: &nsIUpdateProcessor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateProcessorCoerce for nsIUpdateProcessor {
    #[inline]
    fn coerce_from(v: &nsIUpdateProcessor) -> &Self {
        v
    }
}

impl nsIUpdateProcessor {
    /// Cast this `nsIUpdateProcessor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateProcessorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdateProcessor {
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
impl<T: nsISupportsCoerce> nsIUpdateProcessorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateProcessor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdateProcessor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateProcessorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void processUpdate (); */
    pub ProcessUpdate: unsafe extern "system" fn (this: *const nsIUpdateProcessor) -> ::nserror::nsresult,

    /* void fixUpdateDirectoryPerms (in boolean useServiceOnFailure); */
    pub FixUpdateDirectoryPerms: unsafe extern "system" fn (this: *const nsIUpdateProcessor, useServiceOnFailure: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdateProcessor {

    /// ```text
    /// /**
    ///    * Stages an update while the application is running.
    ///    */
    /// ```
    ///

    /// `void processUpdate ();`
    #[inline]
    pub unsafe fn ProcessUpdate(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ProcessUpdate)(self, )
    }


    /// ```text
    /// /**
    ///    * Attempts to fix the permissions of the update directory. This can be done
    ///    * in two ways. Firefox can attempt to fix the permissions itself, or it can
    ///    * call into the maintenance service to request that it attempt to fix the
    ///    * permissions.
    ///    *
    ///    * Fixing the permissions can take some time, so this work is all done off of
    ///    * the main thread.
    ///    *
    ///    * Currently, this function only has a Windows implementation. On other
    ///    * operating systems, it will throw NS_ERROR_NOT_IMPLEMENTED.
    ///    *
    ///    * Since this function does its work off of the main thread and does not
    ///    * block, it will only throw if it was unable to dispatch to another thread.
    ///    *
    ///    * @param useServiceOnFailure
    ///    *        If set to false, Firefox will attempt to fix the permissions itself,
    ///    *        but the maintenance service will not be used. If set to true and
    ///    *        Firefox is unable to fix the permissions itself, it will attempt to
    ///    *        call into the maintenance service to request that it attempt to fix
    ///    *        the permissions.
    ///    */
    /// ```
    ///

    /// `void fixUpdateDirectoryPerms (in boolean useServiceOnFailure);`
    #[inline]
    pub unsafe fn FixUpdateDirectoryPerms(&self, useServiceOnFailure: bool) -> ::nserror::nsresult {
        ((*self.vtable).FixUpdateDirectoryPerms)(self, useServiceOnFailure)
    }


}


/// `interface nsIUpdateSyncManager : nsISupports`
///

/// ```text
/// /**
///  * Upon creation, which should happen early during startup, the sync manager
///  * creates/opens and locks a file. All other running instances of the same
///  * installation of the app also open the same lock, so we can use it to
///  * determine whether any other instance is running. If so, we'll temporarily
///  * hold off on performing update tasks until there are no other instances or
///  * until a timeout expires, whichever comes first. That way we can avoid
///  * updating behind the back of copies that are still running, so we don't force
///  * all running instances to restart (see bug 1366808, where an error was added
    ///  * informing the user of the need to restart any running instances that have
    ///  * been updated).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdateSyncManager {
    vtable: *const nsIUpdateSyncManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdateSyncManager.
unsafe impl XpCom for nsIUpdateSyncManager {
    const IID: nsIID = nsID(0xcf4c4487, 0x66d9, 0x4e18,
        [0xa2, 0xe9, 0x39, 0x00, 0x22, 0x45, 0x33, 0x2f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdateSyncManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdateSyncManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateSyncManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdateSyncManager`.
    fn coerce_from(v: &nsIUpdateSyncManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateSyncManagerCoerce for nsIUpdateSyncManager {
    #[inline]
    fn coerce_from(v: &nsIUpdateSyncManager) -> &Self {
        v
    }
}

impl nsIUpdateSyncManager {
    /// Cast this `nsIUpdateSyncManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateSyncManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdateSyncManager {
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
impl<T: nsISupportsCoerce> nsIUpdateSyncManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateSyncManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdateSyncManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateSyncManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool isOtherInstanceRunning (); */
    pub IsOtherInstanceRunning: unsafe extern "system" fn (this: *const nsIUpdateSyncManager, _retval: *mut bool) -> ::nserror::nsresult,

    /* void resetLock (); */
    pub ResetLock: unsafe extern "system" fn (this: *const nsIUpdateSyncManager) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdateSyncManager {

    /// ```text
    /// /**
    ///    * Returns whether another instance of this application is running.
    ///    * @returns true if another instance has the lock open, false if not
    ///    */
    /// ```
    ///

    /// `bool isOtherInstanceRunning ();`
    #[inline]
    pub unsafe fn IsOtherInstanceRunning(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsOtherInstanceRunning)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Should only be used for testing.
    ///    * Closes and reopens the lock file, possibly under a different name if the
    ///    * path hash has changed (which should only happen if a test is forcing it).
    ///    */
    /// ```
    ///

    /// `void resetLock ();`
    #[inline]
    pub unsafe fn ResetLock(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetLock)(self, )
    }


}


/// `interface nsIUpdateManager : nsISupports`
///

/// ```text
/// /**
///  * An interface describing a global application service that maintains a list
///  * of updates previously performed as well as the current active update.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdateManager {
    vtable: *const nsIUpdateManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdateManager.
unsafe impl XpCom for nsIUpdateManager {
    const IID: nsIID = nsID(0x0f1098e9, 0xa447, 0x4af9,
        [0xb0, 0x30, 0x6f, 0x8f, 0x35, 0xc8, 0x5f, 0x89]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdateManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdateManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdateManager`.
    fn coerce_from(v: &nsIUpdateManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateManagerCoerce for nsIUpdateManager {
    #[inline]
    fn coerce_from(v: &nsIUpdateManager) -> &Self {
        v
    }
}

impl nsIUpdateManager {
    /// Cast this `nsIUpdateManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdateManager {
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
impl<T: nsISupportsCoerce> nsIUpdateManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdateManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIUpdate getUpdateAt (in long index); */
    pub GetUpdateAt: unsafe extern "system" fn (this: *const nsIUpdateManager, index: i32, _retval: *mut *const nsIUpdate) -> ::nserror::nsresult,

    /* long getUpdateCount (); */
    pub GetUpdateCount: unsafe extern "system" fn (this: *const nsIUpdateManager, _retval: *mut i32) -> ::nserror::nsresult,

    /* attribute nsIUpdate readyUpdate; */
    pub GetReadyUpdate: unsafe extern "system" fn (this: *const nsIUpdateManager, aReadyUpdate: *mut *const nsIUpdate) -> ::nserror::nsresult,

    /* attribute nsIUpdate readyUpdate; */
    pub SetReadyUpdate: unsafe extern "system" fn (this: *const nsIUpdateManager, aReadyUpdate: *const nsIUpdate) -> ::nserror::nsresult,

    /* attribute nsIUpdate downloadingUpdate; */
    pub GetDownloadingUpdate: unsafe extern "system" fn (this: *const nsIUpdateManager, aDownloadingUpdate: *mut *const nsIUpdate) -> ::nserror::nsresult,

    /* attribute nsIUpdate downloadingUpdate; */
    pub SetDownloadingUpdate: unsafe extern "system" fn (this: *const nsIUpdateManager, aDownloadingUpdate: *const nsIUpdate) -> ::nserror::nsresult,

    /* void addUpdateToHistory (in nsIUpdate update); */
    pub AddUpdateToHistory: unsafe extern "system" fn (this: *const nsIUpdateManager, update: *const nsIUpdate) -> ::nserror::nsresult,

    /* void saveUpdates (); */
    pub SaveUpdates: unsafe extern "system" fn (this: *const nsIUpdateManager) -> ::nserror::nsresult,

    /* void refreshUpdateStatus (); */
    pub RefreshUpdateStatus: unsafe extern "system" fn (this: *const nsIUpdateManager) -> ::nserror::nsresult,

    /* void elevationOptedIn (); */
    pub ElevationOptedIn: unsafe extern "system" fn (this: *const nsIUpdateManager) -> ::nserror::nsresult,

    /* void cleanupDownloadingUpdate (); */
    pub CleanupDownloadingUpdate: unsafe extern "system" fn (this: *const nsIUpdateManager) -> ::nserror::nsresult,

    /* void cleanupReadyUpdate (); */
    pub CleanupReadyUpdate: unsafe extern "system" fn (this: *const nsIUpdateManager) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdateManager {

    /// ```text
    /// /**
    ///    * Gets the update at the specified index
    ///    * @param   index
    ///    *          The index within the updates array
    ///    * @returns The nsIUpdate object at the specified index
    ///    */
    /// ```
    ///

    /// `nsIUpdate getUpdateAt (in long index);`
    #[inline]
    pub unsafe fn GetUpdateAt(&self, index: i32, _retval: *mut *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdateAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Gets the total number of updates in the history list.
    ///    */
    /// ```
    ///

    /// `long getUpdateCount ();`
    #[inline]
    pub unsafe fn GetUpdateCount(&self, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdateCount)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * The update that has been downloaded, or null if there isn't one.
    ///    */
    /// ```
    ///

    /// `attribute nsIUpdate readyUpdate;`
    #[inline]
    pub unsafe fn GetReadyUpdate(&self, aReadyUpdate: *mut *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).GetReadyUpdate)(self, aReadyUpdate)
    }


    /// ```text
    /// /**
    ///    * The update that has been downloaded, or null if there isn't one.
    ///    */
    /// ```
    ///

    /// `attribute nsIUpdate readyUpdate;`
    #[inline]
    pub unsafe fn SetReadyUpdate(&self, aReadyUpdate: *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).SetReadyUpdate)(self, aReadyUpdate)
    }


    /// ```text
    /// /**
    ///    * The update that is currently downloading, or null if there isn't one.
    ///    * An update is no longer considered to be downloading once onStopRequest is
    ///    * called. This means that both onStopRequest handlers for download listeners
    ///    * and observers of the "update-downloaded" topic should expect the update
    ///    * that was just downloaded to be stored in readyUpdate, not
    ///    * downloadingUpdate.
    ///    */
    /// ```
    ///

    /// `attribute nsIUpdate downloadingUpdate;`
    #[inline]
    pub unsafe fn GetDownloadingUpdate(&self, aDownloadingUpdate: *mut *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).GetDownloadingUpdate)(self, aDownloadingUpdate)
    }


    /// ```text
    /// /**
    ///    * The update that is currently downloading, or null if there isn't one.
    ///    * An update is no longer considered to be downloading once onStopRequest is
    ///    * called. This means that both onStopRequest handlers for download listeners
    ///    * and observers of the "update-downloaded" topic should expect the update
    ///    * that was just downloaded to be stored in readyUpdate, not
    ///    * downloadingUpdate.
    ///    */
    /// ```
    ///

    /// `attribute nsIUpdate downloadingUpdate;`
    #[inline]
    pub unsafe fn SetDownloadingUpdate(&self, aDownloadingUpdate: *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).SetDownloadingUpdate)(self, aDownloadingUpdate)
    }


    /// ```text
    /// /**
    ///    * Adds the specified update to the update history. The update history is
    ///    * limited to 10 items, so this may also remove the last item from the
    ///    * history.
    ///    */
    /// ```
    ///

    /// `void addUpdateToHistory (in nsIUpdate update);`
    #[inline]
    pub unsafe fn AddUpdateToHistory(&self, update: *const nsIUpdate) -> ::nserror::nsresult {
        ((*self.vtable).AddUpdateToHistory)(self, update)
    }


    /// ```text
    /// /**
    ///    * Saves all updates to disk.
    ///    */
    /// ```
    ///

    /// `void saveUpdates ();`
    #[inline]
    pub unsafe fn SaveUpdates(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SaveUpdates)(self, )
    }


    /// ```text
    /// /**
    ///    * Refresh the update status based on the information in update.status.
    ///    */
    /// ```
    ///

    /// `void refreshUpdateStatus ();`
    #[inline]
    pub unsafe fn RefreshUpdateStatus(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RefreshUpdateStatus)(self, )
    }


    /// ```text
    /// /**
    ///    * The user agreed to proceed with an elevated update and we are now
    ///    * permitted to show an elevation prompt.
    ///    */
    /// ```
    ///

    /// `void elevationOptedIn ();`
    #[inline]
    pub unsafe fn ElevationOptedIn(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ElevationOptedIn)(self, )
    }


    /// ```text
    /// /**
    ///    * These functions both clean up and remove an active update without applying
    ///    * it. The first function does this for the update that is currently being
    ///    * downloaded. The second function does this for the update that has already
    ///    * been downloaded.
    ///    */
    /// ```
    ///

    /// `void cleanupDownloadingUpdate ();`
    #[inline]
    pub unsafe fn CleanupDownloadingUpdate(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CleanupDownloadingUpdate)(self, )
    }



    /// `void cleanupReadyUpdate ();`
    #[inline]
    pub unsafe fn CleanupReadyUpdate(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CleanupReadyUpdate)(self, )
    }


}


