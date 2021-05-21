//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIFormFillController.idl
//


/// `interface nsIFormFillController : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFormFillController {
    vtable: *const nsIFormFillControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFormFillController.
unsafe impl XpCom for nsIFormFillController {
    const IID: nsIID = nsID(0x07f0a0dc, 0xf6e9, 0x4cdd,
        [0xa5, 0x5f, 0x56, 0xd7, 0x70, 0x52, 0x3a, 0x4c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFormFillController {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFormFillController.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFormFillControllerCoerce {
    /// Cheaply cast a value of this type from a `nsIFormFillController`.
    fn coerce_from(v: &nsIFormFillController) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFormFillControllerCoerce for nsIFormFillController {
    #[inline]
    fn coerce_from(v: &nsIFormFillController) -> &Self {
        v
    }
}

impl nsIFormFillController {
    /// Cast this `nsIFormFillController` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFormFillControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFormFillController {
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
impl<T: nsISupportsCoerce> nsIFormFillControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormFillController) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFormFillController
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFormFillControllerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute HTMLInputElement focusedInput; */
    pub GetFocusedInput: unsafe extern "system" fn (this: *const nsIFormFillController, aFocusedInput: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute boolean passwordPopupAutomaticallyOpened; */
    pub GetPasswordPopupAutomaticallyOpened: unsafe extern "system" fn (this: *const nsIFormFillController, aPasswordPopupAutomaticallyOpened: *mut bool) -> ::nserror::nsresult,

    /* void attachPopupElementToDocument (in Document document, in Element popup); */
    pub AttachPopupElementToDocument: unsafe extern "system" fn (this: *const nsIFormFillController, document: *const libc::c_void, popup: *const libc::c_void) -> ::nserror::nsresult,

    /* void detachFromDocument (in Document document); */
    pub DetachFromDocument: unsafe extern "system" fn (this: *const nsIFormFillController, document: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void markAsLoginManagerField (in HTMLInputElement aInput); */
    pub MarkAsLoginManagerField: unsafe extern "system" fn (this: *const nsIFormFillController, aInput: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void markAsAutofillField (in HTMLInputElement aInput); */
    pub MarkAsAutofillField: unsafe extern "system" fn (this: *const nsIFormFillController, aInput: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void showPopup (); */
    pub ShowPopup: unsafe extern "system" fn (this: *const nsIFormFillController) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFormFillController {


    /// `readonly attribute HTMLInputElement focusedInput;`
    #[inline]
    pub unsafe fn GetFocusedInput(&self, aFocusedInput: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedInput)(self, aFocusedInput)
    }



    /// `readonly attribute boolean passwordPopupAutomaticallyOpened;`
    #[inline]
    pub unsafe fn GetPasswordPopupAutomaticallyOpened(&self, aPasswordPopupAutomaticallyOpened: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPasswordPopupAutomaticallyOpened)(self, aPasswordPopupAutomaticallyOpened)
    }



    /// `void attachPopupElementToDocument (in Document document, in Element popup);`
    #[inline]
    pub unsafe fn AttachPopupElementToDocument(&self, document: *const libc::c_void, popup: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).AttachPopupElementToDocument)(self, document, popup)
    }



    /// `void detachFromDocument (in Document document);`
    #[inline]
    pub unsafe fn DetachFromDocument(&self, document: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).DetachFromDocument)(self, document)
    }



    /// `[can_run_script] void markAsLoginManagerField (in HTMLInputElement aInput);`
    #[inline]
    pub unsafe fn MarkAsLoginManagerField(&self, aInput: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).MarkAsLoginManagerField)(self, aInput)
    }



    /// `[can_run_script] void markAsAutofillField (in HTMLInputElement aInput);`
    #[inline]
    pub unsafe fn MarkAsAutofillField(&self, aInput: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).MarkAsAutofillField)(self, aInput)
    }



    /// `[can_run_script] void showPopup ();`
    #[inline]
    pub unsafe fn ShowPopup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ShowPopup)(self, )
    }


}


