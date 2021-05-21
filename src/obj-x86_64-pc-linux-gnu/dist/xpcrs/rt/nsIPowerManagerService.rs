//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIPowerManagerService.idl
//


/// `interface nsIPowerManagerService : nsISupports`
///

/// ```text
/// /**
///  * For use with non-content code.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPowerManagerService {
    vtable: *const nsIPowerManagerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPowerManagerService.
unsafe impl XpCom for nsIPowerManagerService {
    const IID: nsIID = nsID(0xba7ca4c1, 0x9d92, 0x4425,
        [0xa8, 0x3b, 0x85, 0xdd, 0x7f, 0xa9, 0x53, 0xf7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPowerManagerService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPowerManagerService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPowerManagerServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPowerManagerService`.
    fn coerce_from(v: &nsIPowerManagerService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPowerManagerServiceCoerce for nsIPowerManagerService {
    #[inline]
    fn coerce_from(v: &nsIPowerManagerService) -> &Self {
        v
    }
}

impl nsIPowerManagerService {
    /// Cast this `nsIPowerManagerService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPowerManagerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPowerManagerService {
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
impl<T: nsISupportsCoerce> nsIPowerManagerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPowerManagerService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPowerManagerService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPowerManagerServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
    pub AddWakeLockListener: unsafe extern "system" fn (this: *const nsIPowerManagerService, aListener: *const nsIDOMMozWakeLockListener) -> ::nserror::nsresult,

    /* void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
    pub RemoveWakeLockListener: unsafe extern "system" fn (this: *const nsIPowerManagerService, aListener: *const nsIDOMMozWakeLockListener) -> ::nserror::nsresult,

    /* AString getWakeLockState (in AString aTopic); */
    pub GetWakeLockState: unsafe extern "system" fn (this: *const nsIPowerManagerService, aTopic: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIWakeLock newWakeLock (in AString aTopic, [optional] in mozIDOMWindow aWindow); */
    pub NewWakeLock: unsafe extern "system" fn (this: *const nsIPowerManagerService, aTopic: *const ::nsstring::nsAString, aWindow: *const mozIDOMWindow, _retval: *mut*const nsIWakeLock) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPowerManagerService {


    /// `void addWakeLockListener (in nsIDOMMozWakeLockListener aListener);`
    #[inline]
    pub unsafe fn AddWakeLockListener(&self, aListener: *const nsIDOMMozWakeLockListener) -> ::nserror::nsresult {
        ((*self.vtable).AddWakeLockListener)(self, aListener)
    }



    /// `void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener);`
    #[inline]
    pub unsafe fn RemoveWakeLockListener(&self, aListener: *const nsIDOMMozWakeLockListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWakeLockListener)(self, aListener)
    }



    /// `AString getWakeLockState (in AString aTopic);`
    #[inline]
    pub unsafe fn GetWakeLockState(&self, aTopic: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetWakeLockState)(self, aTopic, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a wake lock (MozWakeLock) object of aTopic associated with aWindow.
    ///    * A wake lock without associated window, e.g. used in chrome, is
    ///    * always considered invisible.
    ///    */
    /// ```
    ///

    /// `nsIWakeLock newWakeLock (in AString aTopic, [optional] in mozIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn NewWakeLock(&self, aTopic: *const ::nsstring::nsAString, aWindow: *const mozIDOMWindow, _retval: *mut*const nsIWakeLock) -> ::nserror::nsresult {
        ((*self.vtable).NewWakeLock)(self, aTopic, aWindow, _retval)
    }


}


