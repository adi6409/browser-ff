//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULButtonElement.idl
//


/// `interface nsIDOMXULButtonElement : nsIDOMXULControlElement`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULButtonElement {
    vtable: *const nsIDOMXULButtonElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULButtonElement.
unsafe impl XpCom for nsIDOMXULButtonElement {
    const IID: nsIID = nsID(0x6ed53cfb, 0x9e59, 0x424c,
        [0xaf, 0x8d, 0xe7, 0x45, 0x82, 0x38, 0x19, 0x51]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULButtonElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULButtonElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULButtonElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULButtonElement`.
    fn coerce_from(v: &nsIDOMXULButtonElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULButtonElementCoerce for nsIDOMXULButtonElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULButtonElement) -> &Self {
        v
    }
}

impl nsIDOMXULButtonElement {
    /// Cast this `nsIDOMXULButtonElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULButtonElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULButtonElement {
    type Target = nsIDOMXULControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDOMXULControlElementCoerce> nsIDOMXULButtonElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULButtonElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULButtonElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULButtonElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMXULControlElementVTable,

    /* attribute AString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString type; */
    pub SetType: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean open; */
    pub GetOpen: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aOpen: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean open; */
    pub SetOpen: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aOpen: bool) -> ::nserror::nsresult,

    /* attribute boolean checked; */
    pub GetChecked: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aChecked: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean checked; */
    pub SetChecked: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aChecked: bool) -> ::nserror::nsresult,

    /* attribute AString group; */
    pub GetGroup: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aGroup: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString group; */
    pub SetGroup: unsafe extern "system" fn (this: *const nsIDOMXULButtonElement, aGroup: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULButtonElement {


    /// `attribute AString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `attribute AString type;`
    #[inline]
    pub unsafe fn SetType(&self, aType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetType)(self, aType)
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



    /// `attribute boolean checked;`
    #[inline]
    pub unsafe fn GetChecked(&self, aChecked: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetChecked)(self, aChecked)
    }



    /// `attribute boolean checked;`
    #[inline]
    pub unsafe fn SetChecked(&self, aChecked: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetChecked)(self, aChecked)
    }



    /// `attribute AString group;`
    #[inline]
    pub unsafe fn GetGroup(&self, aGroup: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetGroup)(self, aGroup)
    }



    /// `attribute AString group;`
    #[inline]
    pub unsafe fn SetGroup(&self, aGroup: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetGroup)(self, aGroup)
    }


}


