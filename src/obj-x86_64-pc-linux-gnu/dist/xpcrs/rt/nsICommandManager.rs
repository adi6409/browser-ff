//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsICommandManager.idl
//


/// `interface nsICommandManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandManager {
    vtable: *const nsICommandManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandManager.
unsafe impl XpCom for nsICommandManager {
    const IID: nsIID = nsID(0xbb5a1730, 0xd83b, 0x4fa2,
        [0x83, 0x1b, 0x35, 0xb9, 0xd5, 0x84, 0x2e, 0x84]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandManagerCoerce {
    /// Cheaply cast a value of this type from a `nsICommandManager`.
    fn coerce_from(v: &nsICommandManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandManagerCoerce for nsICommandManager {
    #[inline]
    fn coerce_from(v: &nsICommandManager) -> &Self {
        v
    }
}

impl nsICommandManager {
    /// Cast this `nsICommandManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandManager {
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
impl<T: nsISupportsCoerce> nsICommandManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve); */
    pub AddCommandObserver: unsafe extern "system" fn (this: *const nsICommandManager, aCommandObserver: *const nsIObserver, aCommandToObserve: *const libc::c_char) -> ::nserror::nsresult,

    /* void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved); */
    pub RemoveCommandObserver: unsafe extern "system" fn (this: *const nsICommandManager, aCommandObserver: *const nsIObserver, aCommandObserved: *const libc::c_char) -> ::nserror::nsresult,

    /* boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
    pub IsCommandSupported: unsafe extern "system" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
    pub IsCommandEnabled: unsafe extern "system" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, _retval: *mut bool) -> ::nserror::nsresult,

    /* void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams); */
    pub GetCommandState: unsafe extern "system" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, aCommandParams: *const nsICommandParams) -> ::nserror::nsresult,

    /* [can_run_script] void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow); */
    pub DoCommand: unsafe extern "system" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aCommandParams: *const nsICommandParams, aTargetWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandManager {


    /// `void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve);`
    #[inline]
    pub unsafe fn AddCommandObserver(&self, aCommandObserver: *const nsIObserver, aCommandToObserve: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).AddCommandObserver)(self, aCommandObserver, aCommandToObserve)
    }



    /// `void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved);`
    #[inline]
    pub unsafe fn RemoveCommandObserver(&self, aCommandObserver: *const nsIObserver, aCommandObserved: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCommandObserver)(self, aCommandObserver, aCommandObserved)
    }



    /// `boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow);`
    #[inline]
    pub unsafe fn IsCommandSupported(&self, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCommandSupported)(self, aCommandName, aTargetWindow, _retval)
    }



    /// `boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow);`
    #[inline]
    pub unsafe fn IsCommandEnabled(&self, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCommandEnabled)(self, aCommandName, aTargetWindow, _retval)
    }



    /// `void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams);`
    #[inline]
    pub unsafe fn GetCommandState(&self, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, aCommandParams: *const nsICommandParams) -> ::nserror::nsresult {
        ((*self.vtable).GetCommandState)(self, aCommandName, aTargetWindow, aCommandParams)
    }



    /// `[can_run_script] void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow);`
    #[inline]
    pub unsafe fn DoCommand(&self, aCommandName: *const libc::c_char, aCommandParams: *const nsICommandParams, aTargetWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).DoCommand)(self, aCommandName, aCommandParams, aTargetWindow)
    }


}


