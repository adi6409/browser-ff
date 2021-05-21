//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISystemStatusBar.idl
//


/// `interface nsISystemStatusBar : nsISupports`
///

/// ```text
/// /**
///  * Allow applications to interface with the Mac OS X system status bar.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISystemStatusBar {
    vtable: *const nsISystemStatusBarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISystemStatusBar.
unsafe impl XpCom for nsISystemStatusBar {
    const IID: nsIID = nsID(0x24493180, 0xee81, 0x4b7c,
        [0x8b, 0x17, 0x9e, 0x69, 0x48, 0x0b, 0x7b, 0x8a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISystemStatusBar {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISystemStatusBar.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISystemStatusBarCoerce {
    /// Cheaply cast a value of this type from a `nsISystemStatusBar`.
    fn coerce_from(v: &nsISystemStatusBar) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISystemStatusBarCoerce for nsISystemStatusBar {
    #[inline]
    fn coerce_from(v: &nsISystemStatusBar) -> &Self {
        v
    }
}

impl nsISystemStatusBar {
    /// Cast this `nsISystemStatusBar` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISystemStatusBarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISystemStatusBar {
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
impl<T: nsISupportsCoerce> nsISystemStatusBarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISystemStatusBar) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISystemStatusBar
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISystemStatusBarVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addItem (in Element aMenuElement); */
    pub AddItem: unsafe extern "system" fn (this: *const nsISystemStatusBar, aMenuElement: *const libc::c_void) -> ::nserror::nsresult,

    /* void removeItem (in Element aMenuElement); */
    pub RemoveItem: unsafe extern "system" fn (this: *const nsISystemStatusBar, aMenuElement: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISystemStatusBar {

    /// ```text
    /// /**
    ///    * Add an item to the system status bar. Each item can only be present once,
    ///    * subsequent addItem calls with the same element will be ignored.
    ///    * The system status bar holds a strong reference to the added XUL menu
    ///    * element and the item will stay in the status bar until it is removed via
    ///    * a call to removeItem, or until the process shuts down.
    ///    * @param aDOMMenuElement A XUL menu element that contains a XUL menupopup
    ///    *                        with regular menu content. The menu's icon is put
    ///    *                        into the system status bar; clicking it will open
    ///    *                        a menu with the contents of the menupopup.
    ///    *                        The menu label is not shown.
    ///    */
    /// ```
    ///

    /// `void addItem (in Element aMenuElement);`
    #[inline]
    pub unsafe fn AddItem(&self, aMenuElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).AddItem)(self, aMenuElement)
    }


    /// ```text
    /// /**
    ///    * Remove a previously-added item from the menu bar. Calling this with an
    ///    * element that has not been added before will be silently ignored.
    ///    * @param aDOMMenuElement The XUL menu element that you called addItem with.
    ///    */
    /// ```
    ///

    /// `void removeItem (in Element aMenuElement);`
    #[inline]
    pub unsafe fn RemoveItem(&self, aMenuElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).RemoveItem)(self, aMenuElement)
    }


}


