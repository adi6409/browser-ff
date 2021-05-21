//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsICategoryManager.idl
//


/// `interface nsICategoryEntry : nsISupportsCString`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICategoryEntry {
    vtable: *const nsICategoryEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICategoryEntry.
unsafe impl XpCom for nsICategoryEntry {
    const IID: nsIID = nsID(0xde021d54, 0x57a3, 0x4025,
        [0xae, 0x63, 0x4c, 0x8e, 0xed, 0xbe, 0x74, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICategoryEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICategoryEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICategoryEntryCoerce {
    /// Cheaply cast a value of this type from a `nsICategoryEntry`.
    fn coerce_from(v: &nsICategoryEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICategoryEntryCoerce for nsICategoryEntry {
    #[inline]
    fn coerce_from(v: &nsICategoryEntry) -> &Self {
        v
    }
}

impl nsICategoryEntry {
    /// Cast this `nsICategoryEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICategoryEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICategoryEntry {
    type Target = nsISupportsCString;
    #[inline]
    fn deref(&self) -> &nsISupportsCString {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCStringCoerce> nsICategoryEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICategoryEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICategoryEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICategoryEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsCStringVTable,

    /* readonly attribute ACString entry; */
    pub GetEntry: unsafe extern "system" fn (this: *const nsICategoryEntry, aEntry: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsICategoryEntry, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICategoryEntry {


    /// `readonly attribute ACString entry;`
    #[inline]
    pub unsafe fn GetEntry(&self, aEntry: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetEntry)(self, aEntry)
    }



    /// `readonly attribute ACString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


}


/// `interface nsICategoryManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICategoryManager {
    vtable: *const nsICategoryManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICategoryManager.
unsafe impl XpCom for nsICategoryManager {
    const IID: nsIID = nsID(0x3275b2cd, 0xaf6d, 0x429a,
        [0x80, 0xd7, 0xf0, 0xc5, 0x12, 0x03, 0x42, 0xac]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICategoryManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICategoryManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICategoryManagerCoerce {
    /// Cheaply cast a value of this type from a `nsICategoryManager`.
    fn coerce_from(v: &nsICategoryManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICategoryManagerCoerce for nsICategoryManager {
    #[inline]
    fn coerce_from(v: &nsICategoryManager) -> &Self {
        v
    }
}

impl nsICategoryManager {
    /// Cast this `nsICategoryManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICategoryManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICategoryManager {
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
impl<T: nsISupportsCoerce> nsICategoryManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICategoryManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICategoryManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICategoryManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getCategoryEntry (in ACString aCategory, in ACString aEntry); */
    pub GetCategoryEntry: unsafe extern "system" fn (this: *const nsICategoryManager, aCategory: *const ::nsstring::nsACString, aEntry: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString addCategoryEntry (in ACString aCategory, in ACString aEntry, in ACString aValue, in boolean aPersist, in boolean aReplace); */
    pub AddCategoryEntry: unsafe extern "system" fn (this: *const nsICategoryManager, aCategory: *const ::nsstring::nsACString, aEntry: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString, aPersist: bool, aReplace: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void deleteCategoryEntry (in ACString aCategory, in ACString aEntry, in boolean aPersist); */
    pub DeleteCategoryEntry: unsafe extern "system" fn (this: *const nsICategoryManager, aCategory: *const ::nsstring::nsACString, aEntry: *const ::nsstring::nsACString, aPersist: bool) -> ::nserror::nsresult,

    /* void deleteCategory (in ACString aCategory); */
    pub DeleteCategory: unsafe extern "system" fn (this: *const nsICategoryManager, aCategory: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsISimpleEnumerator enumerateCategory (in ACString aCategory); */
    pub EnumerateCategory: unsafe extern "system" fn (this: *const nsICategoryManager, aCategory: *const ::nsstring::nsACString, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* nsISimpleEnumerator enumerateCategories (); */
    pub EnumerateCategories: unsafe extern "system" fn (this: *const nsICategoryManager, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICategoryManager {

    /// ```text
    /// /**
    ///      * Get the value for the given category's entry.
    ///      * @param aCategory The name of the category ("protocol")
    ///      * @param aEntry The entry you're looking for ("http")
    ///      * @return The value.
    ///      */
    /// ```
    ///

    /// `ACString getCategoryEntry (in ACString aCategory, in ACString aEntry);`
    #[inline]
    pub unsafe fn GetCategoryEntry(&self, aCategory: *const ::nsstring::nsACString, aEntry: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCategoryEntry)(self, aCategory, aEntry, _retval)
    }


    /// ```text
    /// /**
    ///      * Add an entry to a category.
    ///      * @param aCategory The name of the category ("protocol")
    ///      * @param aEntry The entry to be added ("http")
    ///      * @param aValue The value for the entry ("moz.httprulez.1")
    ///      * @param aPersist Should this data persist between invocations?
    ///      * @param aReplace Should we replace an existing entry?
    ///      * @return Previous entry, if any
    ///      */
    /// ```
    ///

    /// `ACString addCategoryEntry (in ACString aCategory, in ACString aEntry, in ACString aValue, in boolean aPersist, in boolean aReplace);`
    #[inline]
    pub unsafe fn AddCategoryEntry(&self, aCategory: *const ::nsstring::nsACString, aEntry: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString, aPersist: bool, aReplace: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AddCategoryEntry)(self, aCategory, aEntry, aValue, aPersist, aReplace, _retval)
    }


    /// ```text
    /// /**
    ///      * Delete an entry from the category.
    ///      * @param aCategory The name of the category ("protocol")
    ///      * @param aEntry The entry to be added ("http")
    ///      * @param aPersist Delete persistent data from registry, if present?
    ///      */
    /// ```
    ///

    /// `void deleteCategoryEntry (in ACString aCategory, in ACString aEntry, in boolean aPersist);`
    #[inline]
    pub unsafe fn DeleteCategoryEntry(&self, aCategory: *const ::nsstring::nsACString, aEntry: *const ::nsstring::nsACString, aPersist: bool) -> ::nserror::nsresult {
        ((*self.vtable).DeleteCategoryEntry)(self, aCategory, aEntry, aPersist)
    }


    /// ```text
    /// /**
    ///      * Delete a category and all entries.
    ///      * @param aCategory The category to be deleted.
    ///      */
    /// ```
    ///

    /// `void deleteCategory (in ACString aCategory);`
    #[inline]
    pub unsafe fn DeleteCategory(&self, aCategory: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).DeleteCategory)(self, aCategory)
    }


    /// ```text
    /// /**
    ///      * Enumerate the entries in a category.
    ///      * @param aCategory The category to be enumerated.
    ///      * @return a simple enumerator, each result QIs to
    ///      *         nsICategoryEntry.
    ///      */
    /// ```
    ///

    /// `nsISimpleEnumerator enumerateCategory (in ACString aCategory);`
    #[inline]
    pub unsafe fn EnumerateCategory(&self, aCategory: *const ::nsstring::nsACString, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateCategory)(self, aCategory, _retval)
    }


    /// ```text
    /// /**
    ///      * Enumerate all existing categories
    ///      * @param aCategory The category to be enumerated.
    ///      * @return a simple enumerator, each result QIs to
    ///      *         nsISupportsCString.
    ///      */
    /// ```
    ///

    /// `nsISimpleEnumerator enumerateCategories ();`
    #[inline]
    pub unsafe fn EnumerateCategories(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateCategories)(self, _retval)
    }


}


