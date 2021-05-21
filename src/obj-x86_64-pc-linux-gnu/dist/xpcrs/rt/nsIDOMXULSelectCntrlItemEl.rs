//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULSelectCntrlItemEl.idl
//


/// `interface nsIDOMXULSelectControlItemElement : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULSelectControlItemElement {
    vtable: *const nsIDOMXULSelectControlItemElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULSelectControlItemElement.
unsafe impl XpCom for nsIDOMXULSelectControlItemElement {
    const IID: nsIID = nsID(0x5c6be58f, 0x17df, 0x4750,
        [0x88, 0xa5, 0x4a, 0x59, 0xac, 0x28, 0xad, 0xc9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULSelectControlItemElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULSelectControlItemElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULSelectControlItemElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULSelectControlItemElement`.
    fn coerce_from(v: &nsIDOMXULSelectControlItemElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULSelectControlItemElementCoerce for nsIDOMXULSelectControlItemElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlItemElement) -> &Self {
        v
    }
}

impl nsIDOMXULSelectControlItemElement {
    /// Cast this `nsIDOMXULSelectControlItemElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULSelectControlItemElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULSelectControlItemElement {
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
impl<T: nsISupportsCoerce> nsIDOMXULSelectControlItemElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlItemElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULSelectControlItemElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULSelectControlItemElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub GetDisabled: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aDisabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean disabled; */
    pub SetDisabled: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aDisabled: bool) -> ::nserror::nsresult,

    /* attribute AString crop; */
    pub GetCrop: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aCrop: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString crop; */
    pub SetCrop: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aCrop: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString image; */
    pub GetImage: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aImage: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString image; */
    pub SetImage: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aImage: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString label; */
    pub GetLabel: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString label; */
    pub SetLabel: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString accessKey; */
    pub GetAccessKey: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aAccessKey: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString accessKey; */
    pub SetAccessKey: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aAccessKey: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString command; */
    pub GetCommand: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aCommand: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString command; */
    pub SetCommand: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aCommand: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean selected; */
    pub GetSelected: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aSelected: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute Element control; */
    pub GetControl: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlItemElement, aControl: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULSelectControlItemElement {


    /// `attribute boolean disabled;`
    #[inline]
    pub unsafe fn GetDisabled(&self, aDisabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDisabled)(self, aDisabled)
    }



    /// `attribute boolean disabled;`
    #[inline]
    pub unsafe fn SetDisabled(&self, aDisabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDisabled)(self, aDisabled)
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



    /// `attribute AString label;`
    #[inline]
    pub unsafe fn GetLabel(&self, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLabel)(self, aLabel)
    }



    /// `attribute AString label;`
    #[inline]
    pub unsafe fn SetLabel(&self, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetLabel)(self, aLabel)
    }



    /// `attribute AString accessKey;`
    #[inline]
    pub unsafe fn GetAccessKey(&self, aAccessKey: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessKey)(self, aAccessKey)
    }



    /// `attribute AString accessKey;`
    #[inline]
    pub unsafe fn SetAccessKey(&self, aAccessKey: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetAccessKey)(self, aAccessKey)
    }



    /// `attribute AString command;`
    #[inline]
    pub unsafe fn GetCommand(&self, aCommand: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCommand)(self, aCommand)
    }



    /// `attribute AString command;`
    #[inline]
    pub unsafe fn SetCommand(&self, aCommand: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCommand)(self, aCommand)
    }



    /// `attribute AString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute AString value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }



    /// `readonly attribute boolean selected;`
    #[inline]
    pub unsafe fn GetSelected(&self, aSelected: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSelected)(self, aSelected)
    }



    /// `readonly attribute Element control;`
    #[inline]
    pub unsafe fn GetControl(&self, aControl: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetControl)(self, aControl)
    }


}


