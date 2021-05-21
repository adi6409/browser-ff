//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIDialogParamBlock.idl
//


/// `interface nsIDialogParamBlock : nsISupports`
///

/// ```text
/// /**
///  * An interface to pass strings, integers and nsISupports to a dialog
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDialogParamBlock {
    vtable: *const nsIDialogParamBlockVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDialogParamBlock.
unsafe impl XpCom for nsIDialogParamBlock {
    const IID: nsIID = nsID(0xf76c0901, 0x437a, 0x11d3,
        [0xb7, 0xa0, 0xe3, 0x5d, 0xb3, 0x51, 0xb4, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDialogParamBlock {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDialogParamBlock.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDialogParamBlockCoerce {
    /// Cheaply cast a value of this type from a `nsIDialogParamBlock`.
    fn coerce_from(v: &nsIDialogParamBlock) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDialogParamBlockCoerce for nsIDialogParamBlock {
    #[inline]
    fn coerce_from(v: &nsIDialogParamBlock) -> &Self {
        v
    }
}

impl nsIDialogParamBlock {
    /// Cast this `nsIDialogParamBlock` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDialogParamBlockCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDialogParamBlock {
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
impl<T: nsISupportsCoerce> nsIDialogParamBlockCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDialogParamBlock) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDialogParamBlock
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDialogParamBlockVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* int32_t GetInt (in int32_t inIndex); */
    pub GetInt: unsafe extern "system" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, _retval: *mut int32_t) -> ::nserror::nsresult,

    /* void SetInt (in int32_t inIndex, in int32_t inInt); */
    pub SetInt: unsafe extern "system" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, inInt: int32_t) -> ::nserror::nsresult,

    /* void SetNumberStrings (in int32_t inNumStrings); */
    pub SetNumberStrings: unsafe extern "system" fn (this: *const nsIDialogParamBlock, inNumStrings: int32_t) -> ::nserror::nsresult,

    /* wstring GetString (in int32_t inIndex); */
    pub GetString: unsafe extern "system" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, _retval: *mut *const i16) -> ::nserror::nsresult,

    /* void SetString (in int32_t inIndex, in wstring inString); */
    pub SetString: unsafe extern "system" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, inString: *const i16) -> ::nserror::nsresult,

    /* attribute nsIMutableArray objects; */
    pub GetObjects: unsafe extern "system" fn (this: *const nsIDialogParamBlock, aObjects: *mut*const nsIMutableArray) -> ::nserror::nsresult,

    /* attribute nsIMutableArray objects; */
    pub SetObjects: unsafe extern "system" fn (this: *const nsIDialogParamBlock, aObjects: *const nsIMutableArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDialogParamBlock {

    /// ```text
    /// /** Get or set an integer to pass.
    ///    * Index must be in the range 0..7
    ///    */
    /// ```
    ///

    /// `int32_t GetInt (in int32_t inIndex);`
    #[inline]
    pub unsafe fn GetInt(&self, inIndex: int32_t, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetInt)(self, inIndex, _retval)
    }



    /// `void SetInt (in int32_t inIndex, in int32_t inInt);`
    #[inline]
    pub unsafe fn SetInt(&self, inIndex: int32_t, inInt: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetInt)(self, inIndex, inInt)
    }


    /// ```text
    /// /** Set the maximum number of strings to pass. Default is 16.
    ///    * Use before setting any string (If you want to change it from the default).
    ///    */
    /// ```
    ///

    /// `void SetNumberStrings (in int32_t inNumStrings);`
    #[inline]
    pub unsafe fn SetNumberStrings(&self, inNumStrings: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetNumberStrings)(self, inNumStrings)
    }


    /// ```text
    /// /** Get or set an string to pass.
    ///     * Index starts at 0
    ///     */
    /// ```
    ///

    /// `wstring GetString (in int32_t inIndex);`
    #[inline]
    pub unsafe fn GetString(&self, inIndex: int32_t, _retval: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).GetString)(self, inIndex, _retval)
    }



    /// `void SetString (in int32_t inIndex, in wstring inString);`
    #[inline]
    pub unsafe fn SetString(&self, inIndex: int32_t, inString: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).SetString)(self, inIndex, inString)
    }


    /// ```text
    /// /**
    ///    * A place where you can store an nsIMutableArray to pass nsISupports
    ///    */
    /// ```
    ///

    /// `attribute nsIMutableArray objects;`
    #[inline]
    pub unsafe fn GetObjects(&self, aObjects: *mut*const nsIMutableArray) -> ::nserror::nsresult {
        ((*self.vtable).GetObjects)(self, aObjects)
    }


    /// ```text
    /// /**
    ///    * A place where you can store an nsIMutableArray to pass nsISupports
    ///    */
    /// ```
    ///

    /// `attribute nsIMutableArray objects;`
    #[inline]
    pub unsafe fn SetObjects(&self, aObjects: *const nsIMutableArray) -> ::nserror::nsresult {
        ((*self.vtable).SetObjects)(self, aObjects)
    }


}


