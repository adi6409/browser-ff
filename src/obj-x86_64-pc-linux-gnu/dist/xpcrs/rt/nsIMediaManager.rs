//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIMediaManager.idl
//


/// `interface nsIMediaManagerService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMediaManagerService {
    vtable: *const nsIMediaManagerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMediaManagerService.
unsafe impl XpCom for nsIMediaManagerService {
    const IID: nsIID = nsID(0x24b23e01, 0x33fd, 0x401f,
        [0xba, 0x25, 0x6e, 0x52, 0x65, 0x87, 0x50, 0xb0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMediaManagerService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMediaManagerService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMediaManagerServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIMediaManagerService`.
    fn coerce_from(v: &nsIMediaManagerService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMediaManagerServiceCoerce for nsIMediaManagerService {
    #[inline]
    fn coerce_from(v: &nsIMediaManagerService) -> &Self {
        v
    }
}

impl nsIMediaManagerService {
    /// Cast this `nsIMediaManagerService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMediaManagerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMediaManagerService {
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
impl<T: nsISupportsCoerce> nsIMediaManagerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMediaManagerService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMediaManagerService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMediaManagerServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIArray activeMediaCaptureWindows; */
    pub GetActiveMediaCaptureWindows: unsafe extern "system" fn (this: *const nsIMediaManagerService, aActiveMediaCaptureWindows: *mut*const nsIArray) -> ::nserror::nsresult,

    /* void mediaCaptureWindowState (in nsIDOMWindow aWindow, out unsigned short aCamera, out unsigned short aMicrophone, out unsigned short aScreenShare, out unsigned short aWindowShare, out unsigned short aBrowserShare, out Array<nsIMediaDevice> devices); */
    pub MediaCaptureWindowState: unsafe extern "system" fn (this: *const nsIMediaManagerService, aWindow: *const nsIDOMWindow, aCamera: *mut u16, aMicrophone: *mut u16, aScreenShare: *mut u16, aWindowShare: *mut u16, aBrowserShare: *mut u16, devices: *mut thin_vec::ThinVec<RefPtr<nsIMediaDevice>>) -> ::nserror::nsresult,

    /* void sanitizeDeviceIds (in long long sinceWhen); */
    pub SanitizeDeviceIds: unsafe extern "system" fn (this: *const nsIMediaManagerService, sinceWhen: i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMediaManagerService {

    pub const STATE_NOCAPTURE: i64 = 0;


    pub const STATE_CAPTURE_ENABLED: i64 = 1;


    pub const STATE_CAPTURE_DISABLED: i64 = 2;


    /// `readonly attribute nsIArray activeMediaCaptureWindows;`
    #[inline]
    pub unsafe fn GetActiveMediaCaptureWindows(&self, aActiveMediaCaptureWindows: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveMediaCaptureWindows)(self, aActiveMediaCaptureWindows)
    }



    /// `void mediaCaptureWindowState (in nsIDOMWindow aWindow, out unsigned short aCamera, out unsigned short aMicrophone, out unsigned short aScreenShare, out unsigned short aWindowShare, out unsigned short aBrowserShare, out Array<nsIMediaDevice> devices);`
    #[inline]
    pub unsafe fn MediaCaptureWindowState(&self, aWindow: *const nsIDOMWindow, aCamera: *mut u16, aMicrophone: *mut u16, aScreenShare: *mut u16, aWindowShare: *mut u16, aBrowserShare: *mut u16, devices: *mut thin_vec::ThinVec<RefPtr<nsIMediaDevice>>) -> ::nserror::nsresult {
        ((*self.vtable).MediaCaptureWindowState)(self, aWindow, aCamera, aMicrophone, aScreenShare, aWindowShare, aBrowserShare, devices)
    }



    /// `void sanitizeDeviceIds (in long long sinceWhen);`
    #[inline]
    pub unsafe fn SanitizeDeviceIds(&self, sinceWhen: i64) -> ::nserror::nsresult {
        ((*self.vtable).SanitizeDeviceIds)(self, sinceWhen)
    }


}


