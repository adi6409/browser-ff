//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadGroupChild.idl
//


/// `interface nsILoadGroupChild : nsISupports`
///

/// ```text
/// /**
///  * nsILoadGroupChild provides a hierarchy of load groups so that the
///  * root load group can be used to conceptually tie a series of loading
///  * operations into a logical whole while still leaving them separate
///  * for the purposes of cancellation and status events.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadGroupChild {
    vtable: *const nsILoadGroupChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadGroupChild.
unsafe impl XpCom for nsILoadGroupChild {
    const IID: nsIID = nsID(0x02efe8e2, 0xfbbc, 0x4718,
        [0xa2, 0x99, 0xb8, 0xa0, 0x9c, 0x60, 0xbf, 0x6b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadGroupChild {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadGroupChild.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadGroupChildCoerce {
    /// Cheaply cast a value of this type from a `nsILoadGroupChild`.
    fn coerce_from(v: &nsILoadGroupChild) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadGroupChildCoerce for nsILoadGroupChild {
    #[inline]
    fn coerce_from(v: &nsILoadGroupChild) -> &Self {
        v
    }
}

impl nsILoadGroupChild {
    /// Cast this `nsILoadGroupChild` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadGroupChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadGroupChild {
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
impl<T: nsISupportsCoerce> nsILoadGroupChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadGroupChild) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadGroupChild
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadGroupChildVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsILoadGroup parentLoadGroup; */
    pub GetParentLoadGroup: unsafe extern "system" fn (this: *const nsILoadGroupChild, aParentLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,

    /* attribute nsILoadGroup parentLoadGroup; */
    pub SetParentLoadGroup: unsafe extern "system" fn (this: *const nsILoadGroupChild, aParentLoadGroup: *const nsILoadGroup) -> ::nserror::nsresult,

    /* readonly attribute nsILoadGroup childLoadGroup; */
    pub GetChildLoadGroup: unsafe extern "system" fn (this: *const nsILoadGroupChild, aChildLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,

    /* readonly attribute nsILoadGroup rootLoadGroup; */
    pub GetRootLoadGroup: unsafe extern "system" fn (this: *const nsILoadGroupChild, aRootLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadGroupChild {

    /// ```text
    /// /**
    ///      * The parent of this load group. It is stored with
    ///      * a nsIWeakReference/nsWeakPtr so there is no requirement for the
    ///      * parentLoadGroup to out live the child, nor will the child keep a
    ///      * reference count on the parent.
    ///      */
    /// ```
    ///

    /// `attribute nsILoadGroup parentLoadGroup;`
    #[inline]
    pub unsafe fn GetParentLoadGroup(&self, aParentLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetParentLoadGroup)(self, aParentLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The parent of this load group. It is stored with
    ///      * a nsIWeakReference/nsWeakPtr so there is no requirement for the
    ///      * parentLoadGroup to out live the child, nor will the child keep a
    ///      * reference count on the parent.
    ///      */
    /// ```
    ///

    /// `attribute nsILoadGroup parentLoadGroup;`
    #[inline]
    pub unsafe fn SetParentLoadGroup(&self, aParentLoadGroup: *const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).SetParentLoadGroup)(self, aParentLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The nsILoadGroup associated with this nsILoadGroupChild
    ///      */
    /// ```
    ///

    /// `readonly attribute nsILoadGroup childLoadGroup;`
    #[inline]
    pub unsafe fn GetChildLoadGroup(&self, aChildLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetChildLoadGroup)(self, aChildLoadGroup)
    }


    /// ```text
    /// /**
    ///      * The rootLoadGroup is the recursive parent of this
    ///      * load group where parent is defined as parentlLoadGroup if set
    ///      * or childLoadGroup.loadGroup as a backup. (i.e. parentLoadGroup takes
        ///      * precedence.) The nsILoadGroup child is the root if neither parent
    ///      * nor loadgroup attribute is specified.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsILoadGroup rootLoadGroup;`
    #[inline]
    pub unsafe fn GetRootLoadGroup(&self, aRootLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetRootLoadGroup)(self, aRootLoadGroup)
    }


}


