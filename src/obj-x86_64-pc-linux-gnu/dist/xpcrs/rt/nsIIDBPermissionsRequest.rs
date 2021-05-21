//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/indexedDB/nsIIDBPermissionsRequest.idl
//


/// `interface nsIIDBPermissionsRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIDBPermissionsRequest {
    vtable: *const nsIIDBPermissionsRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIDBPermissionsRequest.
unsafe impl XpCom for nsIIDBPermissionsRequest {
    const IID: nsIID = nsID(0xc3493c65, 0x0530, 0x496e,
        [0x99, 0x5c, 0xbc, 0xd3, 0x8d, 0xbf, 0xce, 0x21]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIDBPermissionsRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIDBPermissionsRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIDBPermissionsRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIIDBPermissionsRequest`.
    fn coerce_from(v: &nsIIDBPermissionsRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIDBPermissionsRequestCoerce for nsIIDBPermissionsRequest {
    #[inline]
    fn coerce_from(v: &nsIIDBPermissionsRequest) -> &Self {
        v
    }
}

impl nsIIDBPermissionsRequest {
    /// Cast this `nsIIDBPermissionsRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIDBPermissionsRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIDBPermissionsRequest {
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
impl<T: nsISupportsCoerce> nsIIDBPermissionsRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIDBPermissionsRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIDBPermissionsRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIDBPermissionsRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Element browserElement; */
    pub GetBrowserElement: unsafe extern "system" fn (this: *const nsIIDBPermissionsRequest, aBrowserElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute nsIObserver responseObserver; */
    pub GetResponseObserver: unsafe extern "system" fn (this: *const nsIIDBPermissionsRequest, aResponseObserver: *mut*const nsIObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIDBPermissionsRequest {


    /// `readonly attribute Element browserElement;`
    #[inline]
    pub unsafe fn GetBrowserElement(&self, aBrowserElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowserElement)(self, aBrowserElement)
    }



    /// `readonly attribute nsIObserver responseObserver;`
    #[inline]
    pub unsafe fn GetResponseObserver(&self, aResponseObserver: *mut*const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseObserver)(self, aResponseObserver)
    }


}


