//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULSelectCntrlEl.idl
//


/// `interface nsIDOMXULSelectControlElement : nsIDOMXULControlElement`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULSelectControlElement {
    vtable: *const nsIDOMXULSelectControlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULSelectControlElement.
unsafe impl XpCom for nsIDOMXULSelectControlElement {
    const IID: nsIID = nsID(0x9bf188a7, 0xd6f9, 0x431b,
        [0xb5, 0xc7, 0x11, 0x80, 0x13, 0x99, 0x8e, 0x8b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULSelectControlElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULSelectControlElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULSelectControlElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULSelectControlElement`.
    fn coerce_from(v: &nsIDOMXULSelectControlElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULSelectControlElementCoerce for nsIDOMXULSelectControlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlElement) -> &Self {
        v
    }
}

impl nsIDOMXULSelectControlElement {
    /// Cast this `nsIDOMXULSelectControlElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULSelectControlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULSelectControlElement {
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
impl<T: nsIDOMXULControlElementCoerce> nsIDOMXULSelectControlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULSelectControlElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULSelectControlElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMXULControlElementVTable,

    /* attribute Element selectedItem; */
    pub GetSelectedItem: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aSelectedItem: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Element selectedItem; */
    pub SetSelectedItem: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aSelectedItem: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute long selectedIndex; */
    pub GetSelectedIndex: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aSelectedIndex: *mut i32) -> ::nserror::nsresult,

    /* attribute long selectedIndex; */
    pub SetSelectedIndex: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aSelectedIndex: i32) -> ::nserror::nsresult,

    /* attribute AString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long itemCount; */
    pub GetItemCount: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, aItemCount: *mut u32) -> ::nserror::nsresult,

    /* long getIndexOfItem (in nsIDOMXULSelectControlItemElement item); */
    pub GetIndexOfItem: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, item: *const nsIDOMXULSelectControlItemElement, _retval: *mut i32) -> ::nserror::nsresult,

    /* Element getItemAtIndex (in long index); */
    pub GetItemAtIndex: unsafe extern "system" fn (this: *const nsIDOMXULSelectControlElement, index: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULSelectControlElement {


    /// `attribute Element selectedItem;`
    #[inline]
    pub unsafe fn GetSelectedItem(&self, aSelectedItem: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedItem)(self, aSelectedItem)
    }



    /// `attribute Element selectedItem;`
    #[inline]
    pub unsafe fn SetSelectedItem(&self, aSelectedItem: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectedItem)(self, aSelectedItem)
    }



    /// `attribute long selectedIndex;`
    #[inline]
    pub unsafe fn GetSelectedIndex(&self, aSelectedIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedIndex)(self, aSelectedIndex)
    }



    /// `attribute long selectedIndex;`
    #[inline]
    pub unsafe fn SetSelectedIndex(&self, aSelectedIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectedIndex)(self, aSelectedIndex)
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



    /// `readonly attribute unsigned long itemCount;`
    #[inline]
    pub unsafe fn GetItemCount(&self, aItemCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetItemCount)(self, aItemCount)
    }



    /// `long getIndexOfItem (in nsIDOMXULSelectControlItemElement item);`
    #[inline]
    pub unsafe fn GetIndexOfItem(&self, item: *const nsIDOMXULSelectControlItemElement, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetIndexOfItem)(self, item, _retval)
    }



    /// `Element getItemAtIndex (in long index);`
    #[inline]
    pub unsafe fn GetItemAtIndex(&self, index: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetItemAtIndex)(self, index, _retval)
    }


}


