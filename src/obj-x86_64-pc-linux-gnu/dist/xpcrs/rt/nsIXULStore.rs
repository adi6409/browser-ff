//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/xulstore/nsIXULStore.idl
//


/// `interface nsIXULStore : nsISupports`
///

/// ```text
/// /**
///  * The XUL store is used to store information related to a XUL document/application.
///  * Typically it is used to store the persisted state for the document, such as
///  * window location, toolbars that are open and nodes that are open and closed in a tree.
///  *
///  * If MOZ_NEW_XULSTORE is enabled:
///  * XULStore.jsm wraps this API in useful abstractions for JS consumers.
///  * XULStore.h provides a more idiomatic API for C++ consumers.
///  * You should use those APIs unless you have good reasons to use this one.
///  *
///  * If MOZ_NEW_XULSTORE is disabled:
///  * The data is serialized to [profile directory]/xulstore.json
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXULStore {
    vtable: *const nsIXULStoreVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXULStore.
unsafe impl XpCom for nsIXULStore {
    const IID: nsIID = nsID(0x987c4b35, 0xc426, 0x4dd7,
        [0xad, 0x49, 0x3c, 0x9f, 0xa4, 0xc6, 0x5d, 0x20]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXULStore {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXULStore.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXULStoreCoerce {
    /// Cheaply cast a value of this type from a `nsIXULStore`.
    fn coerce_from(v: &nsIXULStore) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXULStoreCoerce for nsIXULStore {
    #[inline]
    fn coerce_from(v: &nsIXULStore) -> &Self {
        v
    }
}

impl nsIXULStore {
    /// Cast this `nsIXULStore` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXULStoreCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXULStore {
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
impl<T: nsISupportsCoerce> nsIXULStoreCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULStore) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXULStore
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXULStoreVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void persist (in Node aNode, in AString attr); */
    pub Persist: unsafe extern "system" fn (this: *const nsIXULStore, aNode: *const libc::c_void, attr: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setValue (in AString doc, in AString id, in AString attr, in AString value); */
    pub SetValue: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* bool hasValue (in AString doc, in AString id, in AString attr); */
    pub HasValue: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getValue (in AString doc, in AString id, in AString attr); */
    pub GetValue: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeValue (in AString doc, in AString id, in AString attr); */
    pub RemoveValue: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeDocument (in AString doc); */
    pub RemoveDocument: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIStringEnumerator getIDsEnumerator (in AString doc); */
    pub GetIDsEnumerator: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString, _retval: *mut*const nsIStringEnumerator) -> ::nserror::nsresult,

    /* nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id); */
    pub GetAttributeEnumerator: unsafe extern "system" fn (this: *const nsIXULStore, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, _retval: *mut*const nsIStringEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXULStore {

    /// ```text
    /// /**
    ///    * Sets a value for a specified node's attribute, except in
    ///    * the case below:
    ///    * If the value is empty and if calling `hasValue` with the node's
    ///    * document and ID and `attr` would return true, then the
    ///    * value instead gets removed from the store (see Bug 1476680).
    ///    *
    ///    * @param node - DOM node
    ///    * @param attr - attribute to store
    ///    */
    /// ```
    ///

    /// `void persist (in Node aNode, in AString attr);`
    #[inline]
    pub unsafe fn Persist(&self, aNode: *const libc::c_void, attr: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Persist)(self, aNode, attr)
    }


    /// ```text
    /// /**
    ///    * Sets a value in the store.
    ///    *
    ///    * @param doc - document URI
    ///    * @param id - identifier of the node
    ///    * @param attr - attribute to store
    ///    * @param value - value of the attribute
    ///    */
    /// ```
    ///

    /// `void setValue (in AString doc, in AString id, in AString attr, in AString value);`
    #[inline]
    pub unsafe fn SetValue(&self, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, doc, id, attr, value)
    }


    /// ```text
    /// /**
    ///    * Returns true if the store contains a value for attr.
    ///    *
    ///    * @param doc - URI of the document
    ///    * @param id - identifier of the node
    ///    * @param attr - attribute
    ///    */
    /// ```
    ///

    /// `bool hasValue (in AString doc, in AString id, in AString attr);`
    #[inline]
    pub unsafe fn HasValue(&self, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasValue)(self, doc, id, attr, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieves a value in the store, or an empty string if it does not exist.
    ///    *
    ///    * @param doc - document URI
    ///    * @param id - identifier of the node
    ///    * @param attr - attribute to retrieve
    ///    *
    ///    * @returns the value of the attribute
    ///    */
    /// ```
    ///

    /// `AString getValue (in AString doc, in AString id, in AString attr);`
    #[inline]
    pub unsafe fn GetValue(&self, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, doc, id, attr, _retval)
    }


    /// ```text
    /// /**
    ///    * Removes a value in the store.
    ///    *
    ///    * @param doc - document URI
    ///    * @param id - identifier of the node
    ///    * @param attr - attribute to remove
    ///    */
    /// ```
    ///

    /// `void removeValue (in AString doc, in AString id, in AString attr);`
    #[inline]
    pub unsafe fn RemoveValue(&self, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, attr: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveValue)(self, doc, id, attr)
    }


    /// ```text
    /// /**
    ///    * Removes all values related to the given document.
    ///    *
    ///    * @param doc - document URI
    ///    */
    /// ```
    ///

    /// `void removeDocument (in AString doc);`
    #[inline]
    pub unsafe fn RemoveDocument(&self, doc: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDocument)(self, doc)
    }


    /// ```text
    /// /**
    ///    * Iterates over all of the ids associated with a given document uri that
    ///    * have stored data.
    ///    *
    ///    * @param doc - document URI
    ///    */
    /// ```
    ///

    /// `nsIStringEnumerator getIDsEnumerator (in AString doc);`
    #[inline]
    pub unsafe fn GetIDsEnumerator(&self, doc: *const ::nsstring::nsAString, _retval: *mut*const nsIStringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetIDsEnumerator)(self, doc, _retval)
    }


    /// ```text
    /// /**
    ///    * Iterates over all of the attributes associated with a given document uri
    ///    * and id that have stored data.
    ///    *
    ///    * @param doc - document URI
    ///    * @param id - identifier of the node
    ///    */
    /// ```
    ///

    /// `nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id);`
    #[inline]
    pub unsafe fn GetAttributeEnumerator(&self, doc: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, _retval: *mut*const nsIStringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetAttributeEnumerator)(self, doc, id, _retval)
    }


}


