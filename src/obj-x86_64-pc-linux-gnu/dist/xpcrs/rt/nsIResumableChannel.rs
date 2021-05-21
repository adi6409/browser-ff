//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIResumableChannel.idl
//


/// `interface nsIResumableChannel : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIResumableChannel {
    vtable: *const nsIResumableChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIResumableChannel.
unsafe impl XpCom for nsIResumableChannel {
    const IID: nsIID = nsID(0x4ad136fa, 0x83af, 0x4a22,
        [0xa7, 0x6e, 0x50, 0x36, 0x42, 0xc0, 0xf4, 0xa8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIResumableChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIResumableChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIResumableChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIResumableChannel`.
    fn coerce_from(v: &nsIResumableChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIResumableChannelCoerce for nsIResumableChannel {
    #[inline]
    fn coerce_from(v: &nsIResumableChannel) -> &Self {
        v
    }
}

impl nsIResumableChannel {
    /// Cast this `nsIResumableChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIResumableChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIResumableChannel {
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
impl<T: nsISupportsCoerce> nsIResumableChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIResumableChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIResumableChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIResumableChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resumeAt (in unsigned long long startPos, in ACString entityID); */
    pub ResumeAt: unsafe extern "system" fn (this: *const nsIResumableChannel, startPos: u64, entityID: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString entityID; */
    pub GetEntityID: unsafe extern "system" fn (this: *const nsIResumableChannel, aEntityID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIResumableChannel {

    /// ```text
    /// /**
    ///      * Prepare this channel for resuming. The request will not start until
    ///      * asyncOpen or open is called. Calling resumeAt after open or asyncOpen
    ///      * has been called has undefined behaviour.
    ///      *
    ///      * @param startPos the starting offset, in bytes, to use to download
    ///      * @param entityID information about the file, to match before obtaining
    ///      *  the file. Pass an empty string to use anything.
    ///      *
    ///      * During OnStartRequest, this channel will have a status of
    ///      *  NS_ERROR_NOT_RESUMABLE if the file cannot be resumed, eg because the
    ///      *  server doesn't support this. This error may occur even if startPos
    ///      *  is 0, so that the front end can warn the user.
    ///      * Similarly, the status of this channel during OnStartRequest may be
    ///      *  NS_ERROR_ENTITY_CHANGED, which indicates that the entity has changed,
    ///      *  as indicated by a changed entityID.
    ///      * In both of these cases, no OnDataAvailable will be called, and
    ///      *  OnStopRequest will immediately follow with the same status code.
    ///      */
    /// ```
    ///

    /// `void resumeAt (in unsigned long long startPos, in ACString entityID);`
    #[inline]
    pub unsafe fn ResumeAt(&self, startPos: u64, entityID: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ResumeAt)(self, startPos, entityID)
    }


    /// ```text
    /// /**
    ///      * The entity id for this URI. Available after OnStartRequest.
    ///      * @throw NS_ERROR_NOT_RESUMABLE if this load is not resumable.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString entityID;`
    #[inline]
    pub unsafe fn GetEntityID(&self, aEntityID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEntityID)(self, aEntityID)
    }


}


