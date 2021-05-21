//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboard.idl
//


/// `interface nsIClipboard : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClipboard {
    vtable: *const nsIClipboardVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClipboard.
unsafe impl XpCom for nsIClipboard {
    const IID: nsIID = nsID(0xceaa0047, 0x647f, 0x4b8e,
        [0xad, 0x1c, 0xaf, 0xf9, 0xfa, 0x62, 0xaa, 0x51]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClipboard {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClipboard.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClipboardCoerce {
    /// Cheaply cast a value of this type from a `nsIClipboard`.
    fn coerce_from(v: &nsIClipboard) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClipboardCoerce for nsIClipboard {
    #[inline]
    fn coerce_from(v: &nsIClipboard) -> &Self {
        v
    }
}

impl nsIClipboard {
    /// Cast this `nsIClipboard` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClipboardCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClipboard {
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
impl<T: nsISupportsCoerce> nsIClipboardCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboard) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClipboard
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClipboardVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setData (in nsITransferable aTransferable, in nsIClipboardOwner anOwner, in long aWhichClipboard); */
    pub SetData: unsafe extern "system" fn (this: *const nsIClipboard, aTransferable: *const nsITransferable, anOwner: *const nsIClipboardOwner, aWhichClipboard: i32) -> ::nserror::nsresult,

    /* void getData (in nsITransferable aTransferable, in long aWhichClipboard); */
    pub GetData: unsafe extern "system" fn (this: *const nsIClipboard, aTransferable: *const nsITransferable, aWhichClipboard: i32) -> ::nserror::nsresult,

    /* void emptyClipboard (in long aWhichClipboard); */
    pub EmptyClipboard: unsafe extern "system" fn (this: *const nsIClipboard, aWhichClipboard: i32) -> ::nserror::nsresult,

    /* boolean hasDataMatchingFlavors (in Array<ACString> aFlavorList, in long aWhichClipboard); */
    pub HasDataMatchingFlavors: unsafe extern "system" fn (this: *const nsIClipboard, aFlavorList: *const thin_vec::ThinVec<::nsstring::nsCString>, aWhichClipboard: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean supportsSelectionClipboard (); */
    pub SupportsSelectionClipboard: unsafe extern "system" fn (this: *const nsIClipboard, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean supportsFindClipboard (); */
    pub SupportsFindClipboard: unsafe extern "system" fn (this: *const nsIClipboard, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClipboard {

    pub const kSelectionClipboard: i64 = 0;


    pub const kGlobalClipboard: i64 = 1;


    pub const kFindClipboard: i64 = 2;


    pub const kSelectionCache: i64 = 3;

    /// ```text
    /// /**
    ///     * Given a transferable, set the data on the native clipboard
    ///     *
    ///     * @param  aTransferable The transferable
    ///     * @param  anOwner The owner of the transferable
    ///     * @param  aWhichClipboard Specifies the clipboard to which this operation applies.
    ///     * @result NS_Ok if no errors
    ///     */
    /// ```
    ///

    /// `void setData (in nsITransferable aTransferable, in nsIClipboardOwner anOwner, in long aWhichClipboard);`
    #[inline]
    pub unsafe fn SetData(&self, aTransferable: *const nsITransferable, anOwner: *const nsIClipboardOwner, aWhichClipboard: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, aTransferable, anOwner, aWhichClipboard)
    }


    /// ```text
    /// /**
    ///     * Given a transferable, get the clipboard data.
    ///     *
    ///     * @param  aTransferable The transferable
    ///     * @param  aWhichClipboard Specifies the clipboard to which this operation applies.
    ///     * @result NS_Ok if no errors
    ///     */
    /// ```
    ///

    /// `void getData (in nsITransferable aTransferable, in long aWhichClipboard);`
    #[inline]
    pub unsafe fn GetData(&self, aTransferable: *const nsITransferable, aWhichClipboard: i32) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aTransferable, aWhichClipboard)
    }


    /// ```text
    /// /**
    ///     * This empties the clipboard and notifies the clipboard owner.
    ///     * This empties the "logical" clipboard. It does not clear the native clipboard.
    ///     *
    ///     * @param  aWhichClipboard Specifies the clipboard to which this operation applies.
    ///     * @result NS_OK if successful.
    ///     */
    /// ```
    ///

    /// `void emptyClipboard (in long aWhichClipboard);`
    #[inline]
    pub unsafe fn EmptyClipboard(&self, aWhichClipboard: i32) -> ::nserror::nsresult {
        ((*self.vtable).EmptyClipboard)(self, aWhichClipboard)
    }


    /// ```text
    /// /**
    ///     * This provides a way to give correct UI feedback about, for instance, a paste
    ///     * should be allowed. It does _NOT_ actually retreive the data and should be a very
    ///     * inexpensive call. All it does is check if there is data on the clipboard matching
    ///     * any of the flavors in the given list.
    ///     *
    ///     * @param  aFlavorList     An array of ASCII strings.
    ///     * @param  aWhichClipboard Specifies the clipboard to which this operation applies.
    ///     * @outResult - if data is present matching one of
    ///     * @result NS_OK if successful.
    ///     */
    /// ```
    ///

    /// `boolean hasDataMatchingFlavors (in Array<ACString> aFlavorList, in long aWhichClipboard);`
    #[inline]
    pub unsafe fn HasDataMatchingFlavors(&self, aFlavorList: *const thin_vec::ThinVec<::nsstring::nsCString>, aWhichClipboard: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasDataMatchingFlavors)(self, aFlavorList, aWhichClipboard, _retval)
    }


    /// ```text
    /// /**
    ///     * Allows clients to determine if the implementation supports the concept of a
    ///     * separate clipboard for selection.
    ///     *
    ///     * @outResult - true if
    ///     * @result NS_OK if successful.
    ///     */
    /// ```
    ///

    /// `boolean supportsSelectionClipboard ();`
    #[inline]
    pub unsafe fn SupportsSelectionClipboard(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SupportsSelectionClipboard)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * Allows clients to determine if the implementation supports the concept of a
    ///     * separate clipboard for find search strings.
    ///     *
    ///     * @result NS_OK if successful.
    ///     */
    /// ```
    ///

    /// `boolean supportsFindClipboard ();`
    #[inline]
    pub unsafe fn SupportsFindClipboard(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SupportsFindClipboard)(self, _retval)
    }


}


