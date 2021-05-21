//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleSelectable.idl
//


/// `interface nsIAccessibleSelectable : nsISupports`
///

/// ```text
/// /**
///  * An accessibility interface for selectable widgets.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleSelectable {
    vtable: *const nsIAccessibleSelectableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleSelectable.
unsafe impl XpCom for nsIAccessibleSelectable {
    const IID: nsIID = nsID(0x8efb03d4, 0x1354, 0x4875,
        [0x94, 0xcf, 0x26, 0x13, 0x36, 0x05, 0x76, 0x26]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleSelectable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleSelectable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleSelectableCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleSelectable`.
    fn coerce_from(v: &nsIAccessibleSelectable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleSelectableCoerce for nsIAccessibleSelectable {
    #[inline]
    fn coerce_from(v: &nsIAccessibleSelectable) -> &Self {
        v
    }
}

impl nsIAccessibleSelectable {
    /// Cast this `nsIAccessibleSelectable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleSelectableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleSelectable {
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
impl<T: nsISupportsCoerce> nsIAccessibleSelectableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleSelectable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleSelectable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleSelectableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIArray selectedItems; */
    pub GetSelectedItems: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, aSelectedItems: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute unsigned long selectedItemCount; */
    pub GetSelectedItemCount: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, aSelectedItemCount: *mut u32) -> ::nserror::nsresult,

    /* nsIAccessible getSelectedItemAt (in unsigned long index); */
    pub GetSelectedItemAt: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, index: u32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* boolean isItemSelected (in unsigned long index); */
    pub IsItemSelected: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, index: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* void addItemToSelection (in unsigned long index); */
    pub AddItemToSelection: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, index: u32) -> ::nserror::nsresult,

    /* void removeItemFromSelection (in unsigned long index); */
    pub RemoveItemFromSelection: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, index: u32) -> ::nserror::nsresult,

    /* boolean selectAll (); */
    pub SelectAll: unsafe extern "system" fn (this: *const nsIAccessibleSelectable, _retval: *mut bool) -> ::nserror::nsresult,

    /* void unselectAll (); */
    pub UnselectAll: unsafe extern "system" fn (this: *const nsIAccessibleSelectable) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleSelectable {

    /// ```text
    /// /**
    ///    * Return an nsIArray of selected items within the widget.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray selectedItems;`
    #[inline]
    pub unsafe fn GetSelectedItems(&self, aSelectedItems: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedItems)(self, aSelectedItems)
    }


    /// ```text
    /// /**
    ///    * Return the number of currently selected items.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long selectedItemCount;`
    #[inline]
    pub unsafe fn GetSelectedItemCount(&self, aSelectedItemCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedItemCount)(self, aSelectedItemCount)
    }


    /// ```text
    /// /**
    ///    * Return a nth selected item within the widget.
    ///    */
    /// ```
    ///

    /// `nsIAccessible getSelectedItemAt (in unsigned long index);`
    #[inline]
    pub unsafe fn GetSelectedItemAt(&self, index: u32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedItemAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Return true if the given item is selected.
    ///    */
    /// ```
    ///

    /// `boolean isItemSelected (in unsigned long index);`
    #[inline]
    pub unsafe fn IsItemSelected(&self, index: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsItemSelected)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds the specified item to the widget's selection.
    ///    */
    /// ```
    ///

    /// `void addItemToSelection (in unsigned long index);`
    #[inline]
    pub unsafe fn AddItemToSelection(&self, index: u32) -> ::nserror::nsresult {
        ((*self.vtable).AddItemToSelection)(self, index)
    }


    /// ```text
    /// /**
    ///    * Removes the specified item from the widget's selection.
    ///    */
    /// ```
    ///

    /// `void removeItemFromSelection (in unsigned long index);`
    #[inline]
    pub unsafe fn RemoveItemFromSelection(&self, index: u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveItemFromSelection)(self, index)
    }


    /// ```text
    /// /**
    ///    * Select all items.
    ///    *
    ///    * @return false if the object does not accept multiple selection,
    ///    *         otherwise true.
    ///    */
    /// ```
    ///

    /// `boolean selectAll ();`
    #[inline]
    pub unsafe fn SelectAll(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SelectAll)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Unselect all items.
    ///    */
    /// ```
    ///

    /// `void unselectAll ();`
    #[inline]
    pub unsafe fn UnselectAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UnselectAll)(self, )
    }


}


