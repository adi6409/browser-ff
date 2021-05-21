//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISharePicker.idl
//


/// `interface nsISharePicker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISharePicker {
    vtable: *const nsISharePickerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISharePicker.
unsafe impl XpCom for nsISharePicker {
    const IID: nsIID = nsID(0x1201d357, 0x8417, 0x4926,
        [0xa6, 0x94, 0xe6, 0x40, 0x8f, 0xbe, 0xdc, 0xf8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISharePicker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISharePicker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISharePickerCoerce {
    /// Cheaply cast a value of this type from a `nsISharePicker`.
    fn coerce_from(v: &nsISharePicker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISharePickerCoerce for nsISharePicker {
    #[inline]
    fn coerce_from(v: &nsISharePicker) -> &Self {
        v
    }
}

impl nsISharePicker {
    /// Cast this `nsISharePicker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISharePickerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISharePicker {
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
impl<T: nsISupportsCoerce> nsISharePickerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISharePicker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISharePicker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISharePickerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy openerWindow); */
    pub Init: unsafe extern "system" fn (this: *const nsISharePicker, openerWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy openerWindow; */
    pub GetOpenerWindow: unsafe extern "system" fn (this: *const nsISharePicker, aOpenerWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* Promise share (in AUTF8String title, in AUTF8String text, in nsIURI url); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub Share: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISharePicker {

    /// ```text
    /// /**
    ///    * Initialize the share picker widget.
    ///    * @param nsIDOMWindow openerWindow.
    ///    */
    /// ```
    ///

    /// `void init (in mozIDOMWindowProxy openerWindow);`
    #[inline]
    pub unsafe fn Init(&self, openerWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, openerWindow)
    }


    /// ```text
    /// /**
    ///    * Returns the parent window this was initialized with.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy openerWindow;`
    #[inline]
    pub unsafe fn GetOpenerWindow(&self, aOpenerWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetOpenerWindow)(self, aOpenerWindow)
    }


    /// ```text
    /// /**
    ///    * XPCOM Analog of navigator.share() as per:
    ///    * https://w3c.github.io/web-share/#share-method
    ///    */
    /// ```
    ///

    /// `Promise share (in AUTF8String title, in AUTF8String text, in nsIURI url);`
    const _Share: () = ();

}


