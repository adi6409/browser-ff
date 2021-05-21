//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIPermissionDelegateHandler.idl
//


/// `interface nsIPermissionDelegateHandler : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPermissionDelegateHandler {
    vtable: *const nsIPermissionDelegateHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPermissionDelegateHandler.
unsafe impl XpCom for nsIPermissionDelegateHandler {
    const IID: nsIID = nsID(0x07611dc6, 0xbf4d, 0x4d8a,
        [0xa6, 0x4b, 0xf3, 0xa5, 0x90, 0x4d, 0xdd, 0xc7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPermissionDelegateHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPermissionDelegateHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPermissionDelegateHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIPermissionDelegateHandler`.
    fn coerce_from(v: &nsIPermissionDelegateHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPermissionDelegateHandlerCoerce for nsIPermissionDelegateHandler {
    #[inline]
    fn coerce_from(v: &nsIPermissionDelegateHandler) -> &Self {
        v
    }
}

impl nsIPermissionDelegateHandler {
    /// Cast this `nsIPermissionDelegateHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPermissionDelegateHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPermissionDelegateHandler {
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
impl<T: nsISupportsCoerce> nsIPermissionDelegateHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPermissionDelegateHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPermissionDelegateHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPermissionDelegateHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean maybeUnsafePermissionDelegate (in Array<ACString> aTypes); */
    pub MaybeUnsafePermissionDelegate: unsafe extern "system" fn (this: *const nsIPermissionDelegateHandler, aTypes: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean permissionDelegateFPEnabled; */
    pub GetPermissionDelegateFPEnabled: unsafe extern "system" fn (this: *const nsIPermissionDelegateHandler, aPermissionDelegateFPEnabled: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPermissionDelegateHandler {


    /// `boolean maybeUnsafePermissionDelegate (in Array<ACString> aTypes);`
    #[inline]
    pub unsafe fn MaybeUnsafePermissionDelegate(&self, aTypes: *const thin_vec::ThinVec<::nsstring::nsCString>, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).MaybeUnsafePermissionDelegate)(self, aTypes, _retval)
    }



    /// `readonly attribute boolean permissionDelegateFPEnabled;`
    #[inline]
    pub unsafe fn GetPermissionDelegateFPEnabled(&self, aPermissionDelegateFPEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissionDelegateFPEnabled)(self, aPermissionDelegateFPEnabled)
    }


}


