//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboardHelper.idl
//


/// `interface nsIClipboardHelper : nsISupports`
///

/// ```text
/// /**
///  * helper service for common uses of nsIClipboard.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClipboardHelper {
    vtable: *const nsIClipboardHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClipboardHelper.
unsafe impl XpCom for nsIClipboardHelper {
    const IID: nsIID = nsID(0x438307fd, 0x0c68, 0x4d79,
        [0x92, 0x2a, 0xf6, 0xcc, 0x95, 0x50, 0xcd, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClipboardHelper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClipboardHelper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClipboardHelperCoerce {
    /// Cheaply cast a value of this type from a `nsIClipboardHelper`.
    fn coerce_from(v: &nsIClipboardHelper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClipboardHelperCoerce for nsIClipboardHelper {
    #[inline]
    fn coerce_from(v: &nsIClipboardHelper) -> &Self {
        v
    }
}

impl nsIClipboardHelper {
    /// Cast this `nsIClipboardHelper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClipboardHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClipboardHelper {
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
impl<T: nsISupportsCoerce> nsIClipboardHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboardHelper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClipboardHelper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClipboardHelperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void copyStringToClipboard (in AString aString, in long aClipboardID); */
    pub CopyStringToClipboard: unsafe extern "system" fn (this: *const nsIClipboardHelper, aString: *const ::nsstring::nsAString, aClipboardID: i32) -> ::nserror::nsresult,

    /* void copyString (in AString aString); */
    pub CopyString: unsafe extern "system" fn (this: *const nsIClipboardHelper, aString: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClipboardHelper {

    /// ```text
    /// /**
    ///    * copy string to given clipboard
    ///    *
    ///    * @param aString, the string to copy to the clipboard
    ///    * @param aClipboardID, the ID of the clipboard to copy to
    ///    *        (eg. kSelectionClipboard -- see nsIClipboard.idl)
    ///    */
    /// ```
    ///

    /// `void copyStringToClipboard (in AString aString, in long aClipboardID);`
    #[inline]
    pub unsafe fn CopyStringToClipboard(&self, aString: *const ::nsstring::nsAString, aClipboardID: i32) -> ::nserror::nsresult {
        ((*self.vtable).CopyStringToClipboard)(self, aString, aClipboardID)
    }


    /// ```text
    /// /**
    ///    * copy string to (default) clipboard
    ///    *
    ///    * @param aString, the string to copy to the clipboard
    ///    */
    /// ```
    ///

    /// `void copyString (in AString aString);`
    #[inline]
    pub unsafe fn CopyString(&self, aString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).CopyString)(self, aString)
    }


}


