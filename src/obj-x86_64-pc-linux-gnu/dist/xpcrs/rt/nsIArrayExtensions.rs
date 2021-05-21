//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIArrayExtensions.idl
//


/// `interface nsIArrayExtensions : nsIArray`
///

/// ```text
/// /**
///  * Helper interface for allowing scripts to treat nsIArray instances as if
///  * they were nsISupportsArray instances while iterating.
///  *
///  * nsISupportsArray is convenient to iterate over in JavaScript:
///  *
///  * for (let i = 0; i < array.Count(); ++i) {
    ///  *   let elem = array.GetElementAt(i);
    ///  *   ...
    ///  * }
///  *
///  * but doing the same with nsIArray is somewhat less convenient, since
///  * queryElementAt is not nearly so nice to use from JavaScript.  So we provide
///  * this extension interface so interfaces that currently return
///  * nsISupportsArray can start returning nsIArrayExtensions and all JavaScript
///  * should Just Work. Eventually we'll roll this interface into nsIArray
///  * itself, possibly getting rid of the Count() method, as it duplicates
///  * nsIArray functionality.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIArrayExtensions {
    vtable: *const nsIArrayExtensionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIArrayExtensions.
unsafe impl XpCom for nsIArrayExtensions {
    const IID: nsIID = nsID(0x261d442e, 0x050c, 0x453d,
        [0x8a, 0xaa, 0xb3, 0xf2, 0x3b, 0xcc, 0x52, 0x8b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIArrayExtensions {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIArrayExtensions.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIArrayExtensionsCoerce {
    /// Cheaply cast a value of this type from a `nsIArrayExtensions`.
    fn coerce_from(v: &nsIArrayExtensions) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIArrayExtensionsCoerce for nsIArrayExtensions {
    #[inline]
    fn coerce_from(v: &nsIArrayExtensions) -> &Self {
        v
    }
}

impl nsIArrayExtensions {
    /// Cast this `nsIArrayExtensions` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIArrayExtensionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIArrayExtensions {
    type Target = nsIArray;
    #[inline]
    fn deref(&self) -> &nsIArray {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIArrayCoerce> nsIArrayExtensionsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIArrayExtensions) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIArrayExtensions
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIArrayExtensionsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIArrayVTable,

    /* uint32_t Count (); */
    pub Count: unsafe extern "system" fn (this: *const nsIArrayExtensions, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* nsISupports GetElementAt (in uint32_t index); */
    pub GetElementAt: unsafe extern "system" fn (this: *const nsIArrayExtensions, index: uint32_t, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIArrayExtensions {

    /// ```text
    /// /**
    ///    * Count()
    ///    *
    ///    * Retrieves the length of the array. This is an alias for the
    ///    * |nsIArray.length| attribute.
    ///    */
    /// ```
    ///

    /// `uint32_t Count ();`
    #[inline]
    pub unsafe fn Count(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Count)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * GetElementAt()
    ///    *
    ///    * Retrieve a specific element of the array. null is a valid result for
    ///    * this method.
    ///    *
    ///    * Note: If the index is out of bounds null will be returned.
    ///    *       This differs from the behavior of nsIArray.queryElementAt() which
    ///    *       will throw if an invalid index is specified.
    ///    *
    ///    * @param index position of element
    ///    */
    /// ```
    ///

    /// `nsISupports GetElementAt (in uint32_t index);`
    #[inline]
    pub unsafe fn GetElementAt(&self, index: uint32_t, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetElementAt)(self, index, _retval)
    }


}


