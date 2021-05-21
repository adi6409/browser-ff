//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPaper.idl
//


/// `interface nsIPaper : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaper {
    vtable: *const nsIPaperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaper.
unsafe impl XpCom for nsIPaper {
    const IID: nsIID = nsID(0xa4dd9675, 0x6311, 0x45a9,
        [0xa5, 0x47, 0x44, 0xe0, 0x12, 0x73, 0x04, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaperCoerce {
    /// Cheaply cast a value of this type from a `nsIPaper`.
    fn coerce_from(v: &nsIPaper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaperCoerce for nsIPaper {
    #[inline]
    fn coerce_from(v: &nsIPaper) -> &Self {
        v
    }
}

impl nsIPaper {
    /// Cast this `nsIPaper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaper {
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
impl<T: nsISupportsCoerce> nsIPaperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIPaper, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIPaper, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute double width; */
    pub GetWidth: unsafe extern "system" fn (this: *const nsIPaper, aWidth: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double height; */
    pub GetHeight: unsafe extern "system" fn (this: *const nsIPaper, aHeight: *mut libc::c_double) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute Promise unwriteableMargin; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetUnwriteableMargin: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaper {

    /// ```text
    /// /**
    ///    * The internal name of the paper (a fixed, non-localized ID).
    ///    * (For CUPS, this is the PWG-standardized name as used internally by CUPS;
        ///    * on Windows, it is the integer paper ID as a string.)
    ///    */
    /// ```
    ///

    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }


    /// ```text
    /// /**
    ///    * The human-readable (potentially localized) name of the paper.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * The width of the paper assuming portrait orientation, in points.
    ///    * That is, the length of the shorter edges of the paper.
    ///    */
    /// ```
    ///

    /// `readonly attribute double width;`
    #[inline]
    pub unsafe fn GetWidth(&self, aWidth: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetWidth)(self, aWidth)
    }


    /// ```text
    /// /**
    ///    * The height of the paper assuming portrait orientation, in points.
    ///    * That is, the length of the longer edges of the paper.
    ///    */
    /// ```
    ///

    /// `readonly attribute double height;`
    #[inline]
    pub unsafe fn GetHeight(&self, aHeight: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetHeight)(self, aHeight)
    }


    /// ```text
    /// /**
    ///    * The Promise resolves with an nsIPaperMargin object. The margin widths contained
    ///    * in that object's top/bottom/left/right properties are relative to the paper in
    ///    * portrait orientation. That is, top and bottom are the margins for the short edges,
    ///    * and left and right are the margins for the long edges.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise unwriteableMargin;`
    const _GetUnwriteableMargin: () = ();

}


