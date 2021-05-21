//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIControllers.idl
//


/// `interface nsIControllers : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIControllers {
    vtable: *const nsIControllersVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIControllers.
unsafe impl XpCom for nsIControllers {
    const IID: nsIID = nsID(0xf36e3ec1, 0x9197, 0x4ad8,
        [0x8d, 0x4c, 0xd3, 0xb1, 0x92, 0x7f, 0xd6, 0xdf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIControllers {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIControllers.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIControllersCoerce {
    /// Cheaply cast a value of this type from a `nsIControllers`.
    fn coerce_from(v: &nsIControllers) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIControllersCoerce for nsIControllers {
    #[inline]
    fn coerce_from(v: &nsIControllers) -> &Self {
        v
    }
}

impl nsIControllers {
    /// Cast this `nsIControllers` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIControllersCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIControllers {
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
impl<T: nsISupportsCoerce> nsIControllersCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllers) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIControllers
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIControllersVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIController getControllerForCommand (in string command); */
    pub GetControllerForCommand: unsafe extern "system" fn (this: *const nsIControllers, command: *const libc::c_char, _retval: *mut*const nsIController) -> ::nserror::nsresult,

    /* void insertControllerAt (in unsigned long index, in nsIController controller); */
    pub InsertControllerAt: unsafe extern "system" fn (this: *const nsIControllers, index: u32, controller: *const nsIController) -> ::nserror::nsresult,

    /* nsIController removeControllerAt (in unsigned long index); */
    pub RemoveControllerAt: unsafe extern "system" fn (this: *const nsIControllers, index: u32, _retval: *mut*const nsIController) -> ::nserror::nsresult,

    /* nsIController getControllerAt (in unsigned long index); */
    pub GetControllerAt: unsafe extern "system" fn (this: *const nsIControllers, index: u32, _retval: *mut*const nsIController) -> ::nserror::nsresult,

    /* void appendController (in nsIController controller); */
    pub AppendController: unsafe extern "system" fn (this: *const nsIControllers, controller: *const nsIController) -> ::nserror::nsresult,

    /* void removeController (in nsIController controller); */
    pub RemoveController: unsafe extern "system" fn (this: *const nsIControllers, controller: *const nsIController) -> ::nserror::nsresult,

    /* unsigned long getControllerId (in nsIController controller); */
    pub GetControllerId: unsafe extern "system" fn (this: *const nsIControllers, controller: *const nsIController, _retval: *mut u32) -> ::nserror::nsresult,

    /* nsIController getControllerById (in unsigned long controllerID); */
    pub GetControllerById: unsafe extern "system" fn (this: *const nsIControllers, controllerID: u32, _retval: *mut*const nsIController) -> ::nserror::nsresult,

    /* unsigned long getControllerCount (); */
    pub GetControllerCount: unsafe extern "system" fn (this: *const nsIControllers, _retval: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIControllers {


    /// `nsIController getControllerForCommand (in string command);`
    #[inline]
    pub unsafe fn GetControllerForCommand(&self, command: *const libc::c_char, _retval: *mut*const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).GetControllerForCommand)(self, command, _retval)
    }



    /// `void insertControllerAt (in unsigned long index, in nsIController controller);`
    #[inline]
    pub unsafe fn InsertControllerAt(&self, index: u32, controller: *const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).InsertControllerAt)(self, index, controller)
    }



    /// `nsIController removeControllerAt (in unsigned long index);`
    #[inline]
    pub unsafe fn RemoveControllerAt(&self, index: u32, _retval: *mut*const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).RemoveControllerAt)(self, index, _retval)
    }



    /// `nsIController getControllerAt (in unsigned long index);`
    #[inline]
    pub unsafe fn GetControllerAt(&self, index: u32, _retval: *mut*const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).GetControllerAt)(self, index, _retval)
    }



    /// `void appendController (in nsIController controller);`
    #[inline]
    pub unsafe fn AppendController(&self, controller: *const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).AppendController)(self, controller)
    }



    /// `void removeController (in nsIController controller);`
    #[inline]
    pub unsafe fn RemoveController(&self, controller: *const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).RemoveController)(self, controller)
    }



    /// `unsigned long getControllerId (in nsIController controller);`
    #[inline]
    pub unsafe fn GetControllerId(&self, controller: *const nsIController, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetControllerId)(self, controller, _retval)
    }



    /// `nsIController getControllerById (in unsigned long controllerID);`
    #[inline]
    pub unsafe fn GetControllerById(&self, controllerID: u32, _retval: *mut*const nsIController) -> ::nserror::nsresult {
        ((*self.vtable).GetControllerById)(self, controllerID, _retval)
    }



    /// `unsigned long getControllerCount ();`
    #[inline]
    pub unsafe fn GetControllerCount(&self, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetControllerCount)(self, _retval)
    }


}


