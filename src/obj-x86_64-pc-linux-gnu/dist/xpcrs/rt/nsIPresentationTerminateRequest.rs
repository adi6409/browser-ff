//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationTerminateRequest.idl
//


/// `interface nsIPresentationTerminateRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationTerminateRequest {
    vtable: *const nsIPresentationTerminateRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationTerminateRequest.
unsafe impl XpCom for nsIPresentationTerminateRequest {
    const IID: nsIID = nsID(0x3ddbf3a4, 0x53ee, 0x4b70,
        [0x9b, 0xbc, 0x58, 0xac, 0x90, 0xdc, 0xe6, 0xb5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationTerminateRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationTerminateRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationTerminateRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationTerminateRequest`.
    fn coerce_from(v: &nsIPresentationTerminateRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationTerminateRequestCoerce for nsIPresentationTerminateRequest {
    #[inline]
    fn coerce_from(v: &nsIPresentationTerminateRequest) -> &Self {
        v
    }
}

impl nsIPresentationTerminateRequest {
    /// Cast this `nsIPresentationTerminateRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationTerminateRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationTerminateRequest {
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
impl<T: nsISupportsCoerce> nsIPresentationTerminateRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationTerminateRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationTerminateRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationTerminateRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPresentationDevice device; */
    pub GetDevice: unsafe extern "system" fn (this: *const nsIPresentationTerminateRequest, aDevice: *mut*const nsIPresentationDevice) -> ::nserror::nsresult,

    /* readonly attribute AString presentationId; */
    pub GetPresentationId: unsafe extern "system" fn (this: *const nsIPresentationTerminateRequest, aPresentationId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPresentationControlChannel controlChannel; */
    pub GetControlChannel: unsafe extern "system" fn (this: *const nsIPresentationTerminateRequest, aControlChannel: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult,

    /* readonly attribute boolean isFromReceiver; */
    pub GetIsFromReceiver: unsafe extern "system" fn (this: *const nsIPresentationTerminateRequest, aIsFromReceiver: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationTerminateRequest {


    /// `readonly attribute nsIPresentationDevice device;`
    #[inline]
    pub unsafe fn GetDevice(&self, aDevice: *mut*const nsIPresentationDevice) -> ::nserror::nsresult {
        ((*self.vtable).GetDevice)(self, aDevice)
    }



    /// `readonly attribute AString presentationId;`
    #[inline]
    pub unsafe fn GetPresentationId(&self, aPresentationId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPresentationId)(self, aPresentationId)
    }



    /// `readonly attribute nsIPresentationControlChannel controlChannel;`
    #[inline]
    pub unsafe fn GetControlChannel(&self, aControlChannel: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetControlChannel)(self, aControlChannel)
    }



    /// `readonly attribute boolean isFromReceiver;`
    #[inline]
    pub unsafe fn GetIsFromReceiver(&self, aIsFromReceiver: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFromReceiver)(self, aIsFromReceiver)
    }


}


