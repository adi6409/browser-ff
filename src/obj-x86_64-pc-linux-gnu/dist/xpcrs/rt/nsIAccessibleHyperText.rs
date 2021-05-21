//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHyperText.idl
//


/// `interface nsIAccessibleHyperText : nsISupports`
///

/// ```text
/// /**
///  * A cross-platform interface that deals with text which contains hyperlinks.
///  * Each link is an embedded object representing exactly 1 character within
///  * the hypertext.
///  *
///  * Current implementation assumes every embedded object is a link.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleHyperText {
    vtable: *const nsIAccessibleHyperTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleHyperText.
unsafe impl XpCom for nsIAccessibleHyperText {
    const IID: nsIID = nsID(0xb33684e2, 0x090c, 0x4e1d,
        [0xa3, 0xd9, 0xf4, 0xb4, 0x6f, 0x42, 0x37, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleHyperText {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleHyperText.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleHyperTextCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleHyperText`.
    fn coerce_from(v: &nsIAccessibleHyperText) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleHyperTextCoerce for nsIAccessibleHyperText {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperText) -> &Self {
        v
    }
}

impl nsIAccessibleHyperText {
    /// Cast this `nsIAccessibleHyperText` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleHyperTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleHyperText {
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
impl<T: nsISupportsCoerce> nsIAccessibleHyperTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperText) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleHyperText
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleHyperTextVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long linkCount; */
    pub GetLinkCount: unsafe extern "system" fn (this: *const nsIAccessibleHyperText, aLinkCount: *mut i32) -> ::nserror::nsresult,

    /* nsIAccessibleHyperLink getLinkAt (in long index); */
    pub GetLinkAt: unsafe extern "system" fn (this: *const nsIAccessibleHyperText, index: i32, _retval: *mut *const nsIAccessibleHyperLink) -> ::nserror::nsresult,

    /* long getLinkIndex (in nsIAccessibleHyperLink link); */
    pub GetLinkIndex: unsafe extern "system" fn (this: *const nsIAccessibleHyperText, link: *const nsIAccessibleHyperLink, _retval: *mut i32) -> ::nserror::nsresult,

    /* long getLinkIndexAtOffset (in long offset); */
    pub GetLinkIndexAtOffset: unsafe extern "system" fn (this: *const nsIAccessibleHyperText, offset: i32, _retval: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleHyperText {

    /// ```text
    /// /**
    ///    * Return the number of links contained within this hypertext object.
    ///    */
    /// ```
    ///

    /// `readonly attribute long linkCount;`
    #[inline]
    pub unsafe fn GetLinkCount(&self, aLinkCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLinkCount)(self, aLinkCount)
    }


    /// ```text
    /// /**
    ///    * Return link accessible at the given index.
    ///    *
    ///    * @param index  [in] 0-based index of the link that is to be retrieved
    ///    *
    ///    * @return       link accessible or null if there is no link at that index
    ///    */
    /// ```
    ///

    /// `nsIAccessibleHyperLink getLinkAt (in long index);`
    #[inline]
    pub unsafe fn GetLinkAt(&self, index: i32, _retval: *mut *const nsIAccessibleHyperLink) -> ::nserror::nsresult {
        ((*self.vtable).GetLinkAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Return index of the given link.
    ///    *
    ///    * @param link  [in] link accessible the index is requested for
    ///    *
    ///    * @return      index of the given link or null if there's no link within
    ///    *                hypertext accessible
    ///    */
    /// ```
    ///

    /// `long getLinkIndex (in nsIAccessibleHyperLink link);`
    #[inline]
    pub unsafe fn GetLinkIndex(&self, link: *const nsIAccessibleHyperLink, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLinkIndex)(self, link, _retval)
    }



    /// `long getLinkIndexAtOffset (in long offset);`
    #[inline]
    pub unsafe fn GetLinkIndexAtOffset(&self, offset: i32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLinkIndexAtOffset)(self, offset, _retval)
    }


}


