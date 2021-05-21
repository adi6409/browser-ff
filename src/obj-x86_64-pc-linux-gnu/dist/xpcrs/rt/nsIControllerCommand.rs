//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerCommand.idl
//


/// `interface nsIControllerCommand : nsISupports`
///

/// ```text
/// /**
///  * nsIControllerCommand
///  *
///  * A generic command interface. You can register an nsIControllerCommand
///  * with the nsIControllerCommandTable.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIControllerCommand {
    vtable: *const nsIControllerCommandVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIControllerCommand.
unsafe impl XpCom for nsIControllerCommand {
    const IID: nsIID = nsID(0x0eae9a46, 0x1dd2, 0x11b2,
        [0xac, 0xa0, 0x91, 0x76, 0xf0, 0x5f, 0xe9, 0xdb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIControllerCommand {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIControllerCommand.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIControllerCommandCoerce {
    /// Cheaply cast a value of this type from a `nsIControllerCommand`.
    fn coerce_from(v: &nsIControllerCommand) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIControllerCommandCoerce for nsIControllerCommand {
    #[inline]
    fn coerce_from(v: &nsIControllerCommand) -> &Self {
        v
    }
}

impl nsIControllerCommand {
    /// Cast this `nsIControllerCommand` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIControllerCommandCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIControllerCommand {
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
impl<T: nsISupportsCoerce> nsIControllerCommandCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerCommand) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIControllerCommand
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIControllerCommandVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext); */
    pub IsCommandEnabled: unsafe extern "system" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aCommandContext: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
    pub GetCommandStateParams: unsafe extern "system" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aParams: *const nsICommandParams, aCommandContext: *const nsISupports) -> ::nserror::nsresult,

    /* [can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandContext); */
    pub DoCommand: unsafe extern "system" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aCommandContext: *const nsISupports) -> ::nserror::nsresult,

    /* [can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
    pub DoCommandParams: unsafe extern "system" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aParams: *const nsICommandParams, aCommandContext: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIControllerCommand {

    /// ```text
    /// /**
    ///    * Returns true if the command is currently enabled. An nsIControllerCommand
    ///    * can implement more than one commands; say, a group of related commands
    ///    * (e.g. delete left/delete right). Because of this, the command name is
    ///    * passed to each method.
    ///    *
    ///    * @param aCommandName  the name of the command for which we want the enabled
    ///    *                      state.
    ///    * @param aCommandContext    a cookie held by the nsIControllerCommandTable,
    ///    *                  allowing the command to get some context information.
    ///    *                  The contents of this cookie are implementation-defined.
    ///    */
    /// ```
    ///

    /// `boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext);`
    #[inline]
    pub unsafe fn IsCommandEnabled(&self, aCommandName: *const libc::c_char, aCommandContext: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCommandEnabled)(self, aCommandName, aCommandContext, _retval)
    }



    /// `void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext);`
    #[inline]
    pub unsafe fn GetCommandStateParams(&self, aCommandName: *const libc::c_char, aParams: *const nsICommandParams, aCommandContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetCommandStateParams)(self, aCommandName, aParams, aCommandContext)
    }


    /// ```text
    /// /**
    ///    * Execute the name command.
    ///    *
    ///    * @param aCommandName  the name of the command to execute.
    ///    *
    ///    * @param aCommandContext    a cookie held by the nsIControllerCommandTable,
    ///    *                  allowing the command to get some context information.
    ///    *                  The contents of this cookie are implementation-defined.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandContext);`
    #[inline]
    pub unsafe fn DoCommand(&self, aCommandName: *const libc::c_char, aCommandContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).DoCommand)(self, aCommandName, aCommandContext)
    }



    /// `[can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext);`
    #[inline]
    pub unsafe fn DoCommandParams(&self, aCommandName: *const libc::c_char, aParams: *const nsICommandParams, aCommandContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).DoCommandParams)(self, aCommandName, aParams, aCommandContext)
    }


}


