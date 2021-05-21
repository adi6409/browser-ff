//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIQueryContentEventResult.idl
//


/// `interface nsIQueryContentEventResult : nsISupports`
///

/// ```text
/// /**
///  * The result of query content events.  succeeded propery can be used always.
///  * Whether other properties can be used or not depends on the event.
///  * See nsIDOMWindowUtils.idl, which properites can be used was documented.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQueryContentEventResult {
    vtable: *const nsIQueryContentEventResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQueryContentEventResult.
unsafe impl XpCom for nsIQueryContentEventResult {
    const IID: nsIID = nsID(0xe2c39e0e, 0x345f, 0x451a,
        [0xa7, 0xb2, 0xe0, 0x23, 0x0d, 0x55, 0x58, 0x47]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQueryContentEventResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQueryContentEventResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQueryContentEventResultCoerce {
    /// Cheaply cast a value of this type from a `nsIQueryContentEventResult`.
    fn coerce_from(v: &nsIQueryContentEventResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQueryContentEventResultCoerce for nsIQueryContentEventResult {
    #[inline]
    fn coerce_from(v: &nsIQueryContentEventResult) -> &Self {
        v
    }
}

impl nsIQueryContentEventResult {
    /// Cast this `nsIQueryContentEventResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQueryContentEventResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQueryContentEventResult {
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
impl<T: nsISupportsCoerce> nsIQueryContentEventResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQueryContentEventResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQueryContentEventResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQueryContentEventResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long offset; */
    pub GetOffset: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aOffset: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long tentativeCaretOffset; */
    pub GetTentativeCaretOffset: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aTentativeCaretOffset: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute boolean reversed; */
    pub GetReversed: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aReversed: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long left; */
    pub GetLeft: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aLeft: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long top; */
    pub GetTop: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aTop: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long width; */
    pub GetWidth: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aWidth: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long height; */
    pub GetHeight: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aHeight: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AString text; */
    pub GetText: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void getCharacterRect (in long offset, out long left, out long top, out long width, out long height); */
    pub GetCharacterRect: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, offset: i32, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute boolean succeeded; */
    pub GetSucceeded: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aSucceeded: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean notFound; */
    pub GetNotFound: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aNotFound: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean tentativeCaretOffsetNotFound; */
    pub GetTentativeCaretOffsetNotFound: unsafe extern "system" fn (this: *const nsIQueryContentEventResult, aTentativeCaretOffsetNotFound: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQueryContentEventResult {


    /// `readonly attribute unsigned long offset;`
    #[inline]
    pub unsafe fn GetOffset(&self, aOffset: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetOffset)(self, aOffset)
    }



    /// `readonly attribute unsigned long tentativeCaretOffset;`
    #[inline]
    pub unsafe fn GetTentativeCaretOffset(&self, aTentativeCaretOffset: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTentativeCaretOffset)(self, aTentativeCaretOffset)
    }



    /// `readonly attribute boolean reversed;`
    #[inline]
    pub unsafe fn GetReversed(&self, aReversed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetReversed)(self, aReversed)
    }



    /// `readonly attribute long left;`
    #[inline]
    pub unsafe fn GetLeft(&self, aLeft: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLeft)(self, aLeft)
    }



    /// `readonly attribute long top;`
    #[inline]
    pub unsafe fn GetTop(&self, aTop: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetTop)(self, aTop)
    }



    /// `readonly attribute long width;`
    #[inline]
    pub unsafe fn GetWidth(&self, aWidth: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetWidth)(self, aWidth)
    }



    /// `readonly attribute long height;`
    #[inline]
    pub unsafe fn GetHeight(&self, aHeight: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetHeight)(self, aHeight)
    }



    /// `readonly attribute AString text;`
    #[inline]
    pub unsafe fn GetText(&self, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetText)(self, aText)
    }



    /// `void getCharacterRect (in long offset, out long left, out long top, out long width, out long height);`
    #[inline]
    pub unsafe fn GetCharacterRect(&self, offset: i32, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCharacterRect)(self, offset, left, top, width, height)
    }



    /// `readonly attribute boolean succeeded;`
    #[inline]
    pub unsafe fn GetSucceeded(&self, aSucceeded: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSucceeded)(self, aSucceeded)
    }



    /// `readonly attribute boolean notFound;`
    #[inline]
    pub unsafe fn GetNotFound(&self, aNotFound: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNotFound)(self, aNotFound)
    }



    /// `readonly attribute boolean tentativeCaretOffsetNotFound;`
    #[inline]
    pub unsafe fn GetTentativeCaretOffsetNotFound(&self, aTentativeCaretOffsetNotFound: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetTentativeCaretOffsetNotFound)(self, aTentativeCaretOffsetNotFound)
    }


}


