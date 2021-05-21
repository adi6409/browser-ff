//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULCommandDispatcher.idl
//


/// `interface nsIDOMXULCommandDispatcher : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULCommandDispatcher {
    vtable: *const nsIDOMXULCommandDispatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULCommandDispatcher.
unsafe impl XpCom for nsIDOMXULCommandDispatcher {
    const IID: nsIID = nsID(0xa9fa9fd3, 0x8d62, 0x4f94,
        [0x9e, 0xd8, 0x3e, 0xa9, 0xc3, 0xcf, 0x07, 0x73]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULCommandDispatcher {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULCommandDispatcher.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULCommandDispatcherCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULCommandDispatcher`.
    fn coerce_from(v: &nsIDOMXULCommandDispatcher) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULCommandDispatcherCoerce for nsIDOMXULCommandDispatcher {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCommandDispatcher) -> &Self {
        v
    }
}

impl nsIDOMXULCommandDispatcher {
    /// Cast this `nsIDOMXULCommandDispatcher` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULCommandDispatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULCommandDispatcher {
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
impl<T: nsISupportsCoerce> nsIDOMXULCommandDispatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCommandDispatcher) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULCommandDispatcher
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULCommandDispatcherVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [setter_can_run_script] attribute Element focusedElement; */
    pub GetFocusedElement: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute Element focusedElement; */
    pub SetFocusedElement: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedElement: *const libc::c_void) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute mozIDOMWindowProxy focusedWindow; */
    pub GetFocusedWindow: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute mozIDOMWindowProxy focusedWindow; */
    pub SetFocusedWindow: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* void addCommandUpdater (in Element updater, in AString events, in AString targets); */
    pub AddCommandUpdater: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, updater: *const libc::c_void, events: *const ::nsstring::nsAString, targets: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeCommandUpdater (in Element updater); */
    pub RemoveCommandUpdater: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, updater: *const libc::c_void) -> ::nserror::nsresult,

    /* void updateCommands (in AString eventName); */
    pub UpdateCommands: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, eventName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIController getControllerForCommand (in string command); */
    pub GetControllerForCommand: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, command: *const libc::c_char, _retval: *mut*const nsIController) -> ::nserror::nsresult,

    /* nsIControllers getControllers (); */
    pub GetControllers: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, _retval: *mut*const nsIControllers) -> ::nserror::nsresult,

    /* void advanceFocus (); */
    pub AdvanceFocus: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher) -> ::nserror::nsresult,

    /* void rewindFocus (); */
    pub RewindFocus: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher) -> ::nserror::nsresult,

    /* void advanceFocusIntoSubtree (in Element elt); */
    pub AdvanceFocusIntoSubtree: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher, elt: *const libc::c_void) -> ::nserror::nsresult,

    /* void lock (); */
    pub Lock: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher) -> ::nserror::nsresult,

    /* void unlock (); */
    pub Unlock: unsafe extern "system" fn (this: *const nsIDOMXULCommandDispatcher) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULCommandDispatcher {


    /// `[setter_can_run_script] attribute Element focusedElement;`
    #[inline]
    pub unsafe fn GetFocusedElement(&self, aFocusedElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedElement)(self, aFocusedElement)
    }



    /// `[setter_can_run_script] attribute Element focusedElement;`
    #[inline]
    pub unsafe fn SetFocusedElement(&self, aFocusedElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetFocusedElement)(self, aFocusedElement)
    }



    /// `[setter_can_run_script] attribute mozIDOMWindowProxy focusedWindow;`
    #[inline]
    pub unsafe fn GetFocusedWindow(&self, aFocusedWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedWindow)(self, aFocusedWindow)
    }



    /// `[setter_can_run_script] attribute mozIDOMWindowProxy focusedWindow;`
    #[inline]
    pub unsafe fn SetFocusedWindow(&self, aFocusedWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).SetFocusedWindow)(self, aFocusedWindow)
    }



    /// `void addCommandUpdater (in Element updater, in AString events, in AString targets);`
    #[inline]
    pub unsafe fn AddCommandUpdater(&self, updater: *const libc::c_void, events: *const ::nsstring::nsAString, targets: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddCommandUpdater)(self, updater, events, targets)
    }



    /// `void removeCommandUpdater (in Element updater);`
    #[inline]
    pub unsafe fn RemoveCommandUpdater(&self, updater: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCommandUpdater)(self, updater)
    }



    /// `void updateCommands (in AString eventName);`
    #[inline]
    pub unsafe fn UpdateCommands(&self, eventName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UpdateCommands)(self, eventName)
    }



    /// `nsIController getControllerForCommand (in string command);`
    #[inline]
    pub unsafe fn GetControllerForCommand(&self, command: *const libc::c_char, _retval: *mut*const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).GetControllerForCommand)(self, command, _retval)
    }



    /// `nsIControllers getControllers ();`
    #[inline]
    pub unsafe fn GetControllers(&self, _retval: *mut*const nsIControllers) -> ::nserror::nsresult {
        ((*self.vtable).GetControllers)(self, _retval)
    }



    /// `void advanceFocus ();`
    #[inline]
    pub unsafe fn AdvanceFocus(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AdvanceFocus)(self, )
    }



    /// `void rewindFocus ();`
    #[inline]
    pub unsafe fn RewindFocus(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RewindFocus)(self, )
    }



    /// `void advanceFocusIntoSubtree (in Element elt);`
    #[inline]
    pub unsafe fn AdvanceFocusIntoSubtree(&self, elt: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).AdvanceFocusIntoSubtree)(self, elt)
    }



    /// `void lock ();`
    #[inline]
    pub unsafe fn Lock(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Lock)(self, )
    }



    /// `void unlock ();`
    #[inline]
    pub unsafe fn Unlock(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Unlock)(self, )
    }


}


