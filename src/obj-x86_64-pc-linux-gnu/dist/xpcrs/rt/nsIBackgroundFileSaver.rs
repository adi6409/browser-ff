//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIBackgroundFileSaver.idl
//


/// `interface nsIBackgroundFileSaver : nsISupports`
///

/// ```text
/// /**
///  * Allows saving data to a file, while handling all the input/output on a
///  * background thread, including the initial file name assignment and any
///  * subsequent renaming of the target file.
///  *
///  * This interface is designed for file downloads.  Generally, they start in the
///  * temporary directory, while the user is selecting the final name.  Then, they
///  * are moved to the chosen target directory with a ".part" extension appended to
///  * the file name.  Finally, they are renamed when the download is completed.
///  *
///  * Components implementing both nsIBackgroundFileSaver and nsIStreamListener
///  * allow data to be fed using an implementation of OnDataAvailable that never
///  * blocks the calling thread.  They suspend the request that drives the stream
///  * listener in case too much data is being fed, and resume it when the data has
///  * been written.  Calling OnStopRequest does not necessarily close the target
///  * file, and the Finish method must be called to complete the operation.
///  *
///  * Components implementing both nsIBackgroundFileSaver and nsIAsyncOutputStream
///  * allow data to be fed directly to the non-blocking output stream, that however
///  * may return NS_BASE_STREAM_WOULD_BLOCK in case too much data is being fed.
///  * Closing the output stream does not necessarily close the target file, and the
///  * Finish method must be called to complete the operation.
///  *
///  * @remarks Implementations may require the consumer to always call Finish.  If
///  *          the object reference is released without calling Finish, a memory
///  *          leak may occur, and the target file might be kept locked. All
///  *          public methods of the interface may only be called from the main
///  *          thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBackgroundFileSaver {
    vtable: *const nsIBackgroundFileSaverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBackgroundFileSaver.
unsafe impl XpCom for nsIBackgroundFileSaver {
    const IID: nsIID = nsID(0xc43544a4, 0x682c, 0x4262,
        [0xb4, 0x07, 0x24, 0x53, 0xd2, 0x6e, 0x66, 0x0d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBackgroundFileSaver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBackgroundFileSaver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBackgroundFileSaverCoerce {
    /// Cheaply cast a value of this type from a `nsIBackgroundFileSaver`.
    fn coerce_from(v: &nsIBackgroundFileSaver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBackgroundFileSaverCoerce for nsIBackgroundFileSaver {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaver) -> &Self {
        v
    }
}

impl nsIBackgroundFileSaver {
    /// Cast this `nsIBackgroundFileSaver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBackgroundFileSaverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBackgroundFileSaver {
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
impl<T: nsISupportsCoerce> nsIBackgroundFileSaverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBackgroundFileSaver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBackgroundFileSaverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIBackgroundFileSaverObserver observer; */
    pub GetObserver: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver, aObserver: *mut*const nsIBackgroundFileSaverObserver) -> ::nserror::nsresult,

    /* attribute nsIBackgroundFileSaverObserver observer; */
    pub SetObserver: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver, aObserver: *const nsIBackgroundFileSaverObserver) -> ::nserror::nsresult,

    /* readonly attribute Array<Array<Array<uint8_t>>> signatureInfo; */
    pub GetSignatureInfo: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver, aSignatureInfo: *mut thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>) -> ::nserror::nsresult,

    /* readonly attribute ACString sha256Hash; */
    pub GetSha256Hash: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver, aSha256Hash: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void enableSignatureInfo (); */
    pub EnableSignatureInfo: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver) -> ::nserror::nsresult,

    /* void enableSha256 (); */
    pub EnableSha256: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver) -> ::nserror::nsresult,

    /* void enableAppend (); */
    pub EnableAppend: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver) -> ::nserror::nsresult,

    /* void setTarget (in nsIFile aTarget, in bool aKeepPartial); */
    pub SetTarget: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver, aTarget: *const nsIFile, aKeepPartial: bool) -> ::nserror::nsresult,

    /* void finish (in nsresult aStatus); */
    pub Finish: unsafe extern "system" fn (this: *const nsIBackgroundFileSaver, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBackgroundFileSaver {

    /// ```text
    /// /**
    ///    * This observer receives notifications when the target file name changes and
    ///    * when the operation completes, successfully or not.
    ///    *
    ///    * @remarks A strong reference to the observer is held.  Notification events
    ///    *          are dispatched to the thread that created the object that
    ///    *          implements nsIBackgroundFileSaver.
    ///    */
    /// ```
    ///

    /// `attribute nsIBackgroundFileSaverObserver observer;`
    #[inline]
    pub unsafe fn GetObserver(&self, aObserver: *mut*const nsIBackgroundFileSaverObserver) -> ::nserror::nsresult {
        ((*self.vtable).GetObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * This observer receives notifications when the target file name changes and
    ///    * when the operation completes, successfully or not.
    ///    *
    ///    * @remarks A strong reference to the observer is held.  Notification events
    ///    *          are dispatched to the thread that created the object that
    ///    *          implements nsIBackgroundFileSaver.
    ///    */
    /// ```
    ///

    /// `attribute nsIBackgroundFileSaverObserver observer;`
    #[inline]
    pub unsafe fn SetObserver(&self, aObserver: *const nsIBackgroundFileSaverObserver) -> ::nserror::nsresult {
        ((*self.vtable).SetObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * An Array of Array of Array of bytes, representing a chain of
    ///    * X.509 certificates, the last of which signed the downloaded file.
    ///    * Each list may belong to a different signer and contain certificates
    ///    * all the way up to the root.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *         In case this is called before the onSaveComplete method has been
    ///    *         called to notify success, or enableSignatureInfo has not been
    ///    *         called.
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<Array<Array<uint8_t>>> signatureInfo;`
    #[inline]
    pub unsafe fn GetSignatureInfo(&self, aSignatureInfo: *mut thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>) -> ::nserror::nsresult {
        ((*self.vtable).GetSignatureInfo)(self, aSignatureInfo)
    }


    /// ```text
    /// /**
    ///    * The SHA-256 hash, in raw bytes, associated with the data that was saved.
    ///    *
    ///    * In case the enableAppend method has been called, the hash computation
    ///    * includes the contents of the existing file, if any.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *         In case the enableSha256 method has not been called, or before the
    ///    *         onSaveComplete method has been called to notify success.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString sha256Hash;`
    #[inline]
    pub unsafe fn GetSha256Hash(&self, aSha256Hash: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSha256Hash)(self, aSha256Hash)
    }


    /// ```text
    /// /**
    ///    * Instructs the component to compute the signatureInfo of the target file,
    ///    * and make it available in the signatureInfo property.
    ///    *
    ///    * @remarks This must be set on the main thread before the first call to
    ///    *          setTarget.
    ///    */
    /// ```
    ///

    /// `void enableSignatureInfo ();`
    #[inline]
    pub unsafe fn EnableSignatureInfo(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnableSignatureInfo)(self, )
    }


    /// ```text
    /// /**
    ///    * Instructs the component to compute the SHA-256 hash of the target file, and
    ///    * make it available in the sha256Hash property.
    ///    *
    ///    * @remarks This must be set on the main thread before the first call to
    ///    *          setTarget.
    ///    */
    /// ```
    ///

    /// `void enableSha256 ();`
    #[inline]
    pub unsafe fn EnableSha256(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnableSha256)(self, )
    }


    /// ```text
    /// /**
    ///    * Instructs the component to append data to the initial target file, that
    ///    * will be specified by the first call to the setTarget method, instead of
    ///    * overwriting the file.
    ///    *
    ///    * If the initial target file does not exist, this method has no effect.
    ///    *
    ///    * @remarks This must be set on the main thread before the first call to
    ///    *          setTarget.
    ///    */
    /// ```
    ///

    /// `void enableAppend ();`
    #[inline]
    pub unsafe fn EnableAppend(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnableAppend)(self, )
    }


    /// ```text
    /// /**
    ///    * Sets the name of the output file to be written.  The target can be changed
    ///    * after data has already been fed, in which case the existing file will be
    ///    * moved to the new destination.
    ///    *
    ///    * In case the specified file already exists, and this method is called for
    ///    * the first time, the file may be either overwritten or appended to, based on
    ///    * whether the enableAppend method was called.  Subsequent calls always
    ///    * overwrite the specified target file with the previously saved data.
    ///    *
    ///    * No file will be written until this function is called at least once.  It's
    ///    * recommended not to feed any data until the output file is set.
    ///    *
    ///    * If an input/output error occurs with the specified file, the save operation
    ///    * fails.  Failure is notified asynchronously through the observer.
    ///    *
    ///    * @param aTarget
    ///    *        New output file to be written.
    ///    * @param aKeepPartial
    ///    *        Indicates whether aFile should be kept as partially completed,
    ///    *        rather than deleted, if the operation fails or is canceled.  This is
    ///    *        generally set for downloads that use temporary ".part" files.
    ///    */
    /// ```
    ///

    /// `void setTarget (in nsIFile aTarget, in bool aKeepPartial);`
    #[inline]
    pub unsafe fn SetTarget(&self, aTarget: *const nsIFile, aKeepPartial: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetTarget)(self, aTarget, aKeepPartial)
    }


    /// ```text
    /// /**
    ///    * Terminates access to the output file, then notifies the observer with the
    ///    * specified status code.  A failure code will force the operation to be
    ///    * canceled, in which case the output file will be deleted if requested.
    ///    *
    ///    * This forces the involved streams to be closed, thus no more data should be
    ///    * fed to the component after this method has been called.
    ///    *
    ///    * This is the last method that should be called on this object, and the
    ///    * target file name cannot be changed anymore after this method has been
    ///    * called.  Conversely, before calling this method, the file can still be
    ///    * renamed even if all the data has been fed.
    ///    *
    ///    * @param aStatus
    ///    *        Result code that determines whether the operation should succeed or
    ///    *        be canceled, and is notified to the observer.  If the operation
    ///    *        fails meanwhile for other reasons, or the observer has been already
    ///    *        notified of completion, this status code is ignored.
    ///    */
    /// ```
    ///

    /// `void finish (in nsresult aStatus);`
    #[inline]
    pub unsafe fn Finish(&self, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Finish)(self, aStatus)
    }


}


/// `interface nsIBackgroundFileSaverObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBackgroundFileSaverObserver {
    vtable: *const nsIBackgroundFileSaverObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBackgroundFileSaverObserver.
unsafe impl XpCom for nsIBackgroundFileSaverObserver {
    const IID: nsIID = nsID(0xee7058c3, 0x6e54, 0x4411,
        [0xb7, 0x6b, 0x3c, 0xe8, 0x7b, 0x76, 0xfc, 0xb6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBackgroundFileSaverObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBackgroundFileSaverObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBackgroundFileSaverObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIBackgroundFileSaverObserver`.
    fn coerce_from(v: &nsIBackgroundFileSaverObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBackgroundFileSaverObserverCoerce for nsIBackgroundFileSaverObserver {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaverObserver) -> &Self {
        v
    }
}

impl nsIBackgroundFileSaverObserver {
    /// Cast this `nsIBackgroundFileSaverObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBackgroundFileSaverObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBackgroundFileSaverObserver {
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
impl<T: nsISupportsCoerce> nsIBackgroundFileSaverObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaverObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBackgroundFileSaverObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBackgroundFileSaverObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget); */
    pub OnTargetChange: unsafe extern "system" fn (this: *const nsIBackgroundFileSaverObserver, aSaver: *const nsIBackgroundFileSaver, aTarget: *const nsIFile) -> ::nserror::nsresult,

    /* void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus); */
    pub OnSaveComplete: unsafe extern "system" fn (this: *const nsIBackgroundFileSaverObserver, aSaver: *const nsIBackgroundFileSaver, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBackgroundFileSaverObserver {

    /// ```text
    /// /**
    ///    * Called when the name of the output file has been determined.  This function
    ///    * may be called more than once if the target file is renamed while saving.
    ///    *
    ///    * @param aSaver
    ///    *        Reference to the object that raised the notification.
    ///    * @param aTarget
    ///    *        Name of the file that is being written.
    ///    */
    /// ```
    ///

    /// `void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget);`
    #[inline]
    pub unsafe fn OnTargetChange(&self, aSaver: *const nsIBackgroundFileSaver, aTarget: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).OnTargetChange)(self, aSaver, aTarget)
    }


    /// ```text
    /// /**
    ///    * Called when the operation completed, and the target file has been closed.
    ///    * If the operation succeeded, the target file is ready to be used, otherwise
    ///    * it might have been already deleted.
    ///    *
    ///    * @param aSaver
    ///    *        Reference to the object that raised the notification.
    ///    * @param aStatus
    ///    *        Result code that determines whether the operation succeeded or
    ///    *        failed, as well as the failure reason.
    ///    */
    /// ```
    ///

    /// `void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnSaveComplete(&self, aSaver: *const nsIBackgroundFileSaver, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnSaveComplete)(self, aSaver, aStatus)
    }


}


