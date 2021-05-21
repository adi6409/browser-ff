//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/system/nsIOSPermissionRequest.idl
//


/// `interface nsIOSPermissionRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOSPermissionRequest {
    vtable: *const nsIOSPermissionRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOSPermissionRequest.
unsafe impl XpCom for nsIOSPermissionRequest {
    const IID: nsIID = nsID(0x95790842, 0x75a0, 0x430d,
        [0x98, 0xbf, 0xf5, 0xce, 0x37, 0x88, 0xea, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOSPermissionRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOSPermissionRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOSPermissionRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIOSPermissionRequest`.
    fn coerce_from(v: &nsIOSPermissionRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOSPermissionRequestCoerce for nsIOSPermissionRequest {
    #[inline]
    fn coerce_from(v: &nsIOSPermissionRequest) -> &Self {
        v
    }
}

impl nsIOSPermissionRequest {
    /// Cast this `nsIOSPermissionRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOSPermissionRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOSPermissionRequest {
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
impl<T: nsISupportsCoerce> nsIOSPermissionRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOSPermissionRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOSPermissionRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOSPermissionRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getMediaCapturePermissionState (out uint16_t aVideo, out uint16_t aAudio); */
    pub GetMediaCapturePermissionState: unsafe extern "system" fn (this: *const nsIOSPermissionRequest, aVideo: *mut uint16_t, aAudio: *mut uint16_t) -> ::nserror::nsresult,

    /* void getAudioCapturePermissionState (out uint16_t aAudio); */
    pub GetAudioCapturePermissionState: unsafe extern "system" fn (this: *const nsIOSPermissionRequest, aAudio: *mut uint16_t) -> ::nserror::nsresult,

    /* void getVideoCapturePermissionState (out uint16_t aVideo); */
    pub GetVideoCapturePermissionState: unsafe extern "system" fn (this: *const nsIOSPermissionRequest, aVideo: *mut uint16_t) -> ::nserror::nsresult,

    /* void getScreenCapturePermissionState (out uint16_t aScreen); */
    pub GetScreenCapturePermissionState: unsafe extern "system" fn (this: *const nsIOSPermissionRequest, aScreen: *mut uint16_t) -> ::nserror::nsresult,

    /* [implicit_jscontext,must_use] Promise requestVideoCapturePermission (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub RequestVideoCapturePermission: *const ::libc::c_void,

    /* [implicit_jscontext,must_use] Promise requestAudioCapturePermission (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub RequestAudioCapturePermission: *const ::libc::c_void,

    /* void maybeRequestScreenCapturePermission (); */
    pub MaybeRequestScreenCapturePermission: unsafe extern "system" fn (this: *const nsIOSPermissionRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOSPermissionRequest {

    pub const PERMISSION_STATE_NOTDETERMINED: i64 = 0;


    pub const PERMISSION_STATE_RESTRICTED: i64 = 1;


    pub const PERMISSION_STATE_DENIED: i64 = 2;


    pub const PERMISSION_STATE_AUTHORIZED: i64 = 3;


    /// `void getMediaCapturePermissionState (out uint16_t aVideo, out uint16_t aAudio);`
    #[inline]
    pub unsafe fn GetMediaCapturePermissionState(&self, aVideo: *mut uint16_t, aAudio: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetMediaCapturePermissionState)(self, aVideo, aAudio)
    }



    /// `void getAudioCapturePermissionState (out uint16_t aAudio);`
    #[inline]
    pub unsafe fn GetAudioCapturePermissionState(&self, aAudio: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAudioCapturePermissionState)(self, aAudio)
    }



    /// `void getVideoCapturePermissionState (out uint16_t aVideo);`
    #[inline]
    pub unsafe fn GetVideoCapturePermissionState(&self, aVideo: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetVideoCapturePermissionState)(self, aVideo)
    }



    /// `void getScreenCapturePermissionState (out uint16_t aScreen);`
    #[inline]
    pub unsafe fn GetScreenCapturePermissionState(&self, aScreen: *mut uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).GetScreenCapturePermissionState)(self, aScreen)
    }



    /// `[implicit_jscontext,must_use] Promise requestVideoCapturePermission ();`
    const _RequestVideoCapturePermission: () = ();


    /// `[implicit_jscontext,must_use] Promise requestAudioCapturePermission ();`
    const _RequestAudioCapturePermission: () = ();


    /// `void maybeRequestScreenCapturePermission ();`
    #[inline]
    pub unsafe fn MaybeRequestScreenCapturePermission(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MaybeRequestScreenCapturePermission)(self, )
    }


}


