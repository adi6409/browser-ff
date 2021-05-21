//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIController.idl
//


/// `interface nsIController : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIController {
    vtable: *const nsIControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIController.
unsafe impl XpCom for nsIController {
    const IID: nsIID = nsID(0xd5b61b82, 0x1da4, 0x11d3,
        [0xbf, 0x87, 0x00, 0x10, 0x5a, 0x1b, 0x06, 0x27]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIController {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIController.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIControllerCoerce {
    /// Cheaply cast a value of this type from a `nsIController`.
    fn coerce_from(v: &nsIController) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIControllerCoerce for nsIController {
    #[inline]
    fn coerce_from(v: &nsIController) -> &Self {
        v
    }
}

impl nsIController {
    /// Cast this `nsIController` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIController {
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
impl<T: nsISupportsCoerce> nsIControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIController) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIController
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIControllerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean isCommandEnabled (in string command); */
    pub IsCommandEnabled: unsafe extern "system" fn (this: *const nsIController, command: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean supportsCommand (in string command); */
    pub SupportsCommand: unsafe extern "system" fn (this: *const nsIController, command: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void doCommand (in string command); */
    pub DoCommand: unsafe extern "system" fn (this: *const nsIController, command: *const libc::c_char) -> ::nserror::nsresult,

    /* void onEvent (in string eventName); */
    pub OnEvent: unsafe extern "system" fn (this: *const nsIController, eventName: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIController {


    /// `boolean isCommandEnabled (in string command);`
    #[inline]
    pub unsafe fn IsCommandEnabled(&self, command: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCommandEnabled)(self, command, _retval)
    }



    /// `boolean supportsCommand (in string command);`
    #[inline]
    pub unsafe fn SupportsCommand(&self, command: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SupportsCommand)(self, command, _retval)
    }



    /// `[can_run_script] void doCommand (in string command);`
    #[inline]
    pub unsafe fn DoCommand(&self, command: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).DoCommand)(self, command)
    }



    /// `void onEvent (in string eventName);`
    #[inline]
    pub unsafe fn OnEvent(&self, eventName: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).OnEvent)(self, eventName)
    }


}


/// `interface nsICommandController : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandController {
    vtable: *const nsICommandControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandController.
unsafe impl XpCom for nsICommandController {
    const IID: nsIID = nsID(0xeec0b435, 0x7f53, 0x44fe,
        [0xb0, 0x0a, 0xcf, 0x3e, 0xed, 0x65, 0xc0, 0x1a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandController {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandController.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandControllerCoerce {
    /// Cheaply cast a value of this type from a `nsICommandController`.
    fn coerce_from(v: &nsICommandController) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandControllerCoerce for nsICommandController {
    #[inline]
    fn coerce_from(v: &nsICommandController) -> &Self {
        v
    }
}

impl nsICommandController {
    /// Cast this `nsICommandController` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandController {
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
impl<T: nsISupportsCoerce> nsICommandControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandController) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandController
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandControllerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getCommandStateWithParams (in string command, in nsICommandParams aCommandParams); */
    pub GetCommandStateWithParams: unsafe extern "system" fn (this: *const nsICommandController, command: *const libc::c_char, aCommandParams: *const nsICommandParams) -> ::nserror::nsresult,

    /* [can_run_script] void doCommandWithParams (in string command, in nsICommandParams aCommandParams); */
    pub DoCommandWithParams: unsafe extern "system" fn (this: *const nsICommandController, command: *const libc::c_char, aCommandParams: *const nsICommandParams) -> ::nserror::nsresult,

    /* Array<ACString> getSupportedCommands (); */
    pub GetSupportedCommands: unsafe extern "system" fn (this: *const nsICommandController, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandController {


    /// `void getCommandStateWithParams (in string command, in nsICommandParams aCommandParams);`
    #[inline]
    pub unsafe fn GetCommandStateWithParams(&self, command: *const libc::c_char, aCommandParams: *const nsICommandParams) -> ::nserror::nsresult {
        ((*self.vtable).GetCommandStateWithParams)(self, command, aCommandParams)
    }



    /// `[can_run_script] void doCommandWithParams (in string command, in nsICommandParams aCommandParams);`
    #[inline]
    pub unsafe fn DoCommandWithParams(&self, command: *const libc::c_char, aCommandParams: *const nsICommandParams) -> ::nserror::nsresult {
        ((*self.vtable).DoCommandWithParams)(self, command, aCommandParams)
    }



    /// `Array<ACString> getSupportedCommands ();`
    #[inline]
    pub unsafe fn GetSupportedCommands(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportedCommands)(self, _retval)
    }


}


