//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertTree.idl
//


/// `interface nsICertTreeItem : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertTreeItem {
    vtable: *const nsICertTreeItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertTreeItem.
unsafe impl XpCom for nsICertTreeItem {
    const IID: nsIID = nsID(0xd0180863, 0x606e, 0x49e6,
        [0x83, 0x24, 0xcf, 0x45, 0xed, 0x4d, 0xd8, 0x91]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertTreeItem {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertTreeItem.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertTreeItemCoerce {
    /// Cheaply cast a value of this type from a `nsICertTreeItem`.
    fn coerce_from(v: &nsICertTreeItem) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertTreeItemCoerce for nsICertTreeItem {
    #[inline]
    fn coerce_from(v: &nsICertTreeItem) -> &Self {
        v
    }
}

impl nsICertTreeItem {
    /// Cast this `nsICertTreeItem` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertTreeItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertTreeItem {
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
impl<T: nsISupportsCoerce> nsICertTreeItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertTreeItem) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertTreeItem
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertTreeItemVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute nsIX509Cert cert; */
    pub GetCert: unsafe extern "system" fn (this: *const nsICertTreeItem, aCert: *mut*const nsIX509Cert) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertTreeItem {


    /// `[must_use] readonly attribute nsIX509Cert cert;`
    #[inline]
    pub unsafe fn GetCert(&self, aCert: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).GetCert)(self, aCert)
    }


}


/// `interface nsICertTree : nsITreeView`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICertTree {
    vtable: *const nsICertTreeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICertTree.
unsafe impl XpCom for nsICertTree {
    const IID: nsIID = nsID(0x55d5ad6b, 0x5572, 0x47fe,
        [0x94, 0x1c, 0xf0, 0x1f, 0xe7, 0x23, 0x65, 0x9e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICertTree {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICertTree.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICertTreeCoerce {
    /// Cheaply cast a value of this type from a `nsICertTree`.
    fn coerce_from(v: &nsICertTree) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICertTreeCoerce for nsICertTree {
    #[inline]
    fn coerce_from(v: &nsICertTree) -> &Self {
        v
    }
}

impl nsICertTree {
    /// Cast this `nsICertTree` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICertTreeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICertTree {
    type Target = nsITreeView;
    #[inline]
    fn deref(&self) -> &nsITreeView {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsITreeViewCoerce> nsICertTreeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertTree) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICertTree
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICertTreeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsITreeViewVTable,

    /* [must_use] void loadCertsFromCache (in Array<nsIX509Cert> cache, in unsigned long type); */
    pub LoadCertsFromCache: unsafe extern "system" fn (this: *const nsICertTree, cache: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, type_: u32) -> ::nserror::nsresult,

    /* [must_use] nsIX509Cert getCert (in unsigned long index); */
    pub GetCert: unsafe extern "system" fn (this: *const nsICertTree, index: u32, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult,

    /* [must_use] nsICertTreeItem getTreeItem (in unsigned long index); */
    pub GetTreeItem: unsafe extern "system" fn (this: *const nsICertTree, index: u32, _retval: *mut *const nsICertTreeItem) -> ::nserror::nsresult,

    /* [must_use] void deleteEntryObject (in unsigned long index); */
    pub DeleteEntryObject: unsafe extern "system" fn (this: *const nsICertTree, index: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICertTree {


    /// `[must_use] void loadCertsFromCache (in Array<nsIX509Cert> cache, in unsigned long type);`
    #[inline]
    pub unsafe fn LoadCertsFromCache(&self, cache: *const thin_vec::ThinVec<RefPtr<nsIX509Cert>>, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).LoadCertsFromCache)(self, cache, type_)
    }



    /// `[must_use] nsIX509Cert getCert (in unsigned long index);`
    #[inline]
    pub unsafe fn GetCert(&self, index: u32, _retval: *mut*const nsIX509Cert) -> ::nserror::nsresult {
        ((*self.vtable).GetCert)(self, index, _retval)
    }



    /// `[must_use] nsICertTreeItem getTreeItem (in unsigned long index);`
    #[inline]
    pub unsafe fn GetTreeItem(&self, index: u32, _retval: *mut *const nsICertTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetTreeItem)(self, index, _retval)
    }



    /// `[must_use] void deleteEntryObject (in unsigned long index);`
    #[inline]
    pub unsafe fn DeleteEntryObject(&self, index: u32) -> ::nserror::nsresult {
        ((*self.vtable).DeleteEntryObject)(self, index)
    }


}


