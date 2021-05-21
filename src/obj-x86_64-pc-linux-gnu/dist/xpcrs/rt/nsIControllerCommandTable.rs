//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerCommandTable.idl
//


/// `interface nsIControllerCommandTable : nsISupports`
///

/// ```text
/// /**
///  * nsIControllerCommandTable
///  *
///  * An interface via which a controller can maintain a series of commands,
///  * and efficiently dispatch commands to their respective handlers.
///  *
///  * Controllers that use an nsIControllerCommandTable should support
///  * nsIInterfaceRequestor, and be able to return an interface to their
///  * controller command table via getInterface().
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIControllerCommandTable {
    vtable: *const nsIControllerCommandTableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIControllerCommandTable.
unsafe impl XpCom for nsIControllerCommandTable {
    const IID: nsIID = nsID(0xc847f90e, 0xb8f3, 0x49db,
        [0xa4, 0xdf, 0x88, 0x67, 0x83, 0x1f, 0x28, 0x00]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIControllerCommandTable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIControllerCommandTable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIControllerCommandTableCoerce {
    /// Cheaply cast a value of this type from a `nsIControllerCommandTable`.
    fn coerce_from(v: &nsIControllerCommandTable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIControllerCommandTableCoerce for nsIControllerCommandTable {
    #[inline]
    fn coerce_from(v: &nsIControllerCommandTable) -> &Self {
        v
    }
}

impl nsIControllerCommandTable {
    /// Cast this `nsIControllerCommandTable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIControllerCommandTableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIControllerCommandTable {
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
impl<T: nsISupportsCoerce> nsIControllerCommandTableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerCommandTable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIControllerCommandTable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIControllerCommandTableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void makeImmutable (); */
    pub MakeImmutable: unsafe extern "system" fn (this: *const nsIControllerCommandTable) -> ::nserror::nsresult,

    /* void registerCommand (in string aCommandName, in nsIControllerCommand aCommand); */
    pub RegisterCommand: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommand: *const nsIControllerCommand) -> ::nserror::nsresult,

    /* void unregisterCommand (in string aCommandName, in nsIControllerCommand aCommand); */
    pub UnregisterCommand: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommand: *const nsIControllerCommand) -> ::nserror::nsresult,

    /* nsIControllerCommand findCommandHandler (in string aCommandName); */
    pub FindCommandHandler: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, _retval: *mut *const nsIControllerCommand) -> ::nserror::nsresult,

    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandRefCon); */
    pub IsCommandEnabled: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* void updateCommandState (in string aCommandName, in nsISupports aCommandRefCon); */
    pub UpdateCommandState: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult,

    /* boolean supportsCommand (in string aCommandName, in nsISupports aCommandRefCon); */
    pub SupportsCommand: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandRefCon); */
    pub DoCommand: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult,

    /* [can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
    pub DoCommandParams: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aParam: *const nsICommandParams, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult,

    /* void getCommandState (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
    pub GetCommandState: unsafe extern "system" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aParam: *const nsICommandParams, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult,

    /* Array<ACString> getSupportedCommands (); */
    pub GetSupportedCommands: unsafe extern "system" fn (this: *const nsIControllerCommandTable, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIControllerCommandTable {

    /// ```text
    /// /**
    ///    * Make this command table immutable, so that commands cannot
    ///    * be registered or unregistered. Some command tables are made
    ///    * mutable after command registration so that they can be
    ///    * used as singletons.
    ///    */
    /// ```
    ///

    /// `void makeImmutable ();`
    #[inline]
    pub unsafe fn MakeImmutable(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MakeImmutable)(self, )
    }


    /// ```text
    /// /**
    ///    * Register and unregister commands with the command table.
    ///    *
    ///    * @param aCommandName  the name of the command under which to register or
    ///    *                      unregister the given command handler.
    ///    *
    ///    * @param aCommand      the handler for this command.
    ///    */
    /// ```
    ///

    /// `void registerCommand (in string aCommandName, in nsIControllerCommand aCommand);`
    #[inline]
    pub unsafe fn RegisterCommand(&self, aCommandName: *const libc::c_char, aCommand: *const nsIControllerCommand) -> ::nserror::nsresult {
        ((*self.vtable).RegisterCommand)(self, aCommandName, aCommand)
    }



    /// `void unregisterCommand (in string aCommandName, in nsIControllerCommand aCommand);`
    #[inline]
    pub unsafe fn UnregisterCommand(&self, aCommandName: *const libc::c_char, aCommand: *const nsIControllerCommand) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterCommand)(self, aCommandName, aCommand)
    }


    /// ```text
    /// /**
    ///    * Find the command handler which has been registered to handle the named command.
    ///    *
    ///    * @param aCommandName  the name of the command to find the handler for.
    ///    */
    /// ```
    ///

    /// `nsIControllerCommand findCommandHandler (in string aCommandName);`
    #[inline]
    pub unsafe fn FindCommandHandler(&self, aCommandName: *const libc::c_char, _retval: *mut *const nsIControllerCommand) -> ::nserror::nsresult {
        ((*self.vtable).FindCommandHandler)(self, aCommandName, _retval)
    }


    /// ```text
    /// /**
    ///    * Get whether the named command is enabled.
    ///    *
    ///    * @param aCommandName    the name of the command to test
    ///    * @param aCommandRefCon  the command context data
    ///    */
    /// ```
    ///

    /// `boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandRefCon);`
    #[inline]
    pub unsafe fn IsCommandEnabled(&self, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCommandEnabled)(self, aCommandName, aCommandRefCon, _retval)
    }


    /// ```text
    /// /**
    ///    * Tell the command to update its state (if it is a state updating command)
    ///    *
    ///    * @param aCommandName    the name of the command to update
    ///    * @param aCommandRefCon  the command context data
    ///    */
    /// ```
    ///

    /// `void updateCommandState (in string aCommandName, in nsISupports aCommandRefCon);`
    #[inline]
    pub unsafe fn UpdateCommandState(&self, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).UpdateCommandState)(self, aCommandName, aCommandRefCon)
    }


    /// ```text
    /// /**
    ///    * Get whether the named command is supported.
    ///    *
    ///    * @param aCommandName    the name of the command to test
    ///    * @param aCommandRefCon  the command context data
    ///    */
    /// ```
    ///

    /// `boolean supportsCommand (in string aCommandName, in nsISupports aCommandRefCon);`
    #[inline]
    pub unsafe fn SupportsCommand(&self, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SupportsCommand)(self, aCommandName, aCommandRefCon, _retval)
    }


    /// ```text
    /// /**
    ///    * Execute the named command.
    ///    *
    ///    * @param aCommandName    the name of the command to execute
    ///    * @param aCommandRefCon  the command context data
    ///    */
    /// ```
    ///

    /// `[can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandRefCon);`
    #[inline]
    pub unsafe fn DoCommand(&self, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).DoCommand)(self, aCommandName, aCommandRefCon)
    }



    /// `[can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon);`
    #[inline]
    pub unsafe fn DoCommandParams(&self, aCommandName: *const libc::c_char, aParam: *const nsICommandParams, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).DoCommandParams)(self, aCommandName, aParam, aCommandRefCon)
    }



    /// `void getCommandState (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon);`
    #[inline]
    pub unsafe fn GetCommandState(&self, aCommandName: *const libc::c_char, aParam: *const nsICommandParams, aCommandRefCon: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetCommandState)(self, aCommandName, aParam, aCommandRefCon)
    }



    /// `Array<ACString> getSupportedCommands ();`
    #[inline]
    pub unsafe fn GetSupportedCommands(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportedCommands)(self, _retval)
    }


}


