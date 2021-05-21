//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULMenuListElement.idl
//


/// `interface nsIDOMXULMenuListElement : nsIDOMXULSelectControlElement`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULMenuListElement {
    vtable: *const nsIDOMXULMenuListElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULMenuListElement.
unsafe impl XpCom for nsIDOMXULMenuListElement {
    const IID: nsIID = nsID(0x36c16a17, 0xc0e9, 0x4b35,
        [0x95, 0x1b, 0x81, 0xa1, 0x47, 0x31, 0x4e, 0xf1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULMenuListElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULMenuListElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULMenuListElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULMenuListElement`.
    fn coerce_from(v: &nsIDOMXULMenuListElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULMenuListElementCoerce for nsIDOMXULMenuListElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMenuListElement) -> &Self {
        v
    }
}

impl nsIDOMXULMenuListElement {
    /// Cast this `nsIDOMXULMenuListElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULMenuListElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULMenuListElement {
    type Target = nsIDOMXULSelectControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULSelectControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDOMXULSelectControlElementCoerce> nsIDOMXULMenuListElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMenuListElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULMenuListElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULMenuListElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMXULSelectControlElementVTable,

    /* attribute boolean editable; */
    pub GetEditable: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aEditable: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean editable; */
    pub SetEditable: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aEditable: bool) -> ::nserror::nsresult,

    /* attribute boolean open; */
    pub GetOpen: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aOpen: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean open; */
    pub SetOpen: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aOpen: bool) -> ::nserror::nsresult,

    /* readonly attribute AString label; */
    pub GetLabel: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString crop; */
    pub GetCrop: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aCrop: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString crop; */
    pub SetCrop: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aCrop: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString image; */
    pub GetImage: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aImage: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString image; */
    pub SetImage: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aImage: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute Element inputField; */
    pub GetInputField: unsafe extern "system" fn (this: *const nsIDOMXULMenuListElement, aInputField: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULMenuListElement {


    /// `attribute boolean editable;`
    #[inline]
    pub unsafe fn GetEditable(&self, aEditable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEditable)(self, aEditable)
    }



    /// `attribute boolean editable;`
    #[inline]
    pub unsafe fn SetEditable(&self, aEditable: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEditable)(self, aEditable)
    }



    /// `attribute boolean open;`
    #[inline]
    pub unsafe fn GetOpen(&self, aOpen: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOpen)(self, aOpen)
    }



    /// `attribute boolean open;`
    #[inline]
    pub unsafe fn SetOpen(&self, aOpen: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetOpen)(self, aOpen)
    }



    /// `readonly attribute AString label;`
    #[inline]
    pub unsafe fn GetLabel(&self, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLabel)(self, aLabel)
    }



    /// `attribute AString crop;`
    #[inline]
    pub unsafe fn GetCrop(&self, aCrop: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCrop)(self, aCrop)
    }



    /// `attribute AString crop;`
    #[inline]
    pub unsafe fn SetCrop(&self, aCrop: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCrop)(self, aCrop)
    }



    /// `attribute AString image;`
    #[inline]
    pub unsafe fn GetImage(&self, aImage: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetImage)(self, aImage)
    }



    /// `attribute AString image;`
    #[inline]
    pub unsafe fn SetImage(&self, aImage: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetImage)(self, aImage)
    }



    /// `readonly attribute Element inputField;`
    #[inline]
    pub unsafe fn GetInputField(&self, aInputField: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetInputField)(self, aInputField)
    }


}


