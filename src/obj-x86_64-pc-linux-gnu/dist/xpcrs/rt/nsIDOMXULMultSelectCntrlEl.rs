//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULMultSelectCntrlEl.idl
//


/// `interface nsIDOMXULMultiSelectControlElement : nsIDOMXULSelectControlElement`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMXULMultiSelectControlElement {
    vtable: *const nsIDOMXULMultiSelectControlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMXULMultiSelectControlElement.
unsafe impl XpCom for nsIDOMXULMultiSelectControlElement {
    const IID: nsIID = nsID(0x40654a10, 0x8204, 0x4f06,
        [0x9f, 0x21, 0x7b, 0xaa, 0x31, 0xc7, 0xb1, 0xdd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMXULMultiSelectControlElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMXULMultiSelectControlElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMXULMultiSelectControlElementCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMXULMultiSelectControlElement`.
    fn coerce_from(v: &nsIDOMXULMultiSelectControlElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMXULMultiSelectControlElementCoerce for nsIDOMXULMultiSelectControlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMultiSelectControlElement) -> &Self {
        v
    }
}

impl nsIDOMXULMultiSelectControlElement {
    /// Cast this `nsIDOMXULMultiSelectControlElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMXULMultiSelectControlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMXULMultiSelectControlElement {
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
impl<T: nsIDOMXULSelectControlElementCoerce> nsIDOMXULMultiSelectControlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMultiSelectControlElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMXULMultiSelectControlElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMXULMultiSelectControlElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMXULSelectControlElementVTable,

    /* attribute AString selType; */
    pub GetSelType: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString selType; */
    pub SetSelType: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute Element currentItem; */
    pub GetCurrentItem: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentItem: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Element currentItem; */
    pub SetCurrentItem: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentItem: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute long currentIndex; */
    pub GetCurrentIndex: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentIndex: *mut i32) -> ::nserror::nsresult,

    /* attribute long currentIndex; */
    pub SetCurrentIndex: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aCurrentIndex: i32) -> ::nserror::nsresult,

    /* readonly attribute NodeList selectedItems; */
    pub GetSelectedItems: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelectedItems: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void addItemToSelection (in nsIDOMXULSelectControlItemElement item); */
    pub AddItemToSelection: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult,

    /* void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item); */
    pub RemoveItemFromSelection: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult,

    /* void toggleItemSelection (in nsIDOMXULSelectControlItemElement item); */
    pub ToggleItemSelection: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult,

    /* void selectItem (in nsIDOMXULSelectControlItemElement item); */
    pub SelectItem: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult,

    /* void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item); */
    pub SelectItemRange: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, startItem: *const nsIDOMXULSelectControlItemElement, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult,

    /* void selectAll (); */
    pub SelectAll: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement) -> ::nserror::nsresult,

    /* void clearSelection (); */
    pub ClearSelection: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement) -> ::nserror::nsresult,

    /* readonly attribute long selectedCount; */
    pub GetSelectedCount: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, aSelectedCount: *mut i32) -> ::nserror::nsresult,

    /* [binaryname(MultiGetSelectedItem)] Element getSelectedItem (in long index); */
    pub MultiGetSelectedItem: unsafe extern "system" fn (this: *const nsIDOMXULMultiSelectControlElement, index: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMXULMultiSelectControlElement {


    /// `attribute AString selType;`
    #[inline]
    pub unsafe fn GetSelType(&self, aSelType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSelType)(self, aSelType)
    }



    /// `attribute AString selType;`
    #[inline]
    pub unsafe fn SetSelType(&self, aSelType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSelType)(self, aSelType)
    }



    /// `attribute Element currentItem;`
    #[inline]
    pub unsafe fn GetCurrentItem(&self, aCurrentItem: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentItem)(self, aCurrentItem)
    }



    /// `attribute Element currentItem;`
    #[inline]
    pub unsafe fn SetCurrentItem(&self, aCurrentItem: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentItem)(self, aCurrentItem)
    }



    /// `attribute long currentIndex;`
    #[inline]
    pub unsafe fn GetCurrentIndex(&self, aCurrentIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentIndex)(self, aCurrentIndex)
    }



    /// `attribute long currentIndex;`
    #[inline]
    pub unsafe fn SetCurrentIndex(&self, aCurrentIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentIndex)(self, aCurrentIndex)
    }



    /// `readonly attribute NodeList selectedItems;`
    #[inline]
    pub unsafe fn GetSelectedItems(&self, aSelectedItems: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedItems)(self, aSelectedItems)
    }



    /// `void addItemToSelection (in nsIDOMXULSelectControlItemElement item);`
    #[inline]
    pub unsafe fn AddItemToSelection(&self, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult {
        ((*self.vtable).AddItemToSelection)(self, item)
    }



    /// `void removeItemFromSelection (in nsIDOMXULSelectControlItemElement item);`
    #[inline]
    pub unsafe fn RemoveItemFromSelection(&self, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult {
        ((*self.vtable).RemoveItemFromSelection)(self, item)
    }



    /// `void toggleItemSelection (in nsIDOMXULSelectControlItemElement item);`
    #[inline]
    pub unsafe fn ToggleItemSelection(&self, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult {
        ((*self.vtable).ToggleItemSelection)(self, item)
    }



    /// `void selectItem (in nsIDOMXULSelectControlItemElement item);`
    #[inline]
    pub unsafe fn SelectItem(&self, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult {
        ((*self.vtable).SelectItem)(self, item)
    }



    /// `void selectItemRange (in nsIDOMXULSelectControlItemElement startItem, in nsIDOMXULSelectControlItemElement item);`
    #[inline]
    pub unsafe fn SelectItemRange(&self, startItem: *const nsIDOMXULSelectControlItemElement, item: *const nsIDOMXULSelectControlItemElement) -> ::nserror::nsresult {
        ((*self.vtable).SelectItemRange)(self, startItem, item)
    }



    /// `void selectAll ();`
    #[inline]
    pub unsafe fn SelectAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectAll)(self, )
    }



    /// `void clearSelection ();`
    #[inline]
    pub unsafe fn ClearSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearSelection)(self, )
    }



    /// `readonly attribute long selectedCount;`
    #[inline]
    pub unsafe fn GetSelectedCount(&self, aSelectedCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedCount)(self, aSelectedCount)
    }



    /// `[binaryname(MultiGetSelectedItem)] Element getSelectedItem (in long index);`
    #[inline]
    pub unsafe fn MultiGetSelectedItem(&self, index: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).MultiGetSelectedItem)(self, index, _retval)
    }


}


