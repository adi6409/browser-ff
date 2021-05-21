//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIDOMRequestService.idl
//


/// `interface nsIDOMRequestService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMRequestService {
    vtable: *const nsIDOMRequestServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMRequestService.
unsafe impl XpCom for nsIDOMRequestService {
    const IID: nsIID = nsID(0x9a57e5de, 0xce93, 0x45fa,
        [0x81, 0x45, 0x75, 0x57, 0x22, 0x83, 0x4f, 0x7c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMRequestService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMRequestService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMRequestServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMRequestService`.
    fn coerce_from(v: &nsIDOMRequestService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMRequestServiceCoerce for nsIDOMRequestService {
    #[inline]
    fn coerce_from(v: &nsIDOMRequestService) -> &Self {
        v
    }
}

impl nsIDOMRequestService {
    /// Cast this `nsIDOMRequestService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMRequestServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMRequestService {
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
impl<T: nsISupportsCoerce> nsIDOMRequestServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMRequestService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMRequestService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMRequestServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* DOMRequest createRequest (in mozIDOMWindow window); */
    pub CreateRequest: unsafe extern "system" fn (this: *const nsIDOMRequestService, window: *const mozIDOMWindow, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void fireSuccess (in DOMRequest request, in jsval result); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FireSuccess: *const ::libc::c_void,

    /* void fireError (in DOMRequest request, in AString error); */
    pub FireError: unsafe extern "system" fn (this: *const nsIDOMRequestService, request: *const libc::c_void, error: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void fireSuccessAsync (in DOMRequest request, in jsval result); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FireSuccessAsync: *const ::libc::c_void,

    /* void fireErrorAsync (in DOMRequest request, in AString error); */
    pub FireErrorAsync: unsafe extern "system" fn (this: *const nsIDOMRequestService, request: *const libc::c_void, error: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMRequestService {


    /// `DOMRequest createRequest (in mozIDOMWindow window);`
    #[inline]
    pub unsafe fn CreateRequest(&self, window: *const mozIDOMWindow, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateRequest)(self, window, _retval)
    }



    /// `void fireSuccess (in DOMRequest request, in jsval result);`
    const _FireSuccess: () = ();


    /// `void fireError (in DOMRequest request, in AString error);`
    #[inline]
    pub unsafe fn FireError(&self, request: *const libc::c_void, error: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FireError)(self, request, error)
    }



    /// `void fireSuccessAsync (in DOMRequest request, in jsval result);`
    const _FireSuccessAsync: () = ();


    /// `void fireErrorAsync (in DOMRequest request, in AString error);`
    #[inline]
    pub unsafe fn FireErrorAsync(&self, request: *const libc::c_void, error: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FireErrorAsync)(self, request, error)
    }


}


