//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionDisplay.idl
//


/// `interface nsISelectionDisplay : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISelectionDisplay {
    vtable: *const nsISelectionDisplayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISelectionDisplay.
unsafe impl XpCom for nsISelectionDisplay {
    const IID: nsIID = nsID(0x0ddf9e1c, 0x1dd2, 0x11b2,
        [0xa1, 0x83, 0x90, 0x8a, 0x08, 0xaa, 0x75, 0xae]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISelectionDisplay {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISelectionDisplay.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISelectionDisplayCoerce {
    /// Cheaply cast a value of this type from a `nsISelectionDisplay`.
    fn coerce_from(v: &nsISelectionDisplay) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISelectionDisplayCoerce for nsISelectionDisplay {
    #[inline]
    fn coerce_from(v: &nsISelectionDisplay) -> &Self {
        v
    }
}

impl nsISelectionDisplay {
    /// Cast this `nsISelectionDisplay` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISelectionDisplayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISelectionDisplay {
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
impl<T: nsISupportsCoerce> nsISelectionDisplayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionDisplay) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISelectionDisplay
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISelectionDisplayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setSelectionFlags (in short toggle); */
    pub SetSelectionFlags: unsafe extern "system" fn (this: *const nsISelectionDisplay, toggle: i16) -> ::nserror::nsresult,

    /* short getSelectionFlags (); */
    pub GetSelectionFlags: unsafe extern "system" fn (this: *const nsISelectionDisplay, _retval: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISelectionDisplay {

    pub const DISPLAY_TEXT: i64 = 1;


    pub const DISPLAY_IMAGES: i64 = 2;


    pub const DISPLAY_FRAMES: i64 = 4;


    pub const DISPLAY_ALL: i64 = 7;


    /// `void setSelectionFlags (in short toggle);`
    #[inline]
    pub unsafe fn SetSelectionFlags(&self, toggle: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectionFlags)(self, toggle)
    }



    /// `short getSelectionFlags ();`
    #[inline]
    pub unsafe fn GetSelectionFlags(&self, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionFlags)(self, _retval)
    }


}


