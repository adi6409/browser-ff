//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionRequest.idl
//


/// `interface nsIPresentationSessionRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationSessionRequest {
    vtable: *const nsIPresentationSessionRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationSessionRequest.
unsafe impl XpCom for nsIPresentationSessionRequest {
    const IID: nsIID = nsID(0xd808a084, 0xd0f8, 0x455a,
        [0xa8, 0xdf, 0x58, 0x79, 0xe0, 0x5a, 0x75, 0x5b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationSessionRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationSessionRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationSessionRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationSessionRequest`.
    fn coerce_from(v: &nsIPresentationSessionRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationSessionRequestCoerce for nsIPresentationSessionRequest {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionRequest) -> &Self {
        v
    }
}

impl nsIPresentationSessionRequest {
    /// Cast this `nsIPresentationSessionRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationSessionRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationSessionRequest {
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
impl<T: nsISupportsCoerce> nsIPresentationSessionRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationSessionRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationSessionRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPresentationDevice device; */
    pub GetDevice: unsafe extern "system" fn (this: *const nsIPresentationSessionRequest, aDevice: *mut*const nsIPresentationDevice) -> ::nserror::nsresult,

    /* readonly attribute AString url; */
    pub GetUrl: unsafe extern "system" fn (this: *const nsIPresentationSessionRequest, aUrl: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString presentationId; */
    pub GetPresentationId: unsafe extern "system" fn (this: *const nsIPresentationSessionRequest, aPresentationId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPresentationControlChannel controlChannel; */
    pub GetControlChannel: unsafe extern "system" fn (this: *const nsIPresentationSessionRequest, aControlChannel: *mut*const nsIPresentationControlChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationSessionRequest {


    /// `readonly attribute nsIPresentationDevice device;`
    #[inline]
    pub unsafe fn GetDevice(&self, aDevice: *mut*const nsIPresentationDevice) -> ::nserror::nsresult {
        ((*self.vtable).GetDevice)(self, aDevice)
    }



    /// `readonly attribute AString url;`
    #[inline]
    pub unsafe fn GetUrl(&self, aUrl: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUrl)(self, aUrl)
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


}


