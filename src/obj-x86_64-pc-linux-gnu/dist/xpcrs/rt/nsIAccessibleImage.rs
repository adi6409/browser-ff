//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleImage.idl
//


/// `interface nsIAccessibleImage : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleImage {
    vtable: *const nsIAccessibleImageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleImage.
unsafe impl XpCom for nsIAccessibleImage {
    const IID: nsIID = nsID(0x09086623, 0x0f09, 0x4310,
        [0xac, 0x56, 0xc2, 0xcd, 0xa7, 0xc2, 0x96, 0x48]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleImage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleImage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleImageCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleImage`.
    fn coerce_from(v: &nsIAccessibleImage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleImageCoerce for nsIAccessibleImage {
    #[inline]
    fn coerce_from(v: &nsIAccessibleImage) -> &Self {
        v
    }
}

impl nsIAccessibleImage {
    /// Cast this `nsIAccessibleImage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleImageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleImage {
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
impl<T: nsISupportsCoerce> nsIAccessibleImageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleImage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleImage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleImageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getImagePosition (in unsigned long coordType, out long x, out long y); */
    pub GetImagePosition: unsafe extern "system" fn (this: *const nsIAccessibleImage, coordType: u32, x: *mut i32, y: *mut i32) -> ::nserror::nsresult,

    /* void getImageSize (out long width, out long height); */
    pub GetImageSize: unsafe extern "system" fn (this: *const nsIAccessibleImage, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleImage {

    /// ```text
    /// /**
    ///    * Returns the coordinates of the image.
    ///    *
    ///    * @param coordType  specifies coordinates origin (for available constants
        ///    *                   refer to nsIAccessibleCoordinateType)
    ///    * @param x          the x coordinate
    ///    * @param y          the y coordinate
    ///    */
    /// ```
    ///

    /// `void getImagePosition (in unsigned long coordType, out long x, out long y);`
    #[inline]
    pub unsafe fn GetImagePosition(&self, coordType: u32, x: *mut i32, y: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetImagePosition)(self, coordType, x, y)
    }


    /// ```text
    /// /**
    ///    * Returns the size of the image.
    ///    *
    ///    * @param width      the heigth
    ///    * @param height     the width
    ///    */
    /// ```
    ///

    /// `void getImageSize (out long width, out long height);`
    #[inline]
    pub unsafe fn GetImageSize(&self, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetImageSize)(self, width, height)
    }


}


